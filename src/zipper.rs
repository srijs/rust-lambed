use std::borrow::Cow;
use fixpoint::{Fix, fix, fix_result};

use super::term::Term;

#[derive (Debug, PartialEq, Eq)]
pub enum Ctx<T, V> {
    Top,
    Abs(Box<Ctx<T, V>>),
    AppL(Box<Ctx<T, V>>, Term<T, V>),
    AppR(Term<T, V>, Box<Ctx<T, V>>),
    Let(Box<Ctx<T, V>>),
}

#[derive (Debug, PartialEq, Eq)]
enum Bind<T, V> {
    Var(String, T),
    Let(String, Option<(T, Box<Term<T, V>>)>)
}

impl<T, V> Bind<T, V> {

    fn is_identified_by(&self, id: &str) -> bool {
        match self {
            &Bind::Var(ref s, _) => id == s,
            &Bind::Let(ref s, _) => id == s
        }
    }

}

#[derive (Debug, PartialEq, Eq)]
pub struct Env<T, V>(Vec<Bind<T, V>>);

impl<T, V> Env<T, V> {

    fn new() -> Env<T, V> {
        Env(Vec::new())
    }

    fn push_var(&mut self, s: String, y: T) {
        self.0.push(Bind::Var(s, y))
    }

    fn push_let(&mut self, s: String, o: Option<(T, Box<Term<T, V>>)>) {
        self.0.push(Bind::Let(s, o))
    }

    fn pop_var(&mut self) -> (String, T) {
        self.0.pop().map(|bind| {
            match bind {
                Bind::Let(_, _) => panic!("tried to pop a let"),
                Bind::Var(s, y) => (s, y)
            }
        }).unwrap()
    }

    fn pop_let(&mut self) -> (String, Option<(T, Box<Term<T, V>>)>) {
        self.0.pop().map(|bind| {
            match bind {
                Bind::Let(s, o) => (s, o),
                Bind::Var(_, _) => panic!("tried to pop a var")
            }
        }).unwrap()
    }

    fn lookup<'a>(&'a mut self, s: &str) -> Option<&'a mut Bind<T, V>> {
        self.0.iter_mut().rev().find(|&&mut ref bind| {
            bind.is_identified_by(s)
        })
    }

    pub fn take<'a>(&'a mut self, id: &str) -> Option<Fix<(Cow<'a, T>, Box<Term<T, V>>)>> where T: Clone {
        self.lookup(id).and_then(|bind| {
            match bind {
                &mut Bind::Var(ref s, ref y) => Some(Fix::Fix((Cow::Borrowed(y), Box::new(Term::Var(s.clone()))))),
                &mut Bind::Let(_, ref mut o) => o.take().map(|(y, t_box)| Fix::Pro((Cow::Owned(y), t_box)))
            }
        })
    }

    pub fn take_term(&mut self, id: &str) -> Option<Fix<Box<Term<T, V>>>> {
        self.lookup(id).and_then(|bind| {
            match bind {
                &mut Bind::Var(ref s, _) => Some(Fix::Fix(Box::new(Term::Var(s.clone())))),
                &mut Bind::Let(_, ref mut o) => o.take().map(|(_, t_box)| Fix::Pro(t_box))
            }
        })
    }

}

#[derive (Debug, PartialEq, Eq)]
pub struct Loc<T, V>(pub Term<T, V>, pub Ctx<T, V>, pub Env<T, V>);

impl<T, V> Loc<T, V> {

    pub fn get(self) -> Term<T, V> {
        self.0
    }

    pub fn set(self, term: Term<T, V>) -> Loc<T, V> {
        Loc(term, self.1, self.2)
    }

    pub fn down(self) -> Fix<Loc<T, V>> {
        match self {
            Loc(Term::Abs(s, y, t1), c, mut e) => {
                e.push_var(s, y);
                Fix::Pro(Loc(*t1, Ctx::Abs(Box::new(c)), e))
            },
            Loc(Term::App(t1, t2), c, e) => Fix::Pro(Loc(*t1, Ctx::AppL(Box::new(c), *t2), e)),
            Loc(Term::Let(s, o, t1), c, mut e) => {
                e.push_let(s, o);
                Fix::Pro(Loc(*t1, Ctx::Let(Box::new(c)), e))
            },
            _ => Fix::Fix(self)
        }
    }

    pub fn up(self) -> Fix<Loc<T, V>> {
        match self {
            Loc(t1, Ctx::Abs(c), mut e) => {
                let (s, y) = e.pop_var();
                Fix::Pro(Loc(Term::abs(s, y, t1), *c, e))
            },
            Loc(t1, Ctx::AppL(c, t2), e) => Fix::Pro(Loc(Term::app(t1, t2), *c, e)),
            Loc(t2, Ctx::AppR(t1, c), e) => Fix::Pro(Loc(Term::app(t1, t2), *c, e)),
            Loc(t1, Ctx::Let(c), mut e) => {
                let (s, o) = e.pop_let();
                Fix::Pro(Loc(Term::Let(s, o, Box::new(t1)), *c, e))
            },
            _ => Fix::Fix(self)
        }
    }

    pub fn left(self) -> Fix<Loc<T, V>> {
        match self {
            Loc(t2, Ctx::AppR(t1, c), e) => Fix::Pro(Loc(t1, Ctx::AppL(c, t2), e)),
            _ => Fix::Fix(self)
        }
    }

    pub fn right(self) -> Fix<Loc<T, V>> {
        match self {
            Loc(t1, Ctx::AppL(c, t2), e) => Fix::Pro(Loc(t2, Ctx::AppR(t1, c), e)),
            _ => Fix::Fix(self)
        }
    }

    pub fn top(term: Term<T, V>) -> Loc<T, V> {
        Loc(term, Ctx::Top, Env::new())
    }

    pub fn fix<F: FnMut(Loc<T, V>) -> Fix<Loc<T, V>>>(self, mut f: F) -> Term<T, V> {
        fix(self, |loc| {
            match f(loc) {
                Fix::Fix(loc) => loc.up(),
                fix => fix
            }
        }).get()
    }

    pub fn fix_result<E, F: FnMut(Loc<T, V>) -> Result<Fix<Loc<T, V>>, E>>(self, mut f: F) -> Result<Term<T, V>, E> {
        fix_result(self, |loc| {
            f(loc).map(|fix| match fix {
                Fix::Fix(loc) => loc.up(),
                _ => fix
            })
        }).map(Loc::get)
    }

}
