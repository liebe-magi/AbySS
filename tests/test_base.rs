use abyss::eval::{evaluate, Environment, EvalResult};
use abyss::parser::{build_ast, parse};

pub fn test_base(input: &str) -> Result<Vec<EvalResult>, Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    match parse(input) {
        Ok(pair) => {
            let mut results = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != abyss::parser::Rule::EOI {
                    let ast = build_ast(inner_pair);
                    println!("{:?}", ast);
                    match evaluate(&ast, &mut env) {
                        Ok(result) => {
                            results.push(result);
                        }
                        Err(e) => {
                            return Err(Box::new(e)); // エラーを Box に包んで返す
                        }
                    }
                }
            }
            Ok(results)
        }
        Err(e) => Err(Box::new(e)), // ここでもエラーを Box に包んで返す
    }
}
