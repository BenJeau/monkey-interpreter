use crate::ast::expression::Expression;

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

#[derive(PartialEq, Eq, Debug, Clone, Ord, PartialOrd, Default)]
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
