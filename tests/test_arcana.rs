use abyss::eval::{evaluate_statements, Environment, EvalResult};
use abyss::parser::parse_statements;

#[test]
fn test_parse_number_1() {
    let mut env = Environment::new();
    let input = "123;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 123),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_parse_number_2() {
    let mut env = Environment::new();
    let input = "00123;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 123),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_parse_number_3() {
    let mut env = Environment::new();
    let input = " 123;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 123),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_add_1() {
    let mut env = Environment::new();
    let input = "1+2;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 3),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_add_2() {
    let mut env = Environment::new();
    let input = "123 + 456;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 579),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_add_3() {
    let mut env = Environment::new();
    let input = " 123 + 456 + 789 ;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 1368),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_subtraction() {
    let mut env = Environment::new();
    let input = "10 - 3;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 7),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_multiplication() {
    let mut env = Environment::new();
    let input = "6 * 7;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 42),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_division() {
    let mut env = Environment::new();
    let input = "20 / 5;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 4),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_combined_operations_1() {
    let mut env = Environment::new();
    let input = "10 + 20 * 3 - 5 / 5;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 69),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_combined_operations_2() {
    let mut env = Environment::new();
    let input = " 10 + 20 * 3  - 5 / 5 + 1 ;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 70),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_combined_operations_3() {
    let mut env = Environment::new();
    let input = "10 + 20 *3 - 5/ 5 + 1 * 2;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 71),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_evaluate_statements_var_assign() {
    let mut env = Environment::new();
    let input = "forge x: arcana = 42;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Void => (),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_evaluate_statements_var_usage() {
    let mut env = Environment::new();
    let input = "forge x: arcana = 10; x + 5;";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 15),
        _ => panic!("Expected a number result"),
    }
}

#[test]
fn test_evaluate_statements_var_usage_multiline() {
    let mut env = Environment::new();
    let input = "
        forge x: arcana = 10;
        x + 5;
        forge y: arcana = 3;
        x + y * 2;
        unveil(3 * x + 2 * y);
    ";
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);
    match result {
        EvalResult::Number(n) => assert_eq!(n, 36),
        _ => panic!("Expected a number result"),
    }
}
