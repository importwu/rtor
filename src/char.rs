use crate::{
    Input, 
    Parser, 
    AsChar,
    FindToken, 
    ParseResult, 
    ParseError
};

pub fn char<I, E>(ch: char) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    sat(move |t: &I::Token| t.as_char() == ch)
}

pub fn string<I, E>(string: &str) -> impl FnMut(I) -> ParseResult<I, I, E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
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

pub fn string_no_case<I, E>(string: &str) -> impl FnMut(I) -> ParseResult<I, I, E> + '_ 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
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

pub fn sat<I, E, F>(mut pred: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
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
    char('\n')(input)
}

pub fn crlf<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    string("\r\n")(input)
}

pub fn tab<I, E>(input: I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    char('\t')(input)
}

pub fn eol<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    string("\n").or(string("\r\n")).parse(input)
}

pub fn anychar<I, E>(input: I) -> ParseResult<I::Token, I, E> 
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    sat(|_| true)(input)
}

pub fn one_of<I, E, F>(tokens: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    F: FindToken<I::Token>,
    E: ParseError<I>
{
    sat(move|t: &I::Token| tokens.find_token(t))
}

pub fn none_of<I, E, F>(tokens: F) -> impl FnMut(I) -> ParseResult<I::Token, I, E>
where
    I: Input,
    I::Token: AsChar,
    F: FindToken<I::Token>,
    E: ParseError<I>
{
    sat(move|t: &I::Token| !tokens.find_token(t))
}

pub mod ascii {

    use super::*;

    pub fn digit<I, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_digit())(input)
    }

    pub fn alpha<I, E>(input: I) -> ParseResult<I::Token, I, E>
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphabetic())(input)
    }

    pub fn lowercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_lowercase())(input)
    }

    pub fn uppercase<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_uppercase())(input)
    }

    pub fn alphanum<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_alphanumeric())(input)
    }

    pub fn space<I, E>(input: I) -> ParseResult<I::Token, I, E>   
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_whitespace())(input)
    }

    pub fn hex<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_ascii_hexdigit())(input)
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
        sat(|c: &I::Token| c.as_char().is_alphanumeric())(input)
    }

    pub fn alpha<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_alphabetic())(input)
    }

    pub fn lowercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_lowercase())(input)
    }

    pub fn uppercase<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_uppercase())(input)
    }

    pub fn space<I, E>(input: I) -> ParseResult<I::Token, I, E> 
    where
        I: Input,
        I::Token: AsChar,
        E: ParseError<I>
    {
        sat(|c: &I::Token| c.as_char().is_whitespace())(input)
    }
}
