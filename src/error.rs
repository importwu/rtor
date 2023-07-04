use std::{
    fmt, 
    error,
    cmp::Ordering
};

use crate::{
    Input, 
    Pos, 
    State, 
    AsChar
};

pub trait ParseError<I: Input> {
    fn unexpect(token: Option<I::Token>, input: I) -> Self;
    fn expect(message: String, input: I) -> Self;
    fn merge(self, other: Self) -> Self;
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

#[derive(Debug, PartialEq)]
pub struct MultiError<T> {
    pub pos: Pos,
    pub errors: Vec<SimpleError<T>>
}

impl<I> ParseError<State<I>> for MultiError<I::Token> 
where 
    I: Input,
    I::Token: AsChar
{
    fn unexpect(token: Option<I::Token>, input: State<I>) -> Self {
        Self { 
            pos: input.pos(), 
            errors: vec![SimpleError::Unexpected(token)] 
        }
    }
    
    fn expect(message: String, input: State<I>) -> Self {
        Self { 
            pos: input.pos(), 
            errors: vec![SimpleError::Expected(message)] 
        }
    }

    fn merge(mut self, mut other: Self) -> Self {
        if !self.errors.is_empty() && other.errors.is_empty() {
            return self
        }

        if !other.errors.is_empty() && self.errors.is_empty() {
            return other
        }

        match self.pos.cmp(&other.pos) {
            Ordering::Equal => {
                self.errors.append(&mut other.errors);
                self
            }
            Ordering::Greater => self,
            Ordering::Less => other
        }
    }
}

impl<T> fmt::Display for MultiError<T> 
where
    T: AsChar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at line:{}, column:{}", self.pos.line(), self.pos.column())?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl<T> error::Error for MultiError<T> where T: AsChar + fmt::Display + fmt::Debug {}