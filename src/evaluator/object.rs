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
    String(String),
    Return(Box<Self>),
    Error(String),
    Function {
        parameters: Vec<String>,
        environment: Environment,
        body: BlockStatement,
    },
    Builtin(fn(&[Self]) -> Option<Self>),
    Array(Vec<Object>),
    Null,
}

impl Object {
    pub fn kind(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::Return(_) => "RETURN",
            Object::Error(_) => "ERROR",
            Object::Function { .. } => "FUNCTION",
            Object::Builtin(_) => "BUILTIN",
            Object::Array(_) => "ARRAY",
            Object::Null => "NULL",
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::String(value) => value.to_string(),
            Object::Return(value) => value.inspect(),
            Object::Error(value) => format!("Error: {}", value),
            Object::Function {
                parameters, body, ..
            } => {
                format!("fn({}) {{ {body} }}", parameters.join(", "))
            }
            Object::Builtin(_) => "builtin function".into(),
            Object::Array(elements) => {
                format!(
                    "[{}]",
                    elements
                        .iter()
                        .map(|element| element.inspect())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Object::Null => "null".into(),
        }
    }
}
