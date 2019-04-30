#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.
#[macro_use] extern crate failure;

use failure::Error;

#[derive(Debug, Clone)]
enum Op {
    Add
}

#[derive(Debug, Clone)]
enum Term {
    T,
    F,
    Num(i64),
    BinOp(Op, Box<Term>, Box<Term>),
    IfThenElse(Box<Term>, Box<Term>, Box<Term>)
}


#[derive(Debug, Clone, PartialEq)]
enum Type {
    TyBool,
    TyInt
}

fn main() {
    match compile("foo".into()) {
        Ok(out) => println!("{}", out),
        Err(e) => eprintln!("Error compiling: {}", e)
    }
}

fn compile(source: String) -> Result<String, Error> {
    let expr = Term::IfThenElse(box Term::T, box Term::BinOp(Op::Add, box Term::Num(5), box Term::Num(3)), box Term::Num(10));
    println!("Type: {:?}", type_of(&expr));
    Ok(source)
}


#[derive(Debug, Fail)]
enum ThymeError {
    #[fail(display = "Does not typecheck")]
    DoesNotTypecheck()
}

fn type_of(term: &Term) -> Result<Type, Error> {
    match term {
        Term::T => Ok(Type::TyBool),
        Term::F => Ok(Type::TyBool),
        Term::Num(_) => Ok(Type::TyInt),
        Term::BinOp(_, box Term::Num(_), box Term::Num(_)) => Ok(Type::TyInt),
        Term::IfThenElse(cond, t1, t2)
            if type_of(cond)? == Type::TyBool && type_of(t1)? == type_of(t2)? => type_of(t1),
        _ => Err(ThymeError::DoesNotTypecheck())?
    }
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
