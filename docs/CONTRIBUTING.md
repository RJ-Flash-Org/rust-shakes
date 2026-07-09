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
