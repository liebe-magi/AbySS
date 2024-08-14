use crate::ast::AST;

pub fn evaluate(ast: &AST) -> i64 {
    match ast {
        AST::Number(n) => *n,
        AST::Add(left, right) => evaluate(left) + evaluate(right),
        AST::Subtract(left, right) => evaluate(left) - evaluate(right),
        AST::Multiply(left, right) => evaluate(left) * evaluate(right),
        AST::Divide(left, right) => evaluate(left) / evaluate(right),
    }
}
