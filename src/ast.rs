#[derive(Debug, Clone)]
pub enum Op {
    Add
}

#[derive(Debug, Clone)]
pub enum Term {
    T,
    F,
    Num(i64),
    BinOp(Op, Box<Term>, Box<Term>),
    IfThenElse(Box<Term>, Box<Term>, Box<Term>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TyBool,
    TyInt,
}
