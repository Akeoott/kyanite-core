# kyanite-core

[![Crates.io Version](https://img.shields.io/crates/v/kyanite-core?style=for-the-badge&logo=rust&labelColor=%231a1b1f&color=%23dea584)](https://crates.io/crates/kyanite-core)
[![CodeFactor Grade](https://img.shields.io/codefactor/grade/github/Akeoott/kyanite-core?style=for-the-badge&logoSize=auto&labelColor=%231a1b1f)](https://www.codefactor.io/repository/github/akeoott/kyanite-core)<br>
[![Continuous Integration](https://img.shields.io/github/actions/workflow/status/Akeoott/kyanite-core/ci.yaml?branch=main&style=for-the-badge&logo=githubactions&logoColor=%23ffffff&labelColor=%231a1b1f)](https://github.com/Akeoott/kyanite-core/actions/workflows/ci.yaml)
[![docs.rs](https://img.shields.io/docsrs/kyanite-core?style=for-the-badge&logo=docs.rs&labelColor=%231a1b1f)](https://docs.rs/kyanite-core)

### The core library for the kyanite project

Cross-platform hardware and system telemetry library,<br>
exposing a unified interface for CPU, GPU, Memory, Drive, Network, Process, and System metrics.

> [!WARNING]
> This library is in active early development (`>= 1.0.0-alpha`).
> APIs will change, features are incomplete, and it is **not** recommended for production use yet.

## Overview

`kyanite-core` is a systems-level telemetry library written in Rust.
It exposes a single `Telemetry` aggregate that collects data from all subsystems in parallel and caches the results in immutable snapshots,
so consumers can read without triggering any I/O.

### Platform Support

| Platform | Status           |
|----------|------------------|
| Linux    | Work in progress |
| Windows  | Work in progress |
| macOS    | Not planned      |

### Features

- **Cross-platform** - Linux and Windows implementations behind a single interface
- **Snapshot-based** - each service caches its state, consumers read without I/O
- **Concurrent updates** - `Aggregate::update_all()` uses `rayon` to update every service in parallel
- **Serde-ready** - all snapshot models serialize/deserialize out of the box
- **Feature-gated** - enable only what you need via Cargo features

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
kyanite-core = { version = "1.0.0-alpha.1", features = ["telemetry"] }
```

Basic example:

```rust
use std::thread;
use std::time::Duration;

use kyanite_core::telemetry::Aggregate;

fn main() {
    let mut telemetry = Aggregate::new();

    telemetry.update_all();
    thread::sleep(Duration::from_secs(1));
    telemetry.update_all();

    println!("{:?}", telemetry.cpu.snapshot());
}
```

For a full example covering all subsystems, see [`examples/telemetry.rs`](examples/telemetry.rs):

```sh
cargo run --example telemetry --features telemetry
```

### Cargo Features

| Feature     | Default | Description                                         |
|-------------|---------|-----------------------------------------------------|
| `full`      | No      | Alias that enables every available non-dev feature. |
| `telemetry` | No      | Enables the full telemetry subsystem.               |

## Contributing

Contributions are welcome.<br>
Please read our [CONTRIBUTING.md](https://github.com/Akeoott/kyanite-core/blob/main/.github/CONTRIBUTING.md) and [Code of Conduct](https://github.com/Akeoott/kyanite-core/blob/main/.github/CODE_OF_CONDUCT.md)
for community guidelines, code style and commit conventions before contributing.

### Documentation

  - **API reference** - [docs.rs/kyanite-core](https://docs.rs/kyanite-core)
  - **Source code** - [github.com/Akeoott/kyanite-core](https://github.com/Akeoott/kyanite-core)

### Related Projects
  - [**kyanite-app**](https://github.com/Akeoott/kyanite-app) - The Tauri + Vue desktop application powered by this library.
