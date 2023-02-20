use super::traits::{Input, Parser};

mod statically;
mod streaming;
mod error;
mod position;

pub use self::{
    statically::StaticInput,
    streaming::StreamInput,
    error::ParseError
};

pub type ParseResult<T> = Result<T, ParseError>;

pub fn char<I>(ch: char) -> impl Parser<I, Output = char, Error = ParseError>
    where I: Input<Item = char>
{
    move |input: &mut I| {
        let nch = input.next().ok_or(ParseError::Eof)?;
        if ch == nch {
            Ok(ch)
        }else {
            Err(ParseError::UnexpectChar(nch))
        }
    }
}


pub fn string<I>(s: &str) -> impl Parser<I, Output = String, Error = ParseError> + '_
    where I: Input<Item = char>
{
    move |input: &mut I| {
        for ach in s.chars() {
            let bch = input.next().ok_or(ParseError::Eof)?;
            if ach != bch {
                return Err(ParseError::UnexpectChar(bch))
            }
        }
        Ok(s.to_owned())
    }
}


pub fn sat<I, F>(f: F) -> impl Parser<I, Output = char, Error = ParseError>
    where I: Input<Item = char>,
        F: Fn(char) -> bool
{
    move |input: &mut I| {
        let ch = input.next().ok_or(ParseError::Eof)?;
        if (f)(ch) {
            return Ok(ch);
        }
        Err(ParseError::UnexpectChar(ch))
    }
}

pub fn oneof<I>(s: &str) -> impl Parser<I, Output = char, Error = ParseError> + '_
    where I: Input<Item = char>
{
    move |input: &mut I| {
        let bch = input.next().ok_or(ParseError::Eof)?;
        for ach in s.chars() {
            if ach == bch {
                return Ok(ach)
            }
            continue
        }
        Err(ParseError::UnexpectChar(bch))
    }
}

#[inline]
pub fn digit<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, '0'..='9')).parse(input)
}

#[inline]
pub fn alpha<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, 'a'..='z' |'A'..='Z')).parse(input)
}

#[inline]
pub fn hex<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, '0'..='9' |'A'..='F' | 'a'..='f')).parse(input)
}

#[inline]
pub fn whitespace<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, ' '| '\r' |'\n'| '\t')).parse(input)
}

#[inline]
pub fn lowercase<S: Input<Item = char>>(input: &mut S) ->  ParseResult<char>{
    sat(|ch| matches!(ch, 'a'..='z')).parse(input)
}

#[inline]
pub fn uppercase<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, 'A'..='Z')).parse(input)
}

#[inline]
pub fn alphanum<S: Input<Item = char>>(input: &mut S) -> ParseResult<char>{
    sat(|ch| matches!(ch, 'a'..='z' |'A'..='Z' | '0'..='9')).parse(input)
}

pub fn eof<S: Input<Item = char>>(input: &mut S) -> ParseResult<()> {
    input.next().map_or(Ok(()), |ch| Err(ParseError::UnexpectChar(ch)))
}

pub fn token<I, P>(mut parser: P) -> impl FnMut(&mut I) -> Result<P::Output, P::Error>  
    where I: Input<Item = char>,
        P: Parser<I>  
{
    move |input: &mut I| {
        loop {
            let mut cursor = input.cursor();
            match input.next() {
                Some(ch) => {
                    if ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t' {
                        continue;
                    }
                    cursor.restore();
                    break
                },
                None => break
            }
        }

        parser.parse(input)
    }
}

pub fn anychar<I>() -> impl Parser<I, Output = char, Error = ParseError>
    where I: Input<Item = char>
{
    move|input: &mut I| {
        let ch = input.next().ok_or(ParseError::Eof)?;
        Ok(ch)
    }
}


#[test]
fn test() {

}