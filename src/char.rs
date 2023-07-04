use crate::{
    Input, 
    Parser, 
    AsChar,
    FindToken, 
    ParseResult, 
    ParseError
};

pub fn char<I, E, S>(ch: char) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    sat(move |t: &I::Token| t.as_char() == ch)
}

pub fn string<I, E, S>(string: &str) -> impl FnMut(I) -> ParseResult<I, I, E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    move |mut input: I| {
        let src = input.clone();

        for ch in string.chars() {
            let (_, i) = char(ch)(input)?;
            input = i;
        }
        
        Ok((src.diff(&input), input))
    }
}

pub fn string_no_case<I, E, S>(string: &str) -> impl FnMut(I) -> ParseResult<I, I, E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    move |mut input: I| {
        let src = input.clone();

        for ch in string.chars() {
            let (_, i) = sat(|t: &I::Token| t.as_char().eq_ignore_ascii_case(&ch))(input)?;
            input = i;
        }
        
        Ok((src.diff(&input), input))
    }
}

pub fn sat<I, E, S, F>(mut pred: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
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

pub fn newline<I, E, S>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    char('\n')(input)
}

pub fn crlf<I, E, S>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    string("\r\n")(input)
}

pub fn tab<I, E, S>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    char('\t')(input)
}

pub fn eol<I, E, S>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    string("\n").or(string("\r\n")).parse(input)
}

pub fn anychar<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I, S>
{
    sat(|_| true)(input)
}

pub fn one_of<I, E, S, F>(tokens: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    F: FindToken<I::Token>,
    E: ParseError<I, S>
{
    sat(move|t: &I::Token| tokens.find_token(t))
}

pub fn none_of<I, E, S, F>(tokens: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    F: FindToken<I::Token>,
    E: ParseError<I, S>
{
    sat(move|t: &I::Token| !tokens.find_token(t))
}

pub mod ascii {

    use super::*;

    pub fn digit<I, E, S>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_digit())(input)
    }

    pub fn alpha<I, E, S>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphabetic())(input)
    }

    pub fn lowercase<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_lowercase())(input)
    }

    pub fn uppercase<I, E, S>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_uppercase())(input)
    }

    pub fn alphanum<I, E, S>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphanumeric())(input)
    }

    pub fn space<I, E, S>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_whitespace())(input)
    }

    pub fn hex<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_hexdigit())(input)
    }
}

pub mod unicode {
    use super::*;

    pub fn alphanum<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_alphanumeric())(input)
    }


    pub fn alpha<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_alphabetic())(input)
    }

    pub fn lowercase<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_lowercase())(input)
    }

    pub fn uppercase<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_uppercase())(input)
    }

    pub fn space<I, E, S>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I, S>
    {
        sat(|c: &I::Token| c.as_char().is_whitespace())(input)
    }
}