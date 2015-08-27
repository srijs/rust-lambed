use std::collections::HashMap;

use super::term::Term;

#[derive (Debug, PartialEq, Eq)]
pub enum ReferenceError { NotFound }

pub struct Context(HashMap<String, Term>);

impl Context {

    pub fn new() -> Context {
        Context(HashMap::new())
    }

    pub fn lookup(&mut self, id: String) -> Result<Term, ReferenceError> {
        self.0.remove(&id).ok_or(ReferenceError::NotFound)
    }

    pub fn bind(&mut self, id: String, term: Term) {
        self.0.insert(id, term);
    }

}

fn eval_shallow(ctx: &mut Context, term: Term, depth: u32) -> Result<Term, ReferenceError> {
    if depth == 0 {
        return Result::Ok(term)
    }
    match term {
        Term::Val(val) => Result::Ok(Term::Val(val)),
        Term::Abs(id, term) => Result::Ok(Term::Abs(id, term)),
        Term::Ref(id) => ctx.lookup(id),
        Term::App(fun, arg) => {
            match try!(eval_shallow(ctx, *fun, depth-1)) {
                Term::Abs(id, term) => {
                    ctx.bind(id, *arg);
                    Result::Ok(*term)
                },
                fun_eval => {
                    Result::Ok(Term::App(Box::new(fun_eval), arg))
                }
            }
        }
    }
}

pub fn eval(ctx: &mut Context, term: Term) -> Result<Term, ReferenceError> {
    let mut current_term = term;
     // find the head normal form of the term,
     // which is equivalent to the fixpoint of
     // the eval_shallow function
    loop {
        match current_term {
            Term::Val(val) => return Result::Ok(Term::Val(val)),
            Term::Abs(id, term) => return Result::Ok(Term::Abs(id, term)),
            term => current_term = try!(eval_shallow(ctx, term, 2))
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
