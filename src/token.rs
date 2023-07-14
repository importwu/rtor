use crate::{
    Parser,
    Input,
    ParseError,
    AsChar,
    ParseResult,
    char::{
        char,
        unicode,
        ascii
    },
    combinator::{
        between,
        sep_by,
        sep_by1,
        recognize,
        opt,
        alt, 
    }, 
}; 

pub fn token<I, E, P>(mut parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    move |input: I| {
        let (_, i) = unicode::multi_space(input)?;
        parser.parse(i)
    }
}

pub fn parens<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = between(token(char('(')), parser, token(char(')')));
    move |input: I| parser.parse(input)
}

pub fn braces<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = between(token(char('{')), parser, token(char('}')));
    move |input: I| parser.parse(input)
}

pub fn angles<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = between(token(char('<')), parser, token(char('>')));
    move |input: I| parser.parse(input)
}

pub fn brackets<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<P::Output, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = between(token(char('[')), parser, token(char(']')));
    move |input: I| parser.parse(input)
}

pub fn comma_sep<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = sep_by(parser, token(char(',')));
    move |input: I| parser.parse(input)
}

pub fn comma_sep1<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = sep_by1(parser, token(char(',')));
    move |input: I| parser.parse(input)
}

pub fn semi_sep<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E>
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = sep_by(parser, token(char(';')));
    move |input: I| parser.parse(input)
}


pub fn semi_sep1<I, E, P>(parser: P) -> impl FnMut(I) -> ParseResult<Vec<P::Output>, I, E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, E>,
    E: ParseError<I>
{
    let mut parser = sep_by1(parser, token(char(';')));
    move |input: I| parser.parse(input)
}

pub fn number<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: ParseError<I>
{
    let exponent = (alt((char('e'), char('E'))), opt(alt((char('+'), char('-')))), ascii::multi_digit1);
    let fraction = (char('.'), ascii::multi_digit1);
    token(recognize((opt(char('-')), ascii::multi_digit1, opt(fraction), opt(exponent))))(input)
}