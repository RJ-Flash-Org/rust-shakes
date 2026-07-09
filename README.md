# Rust-Shakes Workspace

This directory contains the rust-shakes library suite: a batteries-included Rust ecosystem for numerics, data, and machine learning.

## Quick Start

Run all tests from this workspace root:

```powershell
$env:PATH = "C:\Users\RJG\AppData\Local\puccinialin\puccinialin\Cache\rustup\toolchains\stable-x86_64-pc-windows-msvc\bin;" + $env:PATH
cargo test --workspace --all-targets
```

Or, if `cargo` is in your PATH:

```bash
cargo test --workspace --all-targets
```

## Crates

See the org-foundation README at `rust-shakes/README.md` for the full vision and roadmap.

Current Phase-1 MVP crates (all compile and have tests where applicable):

- `foundry` — shared core
- `rax` — n-dim arrays
- `aurora-df` — DataFrames
- `optima` — autodiff/optimizers
- `forge` — classical ML
- `ember` — deep learning skeleton
- `vista` — plotting
- `flash` — web API skeleton
- `quarry` — connectors
- `relay` — Python interop helpers
- `scribe` — docs/notebooks
- `cookbook` — recipes
- `kits` — domain utilities

## State

See `AI_STATE.md` for the latest execution state, blockers, and next steps.
