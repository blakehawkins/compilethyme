#![feature(box_syntax, box_patterns)]

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator1);  // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator2);  // Synthesised by LALRPOP.
lalrpop_mod!(pub emoji);        // Synthesised by LALRPOP.
lalrpop_mod!(pub identifier);   // Synthesised by LALRPOP.
lalrpop_mod!(pub calculator3);  // Synthesised by LALRPOP.
lalrpop_mod!(pub ast_parser);   // Synthesised by LALRPOP.

#[macro_use] extern crate failure;
#[macro_use] extern crate structopt;

use failure::Error;
use structopt::StructOpt;

use ast::{Op, Expr, Term, Type};
mod ast;

#[derive(Debug, StructOpt)]
#[structopt(name = "compilerthyme", about = "Compile thyme y'all.")]
struct CliOpts {
    input: String
}


fn main() {
    // println!(
    //     "{}",
    //     emoji::EmojiParser::new().parse(":vomit:").unwrap()
    // );
    let opt = CliOpts::from_args();
    match compile(Some(opt.input)) {
        Ok(_) => (),
        Err(e) => eprintln!("Error compiling: {}", e)
    }
}

fn compile(source: Option<String>) -> Result<(), Error> {
    let expr = source.map(
        |src| ast_parser::ExprParser::new().parse(&src).unwrap()
    ).unwrap_or_else(
        || box Expr::IfThenElse(
            box Expr::BinOp(
                Op::Eq,
                box Expr::Trm(Term::Num(4)),
                box::Expr::BinOp(
                    Op::Add,
                    box Expr::Trm(Term::Num(2)),
                    box Expr::Trm(Term::Num(2)),
                )
            ),
            box Expr::BinOp(
                Op::Add,
                box Expr::Trm(Term::Num(5)),
                box Expr::Trm(Term::Num(3)),
            ),
            box Expr::Trm(Term::Num(10))
        )
    );

    println!("{:?}", expr);
    typecheck(&expr)?;

    let mut genvar = {
        let mut c = 0; move || {
            let ret = c;
            c = c + 1;
            format!("var{}", ret)
        }
    };

    println!("fn main() {}", "{");
    let ret = emit(&expr, &mut genvar)?;
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

fn type_of(term: &Expr) -> Result<Type, Error> {
    match term {
        Expr::Trm(Term::T) => Ok(Type::TyBool),
        Expr::Trm(Term::F) => Ok(Type::TyBool),
        Expr::Trm(Term::Num(_)) => Ok(Type::TyInt),
        Expr::BinOp(Op::Add, t1, t2) => {
            let ty1 = type_of(t1)?;
            let ty2 = type_of(t2)?;

            if ty1 != Type::TyInt || ty2 != Type::TyInt {
                Err(ThymeError::TypeError{
                    err: format!("Non numeric types in addition; {:?}: {:?}, {:?}, {:?}", t1, ty1, t2, ty2)
                })?
            } else {
                Ok(Type::TyInt)
            }
        },
        Expr::BinOp(Op::Eq, t1, t2) => {
            if type_of(t1)? == type_of(t2)? {
                Ok(Type::TyBool)
            } else {
                Err(ThymeError::TypeError{
                    err: "Branches of equality comparison differ in type".into()
                })?
            }
        },
        Expr::IfThenElse(cond, t1, t2) => {
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

fn typecheck(term: &Expr) -> Result<(), Error> {
    type_of(term)?;
    Ok(())
}

fn emit<F>(term: &Expr, gen_var: &mut F) -> Result<String, Error> where
   F: FnMut() -> String
{

    let bra = || println!("{}", "{");
    let ket = || println!("{}", "}");

    match term {
        Expr::Trm(Term::T) => {
            let name = gen_var();
            println!("let {} = true;", name);
            Ok(name)
        }

        Expr::Trm(Term::F) => {
            let name = gen_var();

            println!("let {} = false;", name);
            Ok(name)
        }

        Expr::Trm(Term::Num(n)) => {
            let name = gen_var();
            println!("let {}: i64 = {};", name, n);
            Ok(name)
        }

        Expr::BinOp(Op::Add, t1, t2) => {
            let name = gen_var();
            let v1 = emit(t1, gen_var)?;
            let v2 = emit(t2, gen_var)?;
            println!("let {} = {} + {};", name, v1, v2);
            Ok(name)

        },

        Expr::BinOp(Op::Eq, t1, t2) => {
            let name = gen_var();
            let v1 = emit(t1, gen_var)?;
            let v2 = emit(t2, gen_var)?;
            println!("let {} = {} == {};", name, v1, v2);
            Ok(name)

        },

        Expr::IfThenElse(cond, t1, t2) => {
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
        assert!(ast_parser::ExprParser::new().parse("1 + 1").is_ok());
        assert!(ast_parser::ExprParser::new().parse("23 + 69 + 1").is_ok());
        assert!(ast_parser::ExprParser::new().parse("32 == 32").is_ok());
        assert!(ast_parser::ExprParser::new().parse("true == true").is_ok());
        assert!(ast_parser::ExprParser::new().parse("true ? 1 : 2").is_ok());
        assert!(ast_parser::ExprParser::new().parse("1 ?: 2").is_err());
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
        let term = Expr::IfThenElse(box Expr::Trm(Term::Num(4)), box Expr::Trm(Term::Num(6)), box Expr::Trm(Term::Num(7)));
        assert!(type_of(&term).is_err())
    }

    #[test]
    fn typecheck_pass() -> Result<(), Error> {
        let term = Expr::IfThenElse(box Expr::Trm(Term::T), box Expr::BinOp(Op::Add, box Expr::Trm(Term::Num(5)), box Expr::Trm(Term::Num(3))), box Expr::Trm(Term::Num(10)));
        assert_eq!(type_of(&term)?, Type::TyInt);
        Ok(())
    }

}
