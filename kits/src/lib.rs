//! `kits` — domain accelerators for rust-shakes.
//!
//! Phase-1 MVP: submodules for NLP, time-series, vision, and graphs.

pub mod nlp;
pub mod ts;
pub mod vision;
pub mod graph;

/// Kit version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
