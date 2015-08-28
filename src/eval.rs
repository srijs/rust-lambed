use std::collections::HashMap;

use fixpoint::Fix;

use super::term::{Primitive, Term};
use super::zoom::{Zoom, ZoomStack};

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

pub struct Context(HashMap<String, Term>);

impl Context {

    pub fn new() -> Context {
        Context(HashMap::new())
    }

    pub fn lookup(&mut self, id: String) -> Result<Term, ReferenceError> {
        self.0.remove(&id).ok_or(ReferenceError::NotFound(id))
    }

    pub fn bind(&mut self, id: String, term: Term) {
        self.0.insert(id, term);
    }

}

type EvalResult = Result<Fix<Term>, EvalError>;

fn eval_shallow(ctx: &mut Context, zooms: &mut ZoomStack, term: Term) -> EvalResult {
    match term {
        Term::Val(val) => Result::Ok(Fix::Fix(Term::Val(val))),
        Term::Abs(id, term_box) => Result::Ok(Fix::Fix(Term::Abs(id, term_box))),
        Term::Ref(id) => ctx.lookup(id).map(Fix::Pro).map_err(EvalError::ReferenceError),
        Term::App(fun_box, arg_box) => {
            let fun: Term = *fun_box;
            match fun {
                Term::Val(val) => {
                    Result::Err(EvalError::TypeError(TypeError::NotAFunction(val)))
                },
                Term::Abs(id, term_box) => {
                    ctx.bind(id, *arg_box);
                    Result::Ok(Fix::Pro(*term_box))
                },
                fun_term => {
                    zooms.push(Zoom::AppL(arg_box));
                    Result::Ok(Fix::Pro(fun_term))
                }
            }
        }
    }
}

pub fn eval(ctx: &mut Context, term: Term) -> Result<Term, EvalError> {
    // find the head normal form of the term,
    // which is equivalent to the fixpoint of
    // the eval_shallow function
    ZoomStack::new().fix_result(term, |zooms, term| {
        eval_shallow(ctx, zooms, term)
    })
}

#[test]
fn eval_val() {
    let mut c = Context::new();
    let x = eval(&mut c, Term::val_int(42));
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}

#[test]
fn eval_id() {
    let mut c = Context::new();
    let x = eval(&mut c,
        Term::app(
            Term::fun(
                "x".to_string(),
                Term::id("x".to_string())
            ),
            Term::val_int(42)
        )
    );
    assert_eq!(x, Result::Ok(Term::val_int(42)));
}
