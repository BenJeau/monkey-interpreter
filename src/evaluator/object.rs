use crate::{ast::BlockStatement, evaluator::environment::Environment};

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(target_family = "wasm", derive(serde::Serialize))]
#[cfg_attr(
    target_family = "wasm",
    serde(tag = "kind", content = "value", rename_all = "snake_case")
)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Return(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<String>,
        environment: Environment,
        body: BlockStatement,
    },
    Null,
}

impl Object {
    pub fn kind(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::Return(_) => "RETURN",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION",
            Object::Null => "NULL",
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Return(value) => value.inspect(),
            Object::Error(value) => format!("Error: {}", value),
            Object::Function {
                parameters, body, ..
            } => {
                format!("fn({}) {{ {body} }}", parameters.join(", "))
            }
            Object::Null => "null".into(),
        }
    }
}
