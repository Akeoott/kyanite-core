// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::SystemTelSnapshot;

/// Stub
pub struct SystemTel {
    snapshot: SystemTelSnapshot,
}

impl Default for SystemTel {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTel {
    /// Stub
    pub fn new() -> Self {
        Self {
            snapshot: SystemTelSnapshot::default(),
        }
    }

    /// Fetches new data and overwrites the cached snapshot
    pub fn update(&mut self) {
        self.snapshot = SystemTelSnapshot {
            ..Default::default()
        };
    }

    /// Returns a reference to the current cached snapshot
    pub fn snapshot(&self) -> &SystemTelSnapshot {
        &self.snapshot
    }
}
