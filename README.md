# Rust-Shakes Workspace

A batteries-included Rust ecosystem for numerics, data, and machine learning.  
This monorepo contains the Phase-1 workspace with all core crates; the org foundation (governance, branding, templates) lives in `rust-shakes/`.

## Quick Start

Run all tests from this workspace root:

```bash
cargo test --workspace --all-targets
```

## Crates

Current Phase-1 MVP crates (all compile and have tests where applicable):

- `foundry` — shared numerics, dtypes, errors, traits, small math utils
- `rax` — n-dimensional arrays & tensors (NumPy-class ergonomics)
- `aurora-df` — DataFrames + query engine (pandas/Polars space)
- `optima` — autodiff + optimizers (SciPy-style core)
- `forge` — classical ML (scikit-learn space)
- `ember` — deep learning (PyTorch-class training ergonomics)
- `vista` — plotting & viz (Matplotlib/ggplot feel)
- `flash` — web APIs & model serving (FastAPI-like DX, Rust performance)
- `quarry` — connectors (DBs, files, cloud, Arrow bridges)
- `relay` — Python interop helpers (PyO3, Arrow zero-copy)
- `scribe` — docs tools, examples, notebook-style runners
- `cookbook` — verified recipes, runnable end-to-end
- `kits` — domain-specific utilities (NLP, time-series, graph, vision)
- `hyperc` — fast compile/test loop tooling for Rust research workflows

## Vision & Roadmap

See `rust-shakes/docs/VISION.md` and `rust-shakes/docs/ROADMAP.md` for the full vision, governance, and release roadmap.

## State

See `AI_STATE.md` for the latest execution state, blockers, and next steps.

## License

Apache-2.0. See `rust-shakes/LICENSE`.
