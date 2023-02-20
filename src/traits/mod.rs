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
        F: FnMut(Self::Output) -> R,
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

    fn expect<Msg>(self, msg: Msg) -> Expect<Self, Msg> where Self: Sized {
        Expect { parser: self, msg: msg }
    }
}


impl<I, F, T, E> Parser<I> for F where F: FnMut(&mut I) -> Result<T, E> {
    type Output = T;
    type Error = E;

    fn parse(&mut self, input: &mut I) -> Result<Self::Output, Self::Error> {
        (self)(input)
    }
} 

pub trait Input: Iterator {

    type Pos: Copy;
    type Err;
    type Errs: IntoIterator<Item = Self::Err>;

    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized;
    
    fn restore_callback(&mut self, cursor: Cursor<Self::Pos>);
    
    fn commit_callback(&mut self, cursor: Cursor<Self::Pos>);

    fn report_err(&mut self, err: Self::Err);

    fn finish(self) -> Result<(), Self::Errs>;
}

