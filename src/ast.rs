use pest::Span;

#[derive(Debug, Clone)]
pub struct LineInfo {
    pub line: usize,
    pub column: usize,
}

impl LineInfo {
    pub fn from_span(span: &Span) -> Self {
        let (line, column) = span.start_pos().line_col();
        LineInfo { line, column }
    }
}

#[derive(Debug, Clone)]
pub enum AST {
    Omen(bool, Option<LineInfo>),
    Arcana(i64, Option<LineInfo>),
    Aether(f64, Option<LineInfo>),
    Rune(String, Option<LineInfo>),
    Add(Box<AST>, Box<AST>, Option<LineInfo>),
    Subtract(Box<AST>, Box<AST>, Option<LineInfo>),
    Multiply(Box<AST>, Box<AST>, Option<LineInfo>),
    Divide(Box<AST>, Box<AST>, Option<LineInfo>),
    Modulo(Box<AST>, Box<AST>, Option<LineInfo>),
    PowArcana(Box<AST>, Box<AST>, Option<LineInfo>),
    PowAether(Box<AST>, Box<AST>, Option<LineInfo>),
    Equal(Box<AST>, Box<AST>, Option<LineInfo>),
    NotEqual(Box<AST>, Box<AST>, Option<LineInfo>),
    LessThan(Box<AST>, Box<AST>, Option<LineInfo>),
    LessThanOrEqual(Box<AST>, Box<AST>, Option<LineInfo>),
    GreaterThan(Box<AST>, Box<AST>, Option<LineInfo>),
    GreaterThanOrEqual(Box<AST>, Box<AST>, Option<LineInfo>),
    LogicalAnd(Box<AST>, Box<AST>, Option<LineInfo>),
    LogicalOr(Box<AST>, Box<AST>, Option<LineInfo>),
    LogicalNot(Box<AST>, Option<LineInfo>),
    VarAssign {
        name: String,
        value: Box<AST>,
        var_type: Type,
        is_morph: bool,
        line_info: Option<LineInfo>,
    },
    Assignment {
        name: String,
        value: Box<AST>, // 定義済み変数への代入用ノード
        op: AssignmentOp,
        line_info: Option<LineInfo>,
    },
    Var(String, Option<LineInfo>),
    Unveil(Vec<AST>, Option<LineInfo>),
    Trans(Box<AST>, Type, Option<LineInfo>),
    Reveal(Box<AST>, Option<LineInfo>),
    Oracle {
        is_match: bool,
        conditionals: Vec<ConditionalAssignment>,
        branches: Vec<OracleBranch>,
        line_info: Option<LineInfo>,
    },
    OracleDefaultBranch(Option<LineInfo>), // Block(Vec<AST>, Option<LineInfo>),
}

#[derive(Debug, Clone)]
pub struct ConditionalAssignment {
    pub variable: String,     // 条件文で参照する変数
    pub expression: Box<AST>, // 変数に代入される式
    pub line_info: Option<LineInfo>,
}

#[derive(Debug, Clone)]
pub struct OracleBranch {
    pub pattern: Vec<AST>,
    pub body: Box<AST>,
    pub line_info: Option<LineInfo>,
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
