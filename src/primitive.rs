use crate::{
    Parser,
    State,
    ParseError
};

pub fn satisfy<F, U>(mut f: F) -> impl Parser<U, Output = char> where 
    F: FnMut(&char) -> bool 
{
    move |state: &mut State<U>| {
        let pos = state.pos();
        match state.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch.into())})
        }
    }
}

#[inline]
pub fn char<U>(ch: char) -> impl Parser<U, Output = char> {
    satisfy(move |t| *t == ch).expect(&ch.to_string())
}

#[inline]
pub fn digit<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_digit).expect("digit")
}

#[inline]
pub fn alpha<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_alphabetic).expect("alpha")
}

#[inline]
pub fn lowercase<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_lowercase).expect("lowercase")
}

#[inline]
pub fn uppercase<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_uppercase).expect("uppercase")
}

#[inline]
pub fn alphanum<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_alphanumeric).expect("alphanum")
}

#[inline]
pub fn space<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_whitespace).expect("space")
}

#[inline]
pub fn hex<U>() -> impl Parser<U, Output = char> {
    satisfy(char::is_ascii_hexdigit).expect("hex")
}

pub fn string<U>(string: &str) -> impl Parser<U, Output = String> + '_{
    move |state: &mut State<U>| {
        for ch in string.chars() {
            let pos = state.pos();
            match state.next() {
                None => return Err(ParseError{pos, expect: vec![ch.into()], unexpect: None}),
                Some(t) if t == ch => continue,
                Some(t) => return Err(ParseError{pos, expect: vec![ch.into()], unexpect: Some(t.into())})
            }
        }
        return Ok(string.to_owned())
    }
}

#[inline]
pub fn oneof<'a, U: 'a>(string: &'a str) -> impl Parser<U, Output = char> + 'a {
    satisfy(|t| string.find(*t).is_some()).expect(&format!("oneof {}", string))
}

#[inline]
pub fn noneof<'a, U: 'a>(string: &'a str) -> impl Parser<U, Output = char> + 'a {
    satisfy(|t| string.find(*t).is_none()).expect(&format!("noneof {}", string))
}

#[inline]
pub fn anychar<U>() -> impl Parser<U, Output = char> {
    satisfy(|_| true).expect("anychar")
}

pub fn eof<U>() -> impl Parser<U, Output = ()> {
    |state: &mut State<U>| {
        let pos = state.pos();
        match state.next() {
            None => Ok(()),
            Some(t) => Err(ParseError { pos, expect: vec!["<eof>".into()], unexpect: Some(t.into()) })
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