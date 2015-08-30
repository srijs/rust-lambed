use fixpoint::{Fix, fix, fix_result};

use super::term::Term;

#[derive (Debug, PartialEq, Eq)]
pub enum Ctx<T> {
    Top,
    Abs(String, T, Box<Ctx<T>>),
    AppL(Box<Ctx<T>>, Term<T>),
    AppR(Term<T>, Box<Ctx<T>>),
    Let(Box<Ctx<T>>),
}

#[derive (Debug, PartialEq, Eq)]
pub struct Env<T>(Vec<(String, Option<(T, Box<Term<T>>)>)>);

impl<T> Env<T> {

    pub fn new() -> Env<T> {
        Env(Vec::new())
    }

    pub fn push(&mut self, s: String, o: Option<(T, Box<Term<T>>)>) {
        self.0.push((s, o))
    }

    pub fn pop(&mut self) -> Option<(String, Option<(T, Box<Term<T>>)>)> {
        self.0.pop()
    }

    pub fn take(&mut self, s: &String) -> Option<(T, Box<Term<T>>)> {
        self.0.iter_mut().rev().find(|&&mut (ref v, _)| s == v).and_then(|&mut (_, ref mut o)| o.take())
    }

}

#[derive (Debug, PartialEq, Eq)]
pub struct Loc<T>(pub Term<T>, pub Ctx<T>, pub Env<T>);

impl<T> Loc<T> {

    pub fn get(self) -> Term<T> {
        self.0
    }

    pub fn set(self, term: Term<T>) -> Loc<T> {
        Loc(term, self.1, self.2)
    }

    pub fn down(self) -> Fix<Loc<T>> {
        match self {
            Loc(Term::Abs(s, y, t1), c, e) => Fix::Pro(Loc(*t1, Ctx::Abs(s, y, Box::new(c)), e)),
            Loc(Term::App(t1, t2), c, e) => Fix::Pro(Loc(*t1, Ctx::AppL(Box::new(c), *t2), e)),
            Loc(Term::Let(s, o, t1), c, mut e) => {
                e.push(s, o);
                Fix::Pro(Loc(*t1, Ctx::Let(Box::new(c)), e))
            },
            _ => Fix::Fix(self)
        }
    }

    pub fn up(self) -> Fix<Loc<T>> {
        match self {
            Loc(t1, Ctx::Abs(s, y, c), e) => Fix::Pro(Loc(Term::abs(s, y, t1), *c, e)),
            Loc(t1, Ctx::AppL(c, t2), e) => Fix::Pro(Loc(Term::app(t1, t2), *c, e)),
            Loc(t2, Ctx::AppR(t1, c), e) => Fix::Pro(Loc(Term::app(t1, t2), *c, e)),
            Loc(t1, Ctx::Let(c), mut e) => {
                let (s, o) = e.pop().unwrap();
                Fix::Pro(Loc(Term::Let(s, o, Box::new(t1)), *c, e))
            },
            _ => Fix::Fix(self)
        }
    }

    pub fn left(self) -> Fix<Loc<T>> {
        match self {
            Loc(t2, Ctx::AppR(t1, c), e) => Fix::Pro(Loc(t1, Ctx::AppL(c, t2), e)),
            _ => Fix::Fix(self)
        }
    }

    pub fn right(self) -> Fix<Loc<T>> {
        match self {
            Loc(t1, Ctx::AppL(c, t2), e) => Fix::Pro(Loc(t2, Ctx::AppR(t1, c), e)),
            _ => Fix::Fix(self)
        }
    }

    pub fn top(term: Term<T>) -> Loc<T> {
        Loc(term, Ctx::Top, Env::new())
    }

    pub fn fix<F: FnMut(Loc<T>) -> Fix<Loc<T>>>(self, mut f: F) -> Term<T> {
        fix(self, |loc| {
            match f(loc) {
                Fix::Fix(loc) => loc.up(),
                fix => fix
            }
        }).get()
    }

    pub fn fix_result<E, F: FnMut(Loc<T>) -> Result<Fix<Loc<T>>, E>>(self, mut f: F) -> Result<Term<T>, E> {
        fix_result(self, |loc| {
            f(loc).map(|fix| match fix {
                Fix::Fix(loc) => loc.up(),
                _ => fix
            })
        }).map(Loc::get)
    }

}
