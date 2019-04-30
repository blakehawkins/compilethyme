#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.

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

#[test]
fn calculator1() {
  assert!(calculator1::TermParser::new().parse("22").is_ok());
  assert!(calculator1::TermParser::new().parse("(22)").is_ok());
  assert!(calculator1::TermParser::new().parse("(((69)))").is_ok());
  assert!(calculator1::TermParser::new().parse("((1)").is_err());
}

#[test]
fn calculator2() {
  assert!(calculator2::TermParser::new().parse("22").is_ok());
  assert!(calculator2::TermParser::new().parse("(22)").is_ok());
  assert!(calculator2::TermParser::new().parse("(((69)))").is_ok());
  assert!(calculator2::TermParser::new().parse("((1)").is_err());
}
