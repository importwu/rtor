use std::{
    fmt, 
    error
};

use crate::Input;

pub trait Error<I: Input> {
    fn unexpect(input: I, token: Option<I::Token>) -> Self;
    fn expect(input: I, message: &str) -> Self;
    fn merge(self, other: Self) -> Self;
}

#[derive(Debug)]
pub enum ParseError<I: Input> {
    Unexpected {
        input: I,
        token: Option<I::Token>
    },
    Expected(String)
}

impl<I: Input> Error<I> for ParseError<I> {
    fn unexpect(input: I, token: Option<I::Token>) -> Self {
        Self::Unexpected {
            input,
            token
        }
    }

    fn expect(input: I, message: &str) -> Self {
        Self::Expected(message.to_owned())
    }

    fn merge(self, other: Self) -> Self {
        other
    }
}

impl<I> fmt::Display for ParseError<I> 
where
    I: Input + fmt::Display,
    I::Token: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected {input, token: Some(t) } => write!(f, "unexpected {} as {}", t, input),
           Self::Unexpected {input: _, token: None} => f.write_str("end of input"),
           Self::Expected(message) => f.write_str(message)
        }        
    }
}

impl<I: Input + fmt::Display + fmt::Debug> error::Error for ParseError<I> where I::Token: fmt::Display + fmt::Debug {}