use crate::{
    Parser,
    Input,
    Error,
    AsChar,
    ParseResult,
    character::{
        char,
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
    alt
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
    let mut parser = between(char('('), parser, symbol(char(')')));
    move |input: I| parser.parse(input)
}

pub fn braces<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(char('{'), parser, symbol(char('}')));
    move |input: I| parser.parse(input)
}

pub fn angles<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(char('<'), parser, symbol(char('>')));
    move |input: I| parser.parse(input)
}

pub fn brackets<I, P, E>(parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    let mut parser = between(char('['), parser, symbol(char(']')));
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

// pub fn float<I, E>(input: I) -> ParseResult<I, I, E>
// where
//     I: Input,
//     I::Token: AsChar,
//     E: Error<I>
// {
//     let exponent = (alt!(char('e'), char('E')), opt(alt!(char('+'), char('-'))), skip_many1(digit));
//     let fraction = (char('.'), skip_many1(digit));
//     let integer = skip_many1(digit);
//     recognize((opt(char('-')), integer, opt(fraction), opt(exponent))).parse(input)
// }

pub fn float<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: Error<I>
{
    let exponent = alt!(char('e'), char('E')).andr(opt(alt!(char('+'), char('-')))).andr(skip_many1(digit));
    let fraction = char('.').andr(skip_many1(digit));
    recognize(opt(char('-')).andr(integer).andr(opt(fraction)).andr(opt(exponent)))
        .parse(input)
}

pub fn integer<I, E>(input: I) -> ParseResult<I, I, E>
where
    I: Input,
    I::Token: AsChar,
    E: Error<I>
{
    recognize(skip_many1(digit)).parse(input)
}