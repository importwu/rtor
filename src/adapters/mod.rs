use crate::{Parser, Input};

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F
}

impl<I, P, F, R> Parser<I> for Map<P, F> 
    where P: Parser<I>, 
        F: FnMut(P::Output) -> R 
{
    type Error = P::Error;
    type Output = R;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.parser.parse(input) {
            Ok(v) => Ok((self.f)(v)),
            Err(e) => Err(e)
        }
    }
}

pub struct MapErr<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F
}

impl<I, P, F, R> Parser<I> for MapErr<P, F> 
    where P: Parser<I>, 
        F: FnMut(P::Error) -> R 
{
    type Error = R;
    type Output = P::Output;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.parser.parse(input) {
            Ok(v) => Ok(v),
            Err(e) => Err((self.f)(e))
        }
    }
}

pub struct AndThen<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F
}

impl<I, P, F, R> Parser<I> for AndThen<P, F> 
    where P: Parser<I>, 
        R: Parser<I, Error = P::Error>, 
        F: FnMut(P::Output) -> R 
{
    type Error = P::Error;
    type Output = R::Output;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.parser.parse(input) {
            Ok(v) => (self.f)(v).parse(input),
            Err(e) => Err(e)
        }
    }
}

pub struct And<A, B> {
    pub(crate) aparser: A,
    pub(crate) bparser: B
}

impl<I, A, B> Parser<I> for And<A, B> 
    where A: Parser<I>,
        B: Parser<I, Error = A::Error>
{
    type Output = B::Output;
    type Error = A::Error;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.aparser.parse(input) {
            Ok(_) => self.bparser.parse(input),
            Err(e) => Err(e)
        }
    }
}

pub struct Or<A, B> {
    pub(crate) aparser: A,
    pub(crate) bparser: B
}

impl<I, A, B> Parser<I> for Or<A, B> 
    where A: Parser<I>,
        B: Parser<I, Output = A::Output>
{
    type Output = A::Output;
    type Error = B::Error;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.aparser.parse(input) {
            Ok(v) => Ok(v),
            Err(_) => self.bparser.parse(input)
        }
    }
}

pub struct Expect<P, Msg> {
    pub(crate) parser: P,
    pub(crate) msg: Msg
}

impl<I, P> Parser<I> for Expect<P, I::Err> 
    where P: Parser<I>,
    I: Input,
    <I as Input>::Err: Clone    
{
    type Error = P::Error;
    type Output = Option<P::Output>;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        match self.parser.parse(input) {
            Ok(v) => Ok(Some(v)),
            Err(_) => {
                input.report_err(self.msg.clone());
                Ok(None)
            }
        }
    }
}

#[test]
fn test() {
}