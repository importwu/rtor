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
    fn expect(message: &str, input: I) -> Self;
    fn merge(self, other: Self) -> Self;
}

#[derive(Debug)]
pub enum SimpleError<I: Input> {
    Unexpected(Option<I::Token>),
    Expected(String)
}

impl<I: Input> ParseError<I> for SimpleError<I> {
    fn unexpect(token: Option<I::Token>, _: I) -> Self {
        Self::Unexpected(token)
    }

    fn expect(message: &str, _: I) -> Self {
        Self::Expected(message.to_owned())
    }

    fn merge(self, other: Self) -> Self {
        other
    }
}

impl<I> fmt::Display for SimpleError<I> 
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

impl<I: Input + fmt::Display + fmt::Debug> error::Error for SimpleError<I> where I::Token: fmt::Display + fmt::Debug {}

#[derive(Debug)]
pub struct MultiError<I: Input> where I::Token: AsChar + fmt::Debug {
    pub pos: Pos,
    pub errors: Vec<SimpleError<I>>
}

impl<I> ParseError<State<I>> for MultiError<I> 
where 
    I: Input,
    I::Token: AsChar + fmt::Debug
{
    fn unexpect(token: Option<I::Token>, input: State<I>) -> Self {
        Self { 
            pos: input.pos(), 
            errors: vec![SimpleError::Unexpected(token)] 
        }
    }
    
    fn expect(message: &str, input: State<I>) -> Self {
        Self { 
            pos: input.pos(), 
            errors: vec![SimpleError::Expected(message.to_owned())] 
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

impl<I: Input> fmt::Display for MultiError<I> where I::Token: AsChar + fmt::Debug + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at line:{}, column:{}", self.pos.line(), self.pos.column())?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error.to_string().as_str())?;
        }
        Ok(())
    }
}

impl<I: Input + fmt::Display + fmt::Debug> error::Error for MultiError<I> where I::Token: AsChar + fmt::Display + fmt::Debug {}