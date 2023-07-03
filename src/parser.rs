use std::marker::PhantomData;

use crate::{
    Input, 
    ParseError
};

//A trait for parser
pub trait Parser<I> {
    type Output;
    type Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error>;

    fn map<F, R>(self, f: F) -> Map<Self, F>
    where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f }
    }

    fn map_err<F, R>(self, f: F) -> MapErr<Self, F> 
    where
        F: FnMut(Self::Error) -> R,
        Self: Sized
    {
        MapErr { parser: self, f }
    }

    fn or<P, S>(self, second: P) -> Or<S, Self, P> where Self: Sized {
        Or { first: self, second, marker: PhantomData }
    }

    fn andl<P>(self, second: P) -> Andl<Self, P> where Self: Sized {
        Andl { first: self, second }
    }

    fn andr<P>(self, second: P) -> Andr<Self, P> where Self: Sized {
        Andr { first: self, second }
    }

    fn and<P>(self, second: P) -> And<Self, P> where Self: Sized {
        And { first: self, second }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<Self, F> 
    where
        F: FnMut(Self::Output) -> P,
        Self: Sized
    {
        AndThen { parser: self, f }
    }

    fn chainl1<P>(self, op: P) -> Chainl1<Self, P> where Self: Sized {
        Chainl1 { parser: self, op }
    }

    fn chainl<P>(self, op: P, value: Self::Output) -> Chainl<Self, P, Self::Output> where Self: Sized {
        Chainl { parser: self, op, value }
    }

    fn chainr1<P>(self, op: P) -> Chainr1<Self, P> where Self: Sized {
        Chainr1 { parser: self, op }
    }

    fn chainr<P>(self, op: P, value: Self::Output) -> Chainr<Self, P, Self::Output> where Self: Sized {
        Chainr { parser: self, op, value }
    }
    
    fn ignore(self) -> Ignore<Self> where Self: Sized {
        Ignore { parser: self }
    }

    fn ref_mut(&mut self) -> RefMut<Self> where Self: Sized {
        RefMut { parser: self }
    }

    fn expect<S>(self, message: S) -> Expect<S, Self, Self::Error> where Self: Sized {
        Expect { parser: self, message, marker: PhantomData }
    }

}

impl<F, O, I, E> Parser<I> for F where F: FnMut(I) -> Result<(O, I), E> {
    type Output = O;
    type Error = E;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        (self)(input)
    }
}

#[derive(Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, P, F, R> Parser<I> for Map<P, F> 
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

#[derive(Clone)]
pub struct MapErr<P, F> {
    parser: P,
    f: F,
}

impl<I, P, F, R> Parser<I> for MapErr<P, F> 
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

#[derive(Clone)]
pub struct Or<S, A, B> {
    first: A,
    second: B,
    marker: PhantomData<S>
}

impl<I, S, A, B> Parser<I> for Or<S, A, B> 
where
    I: Input, 
    A: Parser<I>,
    B: Parser<I, Output = A::Output, Error = A::Error>,
    A::Error: ParseError<I, S>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.first.parse(input.clone()) {
            Ok(t) => Ok(t),
            Err(e1) => match self.second.parse(input) {
                Ok(t) => Ok(t),
                Err(e2) => Err(e1.merge(e2))
            }
        }
    }
}

#[derive(Clone)]
pub struct Andl<A, B> {
    first: A,
    second: B
}

impl<I, A, B> Parser<I> for Andl<A, B> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.first.parse(input)?;
        let (_, i) = self.second.parse(i)?;
        Ok((o, i))
    }
}

#[derive(Clone)]
pub struct Andr<A, B> {
    first: A,
    second: B,
}

impl<I, A, B> Parser<I> for Andr<A, B> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    type Output = B::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (_, i) = self.first.parse(input)?;
        self.second.parse(i)
    }
}

#[derive(Clone)]
pub struct And<A, B> {
    first: A,
    second: B,
}

impl<I, A, B> Parser<I> for And<A, B> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>
{
    type Output = (A::Output, B::Output);
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (o1, i) = self.first.parse(input)?;
        let (o2, i) = self.second.parse(i)?;
        Ok(((o1, o2), i))
    }
}

