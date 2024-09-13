use crate::{
    ast::{BlockStatement, Expression, Statement},
    lexer::Lexer,
    token::Token,
};

#[derive(Default)]
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    pub errors: Vec<String>,
}

#[derive(PartialEq, Eq, Default, PartialOrd, Ord, Debug)]
pub enum ExpressionPrecedence {
    #[default]
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         //+
    Product,     //*
    Prefix,      //-Xor!X
    Call,        // myFunction(X)
    Index,       // array[index]
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
        let value = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        };

        Some(Statement::Let { name, value })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        // TODO: unsure if the correct precendence here
        let value = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        };

        Some(Statement::Return { value })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let value = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token == Some(Token::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression { value })
    }

    fn parse_expression(&mut self, precendence: ExpressionPrecedence) -> Option<Expression> {
        let mut left = match self.current_token.clone()? {
            Token::Integer(integer) => Some(integer.into()),
            Token::Identifier(identifier) => Some(Expression::Identifier(identifier)),
            Token::String(string) => Some(Expression::String(string)),
            Token::LeftBracket => self.parse_array_literal(),
            Token::True => Some(true.into()),
            Token::False => Some(false.into()),
            Token::PlusSign => self.parse_prefix_expression(),
            Token::MinusSign => self.parse_prefix_expression(),
            Token::ExclamationMark => self.parse_prefix_expression(),
            Token::LeftParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            token => {
                self.errors
                    .push(format!("no expression statement parser for {token}"));
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
                Token::LeftParen => {
                    self.next_token();
                    self.parse_call_expression(left.clone())
                }
                Token::LeftBracket => {
                    self.next_token();
                    self.parse_index_expression(left.clone())
                }
                _ => {
                    self.errors.push(format!(
                        "no infix statement parser for {}",
                        self.peek_token.clone().unwrap()
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

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();
        let index = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token != Some(Token::RightBracket) {
            return None;
        }
        self.next_token();

        Some(Expression::Index {
            left: Box::new(left),
            index: Box::new(index),
        })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        Some(Expression::Array(
            self.parse_expression_list(Token::RightBracket)?,
        ))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        debug_assert!(matches!(
            self.current_token,
            Some(Token::MinusSign) | Some(Token::PlusSign) | Some(Token::ExclamationMark)
        ));

        let operator = self.current_token.clone()?;

        self.next_token();

        Some(Expression::PrefixOperator {
            operator,
            expression: Box::new(self.parse_expression(ExpressionPrecedence::Prefix)?),
        })
    }

    fn parse_infix_expression(&mut self, lh_expression: Expression) -> Option<Expression> {
        let operator = self.current_token.clone()?;
        let precedence = self.current_precedence();

        self.next_token();

        Some(Expression::InfixOperator {
            operator,
            rh_expression: Box::new(self.parse_expression(precedence)?),
            lh_expression: Box::new(lh_expression),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token != Some(Token::RightParen) {
            return None;
        };

        self.next_token();

        Some(expression)
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if self.peek_token != Some(Token::LeftParen) {
            self.errors.push(format!(
                "expected next token to be LeftParen, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();
        self.next_token(); // TODO: why two times??

        let condition = self.parse_expression(ExpressionPrecedence::Lowest)?;

        if self.peek_token != Some(Token::RightParen) {
            self.errors.push(format!(
                "expected next token to be RightParen, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        if self.peek_token != Some(Token::LeftBrace) {
            self.errors.push(format!(
                "expected next token to be LeftBrace, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        let consequence = self.parse_block_statement()?;

        let alternative = if self.peek_token == Some(Token::Else) {
            self.next_token();

            if self.peek_token != Some(Token::LeftBrace) {
                self.errors.push(format!(
                    "expected next token to be LeftBrace, got {:?}",
                    self.peek_token
                ));
                return None;
            };

            self.next_token();

            self.parse_block_statement()
        } else {
            None
        };

        Some(Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        if self.peek_token != Some(Token::LeftParen) {
            self.errors.push(format!(
                "expected next token to be LeftParen, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        let parameters = self.parse_function_parameters()?;

        if self.peek_token != Some(Token::LeftBrace) {
            self.errors.push(format!(
                "expected next token to be LeftBrace, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        let body = self.parse_block_statement()?;

        Some(Expression::Function {
            arguments: parameters,
            body,
        })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        let mut parameters = Vec::new();

        if self.peek_token == Some(Token::RightParen) {
            self.next_token();
            return Some(parameters);
        };

        self.next_token();

        if let Some(Token::Identifier(name)) = self.current_token.clone() {
            parameters.push(name);
        } else {
            self.errors.push(format!(
                "expected next token to be Identifier, got {:?}",
                self.current_token
            ));
            return None;
        };

        while self.peek_token == Some(Token::Comma) {
            self.next_token();
            self.next_token();
            if let Some(Token::Identifier(name)) = self.current_token.clone() {
                parameters.push(name);
            } else {
                self.errors.push(format!(
                    "expected next token to be Identifier, got {:?}",
                    self.current_token
                ));
                return None;
            };
        }

        if self.peek_token != Some(Token::RightParen) {
            self.errors.push(format!(
                "expected next token to be RightParen, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        Some(parameters)
    }

    fn parse_call_expression(&mut self, name: Expression) -> Option<Expression> {
        Some(Expression::FunctionCall {
            name: Box::new(name),
            arguments: self.parse_expression_list(Token::RightParen)?,
        })
    }

    fn parse_expression_list(&mut self, end_token: Token) -> Option<Vec<Expression>> {
        let mut arguments = Vec::new();

        if self.peek_token.as_ref() == Some(&end_token) {
            self.next_token();
            return Some(arguments);
        };

        self.next_token();

        arguments.push(self.parse_expression(ExpressionPrecedence::Lowest)?);

        while self.peek_token == Some(Token::Comma) {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(ExpressionPrecedence::Lowest)?);
        }

        if self.peek_token != Some(end_token) {
            self.errors.push(format!(
                "expected next token to be RightParen, got {:?}",
                self.peek_token
            ));
            return None;
        };

        self.next_token();

        Some(arguments)
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let mut statements = Vec::new();
        self.next_token();

        while self.current_token != Some(Token::RightBrace)
            && self.current_token != Some(Token::Eof)
        {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        Some(BlockStatement { statements })
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
pub struct Program {
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::BlockStatement;

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"let x = 5;
let y = true;
let foobar = y;"#;
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
                value: true.into(),
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Let {
                name: "foobar".into(),
                value: Expression::Identifier("y".into())
            }
        );
    }

    #[test]
    fn test_return_statements() {
        let input = r#"return 5;
return y;
return true;"#;
        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 3);

        assert_eq!(program.statements[0], Statement::Return { value: 5.into() });
        assert_eq!(
            program.statements[1],
            Statement::Return {
                value: Expression::Identifier("y".into())
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Return { value: true.into() }
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
                        name: Box::new(Expression::Identifier("print".into())),
                        arguments: vec![
                            123.into(),
                            true.into(),
                            Expression::InfixOperator {
                                operator: Token::MinusSign,
                                rh_expression: Box::new(Expression::PrefixOperator {
                                    operator: Token::ExclamationMark,
                                    expression: Box::new(Expression::Identifier("null".into())),
                                }),
                                lh_expression: Box::new(false.into()),
                            },
                        ],
                    },
                },
            ],
        };

        assert_eq!(
            "let myVar = anotherVar;print(123, true, (false - (!null)))",
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
    fn test_boolean_prefix_expressions() {
        let tests = &[
            ("!true", true, Token::ExclamationMark),
            ("!false", false, Token::ExclamationMark),
        ];

        for (input, value, operator) in tests.into_iter().cloned() {
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
            ("5 + 5", 5, Token::PlusSign, 5),
            ("5 - 5", 5, Token::MinusSign, 5),
            ("5 * 5", 5, Token::Asterisk, 5),
            ("5 / 5", 5, Token::Slash, 5),
            ("5 > 5", 5, Token::GreaterThan, 5),
            ("5 < 5", 5, Token::LessThan, 5),
            ("5 == 5", 5, Token::Equal, 5),
            ("5 != 5", 5, Token::NotEqual, 5),
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
    fn test_boolean_infix_expressions() {
        let tests = &[
            ("true == true", true, Token::Equal, true),
            ("true != false", true, Token::NotEqual, false),
            ("false == false", false, Token::Equal, false),
        ];

        for (input, lh_boolean, operator, rh_boolean) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0],
                Statement::Expression {
                    value: Expression::InfixOperator {
                        operator,
                        lh_expression: Box::new(lh_boolean.into()),
                        rh_expression: Box::new(rh_boolean.into()),
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
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "a * [1, 2, 3, 4][b * c] * d",
                "((a * ([1, 2, 3, 4][(b * c)])) * d)",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            ),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);

            assert_eq!(program.to_string(), expected.to_string());
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::If {
                    condition: Box::new(Expression::InfixOperator {
                        operator: Token::LessThan,
                        lh_expression: Box::new(Expression::Identifier("x".into())),
                        rh_expression: Box::new(Expression::Identifier("y".into()))
                    }),
                    consequence: BlockStatement {
                        statements: vec![Statement::Expression {
                            value: Expression::Identifier("x".into())
                        }]
                    },
                    alternative: None
                }
            }
        )
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::If {
                    condition: Box::new(Expression::InfixOperator {
                        operator: Token::LessThan,
                        lh_expression: Box::new(Expression::Identifier("x".into())),
                        rh_expression: Box::new(Expression::Identifier("y".into()))
                    }),
                    consequence: BlockStatement {
                        statements: vec![Statement::Expression {
                            value: Expression::Identifier("x".into())
                        }]
                    },
                    alternative: Some(BlockStatement {
                        statements: vec![Statement::Expression {
                            value: Expression::Identifier("y".into())
                        }]
                    })
                }
            }
        )
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y }";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::Function {
                    arguments: vec!["x".into(), "y".into()],
                    body: BlockStatement {
                        statements: vec![Statement::Expression {
                            value: Expression::InfixOperator {
                                operator: Token::PlusSign,
                                lh_expression: Box::new(Expression::Identifier("x".into())),
                                rh_expression: Box::new(Expression::Identifier("y".into()))
                            }
                        }]
                    }
                }
            }
        )
    }

    #[test]
    fn test_funciton_parameters_parsing() {
        let tests = &[
            ("fn() {}", vec![]),
            ("fn(x) {}", vec!["x".into()]),
            ("fn(x, y, z) {}", vec!["x".into(), "y".into(), "z".into()]),
        ];

        for (input, arguments) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
            assert_eq!(program.statements.len(), 1);

            assert_eq!(
                program.statements[0],
                Statement::Expression {
                    value: Expression::Function {
                        arguments,
                        body: BlockStatement { statements: vec![] }
                    }
                }
            )
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let input = "add(1, 2 * 3, 4 + 5)";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::FunctionCall {
                    name: Box::new(Expression::Identifier("add".into())),
                    arguments: vec![
                        Expression::Integer(1),
                        Expression::InfixOperator {
                            operator: Token::Asterisk,
                            lh_expression: Box::new(Expression::Integer(2)),
                            rh_expression: Box::new(Expression::Integer(3)),
                        },
                        Expression::InfixOperator {
                            operator: Token::PlusSign,
                            lh_expression: Box::new(Expression::Integer(4)),
                            rh_expression: Box::new(Expression::Integer(5)),
                        },
                    ]
                }
            }
        )
    }

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\"";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1, "{:?}", program.statements);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::String("hello world".to_string())
            }
        )
    }

    #[test]
    fn test_parsing_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1, "{:?}", program.statements);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::Array(vec![
                    Expression::Integer(1),
                    Expression::InfixOperator {
                        operator: Token::Asterisk,
                        lh_expression: Box::new(Expression::Integer(2)),
                        rh_expression: Box::new(Expression::Integer(2)),
                    },
                    Expression::InfixOperator {
                        operator: Token::PlusSign,
                        lh_expression: Box::new(Expression::Integer(3)),
                        rh_expression: Box::new(Expression::Integer(3)),
                    },
                ])
            }
        )
    }

    #[test]
    fn test_parsing_index_expressions() {
        let input = "myArray[1 + 1]";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");

        assert_eq!(parser.errors.len(), 0, "{:?}", parser.errors);
        assert_eq!(program.statements.len(), 1, "{:?}", program.statements);

        assert_eq!(
            program.statements[0],
            Statement::Expression {
                value: Expression::Index {
                    left: Box::new(Expression::Identifier("myArray".into())),
                    index: Box::new(Expression::InfixOperator {
                        operator: Token::PlusSign,
                        lh_expression: Box::new(Expression::Integer(1)),
                        rh_expression: Box::new(Expression::Integer(1)),
                    }),
                }
            }
        )
    }
}
