//! `quarry` — connectors and IO for rust-shakes.
//!
//! Phase-1 MVP: local CSV/JSON readers and writers.

use foundry::Result;
use rax::Array;
use std::fs;
use std::path::Path;

pub mod csv;
pub mod json;

pub use csv::read_csv_f64;

/// Read a CSV file into an f64 array (MVP: no header, comma-separated).
pub fn read_csv(path: impl AsRef<Path>, has_header: bool) -> Result<Array<f64>> {
    read_csv_f64(path, has_header)
}

/// Write a 1-D array of strings to a CSV row (MVP).
pub fn write_csv_string(path: impl AsRef<Path>, rows: &[Vec<String>]) -> Result<()> {
    let mut out = String::new();
    for row in rows {
        out.push_str(&row.join(","));
        out.push('\n');
    }
    fs::write(path, out).map_err(|e| foundry::Error::Io(e.to_string()))?;
    Ok(())
}
