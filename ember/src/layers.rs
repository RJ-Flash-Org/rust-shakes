//! Neural network layers.

use foundry::Result;
use rax::Array;

use crate::module::{Module, Parameter};

/// Fully connected linear layer: y = x @ W^T + b.
#[derive(Debug, Clone)]
pub struct Linear {
    pub in_features: usize,
    pub out_features: usize,
    pub weight: Parameter,
    pub bias: Parameter,
}

impl Linear {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let w = Array::randn([out_features, in_features]) * 0.01;
        let b = Array::zeros([out_features]);
        Self {
            in_features,
            out_features,
            weight: Parameter::new(w),
            bias: Parameter::new(b),
        }
    }
}

impl Module for Linear {
    fn forward(&self, input: &Array<f64>) -> Result<Array<f64>> {
        if input.ndim() != 2 || input.shape()[1] != self.in_features {
            return Err(foundry::Error::ShapeMismatch("linear input shape mismatch".into()));
        }
        let n = input.shape()[0];
        let mut out = Array::zeros([n, self.out_features]);
        for i in 0..n {
            for j in 0..self.out_features {
                let mut v = self.bias.data.get_flat(j)?;
                for k in 0..self.in_features {
                    v += input.get_flat(i * self.in_features + k)?
                        * self.weight.data.get_flat(j * self.in_features + k)?;
                }
                out.set_flat(i * self.out_features + j, v)?;
            }
        }
        Ok(out)
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![self.weight.clone(), self.bias.clone()]
    }
}
