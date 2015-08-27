use std::collections::HashMap;

use fixpoint::{Fix, fix_result};

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

struct ArgStack(Vec<Box<Term>>);

impl ArgStack {

    fn new() -> ArgStack {
        ArgStack(Vec::new())
    }

    fn push(&mut self, arg_box: Box<Term>) {
        self.0.push(arg_box)
    }

    fn pop(&mut self) -> Option<Box<Term>> {
        self.0.pop()
    }

    fn unwind(&mut self, term: Term) -> Fix<Term> {
        match self.pop() {
            Option::None => Fix::Fix(term),
            Option::Some(arg_box) => Fix::Pro(Term::App(Box::new(term), arg_box))
        }
    }

}

type EvalResult = Result<Fix<Term>, EvalError>;

fn eval_shallow(ctx: &mut Context, args: &mut ArgStack, term: Term) -> EvalResult {
    match term {
        Term::Val(val) => Result::Ok(Fix::Fix(Term::Val(val))),
        Term::Abs(id, term_box) => Result::Ok(Fix::Fix(Term::Abs(id, term_box))),
        Term::Ref(id) => match ctx.lookup(id) {
            Result::Err(err) => Result::Err(EvalError::ReferenceError(err)),
            Result::Ok(term) => Result::Ok(Fix::Pro(term))
        },
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
                    args.push(arg_box);
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
    let mut args = ArgStack::new();
    fix_result(term, |term| {
        eval_shallow(ctx, &mut args, term).map(|fix| match fix {
            Fix::Pro(term) => Fix::Pro(term),
            Fix::Fix(term) => args.unwind(term)
        })
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
