use abyss::{
    eval::{evaluate_statements, Environment},
    parser::parse_statements,
};
use std::io::{self, Write};

fn main() {
    let mut env = Environment::new();

    loop {
        println!("Enter an expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        match parse_statements(&input) {
            Ok((_, ast)) => {
                println!("AST: {:?}", ast);
                let result = evaluate_statements(&ast, &mut env);
                match &result {
                    abyss::eval::EvalResult::Number(n) => println!("Result: {}", n),
                    abyss::eval::EvalResult::Text(s) => println!("Result: {}", s),
                    abyss::eval::EvalResult::Void => println!("Result: void"),
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
