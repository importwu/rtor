use crate::cursor::{CursorGuard, Cursor};
use crate::adapters::{Map, AndThen, MapErr, And, Or, Expect};



pub trait Parser<I> {
    type Output;
    type Error;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error>;

    fn map<F, R>(self, f: F) -> Map<Self, F> 
        where Self: Sized,
            F: FnMut(Self::Output) -> R 
    {
        Map { parser: self, f }
    }

    fn map_err<F, R>(self, f: F) -> MapErr<Self, F> 
        where Self: Sized,
            F: FnMut(Self::Error) -> R 
    {
        MapErr { parser: self, f }
    }

    fn and_then<F, R>(self, f: F) -> AndThen<Self, F> 
        where Self: Sized,
        F: FnOnce(Self::Output) -> R,
        R: Parser<I, Error = Self::Error>
    {
        AndThen { parser: self, f }
    }

    fn and<P>(self, parser: P) -> And<Self, P> 
        where Self: Sized,
            P: Parser<I, Error = Self::Error>
    {
        And { aparser: self, bparser: parser }
    }

    fn or<P>(self, parser: P) -> Or<Self, P> 
        where Self: Sized,
            P: Parser<I, Output = Self::Output, Error = Self::Error>
    {
        Or { aparser: self, bparser: parser }
    }

    fn expect(self, msg: I::Msg) -> Expect<Self, I::Msg> where Self: Sized, I: Input {
        Expect { parser: self, msg }
    }
}


impl<I, F, T, E> Parser<I> for F where F: FnMut(&mut I) -> Result<T, E> {
    type Output = T;
    type Error = E;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        (self)(input)
    }
} 

impl<I, T, E> Parser<I> for Box<dyn Parser<I, Output = T, Error = E>> {
    type Output = T;
    type Error = E;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        (**self).parse(input)
    }
}


pub trait Input: Iterator {

    type Pos: Copy;
    type Msg;

    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized;
    
    fn restore_callback(&mut self, cursor: Cursor<Self::Pos>);
    
    fn commit_callback(&mut self, cursor: Cursor<Self::Pos>);

    fn report(&mut self, msg: Self::Msg);

    fn finish(self) -> Vec<Self::Msg>;
}

