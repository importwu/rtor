use std::collections::HashMap;

use rtor::{
    Parser, 
    ParseResult,
    char::{
        one_of,
        ascii, 
        unicode,
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
        preceded,
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
        "color": ["red", "green", "blue"],
        "number": [12e2, 34.5, 45.2e+3],
        "flag": false
    }
    "#;

    let result = terminated(
        json_value, 
        preceded(unicode::multi_space, eof)
    )(s);
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

//https://www.json.org/json-en.html
fn json_value(input: &str) -> ParseResult<JsonValue, &str> {
    preceded(
        unicode::multi_space, 
        alt((
            between(
                char('{'), 
                sep_by(
                    pair(preceded(unicode::multi_space, key), preceded(unicode::multi_space, char(':')), json_value), 
                    preceded(unicode::multi_space, char(','))
                ),
                preceded(unicode::multi_space, char('}'))
            ).map(|members| JsonValue::Object(HashMap::from_iter(members))),
            between(
                char('['),
                sep_by(json_value, preceded(unicode::multi_space, char(','))),
                char(']')
            ).map(JsonValue::Array),
            number.map(JsonValue::Number),
            key.map(JsonValue::String),
            value(JsonValue::Boolean(true), string("true")),
            value(JsonValue::Boolean(false), string("false")),
            value(JsonValue::Null, string("null")),
        ))
    )(input)
}

fn key(input: &str) -> ParseResult<String, &str> {
    let escape = alt((one_of("\"\\/bfnrt"), char('u').andl(skip(ascii::hex, 4))));
    let character = alt((char('\\').andl(escape), not(char('"')).andr(anychar)));
    between(
        char('"'), 
        recognize(skip_many(character)).map(|i: &str| i.to_owned()),
        char('"')
    )(input)
}

pub fn number(input: &str) -> ParseResult<f64, &str> {
    let exponent = (alt((char('e'), char('E'))), opt(alt((char('+'), char('-')))), ascii::multi_digit1);
    let fraction = (char('.'), ascii::multi_digit1);
    recognize((opt(char('-')), ascii::multi_digit1, opt(fraction), opt(exponent)))
        .map(|i: &str| i.parse().unwrap())
        .parse(input)
}
