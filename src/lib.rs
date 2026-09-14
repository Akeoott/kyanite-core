// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Exposes the telemetry feature if enabled
#[cfg(any(feature = "telemetry", feature = "full"))]
pub mod telemetry;
