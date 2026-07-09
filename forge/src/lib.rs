//! `forge` — classical machine learning for rust-shakes.
//!
//! Phase-1 MVP: transformer trait + linear regression + k-means skeleton.

use foundry::Result;
use rax::Array;

pub mod linear;
pub mod cluster;
pub mod metrics;

pub use linear::LinearRegression;
pub use cluster::KMeans;

/// Common estimator interface.
pub trait Estimator {
    fn fit(&mut self, x: &Array<f64>, y: &Array<f64>) -> Result<()>;
    fn predict(&self, x: &Array<f64>) -> Result<Array<f64>>;
}
