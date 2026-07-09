//! `cookbook` — verified recipes for rust-shakes.
//!
//! Phase-1 MVP: recipe registry and a few runnable examples.

use foundry::Result;

pub mod recipes;

pub use recipes::linear_fit_example;

/// Run a named recipe and report a metric.
pub fn run_recipe(name: &str) -> Result<f64> {
    match name {
        "linear_fit" => linear_fit_example(),
        _ => Err(foundry::Error::InvalidArgument(format!("unknown recipe: {}", name))),
    }
}
