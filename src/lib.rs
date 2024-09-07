mod ast;
mod environment;
mod evaluator;
mod lexer;
mod object;
mod parser;

#[cfg(target_family = "wasm")]
mod wasm;

#[cfg(target_family = "wasm")]
pub use crate::wasm::*;
