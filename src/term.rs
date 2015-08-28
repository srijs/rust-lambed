use fixpoint::{Fix, fix, fix_result};

#[derive (Debug, PartialEq, Eq)]
pub enum Primitive {
    Integer(i64),
    String(String)
}

#[derive (Debug, PartialEq, Eq)]
pub enum Term {
    Val(Primitive),
    Ref(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>)
}

impl Term {

    pub fn val_int(x: i64) -> Term {
        Term::Val(Primitive::Integer(x))
    }

    pub fn val_string(x: String) -> Term {
        Term::Val(Primitive::String(x))
    }

    pub fn id(id: String) -> Term {
        Term::Ref(id)
    }

    pub fn fun(id: String, term: Term) -> Term {
        Term::Abs(id, Box::new(term))
    }

    pub fn fun_many(first_id: String, many_ids: Vec<String>, term: Term) -> Term {
        let mut ids = many_ids;
        match ids.pop() {
            Option::None => Term::Abs(first_id, Box::new(term)),
            Option::Some(last_id) => {
                let mut lambda_term = Term::Abs(last_id, Box::new(term));
                while let Some(id) = ids.pop() {
                    lambda_term = Term::Abs(id, Box::new(lambda_term));
                }
                Term::Abs(first_id, Box::new(lambda_term))
            }
        }
    }

    pub fn app(fun: Term, arg: Term) -> Term {
        Term::App(Box::new(fun), Box::new(arg))
    }

    pub fn app_many(fun: Term, first_arg: Term, many_args: Vec<Term>) -> Term {
        let mut args = many_args;
        args.reverse();
        let mut app_term = Term::App(Box::new(fun), Box::new(first_arg));
        while let Some(arg) = args.pop() {
            app_term = Term::App(Box::new(app_term), Box::new(arg));
        }
        app_term
    }

}

pub enum Zoom {
    Abs(String),
    AppL(Box<Term>),
    AppR(Box<Term>)
}

pub struct ZoomStack(Vec<Zoom>);

impl ZoomStack {

    pub fn new() -> ZoomStack {
        ZoomStack(Vec::new())
    }

    pub fn push(&mut self, zoom: Zoom) {
        self.0.push(zoom)
    }

    pub fn unwind(&mut self, term: Term) -> Fix<Term> {
        match self.0.pop() {
            Option::None => Fix::Fix(term),
            Option::Some(Zoom::Abs(id)) => Fix::Pro(Term::Abs(id, Box::new(term))),
            Option::Some(Zoom::AppL(term_box)) => Fix::Pro(Term::App(Box::new(term), term_box)),
            Option::Some(Zoom::AppR(term_box)) => Fix::Pro(Term::App(term_box, Box::new(term))),
        }
    }

    pub fn fix<F: FnMut(&mut ZoomStack, Term) -> Fix<Term>>(&mut self, term: Term, f: F) -> Term {
        let mut g = f;
        fix(term, |term| {
            match g(self, term) {
                Fix::Pro(term) => Fix::Pro(term),
                Fix::Fix(term) => self.unwind(term)
            }
        })
    }

    pub fn fix_result<E, F: FnMut(&mut ZoomStack, Term) -> Result<Fix<Term>, E>>(&mut self, term: Term, f: F) -> Result<Term, E> {
        let mut g = f;
        fix_result(term, |term| {
            g(self, term).map(|fix| match fix {
                Fix::Pro(term) => Fix::Pro(term),
                Fix::Fix(term) => self.unwind(term)
            })
        })
    }

}
