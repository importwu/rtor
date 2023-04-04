use std::collections::HashMap;
use std::fs;

use rtor::{
    Input, 
    Error, 
    Parser, 
    primitive::{
        token, 
        string,
        char, take_while
    }, 
    combine::{between, sepby, pair}
};

fn main() {

    // let mut input = StreamInput::new(fs::File::open("./examples/json_example.json").unwrap());

    let json = r#"
        1
    "#;
    
    let res = parse_json(json);

    println!("{:?}", res)
}

fn parse_json<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    json_true
        .or(json_false)
        .or(json_null)
        .or(json_array)
        .or(json_object)
    .parse(input)
}


fn json_array<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    between(
        token('['),
        sepby(parse_json, token(',')), 
        token(']')
    )
    .map(JsonValue::Array)
    .parse(input)
}

fn json_object<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    between(
        token('{'), 
        sepby(
            pair(token(kstring), token(':'),  parse_json), 
            token(',')
        ), 
        token('}')
    )
    .map(|kvs| JsonValue::Object(HashMap::from_iter(kvs)))
    .parse(input)
}

fn json_null<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    token(string("null"))
        .map(|_| JsonValue::Null)
        .parse(input)
}

fn json_true<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    token(string("true"))
        .map(|_| JsonValue::Boolean(true))
        .parse(input)
}

fn json_false<I: Input<Item = char>>(input: I) -> Result<(JsonValue, I), Error<I::Item>> {
    token(string("false"))
        .map(|_| JsonValue::Boolean(false))
        .parse(input)
}

// fn json_string<I: Input<Item = char>>(input: &mut I) -> ParseResult<JsonValue> { 
//     kstring
//         .map(JsonValue::String)
//         .parse(input)
// }

fn kstring<I: Input<Item = char>>(input: I) -> Result<(String, I), Error<I::Item>> {
    between(
        '"', 
        take_while(|x| *x != '"'),
        '"'
    )
    .map(|i: I| String::from_iter(i.items()))
    .parse(input)
}

// fn character<I: Input<Item = char>>(input: &mut I) -> ParseResult<String> {
//     alt!(
//         seq!(attempt(char('\\')), escape).map(|(slash, escape)| format!("{}{}", slash, escape)),
//         sat(|x| x != '"').map(String::from)
//     ).parse(input)
// }

// fn escape<I: Input<Item = char>>(input: &mut I) -> ParseResult<String> {
//     let ch = oneof("\"\\/bfnrtu").parse(input)?;
//     if ch == 'u' {
//         let unicode = seq!(hex, hex, hex, hex)
//             .map(|h| format!("{}{}{}{}{}", 'u', h.0, h.1, h.2, h.3))
//             .parse(input)?;
//         return Ok(unicode)
//     }
    
//     Ok(ch.into())
// }

// fn json_number<I: Input<Item = char>>(input: &mut I) -> ParseResult<JsonValue> { 
//     attempt(seq!(
//         integer,
//         opt_or_default(fraction),
//         opt_or_default(exponent)
//     ))
//     .map(|(integer, fraction, exponent)| {
//         let number = format!("{}{}{}", integer, fraction, exponent).parse::<f32>().unwrap();
//         JsonValue::Number(number)
//     })
//     .parse(input)
// }

// fn integer<I: Input<Item = char>>(input: &mut I) -> ParseResult<String> { 

//         let mut digits = |input: &mut I| {
//             alt!(
//                 attempt(char('0').map(String::from)), 
//                 sat(|ch| matches!(ch, '1'..='9')).and_then(|ch| many(digit).map(move|digits| format!("{}{}", ch, String::from_iter(digits))))
//             ).parse(input)
//         };

//         alt!(
//             attempt(digits),
//             seq!(char('-'), digits).map(|(minus, digits)| format!("{}{}", minus, digits))
//         ).parse(input)

// }

// fn fraction<I: Input<Item = char>>(input: &mut I) -> ParseResult<String> { 
//     seq!(
//         char('.'),
//         many1(digit)
//     )
//     .map(|(dot, digits)| format!("{}{}", dot, String::from_iter(digits)))
//     .parse(input)
// }

// fn exponent<I: Input<Item = char>>(input: &mut I) -> ParseResult<String> { 
//     seq!(
//         alt!(attempt(char('E')), char('e')),
//         opt_or_default(alt!(attempt(char('+')), char('-'))), 
//         many1(digit)
//     )
//     .map(|(e, sign, digits)| format!("{}{}{}", e, sign, String::from_iter(digits)))
//     .parse(input)
// }

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
                res.push_str(&format!("{}", format_json(value, sp + 2)));
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


