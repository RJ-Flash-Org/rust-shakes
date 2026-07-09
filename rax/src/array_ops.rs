//! Array operations (MVP: matrix multiplication).

use foundry::{Error, Result};
use crate::Array;

/// Matrix multiplication trait.
pub trait Matmul<Rhs = Self> {
    type Output;
    fn matmul(&self, rhs: &Rhs) -> Result<Self::Output>;
}

impl Matmul for Array<f64> {
    type Output = Array<f64>;

    fn matmul(&self, rhs: &Array<f64>) -> Result<Array<f64>> {
        if self.ndim() != 2 || rhs.ndim() != 2 {
            return Err(Error::ShapeMismatch("matmul expects 2-D arrays".into()));
        }
        let (m, k1) = (self.shape()[0], self.shape()[1]);
        let (k2, n) = (rhs.shape()[0], rhs.shape()[1]);
        if k1 != k2 {
            return Err(Error::ShapeMismatch(format!(
                "matmul inner dimensions do not match: {} vs {}",
                k1, k2
            )));
        }
        let mut out = Array::zeros([m, n]);
        for i in 0..m {
            for j in 0..n {
                let mut s = 0.0;
                for k in 0..k1 {
                    s += self.get_flat(i * k1 + k)? * rhs.get_flat(k * n + j)?;
                }
                out.set_flat(i * n + j, s)?;
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matmul_2d() {
        let a = Array::from_shape_vec([2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = Array::from_shape_vec([3, 2], vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
        let c = a.matmul(&b).unwrap();
        assert_eq!(c.shape(), &[2, 2]);
        assert_eq!(c.get_flat(0).unwrap(), 4.0);
        assert_eq!(c.get_flat(1).unwrap(), 5.0);
    }
}
