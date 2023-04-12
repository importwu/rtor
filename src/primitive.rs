use crate::{
    Input, 
    Parser, 
    Error, 
    AsChar,
    FindToken, 
    ParseResult
};

#[inline]
pub fn char<I>(ch: char) -> impl Parser<I, Output = I::Token, Error = Error<I::Token>> 
where
    I: Input,
    I::Token: AsChar
{
    satisfy(move |t: &I::Token| t.as_char() == ch)
}

pub fn string<I>(string: &str) -> impl Parser<I, Output = I, Error = Error<I::Token>> + '_ 
where
    I: Input,
    I::Token: AsChar
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
pub fn take_while<I, F>(mut pred: F) -> impl Parser<I, Output = I, Error = Error<I::Token>> 
where
    I: Input,
    F: FnMut(&I::Token) -> bool
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

pub fn satisfy<I, F>(mut pred: F) -> impl Parser<I, Output = I::Token, Error = Error<I::Token>> 
where
    I: Input,
    F: FnMut(&I::Token) -> bool
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
pub fn digit<I>(input: I) -> ParseResult<I::Token, I> 
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_digit()).parse(input)
}

#[inline]
pub fn alpha<I>(input: I) -> ParseResult<I::Token, I>
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_alphabetic()).parse(input)
}

#[inline]
pub fn lowercase<I>(input: I) -> ParseResult<I::Token, I>  
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_lowercase()).parse(input)
}

#[inline]
pub fn uppercase<I>(input: I) -> ParseResult<I::Token, I>  
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_uppercase()).parse(input)
}

#[inline]
pub fn alphanum<I>(input: I) -> ParseResult<I::Token, I>   
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_alphanumeric()).parse(input)
}

#[inline]
pub fn space<I>(input: I) -> ParseResult<I::Token, I>  
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_whitespace()).parse(input)
}

#[inline]
pub fn hex<I>(input: I) -> ParseResult<I::Token, I> 
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|c: &I::Token| c.as_char().is_ascii_hexdigit()).parse(input)
}

#[inline]
pub fn anychar<I>(input: I) -> ParseResult<I::Token, I> 
where
    I: Input,
    I::Token: AsChar
{
    satisfy(|_| true).parse(input)
}

#[inline]
pub fn oneof<I, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = Error<I::Token>> 
where
    I: Input,
    F: FindToken<I::Token>
{
    satisfy(move|t: &I::Token| tokens.find_token(*t))
}

#[inline]
pub fn noneof<I, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = Error<I::Token>> 
where
    I: Input,
    F: FindToken<I::Token>
{
    satisfy(move|t| !tokens.find_token(*t))
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
pub fn token<I, P>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = Error<I::Token>> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = Error<I::Token>>
{
    move |input: I| {
        let (_, i) = take_while(|t: &I::Token| t.as_char().is_ascii_whitespace()).parse(input)?;
        parser.parse(i)
    }
}

impl<I> Parser<I> for char 
where 
    I: Input,
    I::Token: AsChar
{
    type Output = I::Token;
    type Error = Error<I::Token>;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        satisfy(|t: &I::Token| t.as_char() == *self).parse(input)
    }
}

