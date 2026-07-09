//! `foundry` — shared numerics, dtypes, errors, traits, and layout for rust-shakes.

use std::fmt;
use std::result;

pub mod dtype;
pub mod layout;

pub use dtype::DType;
pub use layout::{Layout, Shape};

/// Common error type for the rust-shakes ecosystem.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ShapeMismatch(String),
    InvalidArgument(String),
    IndexOutOfBounds(String),
    Parse(String),
    Io(String),
    Unsupported(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ShapeMismatch(s) => write!(f, "shape mismatch: {}", s),
            Error::InvalidArgument(s) => write!(f, "invalid argument: {}", s),
            Error::IndexOutOfBounds(s) => write!(f, "index out of bounds: {}", s),
            Error::Parse(s) => write!(f, "parse error: {}", s),
            Error::Io(s) => write!(f, "io error: {}", s),
            Error::Unsupported(s) => write!(f, "unsupported: {}", s),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}

/// Result alias.
pub type Result<T> = result::Result<T, Error>;

/// A numeric type supported by the ecosystem.
pub trait Numeric: Copy + Default + fmt::Debug + 'static {}

impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let e = Error::ShapeMismatch("bad".into());
        assert_eq!(e.to_string(), "shape mismatch: bad");
    }
}
