use crate::{
    Input, 
    Parser, 
    Error, 
    AsChar,
    FindItem, 
    ParseResult
};

#[inline]
pub fn char<I>(ch: char) -> impl Parser<I, Output = I::Item, Error = Error<I::Item>> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(move |t: &I::Item| t.as_char() == ch)
}

pub fn string<I>(string: &str) -> impl Parser<I, Output = I, Error = Error<I::Item>> + '_ 
where
    I: Input,
    I::Item: AsChar
{
    move |mut input: I| {
        let src = input.clone();
        for ch in string.chars() {
            let (_, i) = char(ch).parse(input)?;
            input = i;
        }
        return Ok((src.diff(&input), input))
    }
}

#[inline]
pub fn take_while<I, F>(mut pred: F) -> impl Parser<I, Output = I, Error = Error<I::Item>> 
where
    I: Input,
    F: FnMut(&I::Item) -> bool
{
    move |mut input: I| {
        let src = input.clone();

        loop {
            match input.peek() {
                None => break,
                Some(t) if pred(&t) => {
                    input.next();
                    continue
                },
                Some(_) => break
            }
        }

        Ok((src.diff(&input), input))
    }
}

pub fn satisfy<I, F>(mut pred: F) -> impl Parser<I, Output = I::Item, Error = Error<I::Item>> 
where
    I: Input,
    F: FnMut(&I::Item) -> bool
{
    move |mut input: I| {
        match input.next() {
            Some(t) if pred(&t) => Ok((t, input)),
            Some(t) => Err(Error::Unexpected(t)),
            None => Err(Error::Eoi)
        }
    }
}

#[inline]
pub fn digit<I>(input: I) -> ParseResult<I::Item, I> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_digit()).parse(input)
}

#[inline]
pub fn alpha<I>(input: I) -> ParseResult<I::Item, I>
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_alphabetic()).parse(input)
}

#[inline]
pub fn lowercase<I>(input: I) -> ParseResult<I::Item, I>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_lowercase()).parse(input)
}

#[inline]
pub fn uppercase<I>(input: I) -> ParseResult<I::Item, I>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_uppercase()).parse(input)
}

#[inline]
pub fn alphanum<I>(input: I) -> ParseResult<I::Item, I>   
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_alphanumeric()).parse(input)
}

#[inline]
pub fn space<I>(input: I) -> ParseResult<I::Item, I>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_whitespace()).parse(input)
}

#[inline]
pub fn hex<I>(input: I) -> ParseResult<I::Item, I> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_hexdigit()).parse(input)
}

#[inline]
pub fn anychar<I>(input: I) -> ParseResult<I::Item, I> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|_| true).parse(input)
}

#[inline]
pub fn oneof<I, F>(items: F) -> impl Parser<I, Output = I::Item, Error = Error<I::Item>> 
where
    I: Input,
    F: FindItem<I::Item>
{
    satisfy(move|t: &I::Item| items.find_item(*t))
}

#[inline]
pub fn noneof<I, F>(items: F) -> impl Parser<I, Output = I::Item, Error = Error<I::Item>> 
where
    I: Input,
    F: FindItem<I::Item>
{
    satisfy(move|t| !items.find_item(*t))
}


#[inline]
pub fn eof<I>(mut input: I) -> ParseResult<(), I> 
where
    I: Input
{
    match input.next() {
        None => Ok(((), input)),
        Some(t) => Err(Error::Unexpected(t))
    }
}

#[inline]
pub fn token<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = Error<I::Item>> 
where
    I: Input,
    I::Item: AsChar,
    P: Parser<I, Error = Error<I::Item>>
{
    move |input: I| {
        let (_, i) = take_while(|t: &I::Item| t.as_char().is_ascii_whitespace()).parse(input)?;
        parser.parse(i)
    }
}

impl<I> Parser<I> for char 
where 
    I: Input,
    I::Item: AsChar
{
    type Output = I::Item;
    type Error = Error<I::Item>;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        satisfy(|t: &I::Item| t.as_char() == *self).parse(input)
    }
}


mod test {

    use super::*;

    #[test]
    fn test() {


    }
}