use std::collections::HashMap;

use rtor::{
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
        anychar,
        char,
        string
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

fn main() {
    let s = r#"
    {
        "ulozec": [
            [
                [-780982275, true],
                -1071421524
            ],
            {
                "hyghokilpgf": -694363646.9191926,
                "ialcqh": 1377465553.898079,
                "oozscfql": "spWyychnqYA5R"
            },
            [1959856920, true, false],
            true
        ],
        "yvfuqw": "V",
        "dswcppi": "4I2xOR_q",
        "ytgqmlpld": "Y7wd"
    }
    "#;

    let json_value = token(json).andl(token(eof)).parse(s);

    println!("{:#?}", json_value);
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
fn json(input: &str) -> ParseResult<JsonValue, &str> {
    json_object
        .or(json_array)
        .or(json_number)
        .or(key.map(JsonValue::String))
        .or(string("true").map(|_| JsonValue::Boolean(true)))
        .or(string("false").map(|_| JsonValue::Boolean(false)))
        .or(string("null").map(|_| JsonValue::Null))
        .parse(input)
}

fn json_object(input: &str) -> ParseResult<JsonValue, &str> {
    between(
        char('{'), 
        sepby(pair(token(key), token(char(':')), token(json)), token(char(','))), 
        token(char('}'))
    )
    .map(|members| JsonValue::Object(HashMap::from_iter(members)))
    .parse(input)
}

fn json_array(input: &str) -> ParseResult<JsonValue, &str> {
    between(
        char('['),
        sepby(token(json), token(char(','))), 
        token(char(']'))
    )
    .map(JsonValue::Array)
    .parse(input)
}

fn key(input: &str) -> ParseResult<String, &str> {
    let escape =  oneof("\"\\/bfnrt").or(char('u').andl(skip(hex, 4)));
    let character = char('\\').andr(escape).or(not(char('"')).andr(anychar));
    between(
        char('"'), 
        recognize(skip_many(character)).map(|i: &str| i.to_owned()),
        char('"')
    )
    .parse(input)
}

fn json_number(input: &str) -> ParseResult<JsonValue, &str> { 
    let exponent = char('E').or(char('e')).andr(opt(char('+').or(char('-')))).andr(skip_many1(digit));
    let fraction = char('.').andr(skip_many1(digit));
    let integer = char('0').or(sat(|ch| matches!(ch, '1'..='9')).andl(skip_many(digit)));
    recognize(opt(char('-')).andr(integer).andr(opt(fraction)).andr(opt(exponent)))
        .map(|i: &str| JsonValue::Number(i.parse::<f32>().unwrap()))
        .parse(input)
}