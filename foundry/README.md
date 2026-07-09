# foundry


<!-- RS-MVP START -->
## Purpose
- Provide a stable numeric core: dtypes, error taxonomy, numeric traits, and small math utilities.
- Unify type/shape/error stories across Rust-Shakes libraries; minimize duplication and ambiguity.
- Offer predictable MSRV and SemVer so downstream crates can depend confidently.

## Non-goals
- Heavy linear algebra/BLAS bindings (lives in higher layers).
- DataFrame/query features (aurora-df).
- Autodiff/optimizers (optima/forge).

## MVP surface
- Types
  - `DType` enum covering {f16/f32/f64, i8/i16/i32/i64/isize, u8/u16/u32/u64/usize, bool}.
  - `Shape` and `Strides` newtypes (minimal invariants; no allocation).
  - `FoundryError` with categories: InvalidShape, Overflow, TypeMismatch, NotImplemented.
- Traits
  - `Numeric` (sealed) with required ops bounds + `DType::of::<T>()` mapping.
  - `AsShape`/`TryAsShape` for ergonomic construction and validation.
- Utilities
  - Safe helpers: `checked_mul_extents`, `elem_size(dtype)`, `byte_len(shape, dtype)`.
  - Arrow interop feature flag: `feature = "arrow"` exposes `From<ArrowDataType> for DType` and reverse map.
- Policy
  - MSRV: stable current − 2 releases. SemVer with `-D warnings` CI.
  - Docs: examples for `Shape`, `DType`, and common error cases.
- Acceptance
  - 90%+ coverage for shape/math helpers. No panics on invalid input paths. Lints clean.
<!-- RS-MVP END -->
