#[derive(PartialEq, Eq, Debug)]
pub enum Token {
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
    Illegal(char),
}

#[derive(Default)]
pub struct Lexer {
    input: String,
    chars: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
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
            self.ch = self.chars.get(self.read_position).copied();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let Some(ch) = self.ch else {
            return Token::Eof;
        };

        let token = match ch {
            ';' => Token::Semicolon,
            '=' => Token::EqualSign,
            '+' => Token::PlusSign,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            ' ' | '\t' | '\n' | '\r' => {
                self.read_char();
                return self.next_token();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier();
                return match identifier.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    _ => Token::Identifier(identifier),
                };
            }
            '0'..='9' => {
                return Token::Integer(self.read_integer());
            }
            _ => Token::Illegal(ch),
        };

        self.read_char();

        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_some() && matches!(self.ch.unwrap(), 'a'..='z' | 'A'..='Z' | '_') {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_integer(&mut self) -> isize {
        let position = self.position;
        while self.ch.is_some() && matches!(self.ch.unwrap(), '0'..='9') {
            self.read_char();
        }
        self.input[position..self.position].parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);"#;

        let tests = &[
            Token::Let,
            Token::Identifier("five".into()),
            Token::EqualSign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::EqualSign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::EqualSign,
            Token::Function,
            Token::LeftParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Identifier("x".into()),
            Token::PlusSign,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::EqualSign,
            Token::Identifier("add".into()),
            Token::LeftParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RightParen,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input.into());

        for (index, test) in tests.into_iter().enumerate() {
            let current_token = lexer.next_token();

            assert_eq!(
                current_token, *test,
                "tests[{index}] - token wrong. expected={:?}, got={:?}",
                test, current_token
            );
        }
    }
}
