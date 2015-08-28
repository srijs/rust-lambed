#[macro_use]
extern crate combine;
extern crate fixpoint;

pub mod term;
pub mod zipper;
pub mod parser;
pub mod eval;

#[test]
fn test_id() {
    let mut scope = eval::Scope::new();
    let source = "{|x| x 5}".to_string();
    let parsed = parser::parse_string(&source);
    let result = parsed.map(|(expr, _)| {
        eval::eval(&mut scope, expr)
    });
    assert_eq!(result, Result::Ok(Result::Ok(term::Term::val_int(5))));
}

#[test]
fn test_id_2() {
    let mut scope = eval::Scope::new();
    let source = "{{|x| x |y| y} 5}".to_string();
    let parsed = parser::parse_string(&source);
    let result = parsed.map(|(expr, _)| {
        eval::eval(&mut scope, expr)
    });
    assert_eq!(result, Result::Ok(Result::Ok(term::Term::val_int(5))));
}
