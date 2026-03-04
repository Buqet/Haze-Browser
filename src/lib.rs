//! Haze Browser - A modern web browser built on Servo

pub mod browser;
pub mod config;
pub mod ui;
pub mod window;

/// Browser version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Browser name
pub const NAME: &str = "Haze";
