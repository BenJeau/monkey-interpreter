use std::collections::BTreeMap;

use crate::token::Token;

#[derive(PartialEq, Eq, Debug, Clone, Ord, PartialOrd)]
#[cfg_attr(target_family = "wasm", derive(serde::Serialize))]
#[cfg_attr(
    target_family = "wasm",
    serde(tag = "kind", content = "value", rename_all = "snake_case")
)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    Expression { value: Expression },
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Let { name, value } => write!(f, "let {name} = {value};"),
            Self::Return { value } => write!(f, "return {value};"),
            Self::Expression { value } => write!(f, "{value}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Ord, PartialOrd)]
#[cfg_attr(target_family = "wasm", derive(serde::Serialize))]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Ord, PartialOrd)]
#[cfg_attr(target_family = "wasm", derive(serde::Serialize))]
#[cfg_attr(
    target_family = "wasm",
    serde(tag = "kind", content = "value", rename_all = "snake_case")
)]
pub enum Expression {
    Integer(isize),
    Identifier(String),
    Boolean(bool),
    String(String),
    PrefixOperator {
        operator: Token,
        expression: Box<Expression>,
    },
    InfixOperator {
        operator: Token,
        rh_expression: Box<Expression>,
        lh_expression: Box<Expression>,
    },
    FunctionCall {
        name: Box<Expression>,
        arguments: Vec<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    Function {
        arguments: Vec<String>,
        body: BlockStatement,
    },
    Array(Vec<Expression>),
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    #[cfg_attr(target_family = "wasm", serde(with = "btreemap_as_list"))]
    HashLiteral(BTreeMap<Expression, Expression>),
}

#[cfg(target_family = "wasm")]
mod btreemap_as_list {
    use serde::ser::SerializeSeq;
    use serde::{Serialize, Serializer};
    use std::collections::BTreeMap;

    #[derive(Serialize)]
    struct Entry<K: Serialize, V: Serialize> {
        key: K,
        value: V,
    }

    pub fn serialize<S, K, V>(map: &BTreeMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        K: Serialize + Ord,
        V: Serialize,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (key, value) in map {
            seq.serialize_element(&Entry { key, value })?;
        }
        seq.end()
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Identifier(value) => write!(f, "{value}"),
            Self::Boolean(value) => {
                if *value {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Self::String(value) => write!(f, "{value}"),
            Self::PrefixOperator {
                operator,
                expression,
            } => {
                write!(f, "({operator}{expression})")
            }
            Self::InfixOperator {
                operator,
                rh_expression,
                lh_expression,
            } => write!(f, "({lh_expression} {operator} {rh_expression})"),
            Self::FunctionCall { name, arguments } => {
                write!(f, "{name}(")?;
                for (index, argument) in arguments.iter().enumerate() {
                    write!(f, "{argument}")?;
                    if index != arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            Self::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if ({condition}) {{{consequence}}}")?;
                if let Some(alternative) = alternative {
                    write!(f, "else {{{alternative}}}")?;
                }
                Ok(())
            }
            Self::Function { arguments, body } => {
                write!(f, "fn({}) {{{body}}}", arguments.join(", "))
            }
            Self::Array(elements) => {
                write!(f, "[")?;
                for (index, element) in elements.iter().enumerate() {
                    write!(f, "{element}")?;
                    if index != elements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Self::Index { left, index } => {
                write!(f, "({left}[{index}])")
            }
            Self::HashLiteral(elements) => {
                write!(f, "{{")?;
                for (index, (key, value)) in elements.iter().enumerate() {
                    write!(f, "{key}: {value}")?;
                    if index != elements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

impl From<isize> for Expression {
    fn from(value: isize) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self::Identifier(value)
    }
}

impl From<bool> for Expression {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
