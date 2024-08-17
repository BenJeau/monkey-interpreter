#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    // Literals
    Integer(isize),

    // Identifiers
    Identifier(String),

    // Operators
    EqualSign,
    PlusSign,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Let,
    Function,

    // Other
    Eof,
    Illegal,
}

pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl From<(TokenKind, String)> for Token {
    fn from(value: (TokenKind, String)) -> Self {
        Self {
            kind: value.0,
            literal: value.1,
        }
    }
}

#[derive(Default)]
pub struct Lexer {
    input: String,
    chars: Vec<char>,
    position: usize,
    read_position: usize,
    ch: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            chars: input.clone().chars().collect(),
            input,
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = Default::default();
        } else {
            self.ch = self.chars[self.read_position].to_string();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch.as_str() {
            "=" => (TokenKind::EqualSign, self.ch.clone()),
            "+" => (TokenKind::PlusSign, self.ch.clone()),
            "(" => (TokenKind::LeftParen, self.ch.clone()),
            ")" => (TokenKind::RightParen, self.ch.clone()),
            "{" => (TokenKind::LeftBrace, self.ch.clone()),
            "}" => (TokenKind::RightBrace, self.ch.clone()),
            "," => (TokenKind::Comma, self.ch.clone()),
            ";" => (TokenKind::Semicolon, self.ch.clone()),
            "" => (TokenKind::Eof, self.ch.clone()),
            _ => todo!("Need to support more token!"),
        }
        .into();

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_next_token() {
        let input = "=+(){},;";

        let tests = &[
            (TokenKind::EqualSign, "="),
            (TokenKind::PlusSign, "+"),
            (TokenKind::LeftParen, "("),
            (TokenKind::RightParen, ")"),
            (TokenKind::LeftBrace, "{"),
            (TokenKind::RightBrace, "}"),
            (TokenKind::Comma, ","),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.into());

        for (index, test) in tests.into_iter().enumerate() {
            let current_token = lexer.next_token();

            if current_token.kind != test.0 {
                panic!(
                    "tests[{index}] - token kind wrong. expected={:?}, got={:?}",
                    test.0, current_token.kind
                );
            }

            if current_token.literal != test.1 {
                panic!(
                    "tests[{index}] - literal wrong. expected={}, got={}",
                    test.1, current_token.literal
                );
            }
        }
    }
}
