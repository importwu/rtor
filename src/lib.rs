mod parser;
mod error;
pub mod character;
pub mod combinator;
pub mod token;
mod iter;
mod input;
mod r#macro;

pub use self::{
    error::{
        ParseError,
        Error
    },
    parser::Parser,
    input::{
        Input,
        Location,
        LocatedInput
    },
    iter::ParserIter
};

pub type ParseResult<O, I, E = ParseError<I>> = Result<(O, I), E>;

pub trait AsChar {
    fn as_char(&self) -> char;
}

impl AsChar for u8 {
    fn as_char(&self) -> char {
        *self as char
    }
}

impl AsChar for char {
    fn as_char(&self) -> char {
        *self
    }
}

pub trait FindToken<T> {

    fn find_token(&self, token: &T) -> bool;
}

impl<'a> FindToken<char> for &'a str {
    fn find_token(&self, token: &char) -> bool {
        self.chars().any(|x| x == *token)
    }
}

impl<'a> FindToken<u8> for &'a str {
    fn find_token(&self, token: &u8) -> bool {
        self.chars().any(|x| x == *token as char)
    }
}

impl<T: PartialEq, const N: usize> FindToken<T> for [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a, T: PartialEq, const N: usize> FindToken<T> for &'a [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a,  T: PartialEq> FindToken<T> for &'a [T] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

pub trait Alt<I> {
    type Output;
    type Error;
    fn choice(&mut self, input: I) -> Result<(Self::Output, I), Self::Error>;
}


#[macro_export]
macro_rules! alt {
    ($a: expr, $b: expr, $($rest: expr),*) => {
        |i| {
            match $a.parse(Clone::clone(&i)) {
                Ok(t) => Ok(t),
                Err(e1) => match $b.parse(Clone::clone(&i)) {
                    Ok(t) => Ok(t),
                    Err(e2) => {
                        let e1 = Error::merge(e1, e2);
                        $crate::alt_inner!(i, e1, $($rest),*)
                    }
                }
            }
        }
    };
    ($a: expr, $b: expr) => {
        |i| {
            match $a.parse(Clone::clone(&i)) {
                Ok(t) => Ok(t),
                Err(e1) => match $b.parse(Clone::clone(&i)) {
                    Ok(t) => Ok(t),
                    Err(e2) => Err(Error::merge(e1, e2))
                }
            } 
        }
    }
}

#[macro_export]
macro_rules! alt_inner {
    ($i: expr, $e1: expr, $a: expr, $($rest: expr),+) => {
        match $a.parse(Clone::clone(&$i)) {
            Ok(t) => Ok(t),
            Err(e2) => {
                let e1 = Error::merge($e1, e2);
                $crate::alt_inner!($i, e1, $($rest),+)
            }
        }
    };
    ($i: expr, $e1: expr, $a: expr) => {
        match $a.parse(Clone::clone(&$i)) {
            Ok(t) => Ok(t),
            Err(e2) => Err(Error::merge($e1, e2))
        }
    }
}

