//! Minimal autodiff tape.

use foundry::Result;
use rax::Array;

/// Scalar value with gradient bookkeeping.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value {
    pub data: f64,
    pub grad: f64,
}

impl Value {
    pub fn new(data: f64) -> Self {
        Self { data, grad: 0.0 }
    }
}

/// Autodiff tape (MVP: single-value placeholder).
#[derive(Debug, Default)]
pub struct Tape {
    values: Vec<Value>,
}

impl Tape {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn var(&mut self, data: f64) -> Value {
        let v = Value::new(data);
        self.values.push(v);
        v
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}

/// Numeric gradient using central differences (phase-1 fallback).
pub fn grad<F>(f: F, x: &Array<f64>) -> Result<Array<f64>>
where
    F: Fn(&Array<f64>) -> f64,
{
    let h = 1e-6;
    let mut g = x.zeros_like();
    for i in 0..x.len() {
        let mut xp = x.clone();
        let mut xm = x.clone();
        let _ = xp.set(i, xp.get(i)? + h);
        let _ = xm.set(i, xm.get(i)? - h);
        g.set(i, (f(&xp) - f(&xm)) / (2.0 * h))?;
    }
    Ok(g)
}
