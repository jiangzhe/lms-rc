//! Rust implementation of Lightweight Modular Staging
//!
//! Original Scala repository: https://github.com/TiarkRompf/virtualization-lms-core
pub mod ctx;
pub mod def;
pub mod def_arith;
pub mod def_if;
pub mod exp;
pub mod exp_arith;
pub mod exp_loop;
pub mod stmt;
pub mod unit;
pub mod topo;
pub mod block;
pub mod codegen;
pub mod compile;