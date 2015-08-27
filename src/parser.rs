use std::iter::FromIterator;

use super::term::{Primitive, Term};

use combine::{Parser, ParserExt, State, ParseResult, ParseError, between, spaces, char, digit, many1, alpha_num, parser};
use combine::primitives::{Stream};

fn token_left_paren<I: Stream<Item=char>>(input: State<I>) -> ParseResult<(), I> {
    char('(').map(|_| ()).skip(spaces()).parse_state(input)
}

fn token_right_paren<I: Stream<Item=char>>(input: State<I>) -> ParseResult<(), I> {
    char(')').map(|_| ()).skip(spaces()).parse_state(input)
}

fn token_left_brace<I: Stream<Item=char>>(input: State<I>) -> ParseResult<(), I> {
    char('{').map(|_| ()).skip(spaces()).parse_state(input)
}

fn token_right_brace<I: Stream<Item=char>>(input: State<I>) -> ParseResult<(), I> {
    char('}').map(|_| ()).skip(spaces()).parse_state(input)
}

fn token_integer<I: Stream<Item=char>>(input: State<I>) -> ParseResult<i64, I> {
    many1::<Vec<char>, _>(digit()).map(|s| {
        let string: String = FromIterator::from_iter(s);
        string.parse().unwrap()
    }).skip(spaces()).parse_state(input)
}

fn token_identifier<I: Stream<Item=char>>(input: State<I>) -> ParseResult<String, I> {
    many1::<Vec<char>, _>(alpha_num().or(char('-'))).map(|s| {
        FromIterator::from_iter(s)
    }).skip(spaces()).parse_state(input)
}

#[test]
fn test_parse_token_left_paren() {
    let result = parser(token_left_paren).parse("(");
    assert_eq!(result, Result::Ok(((), "")));
}

#[test]
fn test_parse_token_right_paren() {
    let result = parser(token_right_paren).parse(")");
    assert_eq!(result, Result::Ok(((), "")));
}

#[test]
fn test_parse_token_left_brace() {
    let result = parser(token_left_brace).parse("{");
    assert_eq!(result, Result::Ok(((), "")));
}

#[test]
fn test_parse_token_right_brace() {
    let result = parser(token_right_brace).parse("}");
    assert_eq!(result, Result::Ok(((), "")));
}

#[test]
fn test_parse_token_integer() {
    let result = parser(token_integer).parse("42");
    assert_eq!(result, Result::Ok((42, "")));
}

pub fn primitive<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Primitive, I> {
    parser(token_integer).map(|i| Primitive::Integer(i)).parse_state(input)
}

fn term_val<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Term, I> {
    parser(primitive).map(|val| Term::Val(val)).parse_state(input)
}

fn term_ref<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Term, I> {
    parser(token_identifier).map(|id| Term::Ref(id)).parse_state(input)
}

fn term_app<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Term, I> {
    between(
        parser(token_left_paren),
        parser(token_right_paren),
        (parser(term), parser(term))
    ).map(|(fun, arg)| Term::app(fun, arg)).parse_state(input)
}

fn term_fun<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Term, I> {
    between(
        parser(token_left_brace),
        parser(token_right_brace),
        (parser(token_identifier), parser(term))
    ).map(|(id, term)| Term::fun(id, term)).parse_state(input)
}

pub fn term<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Term, I> {
    parser(term_val).or(parser(term_ref)).or(parser(term_app)).or(parser(term_fun)).parse_state(input)
}

#[test]
fn test_term_val_int() {
    let result = parser(term).parse("42");
    assert_eq!(result, Result::Ok((Term::val_int(42), "")));
}

#[test]
fn test_term_ref() {
    let result = parser(term).parse("x");
    let expr = Term::id("x".to_string());
    assert_eq!(result, Result::Ok((expr, "")));
}

#[test]
fn test_term_app() {
    let result = parser(term).parse("(a b)");
    let expr = Term::app(
        Term::id("a".to_string()),
        Term::id("b".to_string())
    );
    assert_eq!(result, Result::Ok((expr, "")));
}

#[test]
fn test_term_fun() {
    let result = parser(term).parse("{a b}");
    let expr = Term::fun(
        "a".to_string(),
        Term::id("b".to_string())
    );
    assert_eq!(result, Result::Ok((expr, "")));
}

pub fn parse_string(s: &String) -> Result<(Term, &str), ParseError<&str>> {
    parser(term).parse(s)
}
