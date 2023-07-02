use std::{
    str::Chars, 
    slice::Iter, 
    iter::Cloned,
    ops::Deref
};

use crate::AsChar;

pub trait Input: Clone {
    type Token: Clone;
    type Tokens: Iterator<Item = Self::Token>;

    fn next(&mut self) -> Option<Self::Token>;

    fn peek(&mut self) -> Option<Self::Token>;

    fn diff(&self, other: &Self) -> Self;

    fn tokens(&self) -> Self::Tokens;

}

impl<'a> Input for &'a str {
    type Token = char;
    type Tokens = Chars<'a>;

    fn next(&mut self) -> Option<Self::Token> {
        let mut chars = self.chars();
        let ch = chars.next()?;
        *self = chars.as_str();
        Some(ch)
    }

    fn peek(&mut self) -> Option<Self::Token> {
        self.chars().next()
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    fn tokens(&self) -> Self::Tokens {
        self.chars()
    }
}

impl<'a, T: Clone> Input for &'a [T] {
    type Token = T;
    type Tokens = Cloned<Iter<'a, T>>;
    
    fn next(&mut self) -> Option<Self::Token> {
        let mut iter = self.iter();
        let item = iter.next()?.clone();
        *self = iter.as_slice();
        Some(item)
    }
    
    fn peek(&mut self) -> Option<Self::Token> {
        self.iter().cloned().next()
    }
    
    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }
    
    fn tokens(&self) -> Self::Tokens {
        self.iter().cloned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    line: usize,
    column: usize,
}

impl Pos {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
        }
    }

    pub fn advance(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.line += 1; 
                self.column = 1;
            }
            _ => self.column += 1
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone)]
pub struct State<I> {
    input: I,
    pos: Pos
}

impl<I> State<I> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            pos: Pos::new()
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }
}

impl<I> Deref for State<I> {
    type Target = I;
    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl<I> Input for State<I> 
where 
    I: Input,
    I::Token: AsChar
{
    type Token = I::Token;
    type Tokens = I::Tokens;

    fn next(&mut self) -> Option<Self::Token> {
        let t = self.input.next()?;
        self.pos.advance(t.as_char());
        Some(t)
    }

    fn peek(&mut self) -> Option<Self::Token> {
        self.input.peek()
    }

    fn diff(&self, other: &Self) -> Self {
        State { 
            input: self.input.diff(&other.input), 
            pos: self.pos
        }
    }

    fn tokens(&self) -> Self::Tokens {
        self.input.tokens()
    }
}
