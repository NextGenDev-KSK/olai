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
pub mod fixtures;
pub mod models;
pub mod prompts;
pub mod providers;
pub mod services;
pub mod state;

#[cfg(feature = "app")]
mod commands;
#[cfg(feature = "app")]
mod secrets;

#[cfg(feature = "app")]
pub use commands::run;
