# === Rust-Shakes Org Foundation Bootstrap (Adjusted) ===
$ErrorActionPreference = "Stop"

# Helper to write files safely
function Write-TextFile($Path, $Content) {
  $dir = Split-Path $Path -Parent
  if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir | Out-Null }
  $utf8NoBOM = New-Object System.Text.UTF8Encoding($false)
  [System.IO.File]::WriteAllText($Path, $Content, $utf8NoBOM)
  Write-Host "Wrote $Path"
}

# --- Core repo files ---
$readme = @"
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
- Vision: [docs/VISION.md](docs/VISION.md)  
- Architecture Overview: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)  
- Libraries map: [docs/LIBRARIES.md](docs/LIBRARIES.md)  
- Security: [SECURITY.md](SECURITY.md)  
- Code of Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
"@
Write-TextFile ".\README.md" $readme

$license = @"
                                 Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

Copyright (c) 2025 Rust-Shakes Contributors

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
"@
Write-TextFile ".\LICENSE" $license

# --- Docs ---
$vision = @"
# Vision

Rust-Shakes is a **Rust-first scientific and ML ecosystem** that pairs *Python-level ergonomics* with *Rust-grade performance, safety, and deployability*. We remove the last excuses for not choosing Rust in data work: slow iteration, weak interop, and fragmented APIs.

## Design Tenets
1) **Ergonomics win adoptions** – APIs should feel obvious, errors should teach.  
2) **One numeric core** – shared dtypes/traits/errors across libraries.  
3) **Interop is a feature** – Arrow everywhere, PyPI wheels for frictionless trials.  
4) **Reproducible performance** – benchmarks in CI; perf regressions block releases.  
5) **Governance & docs** – sustainable community over heroics.
"@
Write-TextFile ".\docs\VISION.md" $vision

$libraries = @"
# Libraries Map

- **foundry**: shared numerics, dtypes, traits, errors.  
- **rax**: n-dim arrays & tensors.  
- **aurora-df**: DataFrames + query engine, Arrow-native.  
- **optima**: autodiff + optimizers.  
- **forge**: classical ML.  
- **ember**: deep learning.  
- **vista**: plotting & visualization.  
- **flash**: model serving & APIs.  
- **quarry**: IO/connectors (DBs, files, cloud).  
- **relay**: Python interop (PyO3, Arrow zero-copy, wheels).  
- **scribe**: docs tools & notebook-style runners.  
- **cookbook**: runnable recipes & examples.
"@
Write-TextFile ".\docs\LIBRARIES.md" $libraries

$architecture = @"
# Architecture Overview

## Layers
- **Runtime & Tooling**: HyperC (fast builds), cargo profiles, sccache, reproducible benches.
- **Numeric Core (foundry)**: dtypes (f16/f32/f64/i*/u*), shape/stride abstractions, error taxonomy.
- **Array/Tensor (rax)**: views, broadcasting, BLAS/GPU hooks, parallel iterators.
- **Data (aurora-df)**: Arrow tables, lazy query plan, expression engine, joins/groupby, IO.
- **ML (optima/forge/ember)**: autodiff graph, optimizers (SGD/Adam/LBFGS), estimators/pipelines, DL training loops.
- **Viz (vista)**: pluggable backends (SVG/Canvas/WGPU), charting grammar.
- **Interop (relay)**: Arrow bridges, pandas adapters, PyO3 wheels.
- **Serving (flash)**: HTTP/gRPC, batch/stream inference, hot-reload config, A/B & canary.

## Cross-cutting
- **Error & Logging**: unified error types; tracing spans; human-focused messages.
- **Perf & Safety**: Criterion benches; property tests; `unsafe`  fenced and justified.
"@
Write-TextFile ".\docs\ARCHITECTURE.md" $architecture

$roadmap = @"
# Roadmap

## Milestone 0 — Org Foundation (Now)
- ✅ LICENSE (Apache-2.0), SECURITY, CoC, Governance, Contributing
- ✅ Issue/PR templates, CI blueprint, press kit stub

## Milestone 1 — HyperC Public Preview (Month 1–2)
- 10× rebuild speed on typical crates; perf harness + CI benches
- Windows/Linux/macOS parity; sccache integration guide

