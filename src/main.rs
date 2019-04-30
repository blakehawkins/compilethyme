#[derive(Debug, Clone)]
enum Op {
    Add()
}

#[derive(Debug, Clone)]
enum Term {
    T(),
    F(),
    Num(i64),
    BinOp(Op, Box<Term>, Box<Term>),
    IfThenElse(Box<Term>, Box<Term>, Box<Term>)
}

fn main() {
    println!("Hello, world!");
}
