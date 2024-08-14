use abyss::eval::{evaluate_statements, Environment, EvalResult};
use abyss::parser::parse_statements;

#[test]
fn test_evaluate_rune_var_assign() {
    let mut env = Environment::new();
    let input = r#"forge message: rune = "Hello World from Abyss!"; unveil(message);"#;
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);

    match result {
        EvalResult::Text(s) => assert_eq!(s, "Hello World from Abyss!"),
        _ => panic!("Expected a string result"),
    }
}

#[test]
fn test_rune_concatenation() {
    let mut env = Environment::new();
    let input =
        r#"forge part1: rune = "Hello, ";forge part2: rune = "Abyss";unveil(part1 + part2);"#;
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);

    match result {
        EvalResult::Text(s) => assert_eq!(s, "Hello, Abyss"),
        _ => panic!("Expected a concatenated string result"),
    }
}

#[test]
fn test_rune_concatenation_multiline() {
    let mut env = Environment::new();
    let input = r#"
        forge part1: rune = "Hello, ";
        forge part2: rune = "Abyss";
        unveil(part1 + part2);
    "#;
    let ast = parse_statements(input).unwrap().1;
    let result = evaluate_statements(&ast, &mut env);

    match result {
        EvalResult::Text(s) => assert_eq!(s, "Hello, Abyss"),
        _ => panic!("Expected a concatenated string result"),
    }
}
