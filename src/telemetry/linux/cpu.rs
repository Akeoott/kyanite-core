// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::{CpuCoreUsage, CpuTelSnapshot};
use log::{trace, warn};
use vfs::{PhysicalFS, VfsPath};

#[cfg(feature = "test")]
use mock_instant::thread_local::Instant;
#[cfg(not(feature = "test"))]
use std::time::Instant;

const CPUINFO_PATH: &str = "proc/cpuinfo";
const PROCSTAT_PATH: &str = "proc/stat";
const HWMON_ROOT: &str = "sys/class/hwmon";
const POWERCAP_ROOT: &str = "sys/class/powercap";
const RAPL_DIR_PREFIXES: &[&str] = &["intel-rapl", "amd-rapl"];

/// CPU telemetry collector.
pub struct CpuTel {
    /// Virtual file system
    root: VfsPath,

    snapshot: CpuTelSnapshot,

    // /proc/stat delta state
    prev_total_ticks: Vec<i64>,
    prev_core_ticks: Vec<Vec<i64>>,
    first_usage_read: bool,

    // /sys/class/powercap (RAPL) delta state
    energy_path: Option<VfsPath>,
    prev_energy_uj: f64,
    prev_energy_time: Instant,
    rapl_discovered: bool,
}

impl Default for CpuTel {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuTel {
    /// Creates a new CPU telemetry instance backed by the real filesystem.
    pub fn new() -> Self {
        Self::with_root(VfsPath::new(PhysicalFS::new("/")))
    }

    /// Creates a new CPU telemetry instance reading from a custom filesystem root.
    pub fn with_root(root: VfsPath) -> Self {
        Self {
            root,
            snapshot: CpuTelSnapshot::default(),
            prev_total_ticks: Vec::new(),
            prev_core_ticks: Vec::new(),
            first_usage_read: true,
            energy_path: None,
            prev_energy_uj: 0.0,
            prev_energy_time: Instant::now(),
            rapl_discovered: false,
        }
    }

    /// Returns the current snapshot state.
    /// Invalidates after updating.
    pub fn snapshot(&self) -> &CpuTelSnapshot {
        &self.snapshot
    }

    /// Refresh cached snapshot with new data
    pub fn update(&mut self) {
        trace!("Fetching all CPU info...");

        let (cpu_model, core_frequencies) =
            cpu_impl::read_cpu_info(&self.root).unwrap_or_else(|e| {
                warn!("Failed to read {CPUINFO_PATH}: {e}");
                ("Unknown CPU".to_owned(), Vec::new())
            });

        let (cpu_usage, core_usages) = self.poll_usage();
        let (cpu_temperature, core_temperatures) =
            cpu_impl::read_cpu_temps(&self.root, core_usages.len());
        let power_draw = self.poll_power();

        let cpu_frequency = if core_frequencies.is_empty() {
            0.0
        } else {
            let sum: f64 = core_frequencies.iter().map(|c| c.frequency).sum();
            cpu_impl::round_to(sum / core_frequencies.len() as f64, 3)
        };

        self.snapshot = CpuTelSnapshot {
            cpu_model,
            cpu_frequency,
            cpu_usage,
            cpu_temperature,
            power_draw,
            core_frequencies,
            core_usages,
            core_temperatures,
        };
    }

    fn poll_usage(&mut self) -> (i32, Vec<CpuCoreUsage>) {
        let (curr_total, curr_cores) = match cpu_impl::read_current_ticks(&self.root) {
            Ok(v) => v,
            Err(e) => {
                warn!("Failed to read {PROCSTAT_PATH}: {e}");
                return (0, Vec::new());
            }
        };

        if self.first_usage_read {
            self.first_usage_read = false;
            let usages = curr_cores
                .iter()
                .enumerate()
                .map(|(i, _)| CpuCoreUsage {
                    core_index: i as i32,
                    usage: 0.0,
                })
                .collect();
            self.prev_total_ticks = curr_total;
            self.prev_core_ticks = curr_cores;
            return (0, usages);
        }

        let total = cpu_impl::round_to(
            cpu_impl::compute_usage(&curr_total, &self.prev_total_ticks),
            0,
        ) as i32;

        let core_usages = curr_cores
            .iter()
            .enumerate()
            .map(|(i, curr)| CpuCoreUsage {
                core_index: i as i32,
                usage: self
                    .prev_core_ticks
                    .get(i)
                    .map(|prev| cpu_impl::round_to(cpu_impl::compute_usage(curr, prev), 0))
                    .unwrap_or(0.0),
            })
            .collect();

        self.prev_total_ticks = curr_total;
        self.prev_core_ticks = curr_cores;

        (total, core_usages)
    }

