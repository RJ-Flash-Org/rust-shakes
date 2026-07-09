//! A typed column (Series) backed by an array.

use foundry::{DType, Error, Result};
use rax::Array;

#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub name: String,
    pub dtype: DType,
    pub data: Array<f64>,
}

impl Series {
    pub fn new(name: impl Into<String>, data: Array<f64>) -> Self {
        Self {
            name: name.into(),
            dtype: DType::F64,
            data,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, i: usize) -> Result<f64> {
        self.data.get(i)
    }

    pub fn as_slice(&self) -> &[f64] {
        self.data.as_slice()
    }

    pub fn sum(&self) -> Result<f64> {
        let mut s = 0.0;
        for i in 0..self.len() {
            s += self.get(i)?;
        }
        Ok(s)
    }

    pub fn mean(&self) -> Result<f64> {
        if self.is_empty() {
            return Ok(0.0);
        }
        Ok(self.sum()? / self.len() as f64)
    }

    pub fn filter(&self, mask: &[bool]) -> Result<Self> {
        if mask.len() != self.len() {
            return Err(Error::ShapeMismatch("mask length mismatch".into()));
        }
        let mut v = Vec::new();
        for i in 0..self.len() {
            if mask[i] {
                v.push(self.get(i)?);
            }
        }
        Ok(Self::new(self.name.clone(), Array::from_vec(v)))
    }
}
