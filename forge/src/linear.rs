//! Linear regression via ordinary least squares.

use foundry::{Error, Result};
use rax::Array;

use crate::Estimator;

#[derive(Debug, Default, Clone)]
pub struct LinearRegression {
    weights: Array<f64>,
    bias: f64,
}

impl LinearRegression {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Estimator for LinearRegression {
    fn fit(&mut self, x: &Array<f64>, y: &Array<f64>) -> Result<()> {
        if x.ndim() != 2 || y.ndim() != 1 || x.shape()[0] != y.shape()[0] {
            return Err(Error::ShapeMismatch(
                "expected x shape (n, p) and y shape (n,)".into(),
            ));
        }
        let (n, p) = (x.shape()[0], x.shape()[1]);
        if n < p {
            return Err(Error::InvalidArgument("n < p: underdetermined".into()));
        }
        // Solve (X^T X) w = X^T y with naive Gaussian elimination (MVP).
        let mut xt_x = Array::zeros([p, p]);
        for i in 0..p {
            for j in 0..p {
                let mut s = 0.0;
                for k in 0..n {
                    s += x.get_flat(k * p + i)? * x.get_flat(k * p + j)?;
                }
                xt_x.set_flat(i * p + j, s)?;
            }
        }
        let mut xt_y = Array::zeros([p]);
        for i in 0..p {
            let mut s = 0.0;
            for k in 0..n {
                s += x.get_flat(k * p + i)? * y.get_flat(k)?;
            }
            xt_y.set_flat(i, s)?;
        }
        self.weights = solve_symmetric(&xt_x, &xt_y)?;
        // bias = mean(y) - mean(x @ w)
        let mut ymean = 0.0;
        let mut ypred = 0.0;
        for k in 0..n {
            ymean += y.get_flat(k)?;
            let mut pred = 0.0;
            for i in 0..p {
                pred += x.get_flat(k * p + i)? * self.weights.get_flat(i)?;
            }
            ypred += pred;
        }
        self.bias = ymean / n as f64 - ypred / n as f64;
        Ok(())
    }

    fn predict(&self, x: &Array<f64>) -> Result<Array<f64>> {
        if x.ndim() != 2 || x.shape()[1] != self.weights.shape()[0] {
            return Err(Error::ShapeMismatch("incompatible shape for prediction".into()));
        }
        let (n, p) = (x.shape()[0], x.shape()[1]);
        let mut y = Array::zeros([n]);
        for k in 0..n {
            let mut pred = self.bias;
            for i in 0..p {
                pred += x.get_flat(k * p + i)? * self.weights.get_flat(i)?;
            }
            y.set_flat(k, pred)?;
        }
        Ok(y)
    }
}

fn solve_symmetric(a: &Array<f64>, b: &Array<f64>) -> Result<Array<f64>> {
    let n = a.shape()[0];
    if a.shape()[1] != n || b.shape()[0] != n {
        return Err(Error::ShapeMismatch("solve: non-square system".into()));
    }
    // Gaussian elimination with partial pivoting.
    let mut aug = Array::zeros([n, n + 1]);
    for i in 0..n {
        for j in 0..n {
            aug.set_flat(i * (n + 1) + j, a.get_flat(i * n + j)?)?;
        }
        aug.set_flat(i * (n + 1) + n, b.get_flat(i)?)?;
    }
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if aug.get_flat(row * (n + 1) + col)?.abs() > aug.get_flat(pivot * (n + 1) + col)?.abs() {
                pivot = row;
            }
        }
        if aug.get_flat(pivot * (n + 1) + col)?.abs() < 1e-12 {
            return Err(Error::InvalidArgument("singular matrix".into()));
        }
        if pivot != col {
            for k in 0..(n + 1) {
                let tmp = aug.get_flat(col * (n + 1) + k)?;
                aug.set_flat(col * (n + 1) + k, aug.get_flat(pivot * (n + 1) + k)?)?;
                aug.set_flat(pivot * (n + 1) + k, tmp)?;
            }
        }
        for row in (col + 1)..n {
            let factor = aug.get_flat(row * (n + 1) + col)? / aug.get_flat(col * (n + 1) + col)?;
            for k in col..(n + 1) {
                let v = aug.get_flat(row * (n + 1) + k)? - factor * aug.get_flat(col * (n + 1) + k)?;
                aug.set_flat(row * (n + 1) + k, v)?;
            }
        }
    }
    let mut x = Array::zeros([n]);
    for i in (0..n).rev() {
        let mut sum = aug.get_flat(i * (n + 1) + n)?;
        for j in ((i + 1)..n).rev() {
            sum -= aug.get_flat(i * (n + 1) + j)? * x.get_flat(j)?;
        }
        let div = aug.get_flat(i * (n + 1) + i)?;
        if div.abs() < 1e-12 {
            return Err(Error::InvalidArgument("singular matrix during back-substitution".into()));
        }
        x.set_flat(i, sum / div)?;
    }
    Ok(x)
}
