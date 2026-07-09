//! CSV helpers.

use foundry::Result;
use rax::Array;
use std::fs;
use std::path::Path;

/// Read a CSV file into an f64 matrix (MVP: no header, comma-separated).
pub fn read_csv_f64(path: impl AsRef<Path>, has_header: bool) -> Result<Array<f64>> {
    let text = fs::read_to_string(path).map_err(|e| foundry::Error::Io(e.to_string()))?;
    let mut rows = Vec::new();
    let mut ncols = 0usize;
    for (i, line) in text.lines().enumerate() {
        if has_header && i == 0 {
            continue;
        }
        let cols: Result<Vec<f64>> = line
            .split(',')
            .map(|s| s.trim().parse::<f64>().map_err(|_| foundry::Error::Parse(s.into())))
            .collect();
        let cols = cols?;
        if ncols == 0 {
            ncols = cols.len();
        } else if cols.len() != ncols {
            return Err(foundry::Error::Parse("inconsistent row length".into()));
        }
        rows.extend(cols);
    }
    let nrows = rows.len() / ncols;
    Array::from_shape_vec([nrows, ncols], rows)
}
