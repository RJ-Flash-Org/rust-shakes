//! Simple training loop.

use foundry::Result;
use rax::Array;

use crate::{Loss, Module, SGD};

/// Trainer for a single model + optimizer + loss.
#[derive(Debug)]
pub struct Trainer<M: Module, L: Loss> {
    pub model: M,
    pub loss: L,
    pub optimizer: SGD,
}

impl<M: Module, L: Loss> Trainer<M, L> {
    pub fn new(model: M, optimizer: SGD, loss: L) -> Self {
        Self {
            model,
            loss,
            optimizer,
        }
    }

    /// Run one epoch of gradient descent (MVP: numeric gradients via optima).
    pub fn train_step(&mut self, x: &Array<f64>, y: &Array<f64>) -> Result<f64> {
        let pred = self.model.forward(x)?;
        let loss = self.loss.forward(&pred, y)?;
        // Numeric gradient for each parameter (MVP).
        let mut params = self.model.parameters();
        self.optimizer.zero_grad(&mut params);
        for p in &mut params {
            for i in 0..p.data.len() {
                let h = 1e-5;
                let orig = p.data.get_flat(i)?;
                p.data.set_flat(i, orig + h)?;
                let l_plus = self.loss.forward(&self.model.forward(x)?, y)?;
                p.data.set_flat(i, orig - h)?;
                let l_minus = self.loss.forward(&self.model.forward(x)?, y)?;
                p.data.set_flat(i, orig)?;
                let grad = (l_plus - l_minus) / (2.0 * h);
                p.grad.set_flat(i, grad)?;
            }
        }
        self.optimizer.step(&mut params);
        Ok(loss)
    }
}
