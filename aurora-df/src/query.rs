//! Query DSL (MVP).

use foundry::Result;

use crate::DataFrame;

/// Column reference.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub name: String,
}

impl Expr {
    pub fn col(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn eq(self, value: f64) -> Filter {
        Filter::Eq(self.name, value)
    }

    pub fn gt(self, value: f64) -> Filter {
        Filter::Gt(self.name, value)
    }

    pub fn lt(self, value: f64) -> Filter {
        Filter::Lt(self.name, value)
    }
}

/// Predicate filter.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Eq(String, f64),
    Gt(String, f64),
    Lt(String, f64),
    And(Box<Filter>, Box<Filter>),
}

impl Filter {
    pub fn and(self, other: Filter) -> Filter {
        Filter::And(Box::new(self), Box::new(other))
    }

    pub fn apply(&self, df: &DataFrame) -> Result<Vec<bool>> {
        let rows = df.shape().0;
        let mut mask = vec![true; rows];
        match self {
            Filter::Eq(name, value) => {
                let col = df.get(name)?;
                for i in 0..rows {
                    mask[i] &= (col.get(i)? - value).abs() < 1e-9;
                }
            }
            Filter::Gt(name, value) => {
                let col = df.get(name)?;
                for i in 0..rows {
                    mask[i] &= col.get(i)? > *value;
                }
            }
            Filter::Lt(name, value) => {
                let col = df.get(name)?;
                for i in 0..rows {
                    mask[i] &= col.get(i)? < *value;
                }
            }
            Filter::And(a, b) => {
                let left = a.apply(df)?;
                let right = b.apply(df)?;
                for i in 0..rows {
                    mask[i] = left[i] && right[i];
                }
            }
        }
        Ok(mask)
    }
}

/// Apply a filter to a dataframe and return the filtered frame.
pub fn filter(df: &DataFrame, f: Filter) -> Result<DataFrame> {
    let mask = f.apply(df)?;
    df.filter(&mask)
}
