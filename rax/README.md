# rax


<!-- RS-MVP START -->
## Purpose
- Ergonomic n-dim arrays/tensors with predictable performance and safe views/slices.
- NumPy-class ergonomics for core workflows: create, slice, map, reduce, broadcast.
- A composable base for later GPU/BLAS and autodiff layers.

## Non-goals
- GPU backends/BLAS acceleration in MVP (will arrive via feature flags later).
- Autograd (lives in optima/ember).
- Distributed out-of-core operations.

## MVP surface
- Core types
  - `Array<T, D>` (owned), `View<'a, T, D>` (borrowed), `ViewMut<'a, T, D>`.
  - Dimensions D via const generics (e.g., `D2`, `D3`) + dynamic `Dx`.
- Construction
  - `Array::from_vec(vec, shape)`, `zeros(shape)`, `ones(shape)`, `full(shape, val)`.
- Indexing & slicing
  - `get`, `get_mut`, and slicing DSL: `s![.., 1..3]` (minimal macro).
- Broadcasting & iteration
  - `broadcast(new_shape)`, `map`, `map_inplace`, `fold/reduce(axis)`.
- Arithmetic (elementwise)
  - `Add/Sub/Mul/Div` for `Array` x `Array` and `Array` x scalar; with trait bounds via `Numeric`.
- Invariants & safety
  - Respect `Shape`/`Strides` invariants from foundry. Checked construction; panics only on explicit `unwrap`.
- Policy
  - `cargo fmt/clippy/test` clean in CI. Bench harness for 2–3 hot paths (construction, slice, add).
- Acceptance
  - Shape checks enforced; broadcasting rules documented; examples compile and run in doctests.
<!-- RS-MVP END -->
