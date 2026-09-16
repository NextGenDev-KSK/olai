//! Olai backend library.
//!
//! All logic lives here (not in `main.rs`) so it can be unit- and
//! integration-tested without the Tauri runtime. The optional `app` feature
//! adds the Tauri command/glue layer for the real desktop build.
//!
//! Layering: `commands` (thin handlers) → `services` (logic) → `providers`
//! (the `LlmProvider` trait with real and mock implementations).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod models;
pub mod services;
