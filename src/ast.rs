#[derive(Debug, Clone)]
pub enum AST {
    Number(i64),
    String(String),
    Add(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    VarAssign(String, Box<AST>),
    RuneVarAssign(String, Box<AST>),
    Var(String),
    Unveil(Box<AST>),
}
