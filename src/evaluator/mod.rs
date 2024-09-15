use std::collections::BTreeMap;

use crate::{
    ast::{BlockStatement, Expression, Statement},
    evaluator::{
        environment::Environment,
        object::{Object, FALSE, NULL, TRUE},
    },
    lexer::Token,
    parser::Program,
};

mod builtins;
pub mod environment;
pub mod object;

pub trait Evaluator {
    fn eval(&self, environment: &mut Environment) -> Option<Object>;
}

impl Evaluator for Program {
    fn eval(&self, environment: &mut Environment) -> Option<Object> {
        let mut result = None;

        for statement in self.statements.iter() {
            result = statement.eval(environment);

            if let Some(Object::Return(value)) = result {
                return Some(*value);
            } else if matches!(result, Some(Object::Error(_))) {
                return result;
            }
        }
        result
    }
}

impl Evaluator for Vec<Statement> {
    fn eval(&self, environment: &mut Environment) -> Option<Object> {
        let mut result = None;

        for statement in self {
            result = statement.eval(environment);

            if matches!(result, Some(Object::Return(_) | Object::Error(_))) {
                return result;
            }
        }

        result
    }
}

impl Evaluator for BlockStatement {
    fn eval(&self, environment: &mut Environment) -> Option<Object> {
        self.statements.eval(environment)
    }
}

impl Evaluator for Statement {
    fn eval(&self, environment: &mut Environment) -> Option<Object> {
        match self {
            Statement::Expression { value } => value.eval(environment),
            Statement::Return { value } => {
                let value = value.eval(environment);
                if matches!(value, Some(Object::Error(_))) {
                    value
                } else {
                    value.map(Box::new).map(Object::Return)
                }
            }
            Statement::Let { name, value } => {
                let value = value.eval(environment)?;
                if matches!(value, Object::Error(_)) {
                    return Some(value);
                }

                environment.set(name.clone(), value);

                None
            }
        }
    }
}