#[derive(Clone)]
pub struct AndThen<P, F> {
    parser: P,
    f: F,
}

impl<I, A, B, F> Parser<I> for AndThen<A, F> 
where
    A: Parser<I>,
    B: Parser<I, Error = A::Error>,
    F: FnMut(A::Output) -> B
{
    type Output = B::Output;
    type Error = B::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (o, i) = self.parser.parse(input)?;
        (self.f)(o).parse(i)
    }
}

#[derive(Clone)]
pub struct Ignore<P> {
    parser: P,
}

impl<I, P> Parser<I> for Ignore<P> 
where
    P: Parser<I>
{
    type Output = ();
    type Error = P::Error;
    
    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (_, i) = self.parser.parse(input)?;
        Ok(((), i))
    }
}

pub struct RefMut<'a, P> {
    parser: &'a mut P,
}

impl<I, P> Parser<I> for RefMut<'_, P> 
where
    P: Parser<I>
{
    type Output = P::Output;
    type Error = P::Error;
    
    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        self.parser.parse(input)
    }
}

#[derive(Clone)]
pub struct Expect<S, P, E> {
    parser: P,
    message: S,
    marker: PhantomData<E>
}

impl<I, S, P, E> Parser<I> for Expect<S, P, E> 
where
    I: Input,
    P: Parser<I>,
    E: ParseError<I, S>,
    S: Clone
{
    type Output = P::Output;
    type Error = E;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.parser.parse(input.clone()) {
            Ok(t) => Ok(t),
            Err(_) => Err(ParseError::expect(self.message.clone(), input))
        }
    }
}

#[derive(Clone)]
pub struct Chainl1<A, B> {
    parser: A,
    op: B,
}

impl<I, A, B, F> Parser<I> for Chainl1<A, B> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Output = F, Error = A::Error>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (mut left, mut input) = self.parser.parse(input)?;
        while let Ok((f, i)) = self.op.parse(input.clone()) {
            let (right, i) = self.parser.parse(i)?;
            left = f(left, right);
            input = i;
        }
        Ok((left, input))
    }
}  

#[derive(Clone)]
pub struct Chainl<A, B, V> {
    parser: A,
    op: B,
    value: V
}

impl<I, A, B, F> Parser<I> for Chainl<A, B, A::Output> 
where
    I: Input,
    A: Parser<I>,
    A::Output: Clone,
    B: Parser<I, Output = F, Error = A::Error>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (mut left, mut input) = match self.parser.parse(input.clone()) {
            Ok(t) => t,
            Err(_) => return Ok((self.value.clone(), input))
        };
        while let Ok((f, i)) = self.op.parse(input.clone()) {
            let (right, i) = self.parser.parse(i)?;
            left = f(left, right);
            input = i;
        }
        Ok((left, input))
    }
}  

#[derive(Clone)]
pub struct Chainr1<A, B> {
    parser: A,
    op: B,
}

impl<I, A, B, F> Parser<I> for Chainr1<A, B> 
where
    I: Input,
    A: Parser<I>,
    B: Parser<I, Output = F, Error = A::Error>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (mut left, mut input) = self.parser.parse(input)?;
        while let Ok((f, i)) = self.op.parse(input.clone()) {
            let (right, i) = self.parse(i)?;
            left = f(left, right);
            input = i;
        }
        Ok((left, input))
    }
}  

#[derive(Clone)]
pub struct Chainr<A, B, V> {
    parser: A,
    op: B,
    value: V,
}

impl<I, A, B, F> Parser<I> for Chainr<A, B, A::Output> 
where
    I: Input,
    A: Parser<I>,
    A::Output: Clone,
    B: Parser<I, Output = F, Error = A::Error>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;
    type Error = A::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        let (mut left, mut input) = match self.parser.parse(input.clone()) {
            Ok(t) => t,
            Err(_) => return Ok((self.value.clone(), input))
        };
        while let Ok((f, i)) = self.op.parse(input.clone()) {
            let (right, i) = self.parse(i)?;
            left = f(left, right);
            input = i;
        }
        Ok((left, input))
    }
}  