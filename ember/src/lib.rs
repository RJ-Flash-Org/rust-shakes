//! `ember` — deep learning for rust-shakes.
//!
//! Phase-1 MVP: module trait, linear layer, and a simple trainer.

use foundry::Result;
use rax::Array;

pub mod module;
pub mod layers;
pub mod optim;
pub mod trainer;

pub use module::{Module, Parameter};
pub use layers::Linear;
pub use optim::SGD;
pub use trainer::Trainer;

/// Convenience trait alias for a loss function.
pub trait Loss {
    fn forward(&self, pred: &Array<f64>, target: &Array<f64>) -> Result<f64>;
}

/// Mean squared error loss.
#[derive(Debug, Default, Clone, Copy)]
pub struct Mse;

impl Loss for Mse {
    fn forward(&self, pred: &Array<f64>, target: &Array<f64>) -> Result<f64> {
        if pred.shape() != target.shape() {
            return Err(foundry::Error::ShapeMismatch("mse shape mismatch".into()));
        }
        let mut s = 0.0;
        for i in 0..pred.len() {
            let diff = pred.get_flat(i)? - target.get_flat(i)?;
            s += diff * diff;
        }
        Ok(s / pred.len() as f64)
    }
}
