use crate::{
    Parser,
    Input,
    Error,
    AsChar,
    ParseResult,
    character::{
        char,
        sat,
        ascii::{
            space,
            digit,
        },
    },
    combinator::{
        skip_many,
        skip_many1,
        between,
        sepby,
        sepby1,
        recognize,
        opt,
    }, 
}; 

pub fn symbol<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = skip_many(space).andr(parser);
    move |input: I| parser.parse(input)
}

pub fn parens<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(symbol(char('(')), parser, symbol(char(')')));
    move |input: I| parser.parse(input)
}

pub fn braces<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(symbol(char('{')), parser, symbol(char('}')));
    move |input: I| parser.parse(input)
}

pub fn angles<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(symbol(char('<')), parser, symbol(char('>')));
    move |input: I| parser.parse(input)
}

pub fn brackets<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(symbol(char('[')), parser, symbol(char(']')));
    move |input: I| parser.parse(input)
}

pub fn comma_sep<I, P, E>(parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = sepby(parser, symbol(char(',')));
    move |input: I| parser.parse(input)
}

pub fn comma_sep1<I, P, E>(parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = sepby1(parser, symbol(char(',')));
    move |input: I| parser.parse(input)
}

pub fn semi_sep<I, P, E>(parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = sepby(parser, symbol(char(';')));
    move |input: I| parser.parse(input)
}


pub fn semi_sep1<I, P, E>(parser: P) -> impl Parser<I, Output = Vec<P::Output>, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = sepby1(parser, symbol(char(';')));
    move |input: I| parser.parse(input)
}

pub fn float<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: Error<I>
{
    let exponent = char('E').or(char('e')).andr(opt(char('+').or(char('-')))).andr(skip_many1(digit));
    let fraction = char('.').andr(skip_many1(digit));
    let integer = char('0').or(sat(|ch: &I::Token| matches!(ch.as_char(), '1'..='9')).andl(skip_many(digit)));
    recognize(opt(char('-')).andr(integer).andr(opt(fraction)).andr(opt(exponent)))
        .parse(input)
}
