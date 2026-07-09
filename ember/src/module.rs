//! Module trait and parameter wrapper.

use rax::Array;

/// A trainable module.
pub trait Module {
    fn forward(&self, input: &Array<f64>) -> foundry::Result<Array<f64>>;

    fn parameters(&self) -> Vec<Parameter>;
}

/// Owned or shared parameter array (MVP: owned).
#[derive(Debug, Clone)]
pub struct Parameter {
    pub data: Array<f64>,
    pub grad: Array<f64>,
}

impl Parameter {
    pub fn new(data: Array<f64>) -> Self {
        let grad = data.zeros_like();
        Self { data, grad }
    }
}
