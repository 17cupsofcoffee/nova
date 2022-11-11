#![allow(clippy::new_without_default)]

// ===== Core =====

pub mod app;
pub mod fs;
pub mod graphics;
pub mod input;
pub mod time;
pub mod window;

pub use glam as math;

// ===== Optional =====

#[cfg(feature = "ldtk")]
pub mod ldtk;
