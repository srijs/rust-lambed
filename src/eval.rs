use std::collections::HashMap;

use super::term::{Primitive, Term};

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

fn eval_shallow(ctx: &mut Context, term: Term, depth: u32) -> Result<Term, EvalError> {
    if depth == 0 {
        return Result::Err(EvalError::StackOverflow)
    }
    match term {
        Term::Val(val) => Result::Ok(Term::Val(val)),
        Term::Abs(id, term_box) => Result::Ok(Term::Abs(id, term_box)),
        Term::Ref(id) => ctx.lookup(id).map_err(EvalError::ReferenceError),
        Term::App(fun_box, arg_box) => {
            let fun: Term = *fun_box;
            match fun {
                Term::Val(val) => Result::Err(EvalError::TypeError(TypeError::NotAFunction(val))),
                Term::Abs(id, term_box) => {
                    ctx.bind(id, *arg_box);
                    Result::Ok(*term_box)
                },
                fun_term => eval_shallow(ctx, fun_term, depth-1).map(|new_term| {
                    Term::App(Box::new(new_term), arg_box)
                })
            }
        }
    }
}

pub fn eval(ctx: &mut Context, term: Term) -> Result<Term, EvalError> {
    let mut current_term = term;
     // find the head normal form of the term,
     // which is equivalent to the fixpoint of
     // the eval_shallow function
    loop {
        match current_term {
            Term::Val(val) => return Result::Ok(Term::Val(val)),
            Term::Abs(id, term) => return Result::Ok(Term::Abs(id, term)),
            term => current_term = try!(eval_shallow(ctx, term, 255))
        }
    }
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
