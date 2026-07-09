# aurora-df


<!-- RS-MVP START -->
## Purpose
- Arrow-native DataFrame with a small, composable expression engine for filter/select/groupby/join.
- Interop-first: Arrow at the seams; easy ingestion/export; predictable memory.
- A pragmatic base for future lazy planning and Python bridges.

## Non-goals
- Full SQL parser/optimizer in MVP.
- All file formats/codecs; keep to CSV (read) + Arrow IPC/Parquet (export behind feature) initially.
- Complex window functions/pivots (later).

## MVP surface
- Core types
  - `DataFrame { schema, columns }`, `Series { dtype, data }`, `Schema`.
  - `Expr` (column refs, literals, simple ops, agg: sum/mean/count/min/max).
- Builders & IO
  - `DataFrame::from_arrow(table)`, `from_iter(record_batches)`, `read_csv(path)` (minimal, utf8, header).
  - `to_arrow(table)`, optional `to_parquet(path)` behind `feature = "parquet"`.
- Operations
  - `select(exprs...)`, `filter(expr)`, `group_by(cols...).agg(Expr::...)`, `join(rhs, on, how)`.
- Interop
  - Arrow round-trip guaranteed (zero-copy where feasible). DType mapping via foundry.
- Policy
  - Deterministic semantics: no silent type coercions; explicit casts. Error enum for IO/Schema/Compute.
  - CI includes a tiny dataset test (groupby+join) and perf sanity on filter/select.
- Acceptance
  - Can load a CSV (small), select+filter, groupby+agg, join two frames; export to Arrow. Examples in docs.
<!-- RS-MVP END -->
