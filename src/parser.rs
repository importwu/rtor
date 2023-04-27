use std::marker::PhantomData;

use crate::Input;

pub trait Parser<I> {
    type Output;
    type Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error>;

    fn map<F, R>(self, f: F) -> Map<I, Self, F>
    where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f, marker: PhantomData }
    }

    fn map_err<F, R>(self, f: F) -> MapErr<I, Self, F> 
    where
        F: FnMut(Self::Error) -> R,
        Self: Sized
    {
        MapErr { parser: self, f, marker: PhantomData }
    }

    fn or<P>(self, bparser: P) -> Or<I, Self, P> 
    where
        P: Parser<I>,
        Self: Sized
    {
        Or { aparser: self, bparser, marker: PhantomData }
    }

    fn and<P>(self, bparser: P) -> And<I, Self, P> 
    where
        P: Parser<I>,
        Self: Sized
    {
        And { aparser: self, bparser, marker: PhantomData }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<I, Self, F> 
    where
        P: Parser<I>,
        F: FnOnce(Self::Output) -> P + Clone,
        Self: Sized
    {
        AndThen { parser: self, f, marker: PhantomData }
    }
}

impl<F, O, I, E> Parser<I> for F where F: FnMut(I) -> Result<(O, I), E> {
    type Output = O;
    type Error = E;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        (self)(input)
    }
}

pub struct Map<I, P, F> {
    parser: P,
    f: F,
    marker: PhantomData<I>
}


impl<I, P, F, R> Parser<I> for Map<I, P, F> 
where
    P: Parser<I>,
    F: FnMut(P::Output) -> R
{
    type Output = R;
    type Error = P::Error;

    fn parse(&mut self, input: I) ->  Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.parser.parse(input)?;
        Ok(((self.f)(o), i))
    }
}

pub struct MapErr<I, P, F> {
    parser: P,
    f: F,
    marker: PhantomData<I>
}

impl<I, P, F, R> Parser<I> for MapErr<I, P, F> 
where
    P: Parser<I>,
    F: FnMut(P::Error) -> R
{
    type Output = P::Output;
    type Error = R;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.parser.parse(input) {
            Ok(t) => Ok(t),
            Err(e) => Err((self.f)(e))
        }
    }
}

pub struct Or<I, A, B> {
    aparser: A,
    bparser: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for Or<I, A, B> 
where
    I: Input, 
    A: Parser<I>,
    B: Parser<I, Output = A::Output, Error = A::Error>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.aparser.parse(input.clone()) {
            Ok(t) => Ok(t),
            Err(_) => match self.bparser.parse(input) {
                Ok(t) => Ok(t),
                Err(e) => Err(e)
            }
        }
    }
}

pub struct And<I, A, B> {
    aparser: A,
    bparser: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for And<I, A, B> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    type Output = B::Output;
    type Error = B::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (_, i) = self.aparser.parse(input)?;
        self.bparser.parse(i)
    }
}

pub struct AndThen<I, P, F> {
    parser: P,
    f: F,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for AndThen<I, A, F> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>,
    F: FnOnce(A::Output) -> B + Clone
{
    type Output = B::Output;
    type Error = B::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.parser.parse(input)?;
        (self.f.clone())(o).parse(i)
    }
}
