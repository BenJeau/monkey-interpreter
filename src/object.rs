use crate::{ast::BlockStatement, environment::Environment};

#[derive(PartialEq, Eq, Debug, Clone)]
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
                format!("fn({}) {{ {} }}", parameters.join(", "), body.to_string())
            }
            Object::Null => "null".into(),
        }
    }
}
