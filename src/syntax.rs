#[derive(Debug, Eq, PartialEq)]
pub enum Expr {
    Var(String),
    Int(i64),
    Bool(bool),
    Plus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}
