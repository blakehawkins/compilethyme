#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.
lalrpop_mod!(pub emoji);        // Synthesised by LALRPOP.
lalrpop_mod!(pub identifier);   // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator3);  // Synthesised by LALRPOP.

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
    IfThenElse(Box<Term>, Box<Term>, Box<Term>),
}


#[derive(Debug, Clone, PartialEq)]
enum Type {
    TyBool,
    TyInt
}

fn main() {
    // println!(
    //     "{}",
    //     emoji::EmojiParser::new().parse(":vomit:").unwrap()
    // );
    match compile("foo".into()) {
        Ok(out) => (),
        Err(e) => eprintln!("Error compiling: {}", e)
    }
}

fn compile(source: String) -> Result<(), Error> {
    let expr = Term::IfThenElse(box Term::T, box Term::BinOp(Op::Add, box Term::Num(5), box Term::Num(3)), box Term::Num(10));
    // println!("Type: {:?}", type_of(&expr));

    let mut ctr = {
        let mut c = 0; move || {
            let ret = c;
            c = c + 1;
            ret
        }
    };

    let mut gen_var = || format!("var{}", ctr());

    println!("fn main() {}", "{");
    let ret = emit(&Term::BinOp(Op::Add, box Term::Num(5), box Term::Num(3)), &mut gen_var)?;
    println!("println!(\"{}\", {})", "{}", ret);
    println!("{}", "}");
    // emit(&expr, &mut gen_var)?;
    Ok(())
}


#[derive(Debug, Fail)]
enum ThymeError {
    #[fail(display = "Does not typecheck")]
    DoesNotTypecheck(),

    #[fail(display = "Not implemented")]
    NotImplemented()
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

fn emit<F>(term: &Term, gen_var: &mut F) -> Result<String, Error> where
    F: FnMut() -> String
{

    match term {
        Term::T => {
            let name = gen_var();
            println!("let {} = true;", name);
            Ok(name)
        }

        Term::F => {
            let name = gen_var();
            println!("let {} = false;", name);
            Ok(name)
        }

        Term::Num(n) => {
            let name = gen_var();
            println!("let {}: i64 = {};", name, n);
            Ok(name)
        }

        Term::BinOp(Op::Add, t1, t2) => {
            let name = gen_var();
            let v1 = emit(t1, gen_var)?;
            let v2 = emit(t2, gen_var)?;
            println!("let {} = {} + {};", name, v1, v2);
            Ok(name)

        }
        _ => Err(ThymeError::NotImplemented())?
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

    #[test]
    fn identifier() {
        assert!(identifier::TermParser::new().parse("22").is_ok());
        assert!(identifier::TermParser::new().parse("(22)").is_ok());
        assert!(identifier::TermParser::new().parse("(((69)))").is_ok());
        assert!(identifier::TermParser::new().parse("((1)").is_err());
    }

    #[test]
    fn fullcalc() {
        assert!(calculator3::ExprParser::new().parse("1 + 1").is_ok());
        assert!(calculator3::ExprParser::new().parse("2*(1 + 1)").is_ok());
        assert!(calculator3::ExprParser::new().parse("69/(2 - 1)").is_ok());
        assert!(calculator3::ExprParser::new().parse("  1 + 32    -1").is_ok());
        assert!(calculator3::ExprParser::new().parse("( 1 + 32   -1").is_err());
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
