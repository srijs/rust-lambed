use std::collections::HashMap;

use fixpoint::Fix;

use super::term::{Primitive, Term};
use super::zipper::{Loc, Ctx};

#[derive (Debug, PartialEq, Eq)]
pub enum ReferenceError { NotFound(String) }

#[derive (Debug, PartialEq, Eq)]
pub enum TypeError { NotAFunction(Primitive) }

#[derive (Debug, PartialEq, Eq)]
pub enum EvalError {
    ReferenceError(ReferenceError),
    TypeError(TypeError),
    StackOverflow
}

pub struct Scope(HashMap<String, Term>);

impl Scope {

    pub fn new() -> Scope {
        Scope(HashMap::new())
    }

    pub fn lookup(&mut self, id: String) -> Result<Term, ReferenceError> {
        self.0.remove(&id).ok_or(ReferenceError::NotFound(id))
    }

    pub fn bind(&mut self, id: String, term: Term) {
        self.0.insert(id, term);
    }

}

type EvalResult = Result<Fix<Loc>, EvalError>;

fn eval_shallow(scope: &mut Scope, loc: Loc) -> EvalResult {
    match loc {
        Loc(Term::Var(id), c) => match scope.lookup(id) {
            Result::Ok(term) => Result::Ok(Fix::Pro(Loc(term, c))),
            Result::Err(err) => Result::Err(EvalError::ReferenceError(err))
        },
        Loc(Term::App(fun_box, arg_box), c) => {
            let fun: Term = *fun_box;
            match fun {
                Term::Val(val) => {
                    Result::Err(EvalError::TypeError(TypeError::NotAFunction(val)))
                },
                Term::Abs(id, term_box) => {
                    scope.bind(id, *arg_box);
                    Result::Ok(Fix::Pro(Loc(*term_box, c)))
                },
                _ => {
                    Result::Ok(Fix::Pro(Loc(fun, Ctx::AppL(Box::new(c), *arg_box))))
                }
            }
        },
        _ => Result::Ok(Fix::Fix(loc))
    }
}

pub fn eval(scope: &mut Scope, term: Term) -> Result<Term, EvalError> {
    // find the head normal form of the term,
    // which is equivalent to the fixpoint of
    // the eval_shallow function
    Loc::top(term).fix_result(|loc| {
        eval_shallow(scope, loc)
    })
}

#[test]
fn eval_val() {
    let mut scope = Scope::new();
    let x = eval(&mut scope, Term::val_int(42));
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}

#[test]
fn eval_id() {
    let mut scope = Scope::new();
    let x = eval(&mut scope,
        Term::app(
            Term::abs(
                "x".to_string(),
                Term::var("x".to_string())
            ),
            Term::val_int(42)
        )
    );
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}
