use std::iter::FromIterator;

use super::term::{Primitive, Untyped, Term};

use combine::{Parser, ParserExt, State, ParseResult, ParseError, between, sep_by, letter, spaces, char, digit, many1, parser};
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

fn token_bar<I: Stream<Item=char>>(input: State<I>) -> ParseResult<(), I> {
    char('|').map(|_| ()).skip(spaces()).parse_state(input)
}

fn token_integer<I: Stream<Item=char>>(input: State<I>) -> ParseResult<i64, I> {
    many1::<Vec<char>, _>(digit()).map(|s| {
        let string: String = FromIterator::from_iter(s);
        string.parse().unwrap()
    }).skip(spaces()).parse_state(input)
}

fn token_identifier<I: Stream<Item=char>>(input: State<I>) -> ParseResult<String, I> {
    many1::<Vec<char>, _>(letter().or(char('-'))).map(|s| {
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
fn test_parse_token_bar() {
    let result = parser(token_bar).parse("|");
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

fn term_val<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Untyped<Primitive>, I> {
    parser(primitive).map(|val| Term::Val(val)).parse_state(input)
}

fn term_var<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Untyped<Primitive>, I> {
    parser(token_identifier).map(|id| Term::Var(id)).parse_state(input)
}

fn term_app<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Untyped<Primitive>, I> {
    between(
        parser(token_left_brace), parser(token_right_brace),
        (
            parser(term),
            parser(term),
            sep_by(
                parser(term),
                spaces()
            )
        )
    ).map(|(fun, first_arg, many_args)| {
        Term::app_many(fun, first_arg, many_args)
    }).parse_state(input)
}

fn term_abs<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Untyped<Primitive>, I> {
    (
        between(
            parser(token_bar), parser(token_bar),
            (
                parser(token_identifier),
                sep_by(
                    parser(token_identifier),
                    spaces()
                )
            )
        ),
        parser(term)
    ).map(|((first_id, many_ids), term)| {
        Term::abs_many_untyped(first_id, many_ids, term)
    }).parse_state(input)
}

#[allow(unconditional_recursion)]
pub fn term<I: Stream<Item=char>>(input: State<I>) -> ParseResult<Untyped<Primitive>, I> {
    between(parser(token_left_paren), parser(token_right_paren), parser(term))
    .or(parser(term_val))
    .or(parser(term_var))
    .or(parser(term_app))
    .or(parser(term_abs))
    .parse_state(input)
}

#[test]
fn test_term_val_int() {
    let result = parser(term).parse("42");
    assert_eq!(result, Result::Ok((Term::val_int(42), "")));
}

#[test]
fn test_term_var() {
    let result = parser(term).parse("x");
    let expr = Term::var("x".to_string());
    assert_eq!(result, Result::Ok((expr, "")));
}

#[test]
fn test_term_app() {
    let result = parser(term).parse("{a b}");
    let expr = Term::app(
        Term::var("a".to_string()),
        Term::var("b".to_string())
    );
    assert_eq!(result, Result::Ok((expr, "")));
}

#[test]
fn test_term_abs() {
    let result = parser(term).parse("|a| b");
    let expr = Term::abs(
        "a".to_string(), (),
        Term::var("b".to_string())
    );
    assert_eq!(result, Result::Ok((expr, "")));
}

pub fn parse_string(s: &String) -> Result<(Untyped<Primitive>, &str), ParseError<&str>> {
    parser(term).parse(s)
}
