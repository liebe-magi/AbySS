use abyss::eval::evaluate;
use abyss::parser::parse_expr;

#[test]
fn test_parse_number_1() {
    let input = "123";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 123);
}

#[test]
fn test_parse_number_2() {
    let input = "00123";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 123);
}

#[test]
fn test_parse_number_3() {
    let input = " 123";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 123);
}

#[test]
fn test_add_1() {
    let input = "1+2";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 3);
}

#[test]
fn test_add_2() {
    let input = "123 + 456";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 579);
}

#[test]
fn test_add_3() {
    let input = " 123 + 456 + 789 ";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 1368);
}

#[test]
fn test_subtraction() {
    let input = "10 - 3";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 7);
}

#[test]
fn test_multiplication() {
    let input = "6 * 7";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 42);
}

#[test]
fn test_division() {
    let input = "20 / 5";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 4);
}

#[test]
fn test_combined_operations_1() {
    let input = "10 + 20 * 3 - 5 / 5";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 69);
}

#[test]
fn test_combined_operations_2() {
    let input = " 10 + 20 * 3  - 5 / 5 +1 ";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 70);
}

#[test]
fn test_combined_operations_3() {
    let input = "10 + 20 *3 - 5/ 5 + 1 * 2";
    let ast = parse_expr(input).unwrap().1;
    let result = evaluate(&ast);
    assert_eq!(result, 71);
}
