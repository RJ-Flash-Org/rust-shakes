//! Conversion helpers between rust-shakes arrays and Python-friendly forms.

use foundry::Result;
use rax::Array;

/// Convert a 1-D array to a Vec<f64> (for PyO3 bridge later).
pub fn to_py_list(arr: &Array<f64>) -> Result<Vec<f64>> {
    if arr.ndim() != 1 {
        return Err(foundry::Error::ShapeMismatch("to_py_list expects 1-D array".into()));
    }
    let mut v = Vec::with_capacity(arr.len());
    for i in 0..arr.len() {
        v.push(arr.get_flat(i)?);
    }
    Ok(v)
}

/// Convert a Vec<f64> into an array.
pub fn from_py_list(v: Vec<f64>) -> Array<f64> {
    Array::from_vec(v)
}
