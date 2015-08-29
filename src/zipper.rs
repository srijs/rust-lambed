use fixpoint::{Fix, fix, fix_result};

use super::term::Term;

pub enum Ctx<T> {
    Top,
    Abs(String, T, Box<Ctx<T>>),
    AppL(Box<Ctx<T>>, Term<T>),
    AppR(Term<T>, Box<Ctx<T>>)
}

pub struct Loc<T>(pub Term<T>, pub Ctx<T>);

impl<T> Loc<T> {

    pub fn get(self) -> Term<T> {
        self.0
    }

    pub fn set(self, term: Term<T>) -> Loc<T> {
        Loc(term, self.1)
    }

    pub fn down(self) -> Fix<Loc<T>> {
        match self {
            Loc(Term::Abs(s, y, t1), c) => Fix::Pro(Loc(*t1, Ctx::Abs(s, y, Box::new(c)))),
            Loc(Term::App(t1, t2), c) => Fix::Pro(Loc(*t1, Ctx::AppL(Box::new(c), *t2))),
            _ => Fix::Fix(self)
        }
    }

    pub fn up(self) -> Fix<Loc<T>> {
        match self {
            Loc(t1, Ctx::Abs(s, y, c)) => Fix::Pro(Loc(Term::abs(s, y, t1), *c)),
            Loc(t1, Ctx::AppL(c, t2)) => Fix::Pro(Loc(Term::app(t1, t2), *c)),
            Loc(t2, Ctx::AppR(t1, c)) => Fix::Pro(Loc(Term::app(t1, t2), *c)),
            _ => Fix::Fix(self)
        }
    }

    pub fn left(self) -> Fix<Loc<T>> {
        match self {
            Loc(t2, Ctx::AppR(t1, c)) => Fix::Pro(Loc(t1, Ctx::AppL(c, t2))),
            _ => Fix::Fix(self)
        }
    }

    pub fn right(self) -> Fix<Loc<T>> {
        match self {
            Loc(t1, Ctx::AppL(c, t2)) => Fix::Pro(Loc(t2, Ctx::AppR(t1, c))),
            _ => Fix::Fix(self)
        }
    }

    pub fn top(term: Term<T>) -> Loc<T> {
        Loc(term, Ctx::Top)
    }

    pub fn fix<F: FnMut(Loc<T>) -> Fix<Loc<T>>>(self, f: F) -> Term<T> {
        let mut g = f;
        fix(self, |loc| {
            match g(loc) {
                Fix::Fix(loc) => loc.up(),
                fix => fix
            }
        }).get()
    }

    pub fn fix_result<E, F: FnMut(Loc<T>) -> Result<Fix<Loc<T>>, E>>(self, f: F) -> Result<Term<T>, E> {
        let mut g = f;
        fix_result(self, |loc| {
            g(loc).map(|fix| match fix {
                Fix::Fix(loc) => loc.up(),
                _ => fix
            })
        }).map(Loc::get)
    }

}
