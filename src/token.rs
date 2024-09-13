use crate::parser::ExpressionPrecedence;

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(target_family = "wasm", derive(serde::Serialize))]
#[cfg_attr(
    target_family = "wasm",
    serde(tag = "kind", content = "value", rename_all = "snake_case")
)]
pub enum Token {
    // Literals
    Integer(isize),
    String(String),

    // Identifiers
    Identifier(String),

    // Operators
    EqualSign,
    PlusSign,
    MinusSign,
    ExclamationMark,
    Asterisk,
    Slash,

    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    True,
    False,
    If,
    Else,

    // Statements
    Let,
    Return,

    // Other
    Eof,
    Illegal(char),
}

impl Token {
    pub fn precedence(&self) -> ExpressionPrecedence {
        match self {
            Token::PlusSign => ExpressionPrecedence::Sum,
            Token::MinusSign => ExpressionPrecedence::Sum,
            Token::Asterisk => ExpressionPrecedence::Product,
            Token::Slash => ExpressionPrecedence::Product,
            Token::LessThan => ExpressionPrecedence::LessGreater,
            Token::GreaterThan => ExpressionPrecedence::LessGreater,
            Token::Equal => ExpressionPrecedence::Equals,
            Token::NotEqual => ExpressionPrecedence::Equals,
            Token::LeftParen => ExpressionPrecedence::Call,
            _ => ExpressionPrecedence::Lowest,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{i}"),
            Token::String(s) => write!(f, "\"{s}\""),
            Token::Identifier(i) => write!(f, "{i}"),
            Token::EqualSign => write!(f, "="),
            Token::PlusSign => write!(f, "+"),
            Token::MinusSign => write!(f, "-"),
            Token::ExclamationMark => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Function => write!(f, "fn"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Let => write!(f, "let"),
            Token::Return => write!(f, "return"),
            Token::Eof => write!(f, ""),
            Token::Illegal(c) => write!(f, "{c}"),
        }
    }
}
