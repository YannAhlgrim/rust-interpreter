use super::*;
use crate::lexer;
use crate::parser::new_parser;

fn test_eval(input: &str) -> Option<Box<dyn Object>> {
    let lexer = lexer::new(input.to_string());
    let mut parser = new_parser(lexer);
    let program = parser.parse_program();
    eval_program(program)
}

fn test_integer_object(obj: &dyn Object, expected: i64) {
    assert_eq!(obj.object_type(), ObjectType::INTEGER);
    let int = obj.as_any().downcast_ref::<object::Integer>().unwrap();
    assert_eq!(int.value, expected);
}

fn test_boolean_object(obj: &dyn Object, expected: bool) {
    assert_eq!(obj.object_type(), ObjectType::BOOLEAN);
    let b = obj.as_any().downcast_ref::<object::Boolean>().unwrap();
    assert_eq!(b.value, expected);
}

fn test_null_object(obj: &dyn Object) {
    assert_eq!(obj.object_type(), ObjectType::NULL);
}

#[test]
fn test_eval_integer_expression() {
    let tests = vec![("5", 5), ("10", 10), ("-5", -5), ("-10", -10)];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_boolean_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_boolean_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_eval_integer_arithmetic() {
    let tests = vec![
        ("5 + 5", 10),
        ("5 - 5", 0),
        ("5 * 5", 25),
        ("5 / 5", 1),
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

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests = vec![
        ("if (true) { 10 }", Some(10)),
        ("if (false) { 10 }", None),
        ("if (1) { 10 }", Some(10)),
        ("if (1 < 2) { 10 }", Some(10)),
        ("if (1 > 2) { 10 }", None),
        ("if (1 > 2) { 10 } else { 20 }", Some(20)),
        ("if (1 < 2) { 10 } else { 20 }", Some(10)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        match expected {
            Some(value) => test_integer_object(evaluated.as_ref(), value),
            None => test_null_object(evaluated.as_ref()),
        }
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
            10,
        ),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_function_object() {
    let input = "fn(x) { x + 2; };";
    let evaluated = test_eval(input).unwrap();

    assert_eq!(evaluated.object_type(), ObjectType::FUNCTION);
    let func = evaluated.as_any().downcast_ref::<object::Function>().unwrap();
    assert_eq!(func.parameters.len(), 1);
    assert_eq!(func.parameters[0].value, "x");
    assert_eq!(func.body.string(), "(x + 2)");
}

#[test]
fn test_function_application() {
    let tests = vec![
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_enclosing_environments() {
    let input = "
        let first = 10;
        let second = 10;
        let third = 10;
        let ourFunction = fn(first) {
            let second = 20;
            first + second + third;
        };
        ourFunction(20) + first + second;
    ";
    let evaluated = test_eval(input).unwrap();
    test_integer_object(evaluated.as_ref(), 70);
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        ("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN"),
        ("foobar", "identifier not found: foobar"),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input).unwrap();
        assert_eq!(evaluated.object_type(), ObjectType::ERROR);
        let err = evaluated.as_any().downcast_ref::<object::Error>().unwrap();
        assert!(err.message.contains(expected), "expected '{}' to contain '{}'", err.message, expected);
    }
}
