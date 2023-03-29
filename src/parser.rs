use crate::{
    State, 
    ParseError 
};

type ParseResult<O, I: Input> = Result<O, ParseError<I>>;

pub trait Parser<I: Input> {

    type Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I>;

    fn map<F, R>(self, f: F) -> Map<Self, F> where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f }
    }

    fn or<P>(self, bparser: P) -> Or<Self, P> where
        P: Parser<U, Output = Self::Output>,
        Self: Sized
    {
        Or { aparser: self, bparser }
    }

    fn and<P>(self, bparser: P) -> And<Self, P> where
        P: Parser<U>,
        Self: Sized
    {
        And { aparser: self, bparser }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<Self, F> where
        P: Parser<U>,
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

impl<F, I: Input, O> Parser<I> for F where 
    F: FnMut(&mut I) -> ParseResult<O, I>
{
    type Output = O;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        (self)(input)
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F
}

impl<I, P, F, R> Parser<I> for Map<P, F> where 
    I: Input,
    P: Parser<I>,
    F: FnMut(P::Output) -> R
{
    type Output = R;

    fn parse(&mut self, input: &mut I) ->  ParseResult<Self::Output, I> {
        let o = self.parser.parse(input)?;
        Ok((self.f)(o))
    }
}


pub struct Or<A, B> {
    aparser: A,
    bparser: B
}

impl<I, A, B> Parser<I> for Or<A, B> where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Output = A::Output>
{
    type Output = A::Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        match self.aparser.parse(&mut input.clone()) {
            Ok(t) => Ok(t),
            Err(e1) => match self.bparser.parse(input) {
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

impl<I, A, B> Parser<I> for And<A, B> where
    I: Input,
    A: Parser<I>,
    B: Parser<I>
{
    type Output = B::Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        self.aparser.parse(input)?;
        self.bparser.parse(input)
    }
}

pub struct AndThen<P, F> {
    parser: P,
    f: F
}

impl<I, A, B, F> Parser<I> for AndThen<A, F> where
    A: Parser<I>,
    B: Parser<I>,
    F: FnOnce(A::Output) -> B + Clone
{
    type Output = B::Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        let o = self.parser.parse(input)?;
        (self.f.clone())(o).parse(input)
    }
}

pub struct Expect<P> {
    pub(crate) msg: String,
    pub(crate) parser: P
}

impl<I, P> Parser<I> for Expect<P> where 
    I: Input,
    P: Parser<I>
{
    type Output = P::Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        match self.parser.parse(input) {
            Ok(t) => Ok(t),
            Err(e) => Err(ParseError { pos: e.pos, unexpect: e.unexpect, expect: vec![self.msg.clone()] })
        }
    }
} 

pub struct Iter<P> {
    parser: P
}