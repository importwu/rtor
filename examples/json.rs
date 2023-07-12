use std::collections::HashMap;

use rtor::{
    Parser, 
    ParseResult,
    token::{
        token,
        braces,
        brackets,
        comma_sep,
        number
    },
    char::{
        one_of,
        ascii::hex, 
        anychar,
        char,
        string
    }, 
    combinator::{
        between, 
        pair, 
        skip, 
        skip_many, 
        recognize, 
        not,
        alt,
        eof,
        terminated,
    }, 
};

fn main() {
    let s = r#"
    {
        "color": ["red", "green", "blue"],
        "number": [12e2, 34.5, 45.2e+3],
        "flag": false
    }
    "#;

    let result = terminated(json_value, token(eof)).parse(s);
    println!("{:#?}", result);
}

#[derive(Debug)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null
}

//https://www.json.org/json-en.html
fn json_value(input: &str) -> ParseResult<JsonValue, &str> {
    token(alt((
        braces(comma_sep(pair(key, token(char(':')), json_value))).map(|members| JsonValue::Object(HashMap::from_iter(members))),
        brackets(comma_sep(json_value)).map(JsonValue::Array),
        number.map(|i: &str| JsonValue::Number(i.parse::<f64>().unwrap())),
        key.map(JsonValue::String),
        string("true").map(|_| JsonValue::Boolean(true)),
        string("false").map(|_| JsonValue::Boolean(false)),
        string("null").map(|_| JsonValue::Null)
    )))(input)
}

fn key(input: &str) -> ParseResult<String, &str> {
    let escape = alt((one_of("\"\\/bfnrt"), char('u').andl(skip(hex, 4))));
    let character = alt((char('\\').andl(escape), not(char('"')).andr(anychar)));
    token(between(
        char('"'), 
        recognize(skip_many(character)).map(|i: &str| i.to_owned()),
        char('"')
    ))(input)
}

