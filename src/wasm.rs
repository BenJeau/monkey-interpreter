use crate::{ast, environment, evaluator, lexer, parser};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Default)]
struct EvaluationResult {
    statements: Vec<ast::Statement>,
    program: String,
    errors: Vec<String>,
    environment: Option<environment::Environment>,
    output: Option<String>,
}

#[wasm_bindgen(skip_typescript)]
pub fn execute(input: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let lexer = lexer::Lexer::new(input.into());
    let mut parser = parser::Parser::new(lexer);

    let mut result = EvaluationResult::default();

    let Some(program) = parser.parse_program() else {
        return serde_wasm_bindgen::to_value(&result).unwrap();
    };

    result.program = program.to_string();

    if parser.errors.len() > 0 {
        result.statements = program.statements;
        result.errors = parser.errors;
        return serde_wasm_bindgen::to_value(&result).unwrap();
    }

    let mut environment = environment::Environment::new();
    let output = evaluator::eval_program(&program, &mut environment);

    result.statements = program.statements;
    result.errors = parser.errors;
    result.environment = Some(environment);
    result.output = output.map(|output| output.inspect());

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen(js_name = lexer, skip_typescript)]
pub fn lexer_tokenizer(input: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let tokens = lexer::Lexer::new(input.into())
        .into_iter()
        .collect::<Vec<_>>();

    serde_wasm_bindgen::to_value(&tokens).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = include_str!("monkey_interpreter.d.ts");
