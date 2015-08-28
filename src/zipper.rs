use fixpoint::{Fix, fix, fix_result};

use super::term::Term;

pub enum Ctx {
    Top,
    Abs(String, Box<Ctx>),
    AppL(Box<Ctx>, Term),
    AppR(Term, Box<Ctx>)
}

pub struct Loc(pub Term, pub Ctx);

impl Loc {

    pub fn get(self) -> Term {
        self.0
    }

    pub fn set(self, term: Term) -> Loc {
        Loc(term, self.1)
    }

    pub fn down(self) -> Fix<Loc> {
        match self {
            Loc(Term::Val(s), c) => Fix::Fix(Loc(Term::Val(s), c)),
            Loc(Term::Var(s), c) => Fix::Fix(Loc(Term::Var(s), c)),
            Loc(Term::Abs(s, t1), c) => Fix::Pro(Loc(*t1, Ctx::Abs(s, Box::new(c)))),
            Loc(Term::App(t1, t2), c) => Fix::Pro(Loc(*t1, Ctx::AppL(Box::new(c), *t2)))
        }
    }

    pub fn up(self) -> Fix<Loc> {
        match self {
            Loc(t, Ctx::Top) => Fix::Fix(Loc(t, Ctx::Top)),
            Loc(t1, Ctx::Abs(s, c)) => Fix::Pro(Loc(Term::Abs(s, Box::new(t1)), *c)),
            Loc(t1, Ctx::AppL(c, t2)) => Fix::Pro(Loc(Term::App(Box::new(t1), Box::new(t2)), *c)),
            Loc(t2, Ctx::AppR(t1, c)) => Fix::Pro(Loc(Term::App(Box::new(t1), Box::new(t2)), *c))
        }
    }

    pub fn left(self) -> Fix<Loc> {
        match self {
            Loc(t, Ctx::Top) => Fix::Fix(Loc(t, Ctx::Top)),
            Loc(t1, Ctx::Abs(s, c)) => Fix::Fix(Loc(t1, Ctx::Abs(s, c))),
            Loc(t1, Ctx::AppL(c, t2)) => Fix::Fix(Loc(t1, Ctx::AppL(c, t2))),
            Loc(t2, Ctx::AppR(t1, c)) => Fix::Pro(Loc(t1, Ctx::AppL(c, t2)))
        }
    }

    pub fn right(self) -> Fix<Loc> {
        match self {
            Loc(t, Ctx::Top) => Fix::Fix(Loc(t, Ctx::Top)),
            Loc(t1, Ctx::Abs(s, c)) => Fix::Fix(Loc(t1, Ctx::Abs(s, c))),
            Loc(t1, Ctx::AppL(c, t2)) => Fix::Pro(Loc(t2, Ctx::AppR(t1, c))),
            Loc(t2, Ctx::AppR(t1, c)) => Fix::Fix(Loc(t2, Ctx::AppR(t1, c)))
        }
    }

    pub fn top(term: Term) -> Loc {
        Loc(term, Ctx::Top)
    }

    pub fn fix<F: FnMut(Loc) -> Fix<Loc>>(self, f: F) -> Loc {
        let mut g = f;
        fix(self, |loc| {
            match g(loc) {
                Fix::Pro(loc) => Fix::Pro(loc),
                Fix::Fix(loc) => Loc::up(loc)
            }
        })
    }

    pub fn fix_result<E, F: FnMut(Loc) -> Result<Fix<Loc>, E>>(self, f: F) -> Result<Loc, E> {
        let mut g = f;
        fix_result(self, |loc| {
            g(loc).map(|fix| match fix {
                Fix::Pro(loc) => Fix::Pro(loc),
                Fix::Fix(loc) => Loc::up(loc)
            })
        })
    }

}
