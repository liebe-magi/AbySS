use abyss::eval::{evaluate, Environment, EvalResult};
use abyss::parser::{build_ast, parse, Rule};
use pest::error::Error;

pub fn test_base(input: &str) -> Result<Vec<EvalResult>, Error<Rule>> {
    let mut env = Environment::new();
    match parse(input) {
        Ok(pair) => {
            let mut results = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != abyss::parser::Rule::EOI {
                    let ast = build_ast(inner_pair);
                    println!("{:?}", ast);
                    let result = evaluate(&ast, &mut env);
                    results.push(result);
                }
            }
            Ok(results)
        }
        Err(e) => Err(e),
    }
}
