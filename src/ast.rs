use crate::lexer::Token;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    Expression { value: Expression },
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Self::Let { name, value } => format!("let {name} = {};", value.to_string()),
            Self::Return { value } => format!("return {};", value.to_string()),
            Self::Expression { value } => value.to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    Integer(isize),
    Identifier(String),
    Boolean(bool),
    Null,
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
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(value) => format!("{value}"),
            Self::Identifier(value) => value.into(),
            Self::Boolean(value) => {
                if *value {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            Self::Null => "null".into(),
            Self::PrefixOperator {
                operator,
                expression,
            } => format!("({}{})", operator.to_string(), expression.to_string()),
            Self::InfixOperator {
                operator,
                rh_expression,
                lh_expression,
            } => format!(
                "({} {} {})",
                lh_expression.to_string(),
                operator.to_string(),
                rh_expression.to_string(),
            ),
            Self::FunctionCall { name, arguments } => {
                format!(
                    "{name}({})",
                    arguments
                        .iter()
                        .map(|argument| argument.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            Self::If {
                condition,
                consequence,
                alternative,
            } => format!(
                "if{} {}{}",
                condition.to_string(),
                consequence.to_string(),
                match alternative {
                    Some(alternative) => format!("else {}", alternative.to_string()),
                    None => "".into(),
                }
            ),
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