## Milestone 2 — foundry + rax Alpha (Month 2–4)
- Stable dtypes/traits/error story
- rax: creation, slicing, broadcasting, map/reduce, BLAS hooks

## Milestone 3 — aurora-df Alpha + Arrow (Month 4–6)
- Read/write CSV/Parquet; expressions; groupby/joins; Arrow interchange

## Milestone 4 — relay Interop + Wheels (Month 6–7)
- PyO3 bindings; zero-copy Arrow; `pip install rustshakes-rax`  demo

## Milestone 5 — forge/optima Alpha (Month 7–9)
- Fit/transform pipelines; core estimators; autodiff + optimizers

## Milestone 6 — ember + vista Alpha (Month 9–12)
- DL training loops, dataloaders; charting grammar & backends

### Release Cadence
- Monthly minors, quarterly feature rollups, LTS every 12 months.
"@
Write-TextFile ".\docs\ROADMAP.md" $roadmap

$governance = @"
# Governance

- **Steering Council**: sets roadmap, accepts RFCs, appoints release leads.
- **Code Owners**: per-repo ownership with documented escalation paths.
- **RFC Process**: all non-trivial changes start as an RFC issue using the template.
- **Decision Making**: lazy consensus → vote by Council if contention persists.
- **Releases**: release leads rotate; changelogs required; perf gates enforced.
"@
Write-TextFile ".\docs\GOVERNANCE.md" $governance

$contributing = @"
# Contributing

1. Read the [Code of Conduct](../CODE_OF_CONDUCT.md).  
2. For any non-trivial change, open an **RFC** issue first.  
3. Use conventional commits (`feat:` , `fix:` , `docs:` ) and include benchmarks for perf-sensitive code.  
4. Add/extend tests; no new public API without examples.  
5. For interop: prove zero-copy paths (Arrow) where promised.

## Dev Environment
- Rust stable (latest), rustfmt + clippy
- `cargo test` , `cargo clippy -- -D warnings` , `cargo fmt -- --check` 
- Criterion for micro-benches; benches must run under 60s.

## Security
- See [SECURITY.md](../SECURITY.md). Report privately; we triage within 3 business days.
"@
Write-TextFile ".\docs\CONTRIBUTING.md" $contributing

# --- GitHub meta (.github) ---
$coc = @"
# Code of Conduct

We are committed to a welcoming, harassment-free community. Be kind. Assume good intent. Disagree respectfully.  
Enforcement follows the guidelines at https://contributor-covenant.org (v2.1 aligned).

**Report concerns:** security@yourdomain.com (or GitHub security advisories for vuln reports).
"@
Write-TextFile ".\CODE_OF_CONDUCT.md" $coc

$security = @"
# Security Policy

- Please report vulnerabilities privately to **security@yourdomain.com**.  
- Initial response SLA: **3 business days**.  
- Public disclosure only after a fix is available or within 90 days, whichever comes first.  
- Coordinated disclosure through GitHub Security Advisories is preferred.

## Supported Versions
- Latest minor of each active library; LTS branches receive critical fixes only.
"@
Write-TextFile ".\SECURITY.md" $security

$issue_feature = @"
name: RFC: Proposal
description: Propose a significant change (API, architecture, policy).
title: 'RFC: <short title>'
labels: [rfc, needs-triage]
body:
  - type: textarea
    attributes:
      label: Summary
      description: One paragraph summary of the problem and the proposed solution.
    validations: { required: true }
  - type: textarea
    attributes:
      label: Motivation & Goals
    validations: { required: true }
  - type: textarea
    attributes:
      label: Design Overview
  - type: textarea
    attributes:
      label: Alternatives Considered
  - type: textarea
    attributes:
      label: Drawbacks & Risks
  - type: textarea
    attributes:
      label: Adoption & Migration
  - type: textarea
    attributes:
      label: Test/Bench Strategy
"@
Write-TextFile ".\.github\ISSUE_TEMPLATE\rfc.yml" $issue_feature

$issue_bug = @"
name: Bug Report
description: Report a bug in a Rust-Shakes project.
labels: [bug, needs-triage]
body:
  - type: input
    attributes: { label: Affected repo/version }
    validations: { required: true }
  - type: textarea
    attributes: { label: What happened? }
    validations: { required: true }
  - type: textarea
    attributes: { label: Expected behavior }
  - type: textarea
    attributes: { label: Repro steps (minimal) }
  - type: textarea
    attributes: { label: Logs/stacktrace }
