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

    pub fn app(fun: Term, arg: Term) -> Term {
        Term::App(Box::new(fun), Box::new(arg))
    }

}
