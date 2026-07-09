//! K-means clustering (MVP).

use foundry::{Error, Result};
use rax::Array;

#[derive(Debug, Clone)]
pub struct KMeans {
    pub k: usize,
    pub max_iter: usize,
    pub centroids: Option<Array<f64>>,
}

impl KMeans {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iter: 100,
            centroids: None,
        }
    }

    pub fn fit(&mut self, x: &Array<f64>) -> Result<Array<usize>> {
        if x.ndim() != 2 {
            return Err(Error::ShapeMismatch("KMeans expects 2-D data".into()));
        }
        let (n, p) = (x.shape()[0], x.shape()[1]);
        if self.k > n {
            return Err(Error::InvalidArgument("k > n".into()));
        }
        // Initialize with first k points.
        let mut centroids = Array::zeros([self.k, p]);
        for i in 0..self.k {
            for j in 0..p {
                centroids.set_flat(i * p + j, x.get_flat(i * p + j)?)?;
            }
        }
        let mut labels = vec![0usize; n];
        for _ in 0..self.max_iter {
            // Assign.
            let mut changed = false;
            for i in 0..n {
                let mut best = 0usize;
                let mut best_dist = f64::INFINITY;
                for c in 0..self.k {
                    let mut d = 0.0;
                    for j in 0..p {
                        let diff = x.get_flat(i * p + j)? - centroids.get_flat(c * p + j)?;
                        d += diff * diff;
                    }
                    if d < best_dist {
                        best_dist = d;
                        best = c;
                    }
                }
                if labels[i] != best {
                    labels[i] = best;
                    changed = true;
                }
            }
            // Update.
            let mut counts = vec![0usize; self.k];
            let mut new_centroids = Array::zeros([self.k, p]);
            for i in 0..n {
                let c = labels[i];
                counts[c] += 1;
                for j in 0..p {
                    new_centroids.set_flat(c * p + j, new_centroids.get_flat(c * p + j)? + x.get_flat(i * p + j)?)?;
                }
            }
            for c in 0..self.k {
                if counts[c] == 0 {
                    continue; // keep old centroid
                }
                for j in 0..p {
                    new_centroids.set_flat(c * p + j, new_centroids.get_flat(c * p + j)? / counts[c] as f64)?;
                }
            }
            centroids = new_centroids;
            if !changed {
                break;
            }
        }
        self.centroids = Some(centroids);
        Ok(Array::from_vec(labels))
    }
}
