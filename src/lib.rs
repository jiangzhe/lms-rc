//! Rust implementation of Lightweight Modular Staging
//!
//! Original Scala repository: https://github.com/TiarkRompf/virtualization-lms-core
//!
//! This project is also largely inspired by Weld: https://github.com/weld-project/weld
#[macro_use]
pub mod error;
pub mod ast;
pub mod codegen;
pub mod sir;
pub mod stage;
pub mod sym;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;
