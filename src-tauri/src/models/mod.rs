//! Domain types shared between Rust and the frontend.
//!
//! Every public type derives [`ts_rs::TS`] and is exported to
//! `src/lib/bindings/` so the TypeScript client never hand-duplicates a shape.

mod analysis;
mod claim;
mod document;
mod safety;
mod session;

pub use analysis::*;
pub use claim::*;
pub use document::*;
pub use safety::*;
pub use session::*;
