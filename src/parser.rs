use crate::{
    ast::{Expression, Statement},
    lexer::{Lexer, Token},
};

#[derive(Default)]
struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    errors: Vec<String>,
}

#[derive(PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub enum ExpressionPrecedence {
    #[default]
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         //+
    PRODUCT,     //*
    PREFIX,      //-Xor!X
    CALL,        // myFunction(X)
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            ..Default::default()
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = if self.current_token == Some(Token::Eof) {
            None
        } else {
            Some(self.lexer.next_token())
        };
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::default();

        while self.current_token != Some(Token::Eof) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.as_ref()? {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let name = match self.peek_token.as_ref() {
            Some(Token::Identifier(name)) => name.to_string(),
            token => {
                self.errors.push(format!(
                    "expected next token to be Identifier, got {token:?}"
                ));
                return None;
            }
        };

        self.next_token();

        if self.peek_token != Some(Token::EqualSign) {
            self.errors.push(format!(
                "expected next token to be Equal, got {:?}",
                self.peek_token,
            ));
            return None;
        };

        self.next_token();
        self.next_token();

        // TODO: unsure if the correct precendence here
        let value = self.parse_expression(ExpressionPrecedence::LOWEST)?;

        if self.peek_token != Some(Token::Semicolon) {
            self.errors.push(format!(
                "expected next token to be Semicolon, got {:?}",
                self.current_token,
            ));
            return None;
        };

        self.next_token();

        Some(Statement::Let {
            name: name.into(),
            value,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        // TODO: unsure if the correct precendence here
        let value = self.parse_expression(ExpressionPrecedence::LOWEST)?;

        if self.peek_token != Some(Token::Semicolon) {
            self.errors.push(format!(
                "expected next token to be Semicolon, got {:?}",
                self.current_token,
            ));
            return None;
        };

        self.next_token();

        Some(Statement::Return { value })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let value = self.parse_expression(ExpressionPrecedence::LOWEST)?;

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression { value })
    }

    fn parse_expression(&mut self, precendence: ExpressionPrecedence) -> Option<Expression> {
        let mut left = match self.current_token.clone()? {
            Token::Integer(integer) => Some(integer.into()),
            Token::Identifier(identifier) => Some(Expression::Identifier(identifier)),
            Token::True => Some(true.into()),
            Token::False => Some(false.into()),
            Token::PlusSign => self.parse_prefix_expression(),
            Token::MinusSign => self.parse_prefix_expression(),
            Token::ExclamationMark => self.parse_prefix_expression(),
            token => {
                self.errors.push(format!(
                    "no expression statement parser for {}",
                    token.to_string()
                ));
                None
            }
        }?;

        while self.peek_token != Some(Token::Semicolon) && precendence < self.peek_precedence() {
            let infix = match self.peek_token.clone()? {
                Token::PlusSign
                | Token::MinusSign
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::GreaterThan => {
                    self.next_token();
                    self.parse_infix_expression(left.clone())
                }
                _ => {
                    self.errors.push(format!(
                        "no infix statement parser for {}",
                        self.peek_token.clone().unwrap().to_string()
                    ));
                    return Some(left);
                }
            };

            if let Some(infix) = infix {
                left = infix;
            } else {
                return Some(left);
            }
        }

        Some(left)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        debug_assert!(matches!(
            self.current_token,
            Some(Token::MinusSign) | Some(Token::PlusSign) | Some(Token::ExclamationMark)
        ));

        let operator = self.current_token.clone().unwrap();

        self.next_token();

        Some(Expression::PrefixOperator {
            operator,
            expression: Box::new(self.parse_expression(ExpressionPrecedence::PREFIX).unwrap()),
        })
    }

    fn parse_infix_expression(&mut self, lh_expression: Expression) -> Option<Expression> {
        let operator = self.current_token.clone().unwrap();
        let precedence = self.current_precedence();

        self.next_token();

        Some(Expression::InfixOperator {
            operator,
            rh_expression: Box::new(self.parse_expression(precedence).unwrap()),
            lh_expression: Box::new(lh_expression),
        })
    }

    fn peek_precedence(&self) -> ExpressionPrecedence {
        let Some(token) = self.peek_token.as_ref() else {
            return Default::default();
        };

        token.precedence()
    }

    fn current_precedence(&self) -> ExpressionPrecedence {
        let Some(token) = self.current_token.as_ref() else {
            return Default::default();
        };

        token.precedence()
    }
}

