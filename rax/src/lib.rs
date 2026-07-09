//! `rax` — n-dimensional arrays and tensors for rust-shakes.
//!
//! Phase-1 MVP: dense row-major arrays with shape, views, and basic math.

use foundry::{Error, Layout, Numeric, Result, Shape};
pub mod array_ops;

pub use array_ops::Matmul;

/// Dense, row-major n-dimensional array.
#[derive(Debug, Clone, PartialEq)]
pub struct Array<T: Numeric> {
    shape: Shape,
    data: Vec<T>,
}

impl<T: Numeric> Default for Array<T> {
    fn default() -> Self {
        Self {
            shape: Shape::default(),
            data: Vec::new(),
        }
    }
}

impl<T: Numeric> Array<T> {
    /// Create an array from a shape and a data vector.
    pub fn from_shape_vec(shape: impl Into<Shape>, data: Vec<T>) -> Result<Self> {
        let shape = shape.into();
        let expected = shape.len();
        if data.len() != expected {
            return Err(Error::ShapeMismatch(format!(
                "data length {} does not match shape length {}",
                data.len(), expected
            )));
        }
        Ok(Self { shape, data })
    }

    /// Create a 1-D array from a vector.
    pub fn from_vec(data: Vec<T>) -> Self {
        let shape = Shape::new([data.len()]);
        Self { shape, data }
    }

    /// Create an array filled with zeros.
    pub fn zeros(shape: impl Into<Shape>) -> Self {
        let shape = shape.into();
        let len = shape.len();
        Self {
            shape,
            data: vec![T::default(); len],
        }
    }

    /// Return an array of zeros with the same shape.
    pub fn zeros_like(&self) -> Self {
        Self::zeros(self.shape.clone())
    }

    /// Number of dimensions.
    pub fn ndim(&self) -> usize {
        self.shape.ndim()
    }

    /// Shape as a slice.
    pub fn shape(&self) -> &[usize] {
        self.shape.dims()
    }

    /// Total number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a flat element by index.
    pub fn get_flat(&self, index: usize) -> Result<T> {
        self.data
            .get(index)
            .copied()
            .ok_or_else(|| Error::IndexOutOfBounds(format!("flat index {} out of range {}", index, self.len())))
    }

    /// Set a flat element by index.
    pub fn set_flat(&mut self, index: usize, value: T) -> Result<()> {
        if index >= self.len() {
            return Err(Error::IndexOutOfBounds(format!(
                "flat index {} out of range {}",
                index, self.len()
            )));
        }
        self.data[index] = value;
        Ok(())
    }

    /// Get an element by multi-dimensional indices.
    pub fn get_at(&self, indices: &[usize]) -> Result<T> {
        let flat = self.shape.ravel(indices)?;
        self.get_flat(flat)
    }

    /// Set an element by multi-dimensional indices.
    pub fn set_at(&mut self, indices: &[usize], value: T) -> Result<()> {
        let flat = self.shape.ravel(indices)?;
        self.set_flat(flat, value)
    }

    /// Get a flat element by index (1-D convenience).
    pub fn get(&self, index: usize) -> Result<T> {
        self.get_flat(index)
    }

    /// Set a flat element by index (1-D convenience).
    pub fn set(&mut self, index: usize, value: T) -> Result<()> {
        self.set_flat(index, value)
    }

    /// Reshape the array (MVP: copy).
    pub fn reshape(&self, new_shape: impl Into<Shape>) -> Result<Self> {
        let new_shape = new_shape.into();
        if new_shape.len() != self.len() {
            return Err(Error::ShapeMismatch("reshape length mismatch".into()));
        }
        Ok(Self {
            shape: new_shape,
            data: self.data.clone(),
        })
    }

    /// A read-only view of the underlying data.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// A mutable view of the underlying data.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Element-wise apply `f`.
    pub fn map<F>(&self, mut f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        let mut out = self.clone();
        for v in out.data.iter_mut() {
            *v = f(*v);
        }
        out
    }
}

