#[derive (Debug, PartialEq, Eq)]
pub enum Primitive {
    Integer(i64),
    String(String)
}

#[derive (Debug, PartialEq, Eq)]
pub enum Term<T> {
    Val(Primitive),
    Var(String),
    Abs(String, T, Box<Term<T>>),
    App(Box<Term<T>>, Box<Term<T>>)
}

pub type Untyped = Term<()>;

impl<T> Term<T> {

    pub fn val_int(x: i64) -> Term<T> {
        Term::Val(Primitive::Integer(x))
    }

    pub fn val_string(x: String) -> Term<T> {
        Term::Val(Primitive::String(x))
    }

    pub fn var(id: String) -> Term<T> {
        Term::Var(id)
    }

    pub fn abs(id: String, y: T, term: Term<T>) -> Term<T> {
        Term::Abs(id, y, Box::new(term))
    }

    pub fn abs_many(first_arg: (String, T), many_args: Vec<(String, T)>, term: Term<T>) -> Term<T> {
        let mut args = many_args;
        match args.pop() {
            Option::None => Term::abs(first_arg.0, first_arg.1, term),
            Option::Some((last_id, last_y)) => {
                let mut lambda_term = Term::abs(last_id, last_y, term);
                while let Some((id, y)) = args.pop() {
                    lambda_term = Term::abs(id, y, lambda_term);
                }
                Term::abs(first_arg.0, first_arg.1, lambda_term)
            }
        }
    }

    pub fn app(fun: Term<T>, arg: Term<T>) -> Term<T> {
        Term::App(Box::new(fun), Box::new(arg))
    }

    pub fn app_many(fun: Term<T>, first_arg: Term<T>, many_args: Vec<Term<T>>) -> Term<T> {
        let mut args = many_args;
        args.reverse();
        let mut app_term = Term::app(fun, first_arg);
        while let Some(arg) = args.pop() {
            app_term = Term::app(app_term, arg);
        }
        app_term
    }

}

impl Untyped {

    pub fn abs_many_untyped(first_id: String, many_ids: Vec<String>, term: Untyped) -> Untyped {
        let mut ids = many_ids;
        match ids.pop() {
            Option::None => Term::abs(first_id, (), term),
            Option::Some(last_id) => {
                let mut lambda_term = Term::abs(last_id, (), term);
                while let Some(id) = ids.pop() {
                    lambda_term = Term::abs(id, (), lambda_term);
                }
                Term::abs(first_id, (), lambda_term)
            }
        }
    }

}
