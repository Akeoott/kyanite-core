// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

//! Integration test for CPU telemetry.

use std::io::Write;
use std::time::Duration;

use kyanite_core::telemetry::CpuTel;
use mock_instant::thread_local::MockClock;
use vfs::{MemoryFS, VfsPath};

#[cfg(test)]
#[cfg(target_os = "linux")]
fn write_file(root: &VfsPath, path: &str, contents: &str) {
    if let Some((parent, _)) = path.rsplit_once('/')
        && !parent.is_empty()
    {
        let dir = root.join(parent).unwrap();
        if !dir.exists().unwrap_or(false) {
            dir.create_dir_all().unwrap();
        }
    }

    let file = root.join(path).unwrap();
    let mut writer = file.create_file().unwrap();
    writer.write_all(contents.as_bytes()).unwrap();
}

#[test]
#[cfg(test)]
#[cfg(target_os = "linux")]
fn cpu_update_twice_with_one_second_delay() {
    // Deterministic virtual clock for this thread.
    MockClock::set_time(Duration::ZERO);

    // Build an in-memory /proc + /sys via MemoryFS.
    let root: VfsPath = MemoryFS::new().into();
    write_file(&root, "proc/cpuinfo", "model name\t: Test CPU\n");
    write_file(&root, "proc/stat", "cpu 100 0 100 800 0 0 0 0\n");
    write_file(&root, "sys/class/powercap/intel-rapl:0/name", "package-0\n");
    write_file(
        &root,
        "sys/class/powercap/intel-rapl:0/energy_uj",
        "5000000\n", // 5 J
    );

    let mut tel = CpuTel::with_root(root.clone());

    tel.update();
    assert_eq!(tel.snapshot().cpu_model, "Test CPU");
    assert_eq!(tel.snapshot().cpu_usage, 0);
    assert_eq!(tel.snapshot().power_draw, 0.0);

    // Advance the virtual clock by exactly one second
    MockClock::advance(Duration::from_secs(1));

    // Mutate the FS for the second sample
    // /proc/stat diff: total +400, idle +200  ->  50 % busy.
    write_file(&root, "proc/stat", "cpu 200 0 200 1000 0 0 0 0\n");
    // RAPL diff: +10_000_000 uJ = +10 J over 1 s = 10 W.
    write_file(
        &root,
        "sys/class/powercap/intel-rapl:0/energy_uj",
        "15000000\n",
    );

    // Second update
    tel.update();

    assert_eq!(tel.snapshot().cpu_usage, 50);
    assert_eq!(tel.snapshot().power_draw, 10.0);
}
