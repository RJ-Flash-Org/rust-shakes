//! Cookbook recipes.

use foundry::Result;
use forge::{Estimator, LinearRegression};
use rax::Array;

/// Linear regression on a tiny synthetic dataset.
pub fn linear_fit_example() -> Result<f64> {
    let x = Array::from_shape_vec([5, 1], vec![1.0, 2.0, 3.0, 4.0, 5.0])?;
    let y = Array::from_shape_vec([5], vec![2.0, 4.0, 6.0, 8.0, 10.0])?;
    let mut model = LinearRegression::new();
    model.fit(&x, &y)?;
    let pred = model.predict(&x)?;
    let mut err = 0.0;
    for i in 0..y.len() {
        let diff = y.get_flat(i)? - pred.get_flat(i)?;
        err += diff * diff;
    }
    Ok(err / y.len() as f64)
}
