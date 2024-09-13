#![allow(dead_code)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
pub use crate::wasm::*;
