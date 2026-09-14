// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::DriveTelSnapshot;

/// Stub
pub struct DriveTel {
    snapshot: DriveTelSnapshot,
}

impl Default for DriveTel {
    fn default() -> Self {
        Self::new()
    }
}

impl DriveTel {
    /// Stub
    pub fn new() -> Self {
        Self {
            snapshot: DriveTelSnapshot::default(),
        }
    }

    /// Fetches new data and overwrites the cached snapshot
    pub fn update(&mut self) {
        self.snapshot = DriveTelSnapshot {
            ..Default::default()
        };
    }

    /// Returns a reference to the current cached snapshot
    pub fn snapshot(&self) -> &DriveTelSnapshot {
        &self.snapshot
    }
}
