use crate::{
    Parser,
    Input,
    Error,
    AsChar,
    character::ascii::space,
    combinator::skip_many,
}; 

pub fn symbol<I, P, E>(mut parser: P) -> impl Parser<I, Output = P::Output, Error = E> 
where
    I: Input,
    I::Token: AsChar,
    P: Parser<I, Error = E>,
    E: Error<I>
{
    move |input: I| {
        let (_, i) = skip_many(space).parse(input)?;
        parser.parse(i)
    }
}