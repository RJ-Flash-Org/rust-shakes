# Rust-Shakes

**A batteries-included Rust ecosystem for numerics, data, and machine learning.**  
Our goal is to make Rust the most productive place to build scientific software—without giving up safety or speed.

---

## Why Rust-Shakes?

Python owns ergonomics; Rust owns performance, safety, and deployability. Rust-Shakes bridges the two:

- **Unified numeric core** (shared traits, dtypes, errors) across all libraries.
- **HyperC** for **fast compile/test loops** (the #1 adoption blocker for research workflows).
- **Python interop** via Arrow/zero-copy bridges and PyPI wheels to enable **incremental migration**.
- **World-class DX**: clean APIs, human-grade error messages, cookbooks, and predictable releases.

---

## Library Suite (Phase-1 target set)

- **foundry** – shared numerics, dtypes, errors, traits, small math utils.  
- **rax** – n-dim arrays & tensors (NumPy-class ergonomics).  
- **aurora-df** – DataFrames + query engine (pandas/Polars space) with Arrow at the seams.  
- **optima** – autodiff + optimizers (core SciPy style).  
- **forge** – classical ML (scikit-learn space).  
- **ember** – deep learning (PyTorch-class training ergonomics).  
- **vista** – plotting & viz (Matplotlib/ggplot feel, GPU-aware backends).  
- **flash** – web APIs & model serving (FastAPI-like DX, Rust performance).  
- **quarry** – connectors (DBs, files, cloud, Arrow bridges).  
- **relay** – Python interop (PyO3 wheels, Arrow zero-copy, Pandas adapters).  
- **scribe** – docs tools, examples, notebook-style runners.  
- **cookbook** – hundreds of verified recipes, runnable end-to-end.

> This repo is the **org foundation** (policies, governance, CI templates). Each library lives in its own repo under the org.

---

## Roadmap (high level)

**M0: Org foundation (you are here)**  
Policies, security, CoC, governance, issue/pr templates, CI blueprints, press kit.

**M1: HyperC public preview**  
10× faster rebuilds for common Rust dev loops; baseline benchmarks & reproducible perf harness.

**M2: Core numerics (foundry) + rax alpha**  
Stable dtypes/traits/error story + first ergonomic ndarray/tensor API.

**M3: aurora-df alpha + Arrow bridges**  
Interchange with Pandas/Polars; early query engine with groupby/joins.

**M4: Interop (relay) + PyPI wheels**  
Zero-copy bridges; simple “pip install” for data scientists.

**M5: forge/optima alpha**  
Classical ML + autodiff/optimizers sufficient to replicate popular sklearn recipes.

**M6: ember alpha + vista alpha**  
DL training loops + plotting backends for research workflow parity.

Full, dated roadmap with acceptance criteria: see [docs/ROADMAP.md](docs/ROADMAP.md).

---

## Governance & Community

- **Meritocratic** with **clear RFC process**, code ownership, and release cadence.  
- **Security first** (coordinated disclosure; SLAs in SECURITY.md).  
- **Inclusive, respectful** community (see CODE_OF_CONDUCT.md).  

Details: [docs/GOVERNANCE.md](docs/GOVERNANCE.md) · [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md)

---

## Getting Involved

- Open a proposal with the **RFC template** (Issues → “RFC: Proposal”).  
- Browse good first issues (per-repo).  
- Add a recipe to the **cookbook** (fastest way to contribute value).

---

## License

Apache-2.0. See [LICENSE](LICENSE).

---

### Quick Links

- Website: [rustshakes.org](https://rustshakes.org)  
- Vision: [docs/VISION.md](docs/VISION.md)  
- Architecture Overview: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)  
- Libraries map: [docs/LIBRARIES.md](docs/LIBRARIES.md)  
- Security: [SECURITY.md](SECURITY.md)  
- Code of Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
