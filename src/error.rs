use std::{
    fmt, 
    error,
    cmp::Ordering
};

use crate::{
    Input, 
    Location, 
    LocatedInput, 
    AsChar
};

pub trait Error<I: Input> {
    fn unexpect(token: Option<I::Token>, input: I) -> Self;
    fn expect(message: &str, input: I) -> Self;
    fn merge(self, other: Self) -> Self;
}

#[derive(Debug)]
pub enum ParseError<I: Input> {
    Unexpected(Option<I::Token>),
    Expected(String)
}

impl<I: Input> Error<I> for ParseError<I> {
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

#[derive(Debug)]
pub struct ParseErrors<I: Input> where I::Token: AsChar + fmt::Debug {
    pub location: Location,
    pub errors: Vec<ParseError<I>>
}

impl<I> Error<LocatedInput<I>> for ParseErrors<I> 
where 
    I: Input,
    I::Token: AsChar + fmt::Debug
{
    fn unexpect(token: Option<I::Token>, input: LocatedInput<I>) -> Self {
        Self { 
            location: input.location(), 
            errors: vec![ParseError::Unexpected(token)] 
        }
    }
    
    fn expect(message: &str, input: LocatedInput<I>) -> Self {
        Self { 
            location: input.location(), 
            errors: vec![ParseError::Expected(message.to_owned())] 
        }
    }

    fn merge(mut self, mut other: Self) -> Self {
        if !self.errors.is_empty() && other.errors.is_empty() {
            return self
        }

        if !other.errors.is_empty() && self.errors.is_empty() {
            return other
        }

        match self.location.cmp(&other.location) {
            Ordering::Equal => {
                self.errors.append(&mut other.errors);
                self
            }
            Ordering::Greater => self,
            Ordering::Less => other
        }
    }
}

impl<I: Input> fmt::Display for ParseErrors<I> where I::Token: AsChar + fmt::Debug + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at line:{}, column:{}", self.location.line(), self.location.column())?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error.to_string().as_str())?;
        }
        Ok(())
    }
}

impl<I: Input + fmt::Display + fmt::Debug> error::Error for ParseErrors<I> where I::Token: AsChar + fmt::Display + fmt::Debug {}