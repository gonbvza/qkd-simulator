# QKD Simulator

A research-grade simulator for Quantum Key Distribution (QKD) protocols implemented in Rust.

## Overview

This repository implements a discrete-event QKD simulator used for testing link behavior, error correction, and key-distillation strategies. The codebase is structured to separate core simulation logic, protocol models, networking/link abstractions, and persistence layers.

## Features

- Discrete-event simulation core
- Pluggable protocol models and detectors
- Link and node abstractions for modular topologies
- Optional database-backed state via Diesel migrations

## Requirements

- Rust and Cargo (stable toolchain)
- Optional: PostgreSQL (or other DB supported by Diesel) and `diesel` CLI to run migrations

## Quickstart

Build the project:

```bash
cargo build --release
```

Run the simulator (release build recommended):

```bash
cargo run --release
```

Run the test suite:

```bash
cargo test
```

If you use the database features, run migrations before starting the simulator:

```bash
diesel migration run
```

## Project structure

- [Cargo.toml](Cargo.toml) — crate manifest
- [src/main.rs](src/main.rs) — CLI entrypoint
- [src/api.rs](src/api.rs) — external API bindings
- [src/cli/](src/cli/) — command-line parsing and runners
- [src/core/](src/core/) — simulation core (event loop, processes, settings)
- [src/models/](src/models/) — protocol models, detectors, and messages
- [src/database/](src/database/) — database layer and migrations integration
- [migrations/](migrations/) — Diesel migrations for persistent state
- [scripts/](scripts/) — helper scripts (e.g., distribution tests)
- [tests/](tests/) — integration and unit tests

## Contributing

- Run `cargo test` and add tests for new features.
- Follow idiomatic Rust style; run `cargo fmt` and `cargo clippy` before submitting PRs.
- Describe experimental protocols or configurations in the PR description.

## Notes

- Configuration and runtime settings are defined in the `src/core` and `src/utility.rs` modules; inspect them to adapt simulation parameters.
- This project currently does not include a repository-level license file. Add `LICENSE` if you plan to publish or share the code.

## Contact

Open an issue or PR in this repository for questions, bug reports, or feature requests.
