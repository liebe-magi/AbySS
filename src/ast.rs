#[derive(Debug, Clone)]
pub enum AST {
    Arcana(i64),
    Rune(String),
    Add(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    VarAssign(String, Box<AST>),
    Var(String),
    Unveil(Vec<AST>),
}
