use abyss_lang::{
    env::{Environment, SymbolTable},
    eval::{evaluate, EvalResult},
    parser::{build_ast, parse, Rule},
};

pub fn test_base(input: &str) -> Result<Vec<EvalResult>, Box<dyn std::error::Error>> {
    let mut st = SymbolTable::new();
    let mut env = Environment::new();
    match parse(input) {
        Ok(pair) => {
            let mut results = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != Rule::EOI {
                    let ast = build_ast(inner_pair, &mut st);
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
