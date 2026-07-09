//! JSON helpers.

use foundry::Result;
use std::collections::HashMap;

/// Parse a JSON array of objects into a row map (MVP: strings only).
pub fn parse_json_objects(text: &str) -> Result<Vec<HashMap<String, String>>> {
    // Phase-1: minimal parser; replace with serde_json later.
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    Err(foundry::Error::Parse("full JSON parser not yet implemented".into()))
}
