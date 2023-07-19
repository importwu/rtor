use std::collections::HashMap;

use rtor::{
    Parser, 
    Input,
    ParseResult,
    ParseError,
    AsChar,
    char::{
        one_of,
        ascii, 
        anychar,
        char,
        string,
    }, 
    combinator::{
        between, 
        skip_many, 
        skip, 
        recognize, 
        terminated, 
        value,
        pair, 
        sep_by,
        not,
        alt,
        eof,
        opt,
    },
};

fn main() {
    let s = r#"
    {
        "color": [ "red", "green", "blue" ],
        "number": [ 12e2, 34.5, 45.2e+3 ],
        "flag": false
    }
    "#;

    let result = parse_json(s);
    println!("{:#?}", result);
}

#[derive(Debug, Clone)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null
}

fn parse_json(input: &str) -> ParseResult<JsonValue, &str> {
    between(
        ascii::multi_space, 
        json_value,
        eof
    )(input)
}

//https://www.json.org/json-en.html
fn json_value(input: &str) -> ParseResult<JsonValue, &str> {
    alt((
        between(
            lexeme(char('{')), 
            sep_by(pair(key, lexeme(char(':')), json_value), lexeme(char(','))),
            lexeme(char('}'))
        ).map(|members| JsonValue::Object(HashMap::from_iter(members))),
        between(
            lexeme(char('[')),
            sep_by(json_value, lexeme(char(','))),
            lexeme(char(']'))
        ).map(JsonValue::Array),
        number.map(JsonValue::Number),
        key.map(JsonValue::String),
        value(JsonValue::Boolean(true), lexeme(string("true"))),
        value(JsonValue::Boolean(false), lexeme(string("false"))),
        value(JsonValue::Null, lexeme(string("null"))),
    ))(input)
}

fn key(input: &str) -> ParseResult<String, &str> {
    let escape = alt((one_of("\"\\/bfnrt"), char('u').andl(skip(ascii::hex, 4))));
    let character = alt((char('\\').andl(escape), not(char('"')).andr(anychar)));
    between(
        char('"'), 
        recognize(skip_many(character)).map(|i: &str| i.to_owned()),
        lexeme(char('"'))
    )(input)
}

pub fn number(input: &str) -> ParseResult<f64, &str> {
    let exponent = (alt((char('e'), char('E'))), opt(alt((char('+'), char('-')))), ascii::multi_digit1);
    let fraction = (char('.'), ascii::multi_digit1);
    lexeme(recognize((opt(char('-')), ascii::multi_digit1, opt(fraction), opt(exponent)))
        .map(|i: &str| i.parse().unwrap()))(input)
}

pub fn lexeme<P, I, E>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    let mut parser = terminated(parser, ascii::multi_space);
    move |input| parser(input)
}