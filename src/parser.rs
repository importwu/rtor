use std::marker::PhantomData;

use crate::{
    Input, 
    Error
};

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

    fn or<P>(self, second: P) -> Or<I, Self, P> where Self: Sized {
        Or { first: self, second, marker: PhantomData }
    }

    fn andl<P>(self, second: P) -> Andl<I, Self, P> where Self: Sized {
        Andl { first: self, second, marker: PhantomData }
    }

    fn andr<P>(self, second: P) -> Andr<I, Self, P> where Self: Sized {
        Andr { first: self, second, marker: PhantomData }
    }

    fn and<P>(self, second: P) -> And<I, Self, P> where Self: Sized {
        And { first: self, second, marker: PhantomData }
    }

    fn and_then<F, P>(self, f: F) -> AndThen<I, Self, F> 
    where
        F: FnMut(Self::Output) -> P,
        Self: Sized
    {
        AndThen { parser: self, f, marker: PhantomData }
    }

    fn chainl1<P>(self, op: P) -> Chainl1<I, Self, P> where Self: Sized {
        Chainl1 { parser: self, op, marker: PhantomData }
    }

    fn chainl<P>(self, op: P, value: Self::Output) -> Chainl<I, Self, P> where Self: Sized {
        Chainl { parser: self, op, value, marker: PhantomData }
    }

    fn chainr1<P>(self, op: P) -> Chainr1<I, Self, P> where Self: Sized {
        Chainr1 { parser: self, op, marker: PhantomData }
    }

    fn chainr<P>(self, op: P, value: Self::Output) -> Chainr<I, Self, P> where Self: Sized {
        Chainr { parser: self, op, value, marker: PhantomData }
    }
    
    fn ignore(self) -> Ignore<I, Self> where Self: Sized {
        Ignore { parser: self, marker: PhantomData }
    }

    fn ref_mut(&mut self) -> RefMut<I, Self> where Self: Sized {
        RefMut { parser: self, marker: PhantomData }
    }

    fn cloned(&self) -> Cloned<I, Self> where Self: Sized + Clone {
        Cloned { parser: self.clone(), marker: PhantomData }
    }

    fn expect(self, message: &str) -> Expect<I, Self, Self::Error> where Self: Sized {
        Expect { parser: self, message: message.to_owned(), marker: PhantomData }
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Or<I, A, B> {
    first: A,
    second: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for Or<I, A, B> 
where
    I: Input, 
    A: Parser<I>,
    B: Parser<I, Output = A::Output, Error = A::Error>,
    A::Error: Error<I>
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
pub struct Andl<I, A, B> {
    first: A,
    second: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for Andl<I, A, B> 
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
pub struct Andr<I, A, B> {
    first: A,
    second: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for Andr<I, A, B> 
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
pub struct And<I, A, B> {
    first: A,
    second: B,
    marker: PhantomData<I>
}

impl<I, A, B> Parser<I> for And<I, A, B> 
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
pub struct AndThen<I, P, F> {
    parser: P,
    f: F,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for AndThen<I, A, F> 
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
pub struct Ignore<I, P> {
    parser: P,
    marker: PhantomData<I>
}

impl<I, P> Parser<I> for Ignore<I, P> 
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

pub struct RefMut<'a, I, P> {
    parser: &'a mut P,
    marker: PhantomData<I>
}

impl<I, P> Parser<I> for RefMut<'_, I, P> 
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
pub struct Cloned<I, P> {
    parser: P,
    marker: PhantomData<I>
}

impl<I, P> Parser<I> for Cloned<I, P> 
where
    P: Parser<I>,
{
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        self.parser.parse(input)
    }
}

#[derive(Clone)]
pub struct Expect<I, P, E> {
    parser: P,
    message: String,
    marker: PhantomData<(I, E)>
}

impl<I, P, E> Parser<I> for Expect<I, P, E> 
where
    I: Input,
    P: Parser<I>,
    E: Error<I>
{
    type Output = P::Output;
    type Error = E;

    fn parse(&mut self, input: I) -> Result<(Self::Output, I), Self::Error> {
        match self.parser.parse(input.clone()) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::expect(&self.message, input))
        }
    }
}

#[derive(Clone)]
pub struct Chainl1<I, A, B> {
    parser: A,
    op: B,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for Chainl1<I, A, B> 
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
pub struct Chainl<I, A: Parser<I>, B> {
    parser: A,
    op: B,
    value: A::Output,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for Chainl<I, A, B> 
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
pub struct Chainr1<I, A, B> {
    parser: A,
    op: B,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for Chainr1<I, A, B> 
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
pub struct Chainr<I, A: Parser<I>, B> {
    parser: A,
    op: B,
    value: A::Output,
    marker: PhantomData<I>
}

impl<I, A, B, F> Parser<I> for Chainr<I, A, B> 
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