mod test_base;

use abyss_lang::eval::EvalResult;
use test_base::test_base;

#[test]
fn test_arc_push_and_get() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 3];
    numbers.push(4);
    forge result: arcana = numbers.get(3);
    result;
    "#;

    let result_rust = 4;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Arcana(result_abyss) = results[3] {
                assert_eq!(result_rust, result_abyss);
            } else {
                panic!("Expected Arcana result");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_arc_pop() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 3];
    forge result: arcana = numbers.pop();
    result;
    "#;

    let result_rust = 3;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Arcana(result_abyss) = results[2] {
                assert_eq!(result_rust, result_abyss);
            } else {
                panic!("Expected Arcana result");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_arc_insert_and_remove() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 4];
    numbers.insert(2, 3);  // 2番目の位置に3を挿入
    forge inserted_result: arcana = numbers.get(2);

    forge removed_result: arcana = numbers.remove(2);  // 2番目の位置の3を削除
    inserted_result;
    removed_result;
    "#;

    let inserted_result_rust = 3;
    let removed_result_rust = 3;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Arcana(inserted_result_abyss) = results[5] {
                assert_eq!(inserted_result_rust, inserted_result_abyss);
            } else {
                panic!("Expected Arcana result for inserted element");
            }

            if let EvalResult::Arcana(removed_result_abyss) = results[6] {
                assert_eq!(removed_result_rust, removed_result_abyss);
            } else {
                panic!("Expected Arcana result for removed element");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_arc_clear_and_is_empty() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 3];
    numbers.clear();
    forge result: omen = numbers.is_empty();
    result;
    "#;

    let result_rust = true;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Omen(result_abyss) = results[3] {
                assert_eq!(result_rust, result_abyss);
            } else {
                panic!("Expected Omen result");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_arc_length() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 3];
    forge result: arcana = numbers.length();
    result;
    "#;

    let result_rust = 3;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Arcana(result_abyss) = results[2] {
                assert_eq!(result_rust, result_abyss);
            } else {
                panic!("Expected Arcana result");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_arc_contains() {
    let input = r#"
    forge numbers: arc<arcana> = [1, 2, 3];
    forge result: omen = numbers.contains(2);
    result;
    "#;

    let result_rust = true;

    match test_base(input) {
        Ok(results) => {
            if let EvalResult::Omen(result_abyss) = results[2] {
                assert_eq!(result_rust, result_abyss);
            } else {
                panic!("Expected Omen result");
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}
