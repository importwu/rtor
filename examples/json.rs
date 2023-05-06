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
        option, 
        skip_many1, 
        followed_by, 
        recognize
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
    
    let (json_value, _) = followed_by(json, token(eof)).parse(s.as_bytes())?;

    println!("{:?}", json_value);

    Ok(())
}

//https://www.json.org/json-en.html
fn json<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    json_object
        .or(json_array)
        .or(json_string)
        .or(json_number)
        .or(json_true)
        .or(json_false)
        .or(json_null)
        .parse(input)
}

fn json_object<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
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

fn json_array<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    between(
        token('['),
        sep_by(json, token(',')), 
        token(']')
    )
    .map(JsonValue::Array)
    .parse(input)
}

fn json_null<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token("null")
        .map(|_| JsonValue::Null)
        .parse(input)
}

fn json_true<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token("true")
        .map(|_| JsonValue::Boolean(true))
        .parse(input)
}

fn json_false<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> {
    token("false")
        .map(|_| JsonValue::Boolean(false))
        .parse(input)
}

fn json_string<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> { 
    token(key)
        .map(JsonValue::String)
        .parse(input)
}

fn key<I: Input<Token = u8>>(input: I) -> ParseResult<String, I> {
    between(
        '"', 
        |input: I| {
            let (o, i) = recognize(skip_many(character)).parse(input)?;
            let s = String::from_utf8(o.tokens().collect()).unwrap();
            Ok((s, i))
        },
        '"'
    )
    .parse(input)

}

fn character<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    ('\\'.andr(escape))
        .or(satisfy(|x| *x != b'"').map(|_|()))
        .parse(input)
}

fn escape<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    let (o, i) = oneof("\"\\/bfnrtu").parse(input)?;
    
    if o == b'u' {
        let (_, i) = count(hex, 4).parse(i)?;
        return Ok(((), i))
    }

    Ok(((), i))
}

fn json_number<I: Input<Token = u8>>(input: I) -> ParseResult<JsonValue, I> { 
    let (_, i) = skip_many(space).parse(input)?;
    let (o, i) = recognize(integer.andr(option(fraction)).andr(option(exponent))).parse(i)?;
    let s = String::from_utf8(o.tokens().collect()).unwrap();
    Ok((JsonValue::Number(s.parse::<f32>().unwrap()), i))
}


fn integer<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    let int = |input: I| {
        '0'
            .or(onenine)
            .andr(skip_many(digit))
            .parse(input)
    };

    int.or('-'.andr(int)).parse(input)
}

fn onenine<I: Input<Token = u8>>(input: I) -> ParseResult<u8, I> {
    satisfy(|ch| matches!(ch, b'1'..=b'9')).parse(input)
}

fn fraction<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    '.'.andr(skip_many1(digit)).parse(input)
}

fn exponent<I: Input<Token = u8>>(input: I) -> ParseResult<(), I> {
    ('E'.or('e'))
        .andr(option(sign))
        .andr(skip_many1(digit))
        .parse(input)
}

fn sign<I: Input<Token = u8>>(input: I) -> ParseResult<u8, I> {
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
