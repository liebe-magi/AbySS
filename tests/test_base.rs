use abyss_lang::{
    env::Environment,
    eval::{display_error_with_source, evaluate, EvalError, EvalResult},
    parser::{build_ast, parse, Rule},
};

pub fn test_base(input: &str) -> Result<Vec<EvalResult>, Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    match parse(input) {
        Ok(pair) => {
            let mut results = Vec::new();
            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() != Rule::EOI {
                    match build_ast(inner_pair) {
                        Ok(ast) => {
                            // println!("{:?}", ast);
                            match evaluate(&ast, &mut env) {
                                Ok(result) => {
                                    results.push(result);
                                }
                                Err(e) => {
                                    let error_message = e.to_string();
                                    match &e {
                                        EvalError::UndefinedVariable(_, line_info)
                                        | EvalError::InvalidOperation(_, line_info)
                                        | EvalError::NegativeExponent(line_info) => {
                                            display_error_with_source(
                                                input,
                                                line_info.clone(),
                                                &error_message,
                                            );
                                            return Err(Box::new(e));
                                        }
                                    }
                                }
                            }
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
