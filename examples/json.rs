use std::{
    collections::HashMap, 
    error::Error
};

use rtor::{
    Input, 
    Parser, 
    ParseResult,
    primitive::{
        token, 
        string,
        oneof,
        hex, 
        satisfy, 
        digit, 
        space, 
        eof
    }, 
    combine::{
        between, 
        sep_by, 
        pair, 
        count, 
        skip_many, 
        opt, 
        skip_many1, 
        followed_by
    }
};

fn main() -> Result<(), Box<dyn Error>>{

    let s = r#"
        {
            "number": [1, 2, 2e2, 3e-2, 4.0],
            "value": [true, false, null, [1, 2, 3]],
            "obj": {
                "v": 2,
                "x": null
            }
        }
    "#;

    let (json_value, _) = followed_by(json, token(eof)).parse(s.as_bytes())?;

    println!("{:?}", json_value);

    Ok(())
}

//https://www.json.org/json-en.html
fn json<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<JsonValue, I> {
    json_object
        .or(json_array)
        .or(json_string)
        .or(json_number)
        .or(json_true)
        .or(json_false)
        .or(json_null)
    .parse(input)
}

fn json_object<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<JsonValue, I> {
    between(
        token('{'), 
        sep_by(
            pair(token(key), token(':'),  json), 
            token(',')
        ), 
        token('}')
    )
    .map(|members| JsonValue::Object(HashMap::from_iter(members)))
    .parse(input)
}

fn json_array<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<JsonValue, I> {
    between(
        token('['),
        sep_by(json, token(',')), 
        token(']')
    )
    .map(JsonValue::Array)
    .parse(input)
}

fn json_null<I: Input<Item = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token(string("null"))
        .map(|_| JsonValue::Null)
        .parse(input)
}

fn json_true<I: Input<Item = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token(string("true"))
        .map(|_| JsonValue::Boolean(true))
        .parse(input)
}

fn json_false<I: Input<Item = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token(string("false"))
        .map(|_| JsonValue::Boolean(false))
        .parse(input)
}

fn json_string<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<JsonValue, I> { 
    token(key)
        .map(JsonValue::String)
        .parse(input)
}

fn key<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<String, I> {
    between(
        '"', 
        |input: I| {
            let src = input.clone();
            let (_, i) = skip_many(character).parse(input)?;
            let s = String::from_utf8(src.diff(&i).as_inner().to_vec()).unwrap();
            Ok((s, i))
        },
        '"'
    )
    .parse(input)

}

fn character<I: Input<Item = u8>>(input: I) -> ParseResult<(), I> {
    ('\\'.and(escape))
        .or(satisfy(|x| *x != b'"').map(|_|()))
        .parse(input)
}

fn escape<I: Input<Item = u8>>(input: I) -> ParseResult<(), I> {
    let (o, i) = oneof("\"\\/bfnrtu").parse(input)?;
    
    if o == b'u' {
        let (_, i) = count(hex, 4).parse(i)?;
        return Ok(((), i))
    }

    Ok(((), i))
}

fn json_number<'a, I: Input<Item = u8, Inner = &'a [u8]>>(input: I) -> ParseResult<JsonValue, I> { 
    let (_, i) = skip_many(space).parse(input)?;
    let src = i.clone();
    let (_, i) = integer.and(opt(fraction)).and(opt(exponent)).parse(i)?;
    let s = String::from_utf8(src.diff(&i).as_inner().to_vec()).unwrap();
    Ok((JsonValue::Number(s.parse::<f32>().unwrap()), i))
}


fn integer<I: Input<Item = u8>>(input: I) -> ParseResult<(), I> {
    let int = |input: I| {
        '0'
            .or(onenine)
            .and(skip_many(digit))
            .parse(input)
    };

    int.or('-'.and(int)).parse(input)
}

fn onenine<I: Input<Item = u8>>(input: I) -> ParseResult<u8, I> {
    satisfy(|ch| matches!(ch, b'1'..=b'9')).parse(input)
}

fn fraction<I: Input<Item = u8>>(input: I) -> ParseResult<(), I> {
    '.'.and(skip_many1(digit)).parse(input)
}

fn exponent<I: Input<Item = u8>>(input: I) -> ParseResult<(), I> {
    ('E'.or('e'))
        .and(opt(sign))
        .and(skip_many1(digit))
        .parse(input)
}

fn sign<I: Input<Item = u8>>(input: I) -> ParseResult<u8, I> {
    '+'.or('-').parse(input)
}

enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f32),
    Boolean(bool),
    Null
}

impl std::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = format_json(self, 0);
        f.write_str(&json)
    }
}

fn format_json(value: &JsonValue, sp: usize) -> String {
    let space = " ".repeat(sp);
    match value {
        JsonValue::Object(pairs) => {
            let mut res = String::new();
            res.push_str(&format!("{}{}", space, '{'));
            res.push('\n');
            for (key, value) in pairs.into_iter() {
                res.push_str(&format!("  {}\"{}\"", space, key));
                res.push(':');
                let value = format_json(value, sp + 2);
                let value = value.chars().skip(sp + 2).collect::<String>();
                res.push_str(&value);
                res.push(',');
                res.push('\n');
            }
            res.pop(); res.pop();
            res.push('\n');
            res.push_str(&format!("{}{}", space, '}'));
            res
        },
        JsonValue::Array(values) => {
            let mut res = String::new();
            let space = " ".repeat(sp);
            res.push_str(&format!("{}{}", space, '['));
            res.push('\n');
            for value in values {
                res.push_str(&format_json(value, sp + 2));
                res.push(',');
                res.push('\n');
            }
            res.pop(); res.pop();
            res.push('\n');
            res.push_str(&format!("{}{}", space, ']'));
            res
        },
        JsonValue::String(str) => format!("{}\"{}\"", space, str),
        JsonValue::Number(num) => format!("{}{}", space, num),
        JsonValue::Boolean(bool) => format!("{}{}", space, bool),
        JsonValue::Null => format!("{}{}", space, "null"),
            
    }
}


