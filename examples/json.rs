use std::{
    collections::HashMap, 
    error::Error
};

use rtor::{
    Input, 
    Parser, 
    ParseResult,
    primitive::{
        oneof,
        sat, 
        ascii::{
            digit,
            hex
        }, 
        eof, 
        anychar
    }, 
    combine::{
        token, 
        between, 
        sepby, 
        pair, 
        skip, 
        skip_many, 
        opt,
        skip_many1, 
        recognize, 
        not
    }
};

fn main() -> Result<(), Box<dyn Error>>{

    let s = r#"
        {
            "number": [1, 2, 2e2, 3e-2, 4.0],
            "value": [true, false, null, [1, 2, 3], "\"\u123afsawda"],
            "obj": {
                "v": 2,
                "x": null
            }
        }
    "#;
    
    let (json_value, _) = token(json).andl(token(eof)).parse(s.as_bytes())?;

    println!("{:#?}", json_value);
    
    Ok(())
}

#[derive(Debug)]
enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f32),
    Boolean(bool),
    Null
}

//https://www.json.org/json-en.html
fn json<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    json_object
        .or(json_array)
        .or(json_number)
        .or(key.map(JsonValue::String))
        .or("true".map(|_| JsonValue::Boolean(true)))
        .or("false".map(|_| JsonValue::Boolean(false)))
        .or("null".map(|_| JsonValue::Null))
        .parse(input)
}

fn json_object<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    between(
        '{', 
        sepby(
            pair(token(key), token(':'),  token(json)), 
            token(',')
        ), 
        token('}')
    )
    .map(|members| JsonValue::Object(HashMap::from_iter(members)))
    .parse(input)
}

fn json_array<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    between(
        '[',
        sepby(token(json), token(',')), 
        token(']')
    )
    .map(JsonValue::Array)
    .parse(input)
}

fn key<I: Input<Token = u8>>(input: I) -> ParseResult<String, I> {
    between(
        '"', 
        recognize(skip_many(character)).map(|i: I| String::from_utf8(i.tokens().collect()).unwrap()),
        '"'
    )
    .parse(input)
}

fn character<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    '\\'.andr(escape)
        .or(not('"').andr(anychar.ignore()))
        .parse(input)
}

fn escape<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    oneof("\"\\/bfnrt").ignore()
        .or('u'.andr(skip(hex, 4)))
        .parse(input)
}

fn json_number<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> { 
    recognize(integer.andr(opt('.'.andr(skip_many1(digit)))).andr(opt(exponent)))
        .map(|i: I| {
            let s = String::from_utf8(i.tokens().collect()).unwrap();
            JsonValue::Number(s.parse::<f32>().unwrap())
        })
        .parse(input)
}


fn integer<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    let int = |input: I| {
        '0'
            .or(sat(|ch| matches!(ch, b'1'..=b'9')))
            .andr(skip_many(digit))
            .parse(input)
    };

    int.or('-'.andr(int)).parse(input)
}

fn exponent<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    'E'.or('e')
        .andr(opt('+'.or('-')))
        .andr(skip_many1(digit))
        .parse(input)
}


