use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Variable(String),
    Negate(Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
}
