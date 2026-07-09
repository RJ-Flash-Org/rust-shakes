pub use foundry::{DType, Result};
pub use rax::Array;

pub mod series;
pub mod dataframe;
pub mod query;

pub use dataframe::DataFrame;
pub use series::Series;
pub use query::{Expr, Filter, filter};

/// Common aggregate functions.
pub mod agg {
    use foundry::Result;
    use rax::Array;

    /// Sum of a 1-D array.
    pub fn sum(arr: &Array<f64>) -> Result<f64> {
        let mut s = 0.0;
        for i in 0..arr.len() {
            s += arr.get(i)?;
        }
        Ok(s)
    }

    /// Mean of a 1-D array.
    pub fn mean(arr: &Array<f64>) -> Result<f64> {
        if arr.is_empty() {
            return Ok(0.0);
        }
        Ok(sum(arr)? / arr.len() as f64)
    }
}
