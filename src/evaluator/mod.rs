use crate::{
    ast::{BlockStatement, Expression, Statement},
    evaluator::{environment::Environment, object::Object},
    lexer::Token,
    parser::Program,
};

pub mod environment;
pub mod object;

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);

pub fn eval_program(program: &Program, environment: &mut Environment) -> Option<Object> {
    let mut result = None;

    for statement in program.statements.iter() {
        result = eval_statement(statement, environment);

        if let Some(Object::Return(value)) = result {
            return Some(*value);
        } else if matches!(result, Some(Object::Error(_))) {
            return result;
        }
    }

    result
}

fn eval_statements(statements: &[Statement], environment: &mut Environment) -> Option<Object> {
    let mut result = None;

    for statement in statements {
        result = eval_statement(statement, environment);

        if matches!(result, Some(Object::Return(_) | Object::Error(_))) {
            return result;
        }
    }

    result
}

fn eval_statement(statement: &Statement, environment: &mut Environment) -> Option<Object> {
    match statement {
        Statement::Expression { value } => eval_expression(value, environment),
        Statement::Return { value } => {
            let value = eval_expression(value, environment);
            if matches!(value, Some(Object::Error(_))) {
                value
            } else {
                value.map(Box::new).map(Object::Return)
            }
        }
        Statement::Let { name, value } => {
            let value = eval_expression(value, environment)?;
            if matches!(value, Object::Error(_)) {
                return Some(value);
            }

            environment.set(name.clone(), value);

            None
        }
    }
}

fn eval_expression(expression: &Expression, environment: &mut Environment) -> Option<Object> {
    match expression {
        Expression::Integer(value) => Some(Object::Integer(*value)),
        Expression::Boolean(value) => Some(native_boolean_to_boolean_object(*value)),
        Expression::Identifier(name) => {
            if let Some(value) = environment.get(name) {
                Some(value.clone())
            } else {
                Some(Object::Error(format!("Identifier not found: {}", name)))
            }
        }
        Expression::PrefixOperator {
            operator,
            expression,
        } => {
            let value = eval_expression(expression, environment)?;
            if matches!(value, Object::Error(_)) {
                return Some(value);
            }
            Some(eval_prefix_expression(operator, value))
        }
        Expression::InfixOperator {
            operator,
            lh_expression,
            rh_expression,
        } => {
            let lh_value = eval_expression(lh_expression, environment)?;
            if matches!(lh_value, Object::Error(_)) {
                return Some(lh_value);
            }
            let rh_value = eval_expression(rh_expression, environment)?;
            if matches!(rh_value, Object::Error(_)) {
                return Some(rh_value);
            }
            Some(eval_infix_expression(operator, lh_value, rh_value))
        }
        Expression::If {
            condition,
            consequence,
            alternative,
        } => {
            let condition = eval_expression(condition, environment)?;
            if matches!(condition, Object::Error(_)) {
                return Some(condition);
            }
            if is_truthy(condition) {
                eval_statements(&consequence.statements, environment)
            } else if let Some(alternative) = alternative {
                eval_statements(&alternative.statements, environment)
            } else {
                Some(NULL)
            }
        }
        Expression::Function { arguments, body } => Some(Object::Function {
            parameters: arguments.clone(),
            environment: environment.clone(),
            body: body.clone(),
        }),
        Expression::FunctionCall { name, arguments } => match *name.clone() {
            Expression::Identifier(name) => {
                if let Some(Object::Function {
                    parameters,
                    environment: inner_env,
                    body,
                }) = environment.get(&name)
                {
                    eval_function(
                        inner_env,
                        &mut environment.clone(),
                        Some(&name),
                        parameters,
                        arguments,
                        body,
                    )
                } else if name == "print" {
                    arguments.iter().enumerate().for_each(|(index, argument)| {
                        let value = eval_expression(argument, environment).unwrap_or(NULL);
                        print!("{}", value.inspect());
                        if index != arguments.len() - 1 {
                            print!(" ");
                        }
                    });
                    println!();
                    None
                } else {
                    Some(Object::Error(format!("Function not found: {}", name)))
                }
            }
            Expression::Function {
                arguments: parameters,
                body,
            } => eval_function(
                &Environment::new(),
                environment,
                None,
                &parameters,
                arguments,
                &body,
            ),
            _ => None,
        },
    }
}

fn eval_function(
    fn_environment: &Environment,
    outer_environment: &mut Environment,
    name: Option<&str>,
    parameters: &[String],
    arguments: &[Expression],
    body: &BlockStatement,
) -> Option<Object> {
    let mut environment = fn_environment.new_child();

    for (param, expression) in parameters.iter().zip(arguments) {
        let value = eval_expression(expression, outer_environment)?;
        if matches!(value, Object::Error(_)) {
            return Some(value);
        }
        environment.set(param.to_string(), value);
    }

    if let Some(name) = name {
        environment.set(
            name.to_string(),
            outer_environment.get(name).unwrap().clone(),
        );
    }

    let value = eval_statements(&body.statements, &mut environment);

    if let Some(Object::Return(value)) = value {
        Some(*value)
    } else {
        value
    }
}

fn is_truthy(value: Object) -> bool {
    !matches!(value, FALSE | NULL)
}

