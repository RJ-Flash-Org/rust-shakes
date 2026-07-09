//! Optimizers for ember.

use crate::module::Parameter;

/// SGD optimizer with optional learning rate.
#[derive(Debug, Clone, Copy)]
pub struct SGD {
    lr: f64,
}

impl SGD {
    pub fn new(lr: f64) -> Self {
        Self { lr }
    }

    pub fn step(&self, params: &mut [Parameter]) {
        for p in params {
            for i in 0..p.data.len() {
                let g = p.grad.get_flat(i).unwrap_or(0.0);
                let v = p.data.get_flat(i).unwrap_or(0.0) - self.lr * g;
                p.data.set_flat(i, v).unwrap();
            }
        }
    }

    pub fn zero_grad(&self, params: &mut [Parameter]) {
        for p in params {
            for i in 0..p.grad.len() {
                p.grad.set_flat(i, 0.0).unwrap();
            }
        }
    }
}
