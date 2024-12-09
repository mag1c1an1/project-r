#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    Or,
    And,
}

#[derive(Debug, Clone)]
pub enum UnaryOpTy {
    Minus,
    Deref,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub ty: UnaryOpTy,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpTy {
    EqualEqual,
    NotEqual,
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone)]
pub struct BinaryOp {
    pub ty: BinaryOpTy,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i32),
    Register(String),
}
