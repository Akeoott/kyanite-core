// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::SystemTelSnapshot;
use log::{trace, warn};
use vfs::{PhysicalFS, VfsPath};

const OSRELEASE_PATH: &str = "proc/sys/kernel/osrelease";
const HOSTNAME_PATH: &str = "proc/sys/kernel/hostname";
const UPTIME_PATH: &str = "proc/uptime";
const LOADAVG_PATH: &str = "proc/loadavg";

/// System telemetry collector.
pub struct SystemTel {
    snapshot: SystemTelSnapshot,

    /// Virtual file system
    root: VfsPath,
}

impl Default for SystemTel {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTel {
    /// Creates a new System telemetry instance backed by the real filesystem.
    pub fn new() -> Self {
        Self::with_root(VfsPath::new(PhysicalFS::new("/")))
    }

    /// Creates a new System telemetry instance reading from a custom filesystem root.
    pub fn with_root(root: VfsPath) -> Self {
        Self {
            snapshot: SystemTelSnapshot::default(),
            root,
        }
    }

    /// Returns the current snapshot state.
    /// Invalidates after updating.
    pub fn snapshot(&self) -> &SystemTelSnapshot {
        &self.snapshot
    }

    /// Refresh cached snapshot with new data
    pub fn update(&mut self) {
        trace!("Fetching all System info...");

        let kernel_version =
            system_impl::read_trimmed(&self.root, OSRELEASE_PATH).unwrap_or_else(|e| {
                warn!("Failed to read {OSRELEASE_PATH}: {e}");
                "Error".to_owned()
            });

        let hostname = system_impl::read_trimmed(&self.root, HOSTNAME_PATH).unwrap_or_else(|e| {
            warn!("Failed to read {HOSTNAME_PATH}: {e}");
            "Error".to_owned()
        });

        let uptime_seconds = system_impl::read_uptime(&self.root).unwrap_or_else(|e| {
            warn!("Failed to read {UPTIME_PATH}: {e}");
            0.0
        });

        let (running_task_count, total_task_count) = system_impl::read_task_counts(&self.root)
            .unwrap_or_else(|e| {
                warn!("Failed to read {LOADAVG_PATH}: {e}");
                (0, 0)
            });

        self.snapshot = SystemTelSnapshot {
            kernel_version,
            hostname,
            uptime_seconds,
            running_task_count,
            total_task_count,
        };
    }
}

/// Virtual-filesystem readers (procfs). All read-only, no state.
mod system_impl {
    use super::{LOADAVG_PATH, UPTIME_PATH};
    use vfs::{VfsPath, VfsResult};

    /// Reads a procfs file and returns its trimmed contents.
    pub(super) fn read_trimmed(root: &VfsPath, path: &str) -> VfsResult<String> {
        Ok(root.join(path)?.read_to_string()?.trim().to_owned())
    }

    /// Parses `/proc/uptime` -> first field (seconds since boot).
    pub(super) fn read_uptime(root: &VfsPath) -> VfsResult<f64> {
        let raw = read_trimmed(root, UPTIME_PATH)?;
        Ok(raw
            .split_ascii_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0))
    }

    /// Parses `/proc/loadavg` -> `(running, total)` from the 4th field.
    pub(super) fn read_task_counts(root: &VfsPath) -> VfsResult<(i32, i32)> {
        let raw = read_trimmed(root, LOADAVG_PATH)?;
        let counts = raw
            .split_ascii_whitespace()
            .nth(3)
            .and_then(|tasks| tasks.split_once('/'))
            .map(|(running, total)| {
                (
                    running.parse::<i32>().unwrap_or(0),
                    total.parse::<i32>().unwrap_or(0),
                )
            })
            .unwrap_or((0, 0));
        Ok(counts)
    }
}
