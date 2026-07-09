//! Simple in-memory router.

use foundry::Result;
use std::collections::HashMap;

use crate::{Method, Request, Response};

/// Request handler trait.
pub trait Handler: Send + Sync {
    fn handle(&self, req: &Request) -> Result<Response>;
}

/// In-memory router.
#[derive(Default)]
pub struct Router {
    routes: HashMap<(Method, String), Box<dyn Handler>>,
}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router").field("routes", &self.routes.len()).finish()
    }
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<H: Handler + 'static>(mut self, method: Method, path: impl Into<String>, handler: H) -> Self {
        self.routes.insert((method, path.into()), Box::new(handler));
        self
    }

    pub async fn handle(&self, req: &Request) -> Result<Response> {
        match self.routes.get(&(req.method, req.path.clone())) {
            Some(h) => h.handle(req),
            None => Ok(Response {
                status: 404,
                headers: HashMap::new(),
                body: b"not found".to_vec(),
            }),
        }
    }
}
