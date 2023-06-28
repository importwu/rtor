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
pub struct Location {
    line: u64,
    column: u64,
}

impl Location {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
        }
    }

    pub fn advance(&mut self, ch: char) {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }else {
            self.column += 1;
        }
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn column(&self) -> u64 {
        self.column
    }
}

#[derive(Debug, Clone)]
pub struct LocatedInput<I> {
    inner: I,
    location: Location
}

impl<I> LocatedInput<I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            location: Location::new()
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }
}

impl<I> Deref for LocatedInput<I> {
    type Target = I;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> Input for LocatedInput<I> 
where 
    I: Input,
    I::Token: AsChar
{
    type Token = I::Token;
    type Tokens = I::Tokens;

    fn next(&mut self) -> Option<Self::Token> {
        let t = self.inner.next()?;
        self.location.advance(t.as_char());
        Some(t)
    }

    fn peek(&mut self) -> Option<Self::Token> {
        self.inner.peek()
    }

    fn diff(&self, other: &Self) -> Self {
        LocatedInput { 
            inner: self.inner.diff(&other.inner), 
            location: self.location
        }
    }

    fn tokens(&self) -> Self::Tokens {
        self.inner.tokens()
    }
}
