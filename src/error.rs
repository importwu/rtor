use std::{
    fmt, 
    error
};

use crate::{Input, Parser};


pub trait Error<I: Input> {
    fn from_token(token: Option<I::Token>) -> Self;

    fn merge(self, other: Self) -> Self;
}


#[derive(Debug)]
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

    fn merge(self, other: Self) -> Self {
        other
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

#[derive(Debug)]
pub struct MultiError<T> {
    pub errors: Vec<ParseError<T>>
}

impl<I: Input> Error<I> for MultiError<I::Token> {
    fn from_token(token: Option<<I as Input>::Token>) -> Self {
        match token {
            Some(t) => Self { errors: vec![ParseError::Unexpected(t)] },
            None => Self { errors: vec![ParseError::Eoi] }
        }
    }

    fn merge(mut self, mut other: Self) -> Self {
        self.errors.append(&mut other.errors);
        self
    }
}

use super::primitive::string;

#[test]
fn test() {
    let a: Result<(_, &str), MultiError<char>> = string("a").andl(string("vb").or(string("c"))).parse("av");

    println!("{:?}", a)
}