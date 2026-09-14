// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use kyanite_core::telemetry::Aggregate;
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::init();
    println!("\nuse `RUST_LOG=trace` to see exactly whats happening in the background.\n");

    // 1. Initialize the aggregate (everything is zeroed/empty)
    let mut telemetry = Aggregate::new();

    // 2. Refresh all hardware data twice
    telemetry.update_all();
    thread::sleep(Duration::from_secs(1));
    telemetry.update_all();

    // 3. Read and print snapshots
    println!("\n{:?}", telemetry.cpu.snapshot());
    println!("{:?}", telemetry.drive.snapshot());
    println!("{:?}", telemetry.gpu.snapshot());
    println!("{:?}", telemetry.memory.snapshot());
    println!("{:?}", telemetry.network.snapshot());
    println!("{:?}", telemetry.process.snapshot());
    println!("{:?}", telemetry.system.snapshot());
}
