use std::fmt::Debug;

use fixpoint::Fix;

use super::term::{Primitive, Term};
use super::zipper::{Loc};

#[derive (Debug, PartialEq, Eq)]
pub enum ReferenceError { NotFound(String) }

#[derive (Debug, PartialEq, Eq)]
pub enum TypeError { NotAFunction(Primitive) }

#[derive (Debug, PartialEq, Eq)]
pub enum EvalError {
    ReferenceError(ReferenceError),
    TypeError(TypeError)
}

type EvalResult<T> = Result<Fix<Loc<T>>, EvalError>;

fn eval_shallow<T: Clone>(loc: Loc<T>) -> EvalResult<T> {
    match loc {
        Loc(Term::Var(s), c, mut e) => match e.take(&s) {
            Option::None => Result::Err(EvalError::ReferenceError(ReferenceError::NotFound(s))),
            Option::Some((_, t_box)) => Result::Ok(Fix::Pro(Loc(*t_box, c, e)))
        },
        Loc(Term::App(fun_box, arg_box), c, e) => {
            let fun: Term<T> = *fun_box;
            match fun {
                Term::Val(val) => {
                    Result::Err(EvalError::TypeError(TypeError::NotAFunction(val)))
                },
                Term::Abs(id, y, term_box) => {
                    Result::Ok(Fix::Pro(Loc(Term::Let(id, Option::Some((y, arg_box)), term_box), c, e)))
                },
                _ => {
                    Result::Ok(Loc::down(Loc(Term::App(Box::new(fun), arg_box), c, e)))
                }
            }
        },
        Loc(Term::Let(s, o, t_box), c, e) => {
            let t: Term<T> = *t_box;
            match t {
                Term::Val(_) => Result::Ok(Fix::Pro(Loc(t, c, e))),
                Term::Abs(abs_s, abs_y, abs_t_box) => {
                    if s == abs_s {
                        // the abstraction shadows the let,
                        // so it is save to drop it.
                        Result::Ok(Fix::Pro(Loc(Term::Abs(abs_s, abs_y, abs_t_box), c, e)))
                    } else {
                        // the abstraction does not shadow the let,
                        // so it is save to move it into the abstraction.
                        Result::Ok(Fix::Pro(Loc(Term::Abs(abs_s, abs_y, Box::new(Term::Let(s, o, abs_t_box))), c, e)))
                    }
                },
                _ => {
                    Result::Ok(Loc::down(Loc(Term::Let(s, o, Box::new(t)), c, e)))
                }
            }
        }
        _ => Result::Ok(Fix::Fix(loc))
    }
}

pub fn eval<T: Clone>(term: Term<T>) -> Result<Term<T>, EvalError> {
    // find the head normal form of the term,
    // which is equivalent to the fixpoint of
    // the eval_shallow function
    Loc::top(term).fix_result(eval_shallow)
}

pub fn eval_debug<T: Clone + Debug>(term: Term<T>) -> Result<Term<T>, EvalError> {
    Loc::top(term).fix_result(|loc| {
        println!("{:?}", loc);
        eval_shallow(loc)
    })
}

#[test]
fn eval_val() {
    let x = eval(Term::<()>::val_int(42));
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}

#[test]
fn eval_id() {
    let x = eval(
        Term::app(
            Term::abs(
                "x".to_string(), (),
                Term::var("x".to_string())
            ),
            Term::val_int(42)
        )
    );
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}
