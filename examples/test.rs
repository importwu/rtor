use core::fmt;
use std::{str::Chars, ops};

use rtor::{
    Pos
};
fn main() {
    let mut s = State::new("abc");

    let r = satisfy(|&x| x == 'a').parse(&mut s).unwrap();

    println!("{:?}", s.len());

}

#[derive(Debug)]
pub struct ParseError<I: Input> {
    pub(crate) pos: I::Pos,
    pub(crate) unexpect: Option<I::Item>,
    pub(crate) expect: Vec<String>
}

type ParseResult<O, I: Input> = Result<O, ParseError<I>>;

pub trait Parser<I: Input> {

    type Output;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I>;
}

impl<F, I: Input, O> Parser<I> for F where 
    F: FnMut(&mut I) -> ParseResult<O, I>
{
    type Output = O;

    fn parse(&mut self, input: &mut I) -> ParseResult<Self::Output, I> {
        (self)(input)
    }
}

pub trait Input: Clone + Iterator {
    type Source;
    type Pos;

    fn as_source(&self) -> Self::Source;

    fn pos(&self) -> Self::Pos;

}

#[derive(Debug, Clone)]
pub struct State<'a> {
    source: Chars<'a>,
    pos: Pos
}


impl<'a> State<'a> {

    pub fn new(str: &'a str) -> Self {
        Self {
            source: str.chars(),
            pos: Pos::new()
        }
    }
}

impl<'a> Iterator for State<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.source.next()?;
        self.pos.move_by(ch);
        Some(ch)
    }
}

impl<'a> Input for State<'a> {
    type Source = &'a str;
    type Pos = Pos;

    fn as_source(&self) -> Self::Source {
        self.source.as_str()
    }

    fn pos(&self) -> Self::Pos {
        self.pos
    }
}

pub fn satisfy<F, I>(mut f: F) -> impl Parser<I, Output = char> where
    I: Input<Item = char>,
    F: FnMut(&char) -> bool 
{
    move |input: &mut I| {
        let pos = input.pos();
        match input.next() {
            None => Err(ParseError{pos, expect: vec![], unexpect: None}),
            Some(ch) if f(&ch) => Ok(ch),
            Some(ch) => Err(ParseError{pos, expect: vec![], unexpect: Some(ch)})
        }
    }
}

#[inline]
pub fn char<I: Input<Item = char>>(ch: char) -> impl Parser<I, Output = char> {
    satisfy(move |t| *t == ch)
}