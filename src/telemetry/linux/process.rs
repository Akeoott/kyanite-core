// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::ProcessTelSnapshot;

/// Stub
pub struct ProcessTel {
    snapshot: ProcessTelSnapshot,
}

impl Default for ProcessTel {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessTel {
    /// Stub
    pub fn new() -> Self {
        Self {
            snapshot: ProcessTelSnapshot::default(),
        }
    }

    /// Fetches new data and overwrites the cached snapshot
    pub fn update(&mut self) {
        self.snapshot = ProcessTelSnapshot {
            ..Default::default()
        };
    }

    /// Returns a reference to the current cached snapshot
    pub fn snapshot(&self) -> &ProcessTelSnapshot {
        &self.snapshot
    }
}
