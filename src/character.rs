use crate::{
    Input, 
    Parser, 
    AsChar,
    FindToken, 
    ParseResult, 
    ParseError
};

pub fn char<I, E>(ch: char) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    sat(move |t: &I::Token| t.as_char() == ch)
}

pub fn string<I, E>(string: &str) -> impl Parser<I, Output = I, Error = E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    move |mut input: I| {
        let src = input.clone();

        for ch in string.chars() {
            let (_, i) = char(ch).parse(input)?;
            input = i;
        }
        
        Ok((src.diff(&input), input))
    }
}

pub fn string_no_case<I, E>(string: &str) -> impl Parser<I, Output = I, Error = E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    move |mut input: I| {
        let src = input.clone();

        for ch in string.chars() {
            let (_, i) = sat(|t: &I::Token| t.as_char().eq_ignore_ascii_case(&ch)).parse(input)?;
            input = i;
        }
        
        Ok((src.diff(&input), input))
    }
}

pub fn sat<I, F, E>(mut pred: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FnMut(&I::Token) -> bool,
    E: ParseError<I>
{
    move |mut input: I| {
        match input.peek() {
            Some(t) if pred(&t) => {input.next(); return Ok((t, input))},
            other => Err(ParseError::unexpect(other, input)),
        }
    }
}

pub fn newline<I, E>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    char('\n').parse(input)
}

pub fn crlf<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    string("\r\n").parse(input)
}

pub fn tab<I, E>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    char('\t').parse(input)
}

pub fn anychar<I, E>(input: I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    sat(|_| true).parse(input)
}

pub fn oneof<I, F, E>(tokens: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FindToken<I::Token>,
    E: ParseError<I>
{
    sat(move|t: &I::Token| tokens.find_token(t))
}

pub fn noneof<I, F, E>(tokens: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FindToken<I::Token>,
    E: ParseError<I>
{
    sat(move|t: &I::Token| !tokens.find_token(t))
}

pub fn eof<I, E>(mut input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I>
{
    match input.peek() {
        None => Ok(((), input)),
        Some(t) => Err(ParseError::unexpect(Some(t), input))
    }
}

pub fn error<I, E>(mut input: I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I>
{
    Err(ParseError::unexpect(input.peek(), input))
}

pub fn pure<I, T, E>(t: T) -> impl Parser<I, Output = T, Error = E> 
where
    I: Input,
    T: Clone,
    E: ParseError<I>
{
    move|input: I| Ok((t.clone(), input))
}

pub mod ascii {

    use super::*;

    pub fn digit<I, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_digit()).parse(input)
    }

    pub fn alpha<I, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphabetic()).parse(input)
    }

    pub fn lowercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_lowercase()).parse(input)
    }

    pub fn uppercase<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_uppercase()).parse(input)
    }

    pub fn alphanum<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphanumeric()).parse(input)
    }

    pub fn space<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_whitespace()).parse(input)
    }

    pub fn hex<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_hexdigit()).parse(input)
    }
}

pub mod unicode {
    use super::*;

    pub fn alphanum<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_alphanumeric()).parse(input)
    }


    pub fn alpha<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_alphabetic()).parse(input)
    }

    pub fn lowercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_lowercase()).parse(input)
    }

    pub fn uppercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_uppercase()).parse(input)
    }

    pub fn space<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_whitespace()).parse(input)
    }
}