    fn poll_power(&mut self) -> f64 {
        if self.energy_path.is_none() && !self.rapl_discovered {
            self.energy_path = cpu_impl::discover_rapl_path(&self.root);
            self.rapl_discovered = self.energy_path.is_none();
        }

        let energy_uj = match self.energy_path.as_ref().and_then(cpu_impl::read_energy_uj) {
            Some(v) => v,
            None => return 0.0,
        };

        let now = Instant::now();
        let mut power = 0.0;

        if self.prev_energy_uj > 0.0 {
            let delta = energy_uj - self.prev_energy_uj;
            let delta_uj = if delta > 0.0 { delta } else { 0.0 };
            let delta_sec = now.duration_since(self.prev_energy_time).as_secs_f64();
            if delta_sec > 0.0 {
                power = delta_uj / 1_000_000.0 / delta_sec;
            }
        }

        self.prev_energy_uj = energy_uj;
        self.prev_energy_time = now;

        cpu_impl::round_to(power, 2)
    }
}
/// Virtual-filesystem readers (procfs / sysfs). All read-only, no state.
mod cpu_impl {
    use crate::telemetry::models::{CpuCoreFrequency, CpuCoreTemperature};
    use log::trace;
    use std::io::{BufRead, BufReader};
    use vfs::{VfsPath, VfsResult};

    use super::{CPUINFO_PATH, HWMON_ROOT, POWERCAP_ROOT, PROCSTAT_PATH, RAPL_DIR_PREFIXES};

    // /proc/cpuinfo
    pub(super) fn read_cpu_info(root: &VfsPath) -> VfsResult<(String, Vec<CpuCoreFrequency>)> {
        let path = root.join(CPUINFO_PATH)?;
        let reader = BufReader::new(path.open_file()?);

        let mut cpu_model = String::from("Unknown CPU");
        let mut model_set = false;
        let mut frequencies = Vec::new();
        let mut core_index: i32 = 0;

        for line in reader.lines() {
            let line = line?;

            if !model_set && line.starts_with("model name") {
                if let Some(colon) = line.find(':') {
                    cpu_model = line[colon + 1..].trim().to_owned();
                    model_set = true;
                }
            } else if line.starts_with("cpu MHz")
                && let Some(colon) = line.find(':')
                && let Ok(frequency) = line[colon + 1..].trim().parse::<f64>()
            {
                frequencies.push(CpuCoreFrequency {
                    core_index,
                    frequency,
                });
                core_index += 1;
            }
        }

        Ok((cpu_model, frequencies))
    }

    // /proc/stat
    pub(super) fn read_current_ticks(root: &VfsPath) -> VfsResult<(Vec<i64>, Vec<Vec<i64>>)> {
        let path = root.join(PROCSTAT_PATH)?;
        let reader = BufReader::new(path.open_file()?);

        let mut total = Vec::new();
        let mut cores = Vec::new();
        let mut first = true;

        for line in reader.lines() {
            let line = line?;
            if !line.starts_with("cpu") {
                continue;
            }

            let ticks: Vec<i64> = line
                .split_ascii_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if first {
                total = ticks;
                first = false;
            } else {
                cores.push(ticks);
            }
        }

        Ok((total, cores))
    }

    pub(super) fn compute_usage(curr: &[i64], prev: &[i64]) -> f64 {
        let len = curr.len().min(prev.len());
        let (curr, prev) = (&curr[..len], &prev[..len]);

        let diff_total: i64 = curr.iter().sum::<i64>() - prev.iter().sum::<i64>();
        if diff_total <= 0 {
            return 0.0;
        }

        let idle = |t: &[i64]| match t.len() {
            5.. => t[3] + t[4],
            4 => t[3],
            _ => 0,
        };
        let diff_idle = idle(curr) - idle(prev);

        (diff_total - diff_idle) as f64 / diff_total as f64 * 100.0
    }

    // /sys/class/hwmon
    pub(super) fn read_cpu_temps(
        root: &VfsPath,
        core_count: usize,
    ) -> (i32, Vec<CpuCoreTemperature>) {
        let mut overall = 0;
        let mut raw: Vec<CpuCoreTemperature> = Vec::new();

        if let Ok(entries) = root.join(HWMON_ROOT).and_then(|p| p.read_dir()) {
            for dir in entries {
                if let Ok(name) = dir.join("name").and_then(|p| p.read_to_string()) {
                    let (dev_overall, dev_temps) = read_hwmon_temps(name.trim(), &dir);
                    if dev_overall != 0 {
                        overall = dev_overall;
                    }
                    raw.extend(dev_temps);
                }
            }
        }

        if !raw.is_empty() && raw.len() == core_count {
            raw.sort_unstable_by_key(|t| t.core_index);
            return (overall, raw);
        }

        let avg = if raw.is_empty() {
            overall as f64
        } else {
            raw.iter().map(|t| t.temperature as f64).sum::<f64>() / raw.len() as f64
        }
        .round() as i32;

        let temps = (0..core_count)
            .map(|i| CpuCoreTemperature {
                core_index: i as i32,
                temperature: avg,
            })
            .collect();

        (overall, temps)
    }

