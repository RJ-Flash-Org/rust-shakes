//! `optima` — autodiff + optimizers for rust-shakes.
//!
//! Phase-1 MVP: forward-mode autodiff tape and gradient-free optimizers.

use foundry::Result;
use rax::Array;

pub mod autodiff;
pub mod optim;

pub use autodiff::{Tape, Value};
pub use optim::{lbfgs, nelder_mead};

/// Compute the gradient of `f` at `x` using reverse-mode autodiff (MVP: numeric fallback).
pub fn grad<F>(f: F, x: &Array<f64>) -> Result<Array<f64>>
where
    F: Fn(&Array<f64>) -> f64,
{
    autodiff::grad(f, x)
}

/// Minimize a scalar function using L-BFGS (placeholder: delegates to gradient-free).
pub fn minimize<F>(f: F, x0: Array<f64>) -> Result<Array<f64>>
where
    F: Fn(&Array<f64>) -> f64,
{
    optim::nelder_mead(f, x0, 100)
}
