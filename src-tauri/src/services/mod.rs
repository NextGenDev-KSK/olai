//! Business-logic services. Each module is pure and independently testable;
//! none reach the network or the Tauri runtime directly.

pub mod dates;
pub mod extraction;
pub mod injection;
pub mod pii;
pub mod safety;
pub mod segmentation;
pub mod verification;
