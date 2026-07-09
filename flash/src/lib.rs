//! `flash` — web APIs and model serving for rust-shakes.
//!
//! Phase-1 MVP: request/response traits and a simple in-memory router.

use foundry::Result;
use std::collections::HashMap;

pub mod router;

pub use router::{Handler, Router};

/// HTTP method (MVP subset).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    #[default]
    Get,
    Post,
}

/// Minimal request abstraction.
#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Minimal response abstraction.
#[derive(Debug, Default, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }
}

/// Route a request through the global router.
pub async fn handle(req: Request) -> Result<Response> {
    Router::new().handle(&req).await
}
