use crate::{
    Parser,
    State,
    ParseError
};

pub fn satisfy<F>(mut f: F) -> impl Parser<Output = char> where 
    F: FnMut(&char) -> bool 
{
    move |state: &mut State| {
        let pos = state.pos();
        match state.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch)})
        }
    }
}

#[inline]
pub fn char(ch: char) -> impl Parser<Output = char> {
    satisfy(move |t| *t == ch).expect(&ch.to_string())
}

#[inline]
pub fn digit() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_digit).expect("digit")
}

#[inline]
pub fn alpha() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_alphabetic).expect("alpha")
}

#[inline]
pub fn lowercase() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_lowercase).expect("lowercase")
}

#[inline]
pub fn uppercase() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_uppercase).expect("uppercase")
}

#[inline]
pub fn alphanum() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_alphanumeric).expect("alphanum")
}

#[inline]
pub fn space() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_whitespace).expect("space")
}

#[inline]
pub fn hex() -> impl Parser<Output = char> {
    satisfy(char::is_ascii_hexdigit).expect("hex")
}

pub fn string(string: &str) -> impl Parser<Output = &str> {
    move |state: &mut State| {
        for ch in string.chars() {
            char(ch).parse(state)?;
        }
        return Ok(string)
    }
}

#[inline]
pub fn oneof<'a>(string: &'a str) -> impl Parser<Output = char> + 'a {
    satisfy(|t| string.find(*t).is_some()).expect(&format!("oneof {}", string))
}

#[inline]
pub fn noneof<'a>(string: &'a str) -> impl Parser<Output = char> + 'a {
    satisfy(|t| string.find(*t).is_none()).expect(&format!("noneof {}", string))
}

#[inline]
pub fn anychar() -> impl Parser<Output = char> {
    satisfy(|_| true).expect("anychar")
}

pub fn eof<U>() -> impl Parser<Output = ()> {
    |state: &mut State| {
        let pos = state.pos();
        match state.next() {
            None => Ok(()),
            Some(t) => Err(ParseError { pos, expect: vec!["<eof>".into()], unexpect: Some(t) })
        }
    }
}

mod test {

    use crate::combine::{pure, opt_or};

    use super::*;

    #[test]
    fn test() {
        let mut state = State::new("abc");

        let mut a = string("a")
            .and_then(|x| {
                string("b").and_then(move |y| {
                    string("c").and_then(move|z| {
                        pure((x, y, z))
                    })
                })
            });

        println!("{:?}", a.parse(&mut state));
        println!("{:?}", state);
        

    }
}