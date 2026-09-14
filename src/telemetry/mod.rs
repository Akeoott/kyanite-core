// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

pub mod models;
use log::debug;

/// use the linux implementation as the platform
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

/// use the windows implementation as the platform
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as platform;

pub use platform::{
    cpu::CpuTel, drive::DriveTel, gpu::GpuTel, memory::MemoryTel, network::NetworkTel,
    process::ProcessTel, system::SystemTel,
};

/// Aggregates all telemetry collectors into a single entry point.
///
/// Each field is a platform-specific implementation of a telemetry
/// service. Platform-specific references are decided at compile time.
pub struct Aggregate {
    /// CPU telemetry collector.
    pub cpu: CpuTel,
    /// Drive (storage) telemetry collector.
    pub drive: DriveTel,
    /// GPU telemetry collector.
    pub gpu: GpuTel,
    /// Memory telemetry collector.
    pub memory: MemoryTel,
    /// Network telemetry collector.
    pub network: NetworkTel,
    /// Process telemetry collector.
    pub process: ProcessTel,
    /// System-level telemetry collector.
    pub system: SystemTel,
}

impl Default for Aggregate {
    fn default() -> Self {
        Self::new()
    }
}

impl Aggregate {
    /// Creates a new [`Aggregate`] instance with all snapshots initialized
    /// to their default (empty/zero) values.
    #[must_use]
    pub fn new() -> Self {
        debug!("Creating new telemetry instance");
        Self {
            cpu: CpuTel::new(),
            drive: DriveTel::new(),
            gpu: GpuTel::new(),
            memory: MemoryTel::new(),
            network: NetworkTel::new(),
            process: ProcessTel::new(),
            system: SystemTel::new(),
        }
    }

    /// Updates all telemetry collectors concurrently.
    ///
    /// Each collector runs on its own thread via `rayon::scope`, so a
    /// single call fetches all data in parallel.
    /// Blocks until all collectors have finished.
    pub fn update_all(&mut self) {
        debug!("Updating all telemetry concurrently");
        rayon::scope(|s| {
            s.spawn(|_| self.cpu.update());
            s.spawn(|_| self.drive.update());
            s.spawn(|_| self.gpu.update());
            s.spawn(|_| self.memory.update());
            s.spawn(|_| self.network.update());
            s.spawn(|_| self.process.update());
            self.system.update();
        });
    }
}
