// ast.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Inteiro(i64),
    Decimal(f64),
    Texto(String),
    Logico(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Inteiro,
    Decimal,
    Texto,
    Logico,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,          // +
    Subtract,     // -
    Multiply,     // *
    Divide,       // /
    Modulo,       // %
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate, // -
    Plus,   // +
    Not,    // !
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub param_type: Type,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Call(CallExpr),
    BinaryOp(BinaryOperator, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOperator, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub function: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub var_type: Type,
    pub name: String,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Block,
    pub else_branch: Option<Block>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub variable: String,
    pub start: Expr,
    pub end: Expr,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WriteStmt {
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadStmt {
    pub target: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDecl(VariableDecl),
    ExprStmt(ExprStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    ForStmt(ForStmt),
    ReturnStmt(ReturnStmt),
    WriteStmt(WriteStmt),
    ReadStmt(ReadStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub return_type: Option<Type>,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
    pub statements: Vec<Statement>,
}

// Implementações de Display para debugging
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Inteiro(n) => write!(f, "{}", n),
            Literal::Decimal(n) => write!(f, "{}", n),
            Literal::Texto(s) => write!(f, "\"{}\"", s),
            Literal::Logico(b) => write!(f, "{}", b),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Inteiro => write!(f, "inteiro"),
            Type::Decimal => write!(f, "decimal"),
            Type::Texto => write!(f, "texto"),
            Type::Logico => write!(f, "logico"),
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Less => write!(f, "<"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}