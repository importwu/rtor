pub fn satisfy<F, I>(mut f: F) -> impl Parser<I, Output = char> where
    I: Input<Item = char>,
    F: FnMut(&char) -> bool 
{
    move |input: &mut I| {
        let pos = input.pos();
        match input.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch)})
        }
    }
}

#[inline]
pub fn char<I>(ch: char) -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(move |t| *t == ch).expect(&ch.to_string())
}

#[inline]
pub fn digit<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_digit).expect("digit")
}

#[inline]
pub fn alpha<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_alphabetic).expect("alpha")
}

#[inline]
pub fn lowercase<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_lowercase).expect("lowercase")
}

#[inline]
pub fn uppercase<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_uppercase).expect("uppercase")
}

#[inline]
pub fn alphanum<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_alphanumeric).expect("alphanum")
}

#[inline]
pub fn space<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_whitespace).expect("space")
}

#[inline]
pub fn hex<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(char::is_ascii_hexdigit).expect("hex")
}

pub fn string<I>(string: &str) -> impl Parser<I, Output = &str> + '_ where
    I: Input<Item = char>
{
    move |input: &mut I| {
        for ch in string.chars() {
            char(ch).parse(input)?;
        }
        return Ok(string)
    }
}

#[inline]
pub fn oneof<'a, I: 'a>(string: &'a str) -> impl Parser<I, Output = char> + 'a where
    I: Input<Item = char>
{
    satisfy(|t| string.find(*t).is_some()).expect(&format!("oneof {}", string))
}

#[inline]
pub fn noneof<'a, I: 'a>(string: &'a str) -> impl Parser<I, Output = char> + 'a where
    I: Input<Item = char>
{
    satisfy(|t| string.find(*t).is_none()).expect(&format!("noneof {}", string))
}

#[inline]
pub fn anychar<I>() -> impl Parser<I, Output = char> where
    I: Input<Item = char>
{
    satisfy(|_| true).expect("anychar")
}

pub fn eof<I>() -> impl Parser<I, Output = ()> where
    I: Input<Item = char>
{
    |input: &mut I| {
        let pos = input.pos();
        match input.next() {
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