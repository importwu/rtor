use std::{
    fmt, 
    error
};

use crate::Input;


pub trait Error<I: Input> {
    fn from_token(token: Option<I::Token>) -> Self;
}


#[derive(Debug, PartialEq)]
pub enum ParseError<T> {
    Unexpected(T),
    Eoi,
    Message(String)
}

impl<I: Input> Error<I> for ParseError<I::Token> {
    fn from_token(token: Option<I::Token>) -> Self {
        match token {
            Some(t) => Self::Unexpected(t),
            None => Self::Eoi
        }
    }
}

impl<T: fmt::Display> fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected(t) =>  write!(f, "unexpected char {}", t),
           Self::Eoi => write!(f, "end of input"),
           Self::Message(msg) => write!(f, "{}", msg)
        }        
    }
}

impl<T: fmt::Display + fmt::Debug> error::Error for ParseError<T> {}
