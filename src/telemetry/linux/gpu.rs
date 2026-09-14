// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::GpuTelSnapshot;

/// Stub
pub struct GpuTel {
    snapshot: GpuTelSnapshot,
}

impl Default for GpuTel {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuTel {
    /// Stub
    pub fn new() -> Self {
        Self {
            snapshot: GpuTelSnapshot::default(),
        }
    }

    /// Fetches new data and overwrites the cached snapshot
    pub fn update(&mut self) {
        self.snapshot = GpuTelSnapshot {
            ..Default::default()
        };
    }

    /// Returns a reference to the current cached snapshot
    pub fn snapshot(&self) -> &GpuTelSnapshot {
        &self.snapshot
    }
}
