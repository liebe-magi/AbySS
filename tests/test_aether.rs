mod test_base;

use abyss::eval::EvalResult;
use test_base::test_base;

#[test]
fn test_parse_aether_1() {
    let input = "123.45;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 123.45),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_parse_aether_2() {
    let input = "00123.45;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 123.45),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_parse_aether_3() {
    let input = " 123.45;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 123.45),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_1() {
    let input = "1.1 + 2.2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 1.1 + 2.2),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_2() {
    let input = "123.45 + 456.78;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 123.45 + 456.78),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_addition_3() {
    let input = " 123.45 + 456.78 + 789.01 ; ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 123.45 + 456.78 + 789.01),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_subtraction_1() {
    let input = "10.5 - 3.2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 - 3.2),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_subtraction_2() {
    let input = "10.5 - 3.2 - 2.1;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 - 3.2 - 2.1),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_multiplication_1() {
    let input = "6.5 * 7.2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 6.5 * 7.2),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_multiplication_2() {
    let input = "6.5 * 7.2 * 2.1;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 6.5 * 7.2 * 2.1),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_division_1() {
    let input = "20.5 / 5.1;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 20.5 / 5.1),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_division_2() {
    let input = "20.5 / 5.1 / 2.2;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 20.5 / 5.1 / 2.2),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_1() {
    let input = "10.5 + 20.2 * 3.1 - 5.5 / 5.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 + 20.2 * 3.1 - 5.5 / 5.0),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_2() {
    let input = "(10.5 + 20.2) * 3.1;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, (10.5 + 20.2) * 3.1),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_3() {
    let input = "10.5 * (20.2 + 3.3) / 5.5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 * (20.2 + 3.3) / 5.5),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_4() {
    let input = "(10.5 + 20.2) / (5.5 - 3.1);";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, (10.5 + 20.2) / (5.5 - 3.1)),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_5() {
    let input = "(1.5 + 2.5) / 3.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, (1.5 + 2.5) / 3.0),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_combined_operations_6() {
    let input = "10.5 / (2.5 + 3.5);";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 / (2.5 + 3.5)),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_aether_assign() {
    let input = "forge x: aether = 42.5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            assert!(matches!(&results[0], EvalResult::Abyss));
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_aether_usage() {
    let input = "forge x: aether = 10.5; x + 5.5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 2);
            match &results[1] {
                EvalResult::Aether(n) => assert_eq!(*n, 10.5 + 5.5),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_aether_usage_multiline() {
    let input = "
        forge x: aether = 10.5;
        forge y: aether = 3.5;
        3.0 * x + 2.0 * y;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Aether(n) => assert_eq!(*n, 3.0 * 10.5 + 2.0 * 3.5),
                _ => panic!("Expected a floating point number result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_aether_power_1() {
    let input = "2.5 ** 3.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 2.5_f64.powf(3.0)),
                _ => panic!("Expected an aether (floating point) result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_aether_power_2() {
    let input = "5.0 ** 0.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 5.0_f64.powf(0.0)), // 結果は常に1.0
                _ => panic!("Expected an aether (floating point) result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_aether_power_3() {
    let input = "(1.5 + 2.5) ** 2.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, (1.5_f64 + 2.5_f64).powf(2.0)),
                _ => panic!("Expected an aether (floating point) result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_aether_power_4() {
    let input = "4.2 ** -1.0;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert!((*n - 1.0 / 4.2).abs() < 1e-9), // 負の指数もサポート
                _ => panic!("Expected an aether (floating point) result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_aether_power_fractional_exponent() {
    let input = "9.0 ** 0.5;";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Aether(n) => assert_eq!(*n, 9.0_f64.powf(0.5)), // 平方根
                _ => panic!("Expected an aether (floating point) result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}
