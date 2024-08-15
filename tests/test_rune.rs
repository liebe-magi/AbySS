mod test_base;

use abyss::eval::EvalResult;
use test_base::test_base;

#[test]
fn test_parse_rune() {
    let input = "\"Hello, Abyss!\";";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            match &results[0] {
                EvalResult::Rune(s) => assert_eq!(s, "Hello, Abyss!"),
                _ => panic!("Expected a string result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_evaluate_rune_assign() {
    let input = r#"forge message: rune = "Hello World from Abyss!"; message;"#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 2);
            match &results[1] {
                EvalResult::Rune(s) => assert_eq!(s, "Hello World from Abyss!"),
                _ => panic!("Expected a string result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_rune_concatenation() {
    let input = r#"forge part1: rune = "Hello, "; forge part2: rune = "Abyss"; part1 + part2;"#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Rune(s) => assert_eq!(s, "Hello, Abyss"),
                _ => panic!("Expected a concatenated string result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_rune_concatenation_multiline() {
    let input = r#"
        forge part1: rune = "Hello, ";
        forge part2: rune = "Abyss";
        part1 + part2 + "!";
    "#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Rune(s) => assert_eq!(s, "Hello, Abyss!"),
                _ => panic!("Expected a concatenated string result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_unveil_rune_1() {
    let input = r#"unveil("Hello, Abyss!");"#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            assert!(matches!(&results[0], EvalResult::Abyss));
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_unveil_rune_2() {
    let input = r#"
        forge part1: rune = "Hello, ";
        forge part2: rune = "Abyss";
        unveil(part1 + part2 + "!");
    "#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            assert!(matches!(&results[0], EvalResult::Abyss));
            assert!(matches!(&results[1], EvalResult::Abyss));
            assert!(matches!(&results[2], EvalResult::Abyss));
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_unveil_rune_3() {
    let input = r#"
        unveil("1 + 3 = ", 1 + 3);
    "#;
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 1);
            assert!(matches!(&results[0], EvalResult::Abyss));
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}
