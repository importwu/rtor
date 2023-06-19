use crate::{
    Input, 
    Parser, 
    ParseError, 
    AsChar,
    FindToken, 
    ParseResult, 
    Error
};

pub fn char<I>(ch: char) -> impl Parser<I, Output = I::Token, Error = ParseError<I::Token>> 
where
    I: Input,
    I::Token: AsChar
{
    sat(move |t: &I::Token| t.as_char() == ch)
}

pub fn char_no_case<I>(ch: char) -> impl Parser<I, Output = I::Token, Error = ParseError<I::Token>> 
where
    I: Input,
    I::Token: AsChar
{
    sat(move |t: &I::Token| t.as_char().eq_ignore_ascii_case(&ch))
}

pub fn string<I>(string: &str) -> impl Parser<I, Output = I, Error = ParseError<I::Token>> + '_ 
where
    I: Input,
    I::Token: AsChar
{
    move |mut input: I| {
        let src = input.clone();

        for mut ch in string.chars() {
            let (_, i) = ch.parse(input)?;
            input = i;
        }
        
        return Ok((src.diff(&input), input))
    }
}

pub fn string_no_case<I>(string: &str) -> impl Parser<I, Output = I, Error = ParseError<I::Token>> + '_ 
where
    I: Input,
    I::Token: AsChar
{
    move |mut input: I| {
        let src = input.clone();

        for ch in string.chars() {
            let (_, i) = char_no_case(ch).parse(input)?;
            input = i;
        }
        
        return Ok((src.diff(&input), input))
    }
}


pub fn sat<I, F>(mut pred: F) -> impl Parser<I, Output = I::Token, Error = ParseError<I::Token>> 
where
    I: Input,
    F: FnMut(&I::Token) -> bool
{
    move |mut input: I| {
        match input.next() {
            Some(t) if pred(&t) => Ok((t, input)),
            Some(t) => Err(ParseError::Unexpected(t)),
            None => Err(ParseError::Eoi)
        }
    }
}

pub mod ascii {

    use super::*;

    pub fn digit<I>(input: I) -> ParseResult<I::Token, I> 
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_digit()).parse(input)
    }

    pub fn alpha<I>(input: I) -> ParseResult<I::Token, I>
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphabetic()).parse(input)
    }

    pub fn lowercase<I>(input: I) -> ParseResult<I::Token, I>  
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_lowercase()).parse(input)
    }

    pub fn uppercase<I>(input: I) -> ParseResult<I::Token, I>  
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_uppercase()).parse(input)
    }

    pub fn alphanum<I>(input: I) -> ParseResult<I::Token, I>   
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphanumeric()).parse(input)
    }

    pub fn space<I>(input: I) -> ParseResult<I::Token, I>  
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_whitespace()).parse(input)
    }

    pub fn hex<I>(input: I) -> ParseResult<I::Token, I> 
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_ascii_hexdigit()).parse(input)
    }
}

pub mod unicode {
    use super::*;

    pub fn alphanum<I>(input: I) -> ParseResult<I::Token, I> 
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_alphanumeric()).parse(input)
    }


    pub fn alpha<I>(input: I) -> ParseResult<I::Token, I>
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_alphabetic()).parse(input)
    }

    pub fn lowercase<I>(input: I) -> ParseResult<I::Token, I>
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_lowercase()).parse(input)
    }

    pub fn uppercase<I>(input: I) -> ParseResult<I::Token, I>
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_uppercase()).parse(input)
    }

    pub fn space<I>(input: I) -> ParseResult<I::Token, I>
    where
        I: Input,
        I::Token: AsChar
    {
        sat(|c: &I::Token| c.as_char().is_whitespace()).parse(input)
    }

}

pub fn newline<I>(input: I) -> ParseResult<I::Token, I>
where
    I: Input,
    I::Token: AsChar
{
    '\n'.parse(input)
}


pub fn anychar<I>(input: I) -> ParseResult<I::Token, I> 
where
    I: Input,
    I::Token: AsChar
{
    sat(|_| true).parse(input)
}

pub fn oneof<I, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = ParseError<I::Token>> 
where
    I: Input,
    F: FindToken<I::Token>
{
    sat(move|t: &I::Token| tokens.find_token(t))
}

pub fn noneof<I, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = ParseError<I::Token>> 
where
    I: Input,
    F: FindToken<I::Token>
{
    sat(move|t: &I::Token| !tokens.find_token(t))
}


pub fn eof<I, E>(mut input: I) ->  Result<((), I), E>
where
    I: Input,
    E: Error<I>
{
    match input.next() {
        None => Ok(((), input)),
        Some(t) => Err(Error::from_token(Some(t)))
    }
}

pub fn error<I, E>(mut input: I) -> Result<((), I), E> 
where
    I: Input,
    E: Error<I>
{
    match input.next() {
        None => Err(Error::from_token(None)),
        Some(t) => Err(Error::from_token(Some(t)))
    }
}


pub fn pure<I, T, E>(t: T) -> impl Parser<I, Output = T, Error = E> 
where
    I: Input,
    T: Clone
{
    move|input: I| {
        Ok((t.clone(), input))
    }
}

impl<I> Parser<I> for char 
where 
    I: Input,
    I::Token: AsChar 
{
    type Output = I::Token;
    type Error = ParseError<I::Token>;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        char(*self).parse(input)
    }
}

impl<I> Parser<I> for &str 
where
    I: Input,
    I::Token: AsChar
{
    type Output = I;
    type Error = ParseError<I::Token>;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        string(self).parse(input)
    }
}
