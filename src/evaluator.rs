use crate::{
    ast::{Expression, Statement},
    object::Object,
};

pub fn eval_statements(statements: &[Statement]) -> Option<Object> {
    let mut result = None;

    for statement in statements {
        result = eval_statement(statement);
    }

    result
}

fn eval_statement(statement: &Statement) -> Option<Object> {
    match statement {
        Statement::Expression { value } => eval_expression(value),
        _ => None,
    }
}

fn eval_expression(expression: &Expression) -> Option<Object> {
    match expression {
        Expression::Integer(value) => Some(Object::Integer(*value)),
        Expression::Boolean(value) => Some(Object::Boolean(*value)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = &[("5", 5), ("10", 10)];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(
                eval_statements(&program.statements),
                Some(Object::Integer(expected))
            );
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = &[("true", true), ("false", false)];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(
                eval_statements(&program.statements),
                Some(Object::Boolean(expected))
            );
        }
    }
}
