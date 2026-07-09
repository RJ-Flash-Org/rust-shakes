//! Classical ML metrics.

use foundry::Result;
use rax::Array;

/// Mean squared error.
pub fn mse(y_true: &Array<f64>, y_pred: &Array<f64>) -> Result<f64> {
    if y_true.shape() != y_pred.shape() {
        return Err(foundry::Error::ShapeMismatch("shape mismatch in mse".into()));
    }
    let mut s = 0.0;
    for i in 0..y_true.len() {
        let diff = y_true.get_flat(i)? - y_pred.get_flat(i)?;
        s += diff * diff;
    }
    Ok(s / y_true.len() as f64)
}

/// Accuracy for integer labels.
pub fn accuracy(y_true: &Array<usize>, y_pred: &Array<usize>) -> Result<f64> {
    if y_true.shape() != y_pred.shape() {
        return Err(foundry::Error::ShapeMismatch("shape mismatch in accuracy".into()));
    }
    let mut correct = 0usize;
    for i in 0..y_true.len() {
        if y_true.get_flat(i)? == y_pred.get_flat(i)? {
            correct += 1;
        }
    }
    Ok(correct as f64 / y_true.len() as f64)
}
