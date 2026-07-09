# hyperc — Hyper Compiler for Rust

**Phase 1.1:** daemon with JSON TCP IPC, cache-aware builds (sccache), Cargo.lock watcher prewarm, and a benchmark harness.

- `cargo-hyper` subcommand: tries daemon first; falls back to tuned local build.
- `hyperc-daemon`: listens on `HYPERC_ADDR` (default `127.0.0.1:7878`), prewarms on `Cargo.lock` changes.
- `hyperc-bench`: N-run median benchmark; writes `hyperc.bench.json`.

> This is a safe scaffold—no rustc internals yet. Portable across Windows/macOS/Linux.
