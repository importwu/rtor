use std::{
    fmt, 
    error
};

use crate::Input;

pub trait Error<I: Input> {
    fn unexpect(token: Option<I::Token>) -> Self;
    fn expect(message: &str) -> Self;
    fn merge(self, other: Self) -> Self;
}

#[derive(Debug)]
pub enum ParseError<I: Input> {
    Unexpected(Option<I::Token>),
    Expected(String)
}

impl<I: Input> Error<I> for ParseError<I> {
    fn unexpect(token: Option<I::Token>) -> Self {
        Self::Unexpected(token)
    }

    fn expect(message: &str) -> Self {
        Self::Expected(message.to_owned())
    }

    fn merge(self, other: Self) -> Self {
        other
    }
}

impl<I> fmt::Display for ParseError<I> 
where
    I: Input,
    I::Token: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected(Some(token)) => write!(f, "unexpected {}", token),
           Self::Unexpected(None) => f.write_str("end of input"),
           Self::Expected(message) => f.write_str(message)
        }        
    }
}

impl<I: Input + fmt::Display + fmt::Debug> error::Error for ParseError<I> where I::Token: fmt::Display + fmt::Debug {}