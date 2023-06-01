use std::{
    fmt, 
    error
};


#[derive(Debug, PartialEq)]
pub enum Error<T> {
    Unexpected(T),
    Eoi,
    Custom(String)
}

impl<T: fmt::Display> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected(t) =>  write!(f, "unexpected {}", t),
           Self::Eoi => write!(f, "end of input"),
           Self::Custom(msg) -> write!(f, msg)
        }        
    }
}

impl<T: fmt::Display + fmt::Debug> error::Error for Error<T> {}
