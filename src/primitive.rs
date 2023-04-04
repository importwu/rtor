use crate::{
    Input, 
    Parser, 
    Error, 
    AsChar,
    FindItem
};

#[inline]
pub fn char<I>(ch: char) -> impl Parser<I, Output = char, Error = Error<I>> 
where
    I: Input<Item = char>
{
    satisfy(move |t| *t == ch)
}

pub fn string<I>(string: &str) -> impl Parser<I, Output = &str, Error = Error<I>> + '_ 
where
    I: Input<Item = char>
{
    move |mut input: I| {
        for ch in string.chars() {
            let (_, i) = char(ch).parse(input)?;
            input = i;
        }
        return Ok((string, input))
    }
}

#[inline]
pub fn item<I>(item: I::Item) -> impl Parser<I, Output = I::Item, Error = Error<I>> 
where
    I: Input,
    I::Item: PartialEq
{
    satisfy(move |t| *t == item)
}

#[inline]
pub fn items<I>(items: &[I::Item]) -> impl Parser<I, Output = &[I::Item], Error = Error<I>> 
where
    I: Input,
    I::Item: PartialEq
{
    move |mut input: I| {
        for i in items {
            let (_, i) = item(*i).parse(input)?;
            input = i;
        }
        return Ok((items, input))
    }
}

#[inline]
pub fn take_while<I, F>(mut pred: F) -> impl Parser<I, Output = I, Error = Error<I>> 
where
    I: Input,
    F: FnMut(&I::Item) -> bool
{
    move |mut input: I| {
        let o = input.take_while(&mut pred);
        Ok((o, input))
    }
}

pub fn satisfy<I, F>(mut pred: F) -> impl Parser<I, Output = I::Item, Error = Error<I>> 
where
    I: Input,
    F: FnMut(&I::Item) -> bool
{
    move |mut input: I| {
        match input.consume() {
            Some(t) if pred(&t) => Ok((t, input)),
            Some(t) => Err(Error::Unexpected(t)),
            None => Err(Error::Eoi)
        }
    }
}

#[inline]
pub fn digit<I>(input: I) -> Result<(I::Item, I), Error<I>> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_digit()).parse(input)
}

#[inline]
pub fn alpha<I>(input: I) -> Result<(I::Item, I), Error<I>> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_alphabetic()).parse(input)
}

#[inline]
pub fn lowercase<I>(input: I) -> Result<(I::Item, I), Error<I>>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_lowercase()).parse(input)
}

#[inline]
pub fn uppercase<I>(input: I) -> Result<(I::Item, I), Error<I>>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_uppercase()).parse(input)
}

#[inline]
pub fn alphanum<I>(input: I) -> Result<(I::Item, I), Error<I>>   
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_alphanumeric()).parse(input)
}

#[inline]
pub fn space<I>(input: I) -> Result<(I::Item, I), Error<I>>  
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_whitespace()).parse(input)
}

#[inline]
pub fn hex<I>(input: I) -> Result<(I::Item, I), Error<I>> 
where
    I: Input,
    I::Item: AsChar
{
    satisfy(|c: &I::Item| c.as_char().is_ascii_hexdigit()).parse(input)
}

#[inline]
pub fn anyitem<I>(input: I) -> Result<(I::Item, I), Error<I>> 
where
    I: Input
{
    satisfy(|_| true).parse(input)
}

#[inline]
pub fn oneof<I, F>(items: F) -> impl Parser<I, Output = I::Item, Error = Error<I>> 
where
    I: Input,
    F: FindItem<I::Item>
{
    satisfy(move|t| items.find_item(*t))
}

#[inline]
pub fn noneof<I, F>(items: F) -> impl Parser<I, Output = I::Item, Error = Error<I>> 
where
    I: Input,
    F: FindItem<I::Item>
{
    satisfy(move|t| !items.find_item(*t))
}


#[inline]
pub fn eof<I>() -> impl Parser<I, Output = (), Error = Error<I>> 
where
    I: Input
{
    |mut input: I| {
        match input.consume() {
            None => Ok(((), input)),
            Some(t) => Err(Error::Unexpected(t))
        }
    }
}

#[inline]
pub fn token<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = P::Error> 
where
    I: Input,
    I::Item: AsChar,
    P: Parser<I>
{
    move |mut input: I| {
        input.take_while(|t| t.as_char().is_ascii_whitespace());
        parser.parse(input)
    }
}


mod test {

    use super::*;

    #[test]
    fn test() {

        let p = item(b'a').parse(&b"aaaabc"[..]);

        println!("{:?}", p);

    }
}