"@
Write-TextFile ".\.github\ISSUE_TEMPLATE\bug.yml" $issue_bug

$issue_docs = @"
name: Docs/Recipe Request
description: Request documentation improvement or a new cookbook recipe.
labels: [docs]
body:
  - type: textarea
    attributes: { label: What is unclear or missing? }
    validations: { required: true }
  - type: textarea
    attributes: { label: Desired example/recipe }
"@
Write-TextFile ".\.github\ISSUE_TEMPLATE\docs.yml" $issue_docs

$pr_template = @"
## Summary
(What this PR changes and why)

## Checklist
- [ ] Tests added/updated
- [ ] `cargo fmt`  and `cargo clippy`  clean
- [ ] Benchmarks updated (if perf-sensitive)
- [ ] Docs/examples added

## Breaking Changes
(If any, describe migration)
"@
Write-TextFile ".\.github\PULL_REQUEST_TEMPLATE.md" $pr_template

$funding = @"
github: [RJ-Flash-Org]
open_collective: rust-shakes
"@
Write-TextFile ".\.github\FUNDING.yml" $funding

$codeowners = @"
# CODEOWNERS
*       @RJ-Flash-Org/core-maintainers
/docs/  @RJ-Flash-Org/docs
"@
Write-TextFile ".\.github\CODEOWNERS" $codeowners

$support = @"
# Support

- Start with Issues → Docs/Recipe Request.  
- For security matters, see SECURITY.md.  
- For community chat, join our Discord (link in org profile).
"@
Write-TextFile ".\SUPPORT.md" $support

$changelog = @"
# Changelog

All notable changes to this organization and its templates will be documented here.
"@
Write-TextFile ".\CHANGELOG.md" $changelog

# --- CI Blueprint (org-wide; adjust per library repos) ---
$ci = @"
name: org-ci-blueprint

on:
  pull_request:
  push:
    branches: [ main ]

jobs:
  lint-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Markdown lint (mdl via docker)
        run: docker run --rm -v $PWD:/work tmknom/mdl:latest mdl -r ~MD013 ~MD033 /work

  meta-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Ensure key files exist
        run: |
          test -f README.md
          test -f LICENSE
          test -f SECURITY.md
          test -f CODE_OF_CONDUCT.md
          test -f docs/ROADMAP.md
"@
Write-TextFile ".\.github\workflows\org-ci.yml" $ci

# --- Press Kit stub ---
$press = @"
# Press Kit (Stub)

- Project: Rust-Shakes
- One-liner: Batteries-included Rust stack for numerics, data, and ML.
- Elevator pitch: Python-class ergonomics with Rust-class performance and safety.
- Talking points: HyperC fast builds; Arrow interop; cookbook-driven onboarding.
- Logos: (placeholder) /branding to be added in Milestone 0.1
"@
Write-TextFile ".\docs\PRESS_KIT.md" $press

# --- Root Library IDs tracker (per process) ---
$libraryMd = @"
# library.md — Canonical Library IDs

Use this file as the source of truth for library IDs across the organization. Always check here before adding/searching for a new library.

## IDs (Phase-1 target set)
- foundry — shared numerics core
- rax — arrays/tensors
- aurora-df — DataFrames + query engine
- optima — autodiff + optimizers
- forge — classical ML
- ember — deep learning
- vista — visualization
- flash — serving & APIs
- quarry — IO/connectors
- relay — Python interop
- scribe — docs tools
- cookbook — recipes/examples

## Notes
- Keep this list alphabetized.
- When opening issues/PRs or searching, reference IDs verbatim.
- For new additions, propose via RFC first; add here upon acceptance.
"@
Write-TextFile ".\library.md" $libraryMd

# --- Git init/commit if needed ---
if (Test-Path ".git") {
  git add -A
  git commit -m "chore(org): bootstrap foundation docs, templates, CI, and licensing"
} else {
  git init
  git add -A
  git commit -m "chore(org): bootstrap foundation docs, templates, CI, and licensing"
  git branch -M main
}
Write-Host "`n✅ Rust-Shakes org foundation is scaffolded. Push with: git remote add origin <repo-url> && git push -u origin main"
