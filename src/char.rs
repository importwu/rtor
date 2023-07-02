use crate::{
    Input, 
    Parser, 
    AsChar,
    FindToken, 
    ParseResult, 
    ParseError
};

pub fn char<I, S, E>(ch: char) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    sat(move |t: &I::Token| t.as_char() == ch)
}

pub fn string<I, S, E>(string: &str) -> impl Parser<I, Output = I, Error = E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
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

pub fn string_no_case<I, S, E>(string: &str) -> impl Parser<I, Output = I, Error = E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
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

pub fn sat<I, S, E, F>(mut pred: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FnMut(&I::Token) -> bool,
    E: ParseError<I, S>
{
    move |mut input: I| {
        match input.peek() {
            Some(t) if pred(&t) => {input.next(); return Ok((t, input))},
            other => Err(ParseError::unexpect(other, input)),
        }
    }
}

pub fn newline<I, S, E>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    char('\n').parse(input)
}

pub fn crlf<I, S, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    string("\r\n").parse(input)
}

pub fn tab<I, S, E>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    char('\t').parse(input)
}

pub fn anychar<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    sat(|_| true).parse(input)
}

pub fn oneof<I, S, E, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FindToken<I::Token>,
    E: ParseError<I, S>
{
    sat(move|t: &I::Token| tokens.find_token(t))
}

pub fn noneof<I, S, E, F>(tokens: F) -> impl Parser<I, Output = I::Token, Error = E> 
where
    I: Input,
    F: FindToken<I::Token>,
    E: ParseError<I, S>
{
    sat(move|t: &I::Token| !tokens.find_token(t))
}

pub fn eof<I, S, E>(mut input: I) ->  ParseResult<(), I, E>
where
    I: Input,
    E: ParseError<I, S>
{
    match input.peek() {
        None => Ok(((), input)),
        Some(t) => Err(ParseError::unexpect(Some(t), input))
    }
}

pub fn error<I, S, E>(mut input: I) -> ParseResult<(), I, E> 
where
    I: Input,
    E: ParseError<I, S>
{
    Err(ParseError::unexpect(input.peek(), input))
}

pub fn pure<I, S, E, T>(t: T) -> impl Parser<I, Output = T, Error = E> 
where
    I: Input,
    T: Clone,
    E: ParseError<I, S>
{
    move|input: I| Ok((t.clone(), input))
}

pub mod ascii {

    use super::*;

    pub fn digit<I, S, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_digit()).parse(input)
    }

    pub fn alpha<I, S, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphabetic()).parse(input)
    }

    pub fn lowercase<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_lowercase()).parse(input)
    }

    pub fn uppercase<I, S, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_uppercase()).parse(input)
    }

    pub fn alphanum<I, S, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphanumeric()).parse(input)
    }

    pub fn space<I, S, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_whitespace()).parse(input)
    }

    pub fn hex<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_hexdigit()).parse(input)
    }
}

pub mod unicode {
    use super::*;

    pub fn alphanum<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_alphanumeric()).parse(input)
    }


    pub fn alpha<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_alphabetic()).parse(input)
    }

    pub fn lowercase<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_lowercase()).parse(input)
    }

    pub fn uppercase<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_uppercase()).parse(input)
    }

    pub fn space<I, S, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_whitespace()).parse(input)
    }
}