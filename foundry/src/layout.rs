//! Layout and shape primitives.

use crate::{Error, Result};

/// Shape of an n-dimensional array.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Shape(pub Vec<usize>);

impl Shape {
    pub fn new(dims: impl Into<Vec<usize>>) -> Self {
        Self(dims.into())
    }

    pub fn ndim(&self) -> usize {
        self.0.len()
    }

    pub fn len(&self) -> usize {
        self.0.iter().product()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn dims(&self) -> &[usize] {
        &self.0
    }

    /// Row-major default strides for this shape.
    pub fn strides(&self) -> Vec<usize> {
        let mut strides = vec![0usize; self.ndim()];
        let mut prod = 1usize;
        for i in (0..self.ndim()).rev() {
            strides[i] = prod;
            prod *= self.0[i];
        }
        strides
    }

    /// Convert flat index to multi-dimensional indices.
    pub fn unravel(&self, mut index: usize) -> Result<Vec<usize>> {
        let len = self.len();
        if index >= len && len > 0 {
            return Err(Error::IndexOutOfBounds(format!(
                "flat index {} out of range {}",
                index, len
            )));
        }
        let mut out = Vec::with_capacity(self.ndim());
        for &dim in self.0.iter().rev() {
            out.push(index % dim);
            index /= dim;
        }
        out.reverse();
        Ok(out)
    }

    /// Convert multi-dimensional indices to flat index.
    pub fn ravel(&self, indices: &[usize]) -> Result<usize> {
        if indices.len() != self.ndim() {
            return Err(Error::ShapeMismatch("indices length does not match ndim".into()));
        }
        let strides = self.strides();
        let mut flat = 0usize;
        for (i, &idx) in indices.iter().enumerate() {
            if idx >= self.0[i] {
                return Err(Error::IndexOutOfBounds(format!(
                    "index {} out of range for dim {}",
                    idx, self.0[i]
                )));
            }
            flat += idx * strides[i];
        }
        Ok(flat)
    }
}

impl From<Vec<usize>> for Shape {
    fn from(v: Vec<usize>) -> Self {
        Self(v)
    }
}

impl From<&[usize]> for Shape {
    fn from(v: &[usize]) -> Self {
        Self(v.to_vec())
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(v: [usize; N]) -> Self {
        Self(v.to_vec())
    }
}

/// Layout trait for arrays and tensors.
pub trait Layout {
    fn shape(&self) -> &[usize];
    fn ndim(&self) -> usize {
        self.shape().len()
    }
    fn len(&self) -> usize {
        self.shape().iter().product()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_strides() {
        let s = Shape::new([2, 3, 4]);
        assert_eq!(s.strides(), vec![12, 4, 1]);
        assert_eq!(s.ravel(&[1, 2, 3]).unwrap(), 23);
        assert_eq!(s.unravel(23).unwrap(), vec![1, 2, 3]);
    }
}
