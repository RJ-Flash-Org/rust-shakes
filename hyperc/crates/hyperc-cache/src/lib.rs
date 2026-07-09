#![forbid(unsafe_code)]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildRequest {
    pub cwd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub ok: bool,
    pub elapsed_ms: u128,
    pub stdout: String,
    pub stderr: String,
}

pub fn ping() -> &'static str { "hyperc-cache ready" }
