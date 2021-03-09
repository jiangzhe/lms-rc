//! Rust implementation of Lightweight Modular Staging
//!
//! Original Scala repository: https://github.com/TiarkRompf/virtualization-lms-core
#[macro_use] mod macros;
pub mod ctx;
pub mod def;
pub mod def_arith;
pub mod def_if;
pub mod exp;
pub mod exp_arith;
pub mod exp_loop;
pub mod stmt;
pub mod topo;
pub mod block;
pub mod codegen;
pub mod compile;

pub mod ast;
pub mod stage;