// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

use crate::telemetry::models::SystemTelSnapshot;
use log::trace;
use vfs::{PhysicalFS, VfsPath};

/// Stub
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

        self.snapshot = SystemTelSnapshot {
            ..Default::default()
        };
    }
}
