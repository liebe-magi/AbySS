mod test_base;

use abyss::eval::EvalResult;
use test_base::test_base;

#[test]
fn test_parse_arcana_1() {
    let input = "123;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 123),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_parse_arcana_2() {
    let input = "00123;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 123),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_parse_arcana_3() {
    let input = " 123;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 123),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_1() {
    let input = "1+2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 1 + 2),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_2() {
    let input = "123 + 456;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 123 + 456),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_3() {
    let input = " 123 + 456 + 789 ; ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 123 + 456 + 789),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_subtraction_1() {
    let input = "10 - 3;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 - 3),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_subtraction_2() {
    let input = "10 - 3 - 2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 - 3 - 2),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_multiplication_1() {
    let input = "6 * 7;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 6 * 7),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_multiplication_2() {
    let input = "6 * 7 * 2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 6 * 7 * 2),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_division_1() {
    let input = "20 / 5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 20 / 5),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_division_2() {
    let input = "20 / 5 / 2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 20 / 5 / 2),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_1() {
    let input = "10 + 20 * 3 - 5 / 5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 + 20 * 3 - 5 / 5),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_2() {
    let input = "(10 + 20) * 3;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, (10 + 20) * 3),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_3() {
    let input = "10 * (20 + 3) / 5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 * (20 + 3) / 5),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_4() {
    let input = "(10 + 20) / (5 - 3);";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, (10 + 20) / (5 - 3)),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_5() {
    let input = "(1 + 2) / 3;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, (1 + 2) / 3),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_6() {
    let input = "10 / (2 + 3);";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 / (2 + 3)),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_arcana_assign() {
    let input = "forge x: arcana = 42;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            assert!(matches!(&results[0], EvalResult::Abyss));
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_arcana_usage() {
    let input = "forge x: arcana = 10; x + 5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 2);
            match &results[1] {
                EvalResult::Arcana(n) => assert_eq!(*n, 10 + 5),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_arcana_usage_multiline() {
    let input = "
        forge x: arcana = 10;
        forge y: arcana = 3;
        3 * x + 2 * y;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Arcana(n) => assert_eq!(*n, 3 * 10 + 2 * 3),
                _ => panic!("Expected a number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}
