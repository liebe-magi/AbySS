mod test_base;

use abyss_lang::eval::EvalResult;
use test_base::test_base;

#[test]
fn test_simple_orbit() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..10) {
        sum += i;
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(45))),
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_orbit_with_nested_orbit() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..3) {
        orbit(j = 0..3) {
            sum += i * j;
        };
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(18))),
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_orbit_with_resume() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..3, j = 0..3) {
        oracle(i == j) {
            (boon) => resume j; // jのループをスキップ
        };
        sum += i * j;
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(12))),
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_orbit_with_eject() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..5) {
        oracle(i == 3) {
            (boon) => eject; // ループ全体を抜ける
        };
        sum += i;
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(3))), // 0 + 1 + 2
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_orbit_with_eject_outer_loop() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..3) {
        orbit(j = 0..3) {
            oracle(i == j) {
                (boon) => eject i; // 外側のループを抜ける
            };
            sum += i * j;
        };
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(0))), // 外側ループが全て抜ける
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_orbit_with_resume_outer_loop() {
    let input = r#"
    forge morph sum: arcana = 0;
    orbit(i = 0..3) {
        oracle(i == 1) {
            (boon) => resume i; // 外側のループをスキップ
        };
        sum += i;
    };
    sum;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(2))), // i == 1 のループがスキップされる
        Err(e) => panic!("Error: {:?}", e),
    }
}

#[test]
fn test_infinite_orbit_with_eject() {
    let input = r#"
    forge morph count: arcana = 0;
    orbit {
        count += 1;
        oracle(count == 10) {
            (boon) => eject; // 無限ループを10回で終了
        };
    };
    count;
    "#;
    match test_base(input) {
        Ok(results) => assert!(matches!(results[2], EvalResult::Arcana(10))),
        Err(e) => panic!("Error: {:?}", e),
    }
}
