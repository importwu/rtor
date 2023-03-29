use crate::{
    State, 
    ParseError 
};

type ParseResult<O> = Result<O, ParseError>;

pub trait Parser {

    type Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output>;

    fn map<F, R>(self, f: F) -> Map<Self, F> where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f }
    }

    fn map_err<F, R>(self, f: F) -> MapErr<Self, F> where 
        F: FnMut(ParseError) -> R,
        Self: Sized
    {
        MapErr { parser: self, f }
    }

    fn or<P>(self, bparser: P) -> Or<Self, P> where
        P: Parser<Output = Self::Output>,
        Self: Sized
    {
        Or { aparser: self, bparser }
    }

    fn and<P>(self, bparser: P) -> And<Self, P> where
        P: Parser,
        Self: Sized
    {
        And { aparser: self, bparser }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<Self, F> where
        P: Parser,
        F: FnOnce(Self::Output) -> P + Clone,
        Self: Sized
    {
        AndThen { parser: self, f }
    }

    fn expect(self, msg: &str) -> Expect<Self> where 
        Self: Sized 
    {
        Expect { parser: self, msg: msg.to_owned() }
    }
}

impl<F, O> Parser for F where 
    F: FnMut(&mut State) -> ParseResult<O>,
{
    type Output = O;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        (self)(state)
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F
}

impl<P, F, R> Parser for Map<P, F> where 
    P: Parser,
    F: FnMut(P::Output) -> R
{
    type Output = R;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        let o = self.parser.parse(state)?;
        Ok((self.f)(o))
    }
}

pub struct MapErr<P, F> {
    parser: P,
    f: F
}

impl<P, F> Parser for MapErr<P, F> where 
    P: Parser,
    F: FnMut(ParseError) -> ParseError
{
    type Output = P::Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        match self.parser.parse(state) {
            Ok(t) => Ok(t),
            Err(e) => Err((self.f)(e))
        }
    }
}

pub struct Or<A, B> {
    aparser: A,
    bparser: B
}

impl<A, B> Parser for Or<A, B> where
    A: Parser,
    B: Parser<Output = A::Output>
{
    type Output = A::Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        match self.aparser.parse(&mut state.clone()) {
            Ok(t) => Ok(t),
            Err(e1) => match self.bparser.parse(state) {
                Ok(t) => Ok(t),
                Err(e2) => Err(e1.merge(e2))
            }
        }
    }
}

pub struct And<A, B> {
    aparser: A,
    bparser: B
}

impl<A, B> Parser for And<A, B> where
    A: Parser,
    B: Parser
{
    type Output = B::Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        self.aparser.parse(state)?;
        self.bparser.parse(state)
    }
}

pub struct AndThen<P, F> {
    parser: P,
    f: F
}

impl<A, B, F> Parser for AndThen<A, F> where
    A: Parser,
    B: Parser,
    F: FnOnce(A::Output) -> B + Clone
{
    type Output = B::Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        let o = self.parser.parse(state)?;
        (self.f.clone())(o).parse(state)
    }
}

pub struct Expect<P> {
    pub(crate) msg: String,
    pub(crate) parser: P
}

impl<P> Parser for Expect<P> where 
    P: Parser
{
    type Output = P::Output;

    fn parse(&mut self, state: &mut State) -> ParseResult<Self::Output> {
        match self.parser.parse(state) {
            Ok(t) => Ok(t),
            Err(e) => Err(ParseError { pos: e.pos, unexpect: e.unexpect, expect: vec![self.msg.clone()] })
        }
    }
} 

pub struct Iter<P> {
    parser: P
}