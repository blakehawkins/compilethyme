#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.
lalrpop_mod!(pub emoji);        // Synthesised by LALRPOP.
lalrpop_mod!(pub identifier);   // Synthesised by LALRPOP.

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
    println!(
        "{}",
        emoji::EmojiParser::new().parse(":vomit:").unwrap()
    );
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



#[cfg(test)]
mod test {

    use super::*;


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

    #[test]
    fn emoji() {
        assert!(emoji::EmojiParser::new().parse(":smile:").is_ok());
        assert!(emoji::EmojiParser::new().parse(":smile").is_err());
        assert!(emoji::EmojiParser::new().parse("frown").is_err());
    }

    fn identifier() {
        assert!(identifier::TermParser::new().parse("22").is_ok());
        assert!(identifier::TermParser::new().parse("(22)").is_ok());
        assert!(identifier::TermParser::new().parse("(((69)))").is_ok());
        assert!(identifier::TermParser::new().parse("((1)").is_err());
    }

    #[test]
    fn typecheck_fail() {
        let term = Term::IfThenElse(box Term::Num(4), box Term::Num(6), box Term::Num(7));
        assert!(type_of(&term).is_err())
    }

    #[test]
    fn typecheck_pass() -> Result<(), Error> {
        let term = Term::IfThenElse(box Term::T, box Term::BinOp(Op::Add, box Term::Num(5), box Term::Num(3)), box Term::Num(10));
        assert_eq!(type_of(&term)?, Type::TyInt);
        Ok(())
    }

}
