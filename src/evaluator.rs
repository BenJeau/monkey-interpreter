use crate::{
    ast::{Expression, Statement},
    lexer::Token,
    object::Object,
};

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);

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
        Expression::Boolean(value) => Some(native_boolean_to_boolean_object(*value)),
        Expression::Null => Some(NULL),
        Expression::PrefixOperator {
            operator,
            expression,
        } => {
            let value = eval_expression(expression)?;
            Some(eval_prefix_expression(operator, value))
        }
        Expression::InfixOperator {
            operator,
            lh_expression,
            rh_expression,
        } => {
            let lh_value = eval_expression(lh_expression)?;
            let rh_value = eval_expression(rh_expression)?;
            Some(eval_infix_expression(operator, lh_value, rh_value))
        }
        _ => None,
    }
}

fn eval_infix_expression(operator: &Token, lh_value: Object, rh_value: Object) -> Object {
    match (lh_value, rh_value) {
        (Object::Integer(lh_integer), Object::Integer(rh_integer)) => {
            eval_integer_infix_expression(operator, lh_integer, rh_integer)
        }
        _ => NULL,
    }
}

fn eval_integer_infix_expression(operator: &Token, lh_integer: isize, rh_integer: isize) -> Object {
    match operator {
        Token::PlusSign => Object::Integer(lh_integer + rh_integer),
        Token::MinusSign => Object::Integer(lh_integer - rh_integer),
        Token::Asterisk => Object::Integer(lh_integer * rh_integer),
        Token::Slash => Object::Integer(lh_integer / rh_integer),
        _ => NULL,
    }
}

fn eval_prefix_expression(operator: &Token, value: Object) -> Object {
    match operator {
        Token::ExclamationMark => eval_bang_operator_expression(value),
        Token::MinusSign => eval_minus_sign_expression(value),
        _ => value,
    }
}

fn eval_bang_operator_expression(value: Object) -> Object {
    match value {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_sign_expression(value: Object) -> Object {
    match value {
        Object::Integer(value) => Object::Integer(-value),
        _ => NULL,
    }
}

fn native_boolean_to_boolean_object(value: bool) -> Object {
    if value {
        TRUE
    } else {
        FALSE
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = &[
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

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
        let tests = &[
            ("true", TRUE),
            ("false", FALSE),
            ("-true", NULL),
            ("-false", NULL),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");

            assert_eq!(eval_statements(&program.statements), Some(expected));
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = &[
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

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