impl Evaluator for Expression {
    fn eval(&self, environment: &mut Environment) -> Option<Object> {
        match self {
            Expression::Integer(value) => Some((*value).into()),
            Expression::Boolean(value) => Some((*value).into()),
            Expression::String(value) => Some(value.clone().into()),
            Expression::Identifier(name) => {
                if let Some(value) = environment.get(name) {
                    Some(value.clone())
                } else {
                    Some(Object::Error(format!("identifier not found: {}", name)))
                }
            }
            Expression::PrefixOperator {
                operator,
                expression,
            } => {
                let value = expression.eval(environment)?;
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
                let lh_value = lh_expression.eval(environment)?;
                if matches!(lh_value, Object::Error(_)) {
                    return Some(lh_value);
                }
                let rh_value = rh_expression.eval(environment)?;
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
                let condition = condition.eval(environment)?;
                if matches!(condition, Object::Error(_)) {
                    return Some(condition);
                }
                if condition.is_truthy() {
                    consequence.eval(environment)
                } else if let Some(alternative) = alternative {
                    alternative.eval(environment)
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
                        return eval_function(
                            inner_env,
                            &mut environment.clone(),
                            Some(&name),
                            parameters,
                            arguments,
                            body,
                        );
                    }

                    if let Some(builtin) = builtins::Builtin::from_str(&name) {
                        if let Object::Builtin(function) = builtin.get() {
                            let evaluated_arguments = arguments
                                .iter()
                                .map(|argument| argument.eval(environment))
                                .collect::<Option<Vec<Object>>>()?;
                            return function(&evaluated_arguments);
                        }
                    }

                    Some(Object::Error(format!("function not found: {}", name)))
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
            Expression::Array(elements) => Some(Object::Array(
                elements
                    .iter()
                    .map(|element| element.eval(environment))
                    .collect::<Option<Vec<Object>>>()?,
            )),
            Expression::Index { left, index } => {
                let left = left.eval(environment)?;
                if matches!(left, Object::Error(_)) {
                    return Some(left);
                }

                let index = index.eval(environment)?;
                if matches!(index, Object::Error(_)) {
                    return Some(index);
                }

                if let (Object::Array(array), Object::Integer(index)) = (&left, &index) {
                    Some(array.get(*index as usize).cloned().unwrap_or_default())
                } else if let Object::Hash(map) = &left {
                    Some(map.get(&index).cloned().unwrap_or_default())
                } else {
                    Some(Object::Error(format!(
                        "index operator not supported: {} With index of: {}",
                        left.kind(),
                        index.kind(),
                    )))
                }
            }
            Expression::HashLiteral(map) => {
                let mut expression_map = BTreeMap::new();

                for (key, value) in map {
                    let evaluated_key = key.eval(environment)?;
                    if matches!(evaluated_key, Object::Error(_)) {
                        return Some(evaluated_key);
                    }

                    let evaluated_value = value.eval(environment)?;
                    if matches!(evaluated_value, Object::Error(_)) {
                        return Some(evaluated_value);
                    }

                    expression_map.insert(evaluated_key, evaluated_value);
                }

                Some(expression_map.into())
            }
        }
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
        let value = expression.eval(outer_environment)?;
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

    let value = body.eval(&mut environment);

    if let Some(Object::Return(value)) = value {
        Some(*value)
    } else {
        value
    }
}

fn eval_infix_expression(operator: &Token, lh_value: Object, rh_value: Object) -> Object {
    match (lh_value, rh_value) {
        (Object::Integer(lh_integer), Object::Integer(rh_integer)) => {
            eval_integer_infix_expression(operator, lh_integer, rh_integer)
        }
        (Object::Boolean(lh_boolean), Object::Boolean(rh_boolean)) => match operator {
            Token::Equal => (lh_boolean == rh_boolean).into(),
            Token::NotEqual => (lh_boolean != rh_boolean).into(),
            _ => Object::Error(format!("unknown operator: BOOLEAN {operator} BOOLEAN")),
        },
        (Object::String(lh_string), Object::String(rh_string)) => match operator {
            Token::PlusSign => format!("{lh_string}{rh_string}").into(),
            _ => Object::Error(format!("unknown operator: STRING {operator} STRING")),
        },
        (lh_value, rh_value) => Object::Error(format!(
            "type mismatch: {} {operator} {}",
            lh_value.kind(),
            rh_value.kind()
        )),
    }
}

fn eval_integer_infix_expression(operator: &Token, lh_integer: isize, rh_integer: isize) -> Object {
    match operator {
        Token::PlusSign => (lh_integer + rh_integer).into(),
        Token::MinusSign => (lh_integer - rh_integer).into(),
        Token::Asterisk => (lh_integer * rh_integer).into(),
        Token::Slash => (lh_integer / rh_integer).into(),
        Token::LessThan => (lh_integer < rh_integer).into(),
        Token::GreaterThan => (lh_integer > rh_integer).into(),
        Token::Equal => (lh_integer == rh_integer).into(),
        Token::NotEqual => (lh_integer != rh_integer).into(),
        _ => Object::Error(format!("unknown operator: INTEGER {operator} INTEGER")),
    }
}

fn eval_prefix_expression(operator: &Token, value: Object) -> Object {
    match operator {
        Token::ExclamationMark => eval_bang_operator_expression(value),
        Token::MinusSign => eval_minus_sign_expression(value),
        _ => Object::Error(format!("unknown operator: {operator}{}", value.kind())),
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
        _ => Object::Error(format!("unknown operator: -{}", value.kind())),
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

            assert_eq!(program.eval(&mut environment), Some(expected.into()));
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

            assert_eq!(program.eval(&mut environment), Some(expected));
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = &[
            ("!true", FALSE),
            ("!false", TRUE),
            ("!5", FALSE),
            ("!!true", TRUE),
            ("!!false", FALSE),
            ("!!5", TRUE),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(program.eval(&mut environment), Some(expected));
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = &[
            ("if (true) { 10 }", 10.into()),
            ("if (false) { 10 }", NULL),
            ("if (1) { 10 }", 10.into()),
            ("if (1 < 2) { 10 }", 10.into()),
            ("if (1 > 2) { 10 }", NULL),
            ("if (1 > 2) { 10 } else { 20 }", 20.into()),
            ("if (1 < 2) { 10 } else { 20 }", 10.into()),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
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
                program.eval(&mut environment),
                Some(10.into()),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = &[
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "let a = 234; true + false; 5",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { return true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }"#,
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
            ("\"Hello\" - \"World\"", "unknown operator: STRING - STRING"),
            (
                "[1,2,3][true]",
                "index operator not supported: ARRAY With index of: BOOLEAN",
            ),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
                Some(Object::Error(expected.into())),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_let_statements() {
        let tests: &[(&str, Object)] = &[
            ("let a = 5; a;", 5.into()),
            ("let a = 5 * 5; a;", 25.into()),
            ("let a = 5; let b = a; b;", 5.into()),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15.into()),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
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

        let results = program.eval(&mut environment).unwrap();

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
                program.eval(&mut environment),
                Some(expected.into()),
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

        assert_eq!(program.eval(&mut environment), Some(4.into()),);
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

        assert_eq!(program.eval(&mut environment), Some(55.into()),);
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
            program.eval(&mut environment),
            Some(Object::Error("identifier not found: data".into())),
        );
    }

    #[test]
    fn test_string_concatenation() {
        let input = r#""Hello" + " " + "World!""#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(program.eval(&mut environment), Some("Hello World!".into()),);
    }

    #[test]
    fn test_builtin_functions() {
        let tests = &[
            (r#"len("")"#, 0.into()),
            (r#"len("four")"#, 4.into()),
            (r#"len("hello world")"#, 11.into()),
            (
                "len(1)",
                Object::Error(r#"argument to "len" not supported, got INTEGER"#.into()),
            ),
            (
                r#"len("one", "two")"#,
                Object::Error(r#"wrong number of arguments. Got 2, expected 1"#.into()),
            ),
            ("first([1, 2, 3])", 1.into()),
            ("first([])", NULL),
            ("last([1, 2, 3])", 3.into()),
            ("last([])", NULL),
            ("rest([1, 2, 3])", vec![2.into(), 3.into()].into()),
            ("rest([1])", vec![].into()),
            ("rest([])", NULL),
            (
                "push([1, 2, 3], true)",
                vec![1.into(), 2.into(), 3.into(), true.into()].into(),
            ),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
                Some(expected.clone()),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3]";

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            program.eval(&mut environment),
            Some(vec![1.into(), 4.into(), 6.into()].into()),
        );
    }

    #[test]
    fn test_array_index_expressions() {
        let tests = &[
            ("[1, 2, 3][0]", 1.into()),
            ("[1, 2, 3][1]", 2.into()),
            ("[1, 2, 3][2]", 3.into()),
            ("let i = 0; [1][i];", 1.into()),
            ("[1, 2, 3][1 + 1];", 3.into()),
            ("let myArray = [1, 2, 3]; myArray[2];", 3.into()),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                6.into(),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                2.into(),
            ),
            ("[1, 2, 3][3]", NULL),
            ("[1, 2, 3][-1]", NULL),
        ];

        for (index, (input, expected)) in tests.into_iter().cloned().enumerate() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
                Some(expected.clone()),
                "test {}",
                index
            );
        }
    }

    #[test]
    fn test_custom_function_map() {
        let input = r#"
let map = fn(arr, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), push(accumulated, f(first(arr))));
        }
    };
    iter(arr, []);
};

let data = [1, 2, 3];
let squared = fn(x) { x * x };

map(data, squared);
"#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            program.eval(&mut environment),
            Some(vec![1.into(), 4.into(), 9.into()].into()),
        );
    }

    #[test]
    fn test_custom_function_reduce() {
        let input = r#"
let reduce = fn(arr, initial, f) {
    let iter = fn(arr, accumulated) {
        if (len(arr) == 0) {
            accumulated
        } else {
            iter(rest(arr), f(accumulated, first(arr)));
        }
    };
    iter(arr, initial);
};

let sum = fn(arr) {
    reduce(arr, 0, fn(initial, element) { initial + element });
};

sum([1, 2, 3, 4, 5]);
"#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(program.eval(&mut environment), Some(15.into()),);
    }

    #[test]
    fn test_hash_literals() {
        let input = r#"let two = "two";
{
    "one": 10 - 9,
    two: 1 + 1,
    "thr" + "ee": 6 / 2,
    4: 4,
    true: 5,
    false: 6
}"#;

        let mut parser = Parser::new(Lexer::new(input.into()));
        let program = parser.parse_program().expect("Failed to parse program");
        let mut environment = Environment::new();

        assert_eq!(
            program.eval(&mut environment),
            Some(
                BTreeMap::from([
                    ("one".into(), 1.into()),
                    ("two".into(), 2.into()),
                    ("three".into(), 3.into()),
                    (4.into(), 4.into()),
                    (TRUE, 5.into()),
                    (FALSE, 6.into())
                ])
                .into()
            ),
        );
    }

    #[test]
    fn test_hash_index_expressions() {
        let tests = &[
            (r#"{"foo": 5}["foo"]"#, 5.into()),
            (r#"{"foo": 5}["bar"]"#, NULL),
            (r#"let key = "foo"; {"foo": 5}[key]"#, 5.into()),
            (r#"{}["foo"]"#, NULL),
            (r#"{5: 5}[5]"#, 5.into()),
            (r#"{true: 5}[true]"#, 5.into()),
            (r#"{false: 5}[false]"#, 5.into()),
        ];

        for (input, expected) in tests.into_iter().cloned() {
            let mut parser = Parser::new(Lexer::new(input.into()));
            let program = parser.parse_program().expect("Failed to parse program");
            let mut environment = Environment::new();

            assert_eq!(
                program.eval(&mut environment),
                Some(expected.clone()),
                "test {}",
                input
            );
        }
    }
}
