//! `scribe` — docs, examples, and notebook-style runners.
//!
//! Phase-1 MVP: example runner and markdown code extractor.

use foundry::Result;
use std::fs;
use std::path::Path;

pub mod runner;

pub use runner::run_rust_blocks;

/// Extract fenced Rust code blocks from a Markdown file.
pub fn extract_rust_code(path: impl AsRef<Path>) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|e| foundry::Error::Io(e.to_string()))?;
    let mut blocks = Vec::new();
    let mut inside = false;
    let mut current = String::new();
    for line in text.lines() {
        if line.trim_start().starts_with("```rust") {
            inside = true;
            current.clear();
        } else if line.trim_start().starts_with("```") && inside {
            inside = false;
            blocks.push(current.trim().to_string());
            current.clear();
        } else if inside {
            current.push_str(line);
            current.push('\n');
        }
    }
    Ok(blocks)
}
