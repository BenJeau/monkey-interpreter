use crate::{lexer, parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseProgram)]
pub fn parse_program(input: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let lexer = lexer::Lexer::new(input.into());

    let mut parser = parser::Parser::new(lexer);
    let Some(program) = parser.parse_program() else {
        return JsValue::null();
    };

    if parser.errors.len() > 0 {
        return JsValue::from_str(&parser.errors.join("\n"));
    }

    JsValue::from_str(&program.to_string())
}
