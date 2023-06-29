use std::collections::HashMap;

use rtor::{
    Parser, 
    ParseResult,
    token::{
        symbol,
        braces,
        brackets,
        comma_sep,
        float
    },
    character::{
        oneof,
        ascii::hex, 
        eof, 
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

    let json_value = symbol(json_value).andl(symbol(eof)).parse(s);

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
    braces(comma_sep(pair(symbol(key), symbol(char(':')), symbol(json_value))))
        .map(|members| JsonValue::Object(HashMap::from_iter(members)))
        .or(brackets(comma_sep(symbol(json_value))).map(JsonValue::Array))
        .or(float.map(|i: &str| JsonValue::Number(i.parse::<f64>().unwrap())))
        .or(key.map(JsonValue::String))
        .or(string("true").map(|_| JsonValue::Boolean(true)))
        .or(string("false").map(|_| JsonValue::Boolean(false)))
        .or(string("null").map(|_| JsonValue::Null))
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