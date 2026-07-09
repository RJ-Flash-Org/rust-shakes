//! Columnar DataFrame.

use foundry::{Error, Result};
use rax::Array;

use crate::Series;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DataFrame {
    columns: Vec<Series>,
    names: Vec<String>,
}

impl DataFrame {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(columns: Vec<Series>) -> Result<Self> {
        let names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
        let len = columns.first().map(|c| c.len()).unwrap_or(0);
        for c in &columns {
            if c.len() != len {
                return Err(Error::ShapeMismatch("column lengths differ".into()));
            }
        }
        Ok(Self { columns, names })
    }

    pub fn shape(&self) -> (usize, usize) {
        let rows = self.columns.first().map(|c| c.len()).unwrap_or(0);
        (rows, self.columns.len())
    }

    pub fn columns(&self) -> &[Series] {
        &self.columns
    }

    pub fn column_names(&self) -> &[String] {
        &self.names
    }

    pub fn get(&self, name: &str) -> Result<&Series> {
        self.columns
            .iter()
            .find(|c| c.name == name)
            .ok_or_else(|| Error::InvalidArgument(format!("column '{}' not found", name)))
    }

    pub fn select(&self, names: &[&str]) -> Result<Self> {
        let mut cols = Vec::new();
        for &name in names {
            cols.push(self.get(name)?.clone());
        }
        Self::new(cols)
    }

    pub fn filter(&self, mask: &[bool]) -> Result<Self> {
        let mut cols = Vec::new();
        for c in &self.columns {
            cols.push(c.filter(mask)?);
        }
        Self::new(cols)
    }

    pub fn add_column(&mut self, series: Series) -> Result<()> {
        let rows = self.shape().0;
        if rows > 0 && series.len() != rows {
            return Err(Error::ShapeMismatch("new column length mismatch".into()));
        }
        self.names.push(series.name.clone());
        self.columns.push(series);
        Ok(())
    }

    /// Build a matrix from numeric columns (MVP: f64 only).
    pub fn to_matrix(&self) -> Result<Array<f64>> {
        if self.columns.is_empty() {
            return Ok(Array::zeros([0, 0]));
        }
        let rows = self.shape().0;
        let cols = self.columns.len();
        let mut data = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in &self.columns {
                data.push(c.get(r)?);
            }
        }
        Array::from_shape_vec([rows, cols], data)
    }
}
