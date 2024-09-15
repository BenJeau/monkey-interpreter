use std::collections::BTreeMap;

use crate::{ast::BlockStatement, evaluator::environment::Environment};

pub const NULL: Object = Object::Null;
pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);

#[derive(PartialEq, Eq, Debug, Clone, Default, Ord, PartialOrd)]
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
    #[cfg_attr(target_family = "wasm", serde(skip))]
    Builtin(fn(&[Self]) -> Option<Self>),
    Array(Vec<Object>),
    Hash(BTreeMap<Object, Object>),
    #[default]
    Null,
}

impl Object {
    pub fn is_truthy(self) -> bool {
        !matches!(self, FALSE | NULL)
    }

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
            Object::Hash(_) => "HASH",
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
            Object::Hash(elements) => {
                format!(
                    "{{{}}}",
                    elements
                        .iter()
                        .map(|(key, value)| format!("{}: {}", key.inspect(), value.inspect()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Object::Null => "null".into(),
        }
    }
}

impl From<isize> for Object {
    fn from(value: isize) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        if value {
            TRUE
        } else {
            FALSE
        }
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<Object>> for Object {
    fn from(value: Vec<Object>) -> Self {
        Self::Array(value)
    }
}

impl From<BTreeMap<Object, Object>> for Object {
    fn from(value: BTreeMap<Object, Object>) -> Self {
        Self::Hash(value)
    }
}
