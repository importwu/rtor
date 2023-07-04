use std::marker::PhantomData;

use crate::{
    Input, 
    ParseError,
    ParseResult
};

///A trait for parser
pub trait Parser<I, E> {
    type Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E>;

    fn map<F, R>(self, f: F) -> Map<Self, F>
    where 
        F: FnMut(Self::Output) -> R,
        Self: Sized
    {
        Map { parser: self, f }
    }

    fn map_err<F, R>(self, f: F) -> MapErr<E, Self, F> 
    where
        F: FnMut(E) -> R,
        Self: Sized
    {
        MapErr { parser: self, f, marker: PhantomData }
    }

    fn or<P>(self, second: P) -> Or<Self, P> where Self: Sized {
        Or { first: self, second }
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

    fn expect(self, message: &str) -> Expect<E, Self> where Self: Sized {
        Expect { parser: self, message: message.to_owned(), marker: PhantomData }
    }

}

impl<F, O, I, E> Parser<I, E> for F where F: FnMut(I) -> ParseResult<O, I, E> {
    type Output = O;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        (self)(input)
    }
}

#[derive(Clone)]
pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, E, P, F, R> Parser<I, E> for Map<P, F> 
where
    P: Parser<I, E>,
    F: FnMut(P::Output) -> R
{
    type Output = R;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        let (o, i) = self.parser.parse(input)?;
        Ok(((self.f)(o), i))
    }
}

#[derive(Clone)]
pub struct MapErr<E, P, F> {
    parser: P,
    f: F,
    marker: PhantomData<E>
}

impl<I, E, P, F, R> Parser<I, R> for MapErr<E, P, F> 
where
    P: Parser<I, E>,
    F: FnMut(E) -> R
{
    type Output = P::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, R> {
        match self.parser.parse(input) {
            Ok(t) => Ok(t),
            Err(e) => Err((self.f)(e))
        }
    }
}

#[derive(Clone)]
pub struct Or<A, B> {
    first: A,
    second: B,
}

impl<I, E, A, B> Parser<I, E> for Or<A, B> 
where
    I: Input, 
    A: Parser<I, E>,
    B: Parser<I, E, Output = A::Output>,
    E: ParseError<I>
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B> Parser<I, E> for Andl<A, B> 
where
    A: Parser<I, E>,
    B: Parser<I, E>
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B> Parser<I, E> for Andr<A, B> 
where
    A: Parser<I, E>,
    B: Parser<I, E>
{
    type Output = B::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        let (_, i) = self.first.parse(input)?;
        self.second.parse(i)
    }
}

#[derive(Clone)]
pub struct And<A, B> {
    first: A,
    second: B,
}

impl<I, E, A, B> Parser<I, E> for And<A, B> 
where
    A: Parser<I, E>,
    B: Parser<I, E>
{
    type Output = (A::Output, B::Output);

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B, F> Parser<I, E> for AndThen<A, F> 
where
    A: Parser<I, E>,
    B: Parser<I, E>,
    F: FnMut(A::Output) -> B
{
    type Output = B::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        let (o, i) = self.parser.parse(input)?;
        (self.f)(o).parse(i)
    }
}

#[derive(Clone)]
pub struct Ignore<P> {
    parser: P,
}

impl<I, E, P> Parser<I, E> for Ignore<P> 
where
    P: Parser<I, E>
{
    type Output = ();
    
    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        let (_, i) = self.parser.parse(input)?;
        Ok(((), i))
    }
}

pub struct RefMut<'a, P> {
    parser: &'a mut P,
}

impl<I, E, P> Parser<I, E> for RefMut<'_, P> 
where
    P: Parser<I, E>
{
    type Output = P::Output;
    
    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
        self.parser.parse(input)
    }
}

#[derive(Clone)]
pub struct Expect<E, P> {
    parser: P,
    message: String,
    marker: PhantomData<E>
}

impl<I, E, P> Parser<I, E> for Expect<E, P> 
where
    I: Input,
    E: ParseError<I>,
    P: Parser<I, E>,
{
    type Output = P::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B, F> Parser<I, E> for Chainl1<A, B> 
where
    I: Input,
    A: Parser<I, E>,
    B: Parser<I, E, Output = F>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B, F> Parser<I, E> for Chainl<A, B, A::Output> 
where
    I: Input,
    A: Parser<I, E>,
    A::Output: Clone,
    B: Parser<I, E, Output = F>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B, F> Parser<I, E> for Chainr1<A, B> 
where
    I: Input,
    A: Parser<I, E>,
    B: Parser<I, E, Output = F>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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

impl<I, E, A, B, F> Parser<I, E> for Chainr<A, B, A::Output> 
where
    I: Input,
    A: Parser<I, E>,
    A::Output: Clone,
    B: Parser<I, E, Output = F>,
    F: Fn(A::Output, A::Output) -> A::Output
{
    type Output = A::Output;

    fn parse(&mut self, input: I) -> ParseResult<Self::Output, I, E> {
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