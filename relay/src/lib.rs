//! `relay` — Python interop for rust-shakes.
//!
//! Phase-1 MVP: module declaration and conversion helpers (PyO3 wiring later).

use foundry::Result;
use rax::Array;

pub mod convert;

pub use convert::to_py_list;

/// Convert a Rust array into a Python-style list representation (for testing).
pub fn array_to_py_repr(arr: &Array<f64>) -> Result<String> {
    if arr.ndim() == 1 {
        let mut parts = Vec::new();
        for i in 0..arr.len() {
            parts.push(format!("{}", arr.get_flat(i)?));
        }
        return Ok(format!("[{}]", parts.join(", ")));
    }
    Err(foundry::Error::Unsupported("only 1-D repr implemented".into()))
}
