#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.
lalrpop_mod!(pub emoji);        // Synthesised by LALRPOP.
lalrpop_mod!(pub identifier);   // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator3);  // Synthesised by LALRPOP.
lalrpop_mod!(pub ast_parser);   // Synthesised by LALRPOP.

#[macro_use] extern crate failure;

use failure::Error;

use ast::{Op, Term, Type};
mod ast;

fn main() {
    // println!(
    //     "{}",
    //     emoji::EmojiParser::new().parse(":vomit:").unwrap()
    // );
    match compile(Some("1 + 2 + 10".into())) {
        Ok(out) => (),
        Err(e) => eprintln!("Error compiling: {}", e)
    }
}

fn compile(source: Option<String>) -> Result<(), Error> {
    let expr = source.map(
        |src| ast_parser::TermParser::new().parse(&src).unwrap()
    ).unwrap_or_else(
        || box Term::IfThenElse(
            box Term::BinOp(
                Op::Eq,
                box Term::Num(4),
                box::Term::BinOp(
                    Op::Add,
                    box Term::Num(2),
                    box Term::Num(2)
                )
            ),
            box Term::BinOp(
                Op::Add,
                box Term::Num(5),
                box Term::Num(3)
            ),
            box Term::Num(10)
        )
    );

    typecheck(&expr)?;

    let mut ctr = {
        let mut c = 0; move || {
            let ret = c;
            c = c + 1;
            ret
        }
    };

    let mut gen_var = || format!("var{}", ctr());

    println!("fn main() {}", "{");
    let ret = emit(&expr, &mut gen_var)?;
    println!("println!(\"{}\", {})", "{}", ret);
    println!("{}", "}");
    Ok(())
}


#[derive(Debug, Fail)]
enum ThymeError {
    #[fail(display = "Type error: {}", err)]
    TypeError{ err: String },

    #[fail(display = "The typechecker hit a branch it did not anticipate")]
    TypeCheckerError(),

    #[fail(display = "Not implemented")]
    NotImplemented()
}

fn type_of(term: &Term) -> Result<Type, Error> {
    match term {
        Term::T => Ok(Type::TyBool),
        Term::F => Ok(Type::TyBool),
        Term::Num(_) => Ok(Type::TyInt),
        Term::BinOp(Op::Add, t1, t2) => {
            let ty1 = type_of(t1)?;
            let ty2 = type_of(t2)?;

            if (ty1 != Type::TyInt || ty2 != Type::TyInt) {
                Err(ThymeError::TypeError{
                    err: "Non-numeric types in addition".into()
                })?
            } else {
                Ok(Type::TyInt)
            }
        },
        Term::BinOp(Op::Eq, t1, t2) => {
            if type_of(t1)? == type_of(t2)? {
                Ok(Type::TyBool)
            } else {
                Err(ThymeError::TypeError{
                    err: "Branches of equality comparison differ in type".into()
                })?
            }
        },
        Term::IfThenElse(cond, t1, t2) => {
            if !(type_of(cond)? == Type::TyBool) {
                Err(ThymeError::TypeError{
                    err: "Condition is not of type Bool".into()
                })?
            } else if type_of(t1)? != type_of(t2)? {
                Err(ThymeError::TypeError{
                    err: "If branches have different types".into()
                })?
            } else {
                Ok(type_of(t1)?)
            }

        }
        _ => Err(ThymeError::TypeCheckerError())?
    }
}

fn typecheck(term: &Term) -> Result<(), Error> {
    type_of(term)?;
    Ok(())
}

fn emit<F>(term: &Term, gen_var: &mut F) -> Result<String, Error> where
    F: FnMut() -> String
{

    let bra = || println!("{}", "{");
    let ket = || println!("{}", "}");

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

        },

        Term::BinOp(Op::Eq, t1, t2) => {
            let name = gen_var();
            let v1 = emit(t1, gen_var)?;
            let v2 = emit(t2, gen_var)?;
            println!("let {} = {} == {};", name, v1, v2);
            Ok(name)

        },

        Term::IfThenElse(cond, t1, t2) => {
            let name = gen_var();
            let c = emit(cond, gen_var)?;
            println!("let {} = if {} ", name, c);
            bra();
            let truthy = emit(t1, gen_var)?;
            println!("{}", truthy);
            ket();
            println!(" else ");
            bra();
            let falsy = emit(t2, gen_var)?;
            println!("{}", falsy);
            ket();
            println!(";");
            Ok(name)
        }
        _ => Err(ThymeError::NotImplemented())?
    }

}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn ast() {
        assert!(ast_parser::TermParser::new().parse("1 + 1").is_ok());
        assert!(ast_parser::TermParser::new().parse("23 + 69 + 1").is_ok());
        assert!(ast_parser::TermParser::new().parse("32 == 32").is_ok());
        assert!(ast_parser::TermParser::new().parse("true == true").is_ok());
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