fn eval_infix_expression(operator: &Token, lh_value: Object, rh_value: Object) -> Object {
    match (lh_value, rh_value) {
        (Object::Integer(lh_integer), Object::Integer(rh_integer)) => {
            eval_integer_infix_expression(operator, lh_integer, rh_integer)
        }
        (Object::Boolean(lh_boolean), Object::Boolean(rh_boolean)) => match operator {
            Token::Equal => native_boolean_to_boolean_object(lh_boolean == rh_boolean),
            Token::NotEqual => native_boolean_to_boolean_object(lh_boolean != rh_boolean),
            _ => Object::Error(format!(
                "Unknown operator: BOOLEAN {} BOOLEAN",
                operator.to_string(),
            )),
        },
        (lh_value, rh_value) => Object::Error(format!(
            "Type mismatch: {} {} {}",
            lh_value.kind(),
            operator.to_string(),
            rh_value.kind()
        )),
    }
}

fn eval_integer_infix_expression(operator: &Token, lh_integer: isize, rh_integer: isize) -> Object {
    match operator {
        Token::PlusSign => Object::Integer(lh_integer + rh_integer),
        Token::MinusSign => Object::Integer(lh_integer - rh_integer),
        Token::Asterisk => Object::Integer(lh_integer * rh_integer),
        Token::Slash => Object::Integer(lh_integer / rh_integer),
        Token::LessThan => Object::Boolean(lh_integer < rh_integer),
        Token::GreaterThan => Object::Boolean(lh_integer > rh_integer),
        Token::Equal => Object::Boolean(lh_integer == rh_integer),
        Token::NotEqual => Object::Boolean(lh_integer != rh_integer),
        _ => Object::Error(format!(
            "Unknown operator: INTEGER {} INTEGER",
            operator.to_string()
        )),
    }
}

fn eval_prefix_expression(operator: &Token, value: Object) -> Object {
    match operator {
        Token::ExclamationMark => eval_bang_operator_expression(value),
        Token::MinusSign => eval_minus_sign_expression(value),
        _ => Object::Error(format!(
            "Unknown operator: {}{}",
            operator.to_string(),
            value.kind()
        )),
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
        _ => Object::Error(format!("Unknown operator: -{}", value.kind())),
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
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(Object::Integer(expected))
            );
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = &[
            ("true", TRUE),
            ("false", FALSE),
            ("1 < 2", TRUE),
            ("1 > 2", FALSE),
            ("1 < 1", FALSE),
            ("1 > 1", FALSE),
            ("1 == 1", TRUE),
            ("1 != 1", FALSE),
            ("1 == 2", FALSE),
            ("1 != 2", TRUE),
            ("true == true", TRUE),
            ("false == false", TRUE),
            ("true == false", FALSE),
            ("false == true", FALSE),
            ("true != false", TRUE),
            ("false != true", TRUE),
            ("(1 < 2) == true", TRUE),
            ("(1 < 2) == false", FALSE),
            ("(1 > 2) == true", FALSE),
            ("(1 > 2) == false", TRUE),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(eval_program(&program, &mut environment), Some(expected));
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
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(Object::Boolean(expected))
            );
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = &[
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", NULL),
            ("if (1) { 10 }", Object::Integer(10)),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", NULL),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(expected.clone()),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = &[
            "return 10;",
            "return 10; 9;",
            "return 2 * 5; 9;",
            "9; return 2 * 5; 9;",
            r#"if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }"#,
        ];

        for (index, input) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(Object::Integer(10)),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = &[
            ("5 + true;", "Type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "Type mismatch: INTEGER + BOOLEAN"),
            ("-true", "Unknown operator: -BOOLEAN"),
            ("true + false", "Unknown operator: BOOLEAN + BOOLEAN"),
            (
                "let a = 234; true + false; 5",
                "Unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { return true + false; }",
                "Unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }"#,
                "Unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "Identifier not found: foobar"),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(Object::Error(expected.into())),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = &[
            ("let a = 5; a;", Object::Integer(5)),
            ("let a = 5 * 5; a;", Object::Integer(25)),
            ("let a = 5; let b = a; b;", Object::Integer(5)),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
                Object::Integer(15),
            ),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(expected.clone()),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; }";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        let results = eval_program(&program, &mut environment).unwrap();

        assert!(matches!(results, Object::Function { .. }));
    }

    #[test]
    fn test_function_application() {
        let tests = &[
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            (
                "let add = fn(x, y) { return x + y; }; add(5 + 5, add(5, 5));",
                20,
            ),
            ("fn(x) { x; }(5)", 5),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                eval_program(&program, &mut environment),
                Some(Object::Integer(expected)),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_closures() {
        let input = r#"
    let newAdder = fn(x) {
        fn(y) { x + y };
    };
    let addTwo = newAdder(2);
    addTwo(2);"#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            eval_program(&program, &mut environment),
            Some(Object::Integer(4)),
        );
    }

    #[test]
    fn test_recursion() {
        let input = r#"
let fib = fn(n) {
    if (n == 0) {
        return 0;
    }
    
    if (n == 1) {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
};

fib(10);
        "#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            eval_program(&program, &mut environment),
            Some(Object::Integer(55)),
        );
    }

    #[test]
    fn test_function_doesnt_capture_global_future_environment() {
        let input = r#"
let test = fn(x) {
    return data + x;
};

let data = 5;

test(5);
"#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            eval_program(&program, &mut environment),
            Some(Object::Error("Identifier not found: data".into())),
        );
    }
}
