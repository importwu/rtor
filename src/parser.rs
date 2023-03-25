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

    fn expect(self, msg: &str) -> Expect<Self> where Self: Sized {
        Expect {parser: self, msg: msg.to_owned()}
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
            Err(e1) => match self.bparser.parse(state) {
                Ok(t) => Ok(t),
                Err(e2) => Err(e1.merge(e2))
            }
        }
    }
}


pub struct Expect<P> {
    pub(crate) msg: String,
    pub(crate) parser: P
}

impl<U, P> Parser<U> for Expect<P> where 
    P: Parser<U>
{
    type Output = P::Output;

    fn parse(&mut self, state: &mut State<U>) -> ParseResult<Self::Output> {
        match self.parser.parse(state) {
            Ok(t) => Ok(t),
            Err(e) => Err(ParseError { pos: e.pos, unexpect: e.unexpect, expect: vec![self.msg.clone()] })
        }
    }
} 