impl<T: Numeric> Layout for Array<T> {
    fn shape(&self) -> &[usize] {
        self.shape.dims()
    }
}

impl Array<f64> {
    /// Random normal matrix with mean 0, std 1 (MVP: simple deterministic PRNG).
    pub fn randn(shape: impl Into<Shape>) -> Self {
        let shape = shape.into();
        let len = shape.len();
        // A simple linear congruential generator for reproducibility.
        let mut seed = 0x12345678u64;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            // Box-Muller-ish transform from uniform u32 to normal-ish.
            let u1 = ((seed >> 16) as u32) as f64 / u32::MAX as f64;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let u2 = ((seed >> 16) as u32) as f64 / u32::MAX as f64;
            let r = (-2.0 * u1.ln()).sqrt();
            let theta = 2.0 * std::f64::consts::PI * u2;
            data.push(r * theta.cos());
        }
        Self { shape, data }
    }
}

macro_rules! impl_elementwise_op {
    ($t:ty) => {
        impl std::ops::Add<&Array<$t>> for &Array<$t> {
            type Output = Array<$t>;
            fn add(self, rhs: &Array<$t>) -> Array<$t> {
                elementwise_op(self, rhs, |a, b| a + b)
            }
        }

        impl std::ops::Sub<&Array<$t>> for &Array<$t> {
            type Output = Array<$t>;
            fn sub(self, rhs: &Array<$t>) -> Array<$t> {
                elementwise_op(self, rhs, |a, b| a - b)
            }
        }

        impl std::ops::Mul<&Array<$t>> for &Array<$t> {
            type Output = Array<$t>;
            fn mul(self, rhs: &Array<$t>) -> Array<$t> {
                elementwise_op(self, rhs, |a, b| a * b)
            }
        }

        impl std::ops::Mul<$t> for Array<$t> {
            type Output = Array<$t>;
            fn mul(self, rhs: $t) -> Array<$t> {
                let mut out = self.clone();
                for v in out.data.iter_mut() {
                    *v = *v * rhs;
                }
                out
            }
        }

        impl std::ops::Mul<$t> for &Array<$t> {
            type Output = Array<$t>;
            fn mul(self, rhs: $t) -> Array<$t> {
                self.clone() * rhs
            }
        }

        impl std::ops::Mul<Array<$t>> for $t {
            type Output = Array<$t>;
            fn mul(self, rhs: Array<$t>) -> Array<$t> {
                rhs * self
            }
        }
    };
}

impl_elementwise_op!(f64);
impl_elementwise_op!(usize);

fn elementwise_op<T: Numeric>(a: &Array<T>, b: &Array<T>, op: fn(T, T) -> T) -> Array<T> {
    let mut out = a.clone();
    if a.shape() == b.shape() {
        for i in 0..a.len() {
            out.data[i] = op(a.data[i], b.data[i]);
        }
    } else if b.len() == 1 {
        let scalar = b.data[0];
        for i in 0..a.len() {
            out.data[i] = op(a.data[i], scalar);
        }
    } else if a.len() == 1 {
        let scalar = a.data[0];
        out = b.clone();
        for i in 0..b.len() {
            out.data[i] = op(scalar, b.data[i]);
        }
    }
    out
}

impl<T: Numeric> From<Vec<T>> for Array<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_basics() {
        let a = Array::from_shape_vec([2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert_eq!(a.shape(), &[2, 3]);
        assert_eq!(a.get_flat(4).unwrap(), 5.0);
        assert_eq!(a.get_at(&[1, 1]).unwrap(), 5.0);
    }

    #[test]
    fn elementwise_mul() {
        let a = Array::from_vec(vec![1.0, 2.0, 3.0]);
        let b = a.clone() * 2.0;
        assert_eq!(b.as_slice(), &[2.0, 4.0, 6.0]);
    }
}
