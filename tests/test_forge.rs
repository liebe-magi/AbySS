mod test_base;

use abyss_lang::eval::{EvalError, EvalResult};
use test_base::test_base;

#[test]
fn test_forge_and_usage() {
    let input = "
        forge x: arcana = 42;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 2);
            match &results[1] {
                EvalResult::Arcana(n) => assert_eq!(*n, 42),
                _ => panic!("Expected Arcana result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_morph_and_usage() {
    let input = "
        forge morph x: arcana = 42;
        x = 100;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Arcana(n) => assert_eq!(*n, 100),
                _ => panic!("Expected Arcana result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_redeclaration_with_different_type() {
    let input = "
        forge x: arcana = 42;
        forge x: aether = 3.14;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3); // 3行目までの結果をチェック
            match &results[2] {
                EvalResult::Aether(n) => assert_eq!(*n, 3.14),
                _ => panic!("Expected Aether result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_morph_type_mismatch() {
    let input = "
        forge morph x: arcana = 42;
        x = 3.14;
    ";
    match test_base(input) {
        Err(e) => match e.downcast_ref::<EvalError>() {
            Some(EvalError::InvalidOperation(_)) => {}
            _ => panic!("Expected an invalid operation error for type mismatch"),
        },
        Ok(_) => panic!("Expected an error for type mismatch with morph variable"),
    }
}

#[test]
fn test_forge_morph_reassign_arcana() {
    let input = "
        forge morph x: arcana = 42;
        x = 84;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Arcana(n) => assert_eq!(*n, 84),
                _ => panic!("Expected Arcana result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_morph_reassign_aether() {
    let input = "
        forge morph x: aether = 3.14;
        x = 3.14159;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Aether(n) => assert_eq!(*n, 3.14159),
                _ => panic!("Expected Arcana result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_morph_reassign_omen() {
    let input = "
        forge morph x: omen = boon;
        x = hex;
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Omen(b) => assert_eq!(*b, false),
                _ => panic!("Expected Arcana result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_reassign_immutable() {
    let input = "
        forge x: arcana = 42;
        x = 84;
    ";
    match test_base(input) {
        Err(e) => match e.downcast_ref::<EvalError>() {
            Some(EvalError::InvalidOperation(_)) => {}
            _ => panic!("Expected an invalid operation error for reassigning immutable variable"),
        },
        Ok(_) => panic!("Expected an error for reassigning immutable variable"),
    }
}

#[test]
fn test_forge_reassign_different_type() {
    let input = "
        forge x: rune = \"Hello\";
        x = 42;
    ";
    match test_base(input) {
        Err(e) => match e.downcast_ref::<EvalError>() {
            Some(EvalError::InvalidOperation(_)) => {}
            _ => panic!("Expected an invalid operation error for type mismatch"),
        },
        Ok(_) => panic!("Expected an error for type mismatch"),
    }
}

#[test]
fn test_forge_morph_different_type_assignment() {
    let input = "
        forge morph x: rune = \"Hello\";
        x = \"World\";
        x;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 3);
            match &results[2] {
                EvalResult::Rune(s) => assert_eq!(s, "World"),
                _ => panic!("Expected Rune result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_forge_boolean_logic() {
    let input = "
        forge b: omen = boon;
        b;
    ";
    match test_base(input) {
        Ok(results) => {
            assert_eq!(results.len(), 2);
            match &results[1] {
                EvalResult::Omen(b) => assert_eq!(*b, true),
                _ => panic!("Expected Omen result"),
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}
