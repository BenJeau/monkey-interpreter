use crate::{environment, evaluator, lexer, parser};
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

#[wasm_bindgen(js_name = evalProgram)]
pub fn eval_program(input: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let lexer = lexer::Lexer::new(input.into());

    let mut parser = parser::Parser::new(lexer);
    let Some(program) = parser.parse_program() else {
        return JsValue::null();
    };

    if parser.errors.len() > 0 {
        return JsValue::from_str(&parser.errors.join("\n"));
    }

    let mut environment = environment::Environment::new();

    let Some(evaluated) = evaluator::eval_program(&program, &mut environment) else {
        return JsValue::null();
    };

    JsValue::from_str(&evaluated.inspect())
}
