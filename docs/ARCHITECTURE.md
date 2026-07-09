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
