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
    Modulo(Box<AST>, Box<AST>),
    PowArcana(Box<AST>, Box<AST>),
    PowAether(Box<AST>, Box<AST>),
    Equal(Box<AST>, Box<AST>),
    NotEqual(Box<AST>, Box<AST>),
    LessThan(Box<AST>, Box<AST>),
    LessThanOrEqual(Box<AST>, Box<AST>),
    GreaterThan(Box<AST>, Box<AST>),
    GreaterThanOrEqual(Box<AST>, Box<AST>),
    LogicalAnd(Box<AST>, Box<AST>),
    LogicalOr(Box<AST>, Box<AST>),
    LogicalNot(Box<AST>),
    VarAssign {
        name: String,
        value: Box<AST>,
        var_type: Type,
        is_morph: bool,
    },
    Assignment {
        name: String,
        value: Box<AST>, // 定義済み変数への代入用ノード
        op: AssignmentOp,
    },
    Var {
        name: String,
        var_type: Type,
        is_morph: bool,
    },
    Unveil(Vec<AST>),
    Trans(Box<AST>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Arcana,
    Aether,
    Rune,
    Omen,
}

#[derive(Debug, Clone)]
pub enum AssignmentOp {
    Assign, // 単純な代入
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,       // 剰余代入を追加
    PowArcanaAssign, // ^=
    PowAetherAssign, // **=
}
