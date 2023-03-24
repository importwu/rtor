use crate::{
    ParseError, State, 
};

type ParseResult<O> = Result<O, ParseError>;

pub trait Parser<U> {

    type Output;

    fn parse(&mut self, state: &mut State<U>) -> ParseResult<Self::Output>;

    fn map<F, R>(self, f: F) -> Map<Self, F> where 
        Self: Sized,
        F: FnMut(Self::Output) -> R 
    {
        Map { parser: self, f }
    }

    fn or<P>(self, bparser: P) -> Or<Self, P> where
        Self: Sized,
        P: Parser<U, Output = Self::Output>
    {
        Or { aparser: self, bparser }
    }
}

impl<F, U, O> Parser<U> for F where 
    F: FnMut(&mut State<U>) -> ParseResult<O>,
{
    type Output = O;

    fn parse(&mut self, state: &mut State<U>) -> ParseResult<Self::Output> {
        (self)(state)
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F
}

impl<U, P, F, R> Parser<U> for Map<P, F> where 
    P: Parser<U>,
    F: FnMut(P::Output) -> R
{
    type Output = R;

    fn parse(&mut self, state: &mut State<U>) -> ParseResult<Self::Output> {
        match self.parser.parse(state) {
            Ok(t) => Ok((self.f)(t)),
            Err(e) => Err(e)
        }
    }
}

pub struct Or<A, B> {
    aparser: A,
    bparser: B
}

impl<U: Clone, A, B> Parser<U> for Or<A, B> where
    A: Parser<U>,
    B: Parser<U, Output = A::Output>
{
    type Output = A::Output;

    fn parse(&mut self, state: &mut State<U>) -> ParseResult<Self::Output> {
        match self.aparser.parse(&mut state.clone()) {
            Ok(t) => Ok(t),
            Err(aparser_err) => match self.bparser.parse(state) {
                Ok(t) => Ok(t),
                Err(bparser_err) => Err(aparser_err.merge(bparser_err))
            }
        }
    }
}


// pub struct Expect<P> {
//     msg: String,
//     parser: P
// }

// impl<I, P> Parser<I> for Expect<P> where 
//     P: Parser<I>
// {
//     type Output = P::Output;

//     fn parse(&mut self, input: I) -> ParseResult<Self::Output, I> {
//         match self.parser.parse(input) {
//             Ok(t) => Ok(t),
//             Err(e) => todo!()
//         }
//     }
// } 