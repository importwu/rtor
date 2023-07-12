use std::{
    fmt, 
    error,
};

use crate::Input;

pub trait ParseError<I: Input> {
    fn unexpect(token: Option<I::Token>, input: I) -> Self;
    fn expect(message: String, input: I) -> Self;
    fn merge(self, other: Self) -> Self where Self: Sized{
        other
    }
}

#[derive(Debug, PartialEq)]
pub enum SimpleError<T> {
    Unexpected(Option<T>),
    Expected(String)
}

impl<I: Input> ParseError<I> for SimpleError<I::Token> {
    fn unexpect(token: Option<I::Token>, _: I) -> Self {
        Self::Unexpected(token)
    }

    fn expect(message: String, _: I) -> Self {
        Self::Expected(message)
    }

    fn merge(self, other: Self) -> Self {
        other
    }
}

impl<T> fmt::Display for SimpleError<T> 
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected(Some(token)) => write!(f, "unexpected {}", token),
           Self::Unexpected(None) => write!(f, "end of input"),
           Self::Expected(message) => write!(f, "{}", message)
        }        
    }
}

impl<T> error::Error for SimpleError<T> where T: fmt::Display + fmt::Debug {}