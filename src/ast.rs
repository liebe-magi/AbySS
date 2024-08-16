#[derive(Debug, Clone)]
pub enum AST {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
    Rune(String),
    Add(Box<AST>, Box<AST>),
    Subtract(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    Divide(Box<AST>, Box<AST>),
    PowArcana(Box<AST>, Box<AST>),
    PowAether(Box<AST>, Box<AST>),
    Equal(Box<AST>, Box<AST>),
    NotEqual(Box<AST>, Box<AST>),
    LessThan(Box<AST>, Box<AST>),
    LessThanOrEqual(Box<AST>, Box<AST>),
    GreaterThan(Box<AST>, Box<AST>),
    GreaterThanOrEqual(Box<AST>, Box<AST>),
    VarAssign(String, Box<AST>),
    Var(String),
    Unveil(Vec<AST>),
    Trans(Box<AST>, Type),
}

#[derive(Debug, Clone)]
pub enum Type {
    Arcana,
    Aether,
    Rune,
    Omen,
}
