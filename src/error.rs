use std::{
    fmt, 
    error,
};

pub trait ParseError<I> {
    fn unexpect(input: I) -> Self;
    fn expect(message: String, input: I) -> Self;
    fn merge(self, other: Self) -> Self where Self: Sized{
        other
    }
}

#[derive(Debug, PartialEq)]
pub struct SimpleError<I> {
    pub input: I,
    pub message: Option<String>
}

impl<I> ParseError<I> for SimpleError<I> {
    fn unexpect(input: I) -> Self {
        SimpleError { input, message: None }
    }

    fn expect(message: String, input: I) -> Self {
        SimpleError { input, message: Some(message) }
    }
}

impl<I> fmt::Display for SimpleError<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self.message {
           Some(ref msg) => write!(f, "expected {}, but found {}", msg, self.input),
           None => write!(f, "unexpected {}", self.input)
       }      
    }
}

impl<T> error::Error for SimpleError<T> where T: fmt::Display + fmt::Debug {}