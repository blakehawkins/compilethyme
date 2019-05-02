#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Eq
}

#[derive(Debug, Clone)]
pub enum Expr {
    Trm(Term),
    BinOp(Op, Box<Expr>, Box<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Term {
    T,
    F,
    Num(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TyBool,
    TyInt,
}