#[derive(Default)]
struct Program {
    statements: Vec<Statement>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
let y = 10;
let foobar = 838383;"#;
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 3);

        assert_eq!(
            program.statements[0],
            Statement::Let {
                name: "x".into(),
                value: 5.into()
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Let {
                name: "y".into(),
                value: 10.into(),
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Let {
                name: "foobar".into(),
                value: 838383.into()
            }
        );
    }

    #[test]
    fn test_return_statements() {
        let input = r#"return 5;
return 10;
return 993322;"#;
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 3);

        assert_eq!(program.statements[0], Statement::Return { value: 5.into() });
        assert_eq!(
            program.statements[1],
            Statement::Return { value: 10.into() }
        );
        assert_eq!(
            program.statements[2],
            Statement::Return {
                value: 993322.into()
            }
        );
    }

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![
                Statement::Let {
                    name: "myVar".into(),
                    value: Expression::Identifier("anotherVar".into()),
                },
                Statement::Expression {
                    value: Expression::FunctionCall {
                        name: "print".into(),
                        arguments: vec![
                            Box::new(123.into()),
                            Box::new(true.into()),
                            Box::new(Expression::InfixOperator {
                                operator: Token::MinusSign,
                                rh_expression: Box::new(Expression::PrefixOperator {
                                    operator: Token::ExclamationMark,
                                    expression: Box::new(Expression::Null),
                                }),
                                lh_expression: Box::new(false.into()),
                            }),
                        ],
                    },
                },
            ],
        };

        assert_eq!(
            "let myVar = anotherVar;print(123,true,(false - (!null)))",
            program.to_string()
        );
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::Identifier("foobar".into())
            }
        );
    }

    #[test]
    fn test_integer_expression() {
        let input = "5;";
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression { value: 5.into() }
        );
    }

    #[test]
    fn test_true_boolean_expression() {
        let input = "true;";
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression { value: true.into() }
        );
    }

    #[test]
    fn test_false_boolean_expression() {
        let input = "false;";
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: false.into()
            }
        );
    }

    #[test]
    fn test_prefix_expressions() {
        let tests = &[
            ("!5;", Token::ExclamationMark, 5),
            ("-15;", Token::MinusSign, 15),
        ];

        for (input, operator, value) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0],
                Statement::Expression {
                    value: Expression::PrefixOperator {
                        operator,
                        expression: Box::new(value.into())
                    }
                }
            )
        }
    }

    #[test]
    fn test_infix_expressions() {
        let tests = &[
            ("5 + 5;", 5, Token::PlusSign, 5),
            ("5 - 5;", 5, Token::MinusSign, 5),
            ("5 * 5;", 5, Token::Asterisk, 5),
            ("5 / 5;", 5, Token::Slash, 5),
            ("5 > 5;", 5, Token::GreaterThan, 5),
            ("5 < 5;", 5, Token::LessThan, 5),
            ("5 == 5;", 5, Token::Equal, 5),
            ("5 != 5;", 5, Token::NotEqual, 5),
        ];

        for (input, lh_integer, operator, rh_integer) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
            assert_eq!(program.statements.len(), 1, "{:?}", program.statements);

            assert_eq!(
                program.statements[0],
                Statement::Expression {
                    value: Expression::InfixOperator {
                        operator,
                        lh_expression: Box::new(lh_integer.into()),
                        rh_expression: Box::new(rh_integer.into()),
                    }
                }
            )
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = &[
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);

            assert_eq!(program.to_string(), expected.to_string(),);
        }
    }
}
