#[derive (Debug, PartialEq, Eq)]
pub enum Primitive {
    Integer(i64),
    String(String)
}

#[derive (Debug, PartialEq, Eq)]
pub enum Term {
    Val(Primitive),
    Var(String),
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

    pub fn var(id: String) -> Term {
        Term::Var(id)
    }

    pub fn abs(id: String, term: Term) -> Term {
        Term::Abs(id, Box::new(term))
    }

    pub fn abs_many(first_id: String, many_ids: Vec<String>, term: Term) -> Term {
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