    /// Handles both `coretemp` (Intel) and `k10temp` (AMD) label conventions.
    fn read_hwmon_temps(name: &str, dir: &VfsPath) -> (i32, Vec<CpuCoreTemperature>) {
        let mut overall = 0;
        let mut has_tdie = false;
        let mut temps = Vec::new();

        let Ok(entries) = dir.read_dir() else {
            return (overall, temps);
        };

        for (input, prefix) in sensor_inputs(entries) {
            let Some(temp) = read_millideg(&input) else {
                continue;
            };
            let label = read_trimmed(
                &dir.join(format!("{prefix}_label"))
                    .unwrap_or_else(|_| dir.clone()),
            );
            let label = label.as_deref();

            match name {
                "coretemp" => match label {
                    Some(l) if l.contains("Package") || l == "CPU" => overall = temp,
                    Some(l) if l.starts_with("Core ") => {
                        if let Ok(idx) = l.rsplit(' ').next().unwrap_or("").parse::<i32>() {
                            temps.push(CpuCoreTemperature {
                                core_index: idx,
                                temperature: temp,
                            });
                        }
                    }
                    _ => {}
                },
                "k10temp" => match label {
                    Some(l) if l.contains("Tdie") => {
                        overall = temp;
                        has_tdie = true;
                    }
                    Some(l) if l.contains("Tctl") && !has_tdie => overall = temp,
                    Some(l) if l.starts_with("Tccd") => {
                        if let Ok(idx) = l[4..].parse::<i32>() {
                            temps.push(CpuCoreTemperature {
                                core_index: idx,
                                temperature: temp,
                            });
                        }
                    }
                    None if overall == 0 => overall = temp,
                    _ => {}
                },
                _ => {}
            }
        }

        (overall, temps)
    }

    /// Yields `(input_path, prefix)` for every `temp*_input` file in a hwmon dir.
    fn sensor_inputs(
        entries: Box<dyn Iterator<Item = VfsPath> + Send>,
    ) -> impl Iterator<Item = (VfsPath, String)> {
        entries.filter_map(|entry| {
            let name = entry.filename();
            let prefix = name.strip_suffix("_input")?;
            prefix
                .starts_with("temp")
                .then(|| (entry, prefix.to_owned()))
        })
    }

    fn read_millideg(path: &VfsPath) -> Option<i32> {
        let raw = read_trimmed(path)?;
        let millideg: i64 = raw.parse().ok()?;
        Some(millideg.div_euclid(1000) as i32)
    }

    // /sys/class/powercap (RAPL)
    pub(super) fn discover_rapl_path(root: &VfsPath) -> Option<VfsPath> {
        let entries = root.join(POWERCAP_ROOT).ok()?.read_dir().ok()?;

        let mut top_level: Option<VfsPath> = None;
        let mut sub_zone: Option<VfsPath> = None;

        for dir in entries {
            let dir_name = dir.filename();
            if !RAPL_DIR_PREFIXES.iter().any(|p| dir_name.starts_with(p)) {
                continue;
            }

            let Ok(energy_path) = dir.join("energy_uj") else {
                continue;
            };
            if !energy_path.exists().unwrap_or(false) {
                continue;
            }

            if dir_name.matches(':').count() > 1 {
                sub_zone.get_or_insert(energy_path);
                continue;
            }

            let name = read_trimmed(&dir.join("name").ok()?).unwrap_or_default();
            if name.starts_with("package") {
                trace!("Discovered RAPL power domain: {}", energy_path.as_str());
                return Some(energy_path);
            }

            top_level.get_or_insert(energy_path);
        }

        let chosen = top_level.or(sub_zone);
        if let Some(ref p) = chosen {
            trace!("Using fallback RAPL domain: {}", p.as_str());
        }
        chosen
    }

    pub(super) fn read_energy_uj(path: &VfsPath) -> Option<f64> {
        read_trimmed(path)?.parse().ok()
    }

    // utils
    fn read_trimmed(path: &VfsPath) -> Option<String> {
        Some(path.read_to_string().ok()?.trim().to_owned())
    }

    #[inline]
    pub(super) fn round_to(value: f64, decimals: u32) -> f64 {
        let factor = 10f64.powi(decimals as i32);
        (value * factor).round() / factor
    }
}
