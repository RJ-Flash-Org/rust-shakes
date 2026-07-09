//! Figure and plot builders.

use foundry::Result;
use rax::Array;

use crate::backend::render_polyline;

/// A figure contains one or more series.
#[derive(Debug, Default, Clone)]
pub struct Figure {
    points: Vec<(f64, f64)>,
}

impl Figure {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line(&mut self, x: &Array<f64>, y: &Array<f64>) -> Result<&mut Self> {
        if x.shape() != y.shape() || x.ndim() != 1 {
            return Err(foundry::Error::ShapeMismatch("add_line expects 1-D arrays of equal length".into()));
        }
        for i in 0..x.len() {
            self.points.push((x.get_flat(i)?, y.get_flat(i)?));
        }
        Ok(self)
    }

    pub fn save_svg(&self, path: &str) -> Result<()> {
        render_polyline(&self.points, 800, 400, path)
    }
}

/// Trait for plottable types.
pub trait Plot {
    fn plot(&self, figure: &mut Figure) -> Result<()>;
}
