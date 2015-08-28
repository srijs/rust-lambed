use fixpoint::{Fix, fix, fix_result};

use super::term::Term;

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
