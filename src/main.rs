use abyss::{eval::evaluate, parser::parse_expr};

fn main() {
    let input = "6 * 7";
    match parse_expr(input) {
        Ok((remaining, result)) => {
            println!("Parsed expression: {:?}", result);
            let evaluated = evaluate(&result);
            println!("Evaluated expression: {}", evaluated);
            println!("Remaining input: {:?}", remaining);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
