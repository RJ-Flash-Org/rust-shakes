//! `vista` — plotting and visualization for rust-shakes.
//!
//! Phase-1 MVP: SVG line and scatter plot primitives.

use foundry::Result;
use rax::Array;

pub mod backend;
pub mod plot;

pub use plot::{Figure, Plot};

/// Save a simple SVG line plot to `path`.
pub fn line_plot(x: &Array<f64>, y: &Array<f64>, path: &str) -> Result<()> {
    let mut fig = Figure::new();
    fig.add_line(x, y)?;
    fig.save_svg(path)
}
