mod token;

pub use token::Token;

#[derive(Default)]
pub struct Lexer {
    input: String,
    chars: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    reached_eof: bool,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::Eof {
            if self.reached_eof {
                return None;
            }
            self.reached_eof = true;
        }
        Some(token)
    }
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

    pub fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.chars.get(self.read_position).copied()
        }
    }

    pub fn next_token(&mut self) -> Token {
        let Some(ch) = self.ch else {
            return Token::Eof;
        };

        let token = match ch {
            '+' => Token::PlusSign,
            '-' => Token::MinusSign,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '"' => Token::String(self.read_string()),
            '=' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::EqualSign
                }
            }
            '!' => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::ExclamationMark
                }
            }
            ' ' | '\t' | '\n' | '\r' => {
                self.read_char();
                return self.next_token();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier();
                return match identifier.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Function,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
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
        while self.ch.is_some() && self.ch.unwrap().is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].parse().unwrap()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == Some('"') || self.ch == Some(0.into()) {
                break;
            }
        }
        self.input[position..self.position].to_string()
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

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
[1, 2];
{"foo": "bar"}
"#;

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
            Token::ExclamationMark,
            Token::MinusSign,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::GreaterThan,
            Token::Integer(5),
            Token::Semicolon,
            Token::If,
            Token::LeftParen,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,
            Token::Integer(10),
            Token::Equal,
            Token::Integer(10),
            Token::Semicolon,
            Token::Integer(10),
            Token::NotEqual,
            Token::Integer(9),
            Token::Semicolon,
            Token::String("foobar".into()),
            Token::String("foo bar".into()),
            Token::LeftBracket,
            Token::Integer(1),
            Token::Comma,
            Token::Integer(2),
            Token::RightBracket,
            Token::Semicolon,
            Token::LeftBrace,
            Token::String("foo".into()),
            Token::Colon,
            Token::String("bar".into()),
            Token::RightBrace,
            Token::Eof,
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
