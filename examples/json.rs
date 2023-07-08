use std::collections::HashMap;

use rtor::{
    Parser, 
    ParseResult,
    token::{
        symbol,
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
        terminated
    }, 
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
            [1959856920, true, false, "\u1234"],
            true
        ],
        "yvfuqw": "V",
        "dswcppi": "4I2xOR_q",
        "ytgqmlpld": "Y7wd"
    }
    "#;

    let json_value = terminated(symbol(json_value), symbol(eof)).parse(s);
    println!("{:#?}", json_value);
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
    alt((
        braces(comma_sep(pair(symbol(key), symbol(char(':')), symbol(json_value)))).map(|members| JsonValue::Object(HashMap::from_iter(members))),
        brackets(comma_sep(symbol(json_value))).map(JsonValue::Array),
        number.map(|i: &str| JsonValue::Number(i.parse::<f64>().unwrap())),
        key.map(JsonValue::String),
        string("true").map(|_| JsonValue::Boolean(true)),
        string("false").map(|_| JsonValue::Boolean(false)),
        string("null").map(|_| JsonValue::Null)
    ))(input)
}

fn key(input: &str) -> ParseResult<String, &str> {
    let escape = alt((one_of("\"\\/bfnrt"), char('u').andl(skip(hex, 4))));
    let character = alt((char('\\').andl(escape), not(char('"')).andr(anychar)));
    between(
        char('"'), 
        recognize(skip_many(character)).map(|i: &str| i.to_owned()),
        char('"')
    )(input)
}

