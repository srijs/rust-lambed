#[derive (Debug, PartialEq, Eq)]
pub enum Primitive {
    Integer(i64),
    String(String)
}

#[derive (Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    Integer, String
}

#[derive (Debug, PartialEq, Eq)]
pub enum Type {
    Val(PrimitiveType),
    Abs(Box<Type>, Box<Type>)
}

#[derive (Debug, PartialEq, Eq)]
pub enum Term<T, V> {
    Val(V),
    Var(String),
    Abs(String, T, Box<Term<T, V>>),
    App(Box<Term<T, V>>, Box<Term<T, V>>),
    Let(String, Option<(T, Box<Term<T, V>>)>, Box<Term<T, V>>)
}

impl<T, V> Term<T, V> {

    pub fn var(id: String) -> Term<T, V> {
        Term::Var(id)
    }

    pub fn abs(id: String, y: T, term: Term<T, V>) -> Term<T, V> {
        Term::Abs(id, y, Box::new(term))
    }

    pub fn abs_many(first_arg: (String, T), mut args: Vec<(String, T)>, term: Term<T, V>) -> Term<T, V> {
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

    pub fn app(fun: Term<T, V>, arg: Term<T, V>) -> Term<T, V> {
        Term::App(Box::new(fun), Box::new(arg))
    }

    pub fn app_many(fun: Term<T, V>, first_arg: Term<T, V>, mut args: Vec<Term<T, V>>) -> Term<T, V> {
        args.reverse();
        let mut app_term = Term::app(fun, first_arg);
        while let Some(arg) = args.pop() {
            app_term = Term::app(app_term, arg);
        }
        app_term
    }

}

impl<T> Term<T, Primitive> {

    pub fn val_int(x: i64) -> Term<T, Primitive> {
        Term::Val(Primitive::Integer(x))
    }

    pub fn val_string(x: String) -> Term<T, Primitive> {
        Term::Val(Primitive::String(x))
    }

}

pub type Untyped<V> = Term<(), V>;

impl<V> Untyped<V> {

    pub fn abs_many_untyped(first_id: String, many_ids: Vec<String>, term: Untyped<V>) -> Untyped<V> {
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
