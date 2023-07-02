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

pub trait ParseError<I: Input, S> {
    fn unexpect(token: Option<I::Token>, input: I) -> Self;
    fn expect(message: S, input: I) -> Self;
    fn merge(self, other: Self) -> Self;
}

pub enum SimpleError<I: Input, S> {
    Unexpected(Option<I::Token>),
    Expected(S)
}

impl<I: Input, S> ParseError<I, S> for SimpleError<I, S> {
    fn unexpect(token: Option<I::Token>, _: I) -> Self {
        Self::Unexpected(token)
    }

    fn expect(message: S, _: I) -> Self {
        Self::Expected(message)
    }

    fn merge(self, other: Self) -> Self {
        other
    }
}

impl<I, S> PartialEq for SimpleError<I, S> 
where
    I: Input,
    I::Token: PartialEq,
    S: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Unexpected(t1) => match other {
                Self::Unexpected(t2) => t1 == t2,
                _ => false
            }
            Self::Expected(msg1) => match other {
                Self::Expected(msg2) => msg1 == msg2,
                _ => false
            }
        }
    }
}

impl<I, S> fmt::Debug for SimpleError<I, S> 
where 
    I: Input,
    I::Token: fmt::Debug,
    S: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unexpected(t) => f.debug_tuple("Unexpected").field(&t).finish(),
            Self::Expected(msg) => f.debug_tuple("Expected").field(&msg).finish()
        }
    }
}

impl<I, S> fmt::Display for SimpleError<I, S> 
where
    I: Input,
    I::Token: fmt::Display,
    S: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Self::Unexpected(Some(token)) => write!(f, "unexpected {}", token),
           Self::Unexpected(None) => write!(f, "end of input"),
           Self::Expected(message) => write!(f, "{}", message)
        }        
    }
}

impl<I, S> error::Error for SimpleError<I, S> 
where 
    I: Input + fmt::Display + fmt::Debug,
    I::Token: fmt::Display + fmt::Debug, 
    S: fmt::Debug + fmt::Display 
    {}


pub struct MultiError<I, S> 
where
    I: Input,
    I::Token: AsChar 
{
    pub pos: Pos,
    pub errors: Vec<SimpleError<I, S>>
}

impl<I, S> ParseError<State<I>, S> for MultiError<I, S> 
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
    
    fn expect(message: S, input: State<I>) -> Self {
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
            // Ordering::Equal => {
            //     self.errors.append(&mut other.errors);
            //     self
            // }
            // Ordering::Greater => self,
            // Ordering::Less => other
            Ordering::Equal => self,
            Ordering::Greater => {
                self.errors.append(&mut other.errors);
                self
            },
            Ordering::Less => {
                self.errors.append(&mut other.errors);
                self
            }
        }
    }
}

impl<I, S> PartialEq for MultiError<I, S> 
where
    I: Input,
    I::Token: AsChar + PartialEq,
    S: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.errors == other.errors
    }
}

impl<I, S> fmt::Debug for MultiError<I, S> 
where
    I: Input,
    I::Token: AsChar + fmt::Debug,
    S: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiError")
            .field("pos", &self.pos)
            .field("errors", &self.errors)
            .finish()
    }
}

impl<I, S> fmt::Display for MultiError<I, S> 
where
    I: Input,
    I::Token: AsChar + fmt::Debug + fmt::Display,
    S: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at line:{}, column:{}", self.pos.line(), self.pos.column())?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl<I, S> error::Error for MultiError<I, S> 
where 
    I: Input + fmt::Display + fmt::Debug,
    I::Token: AsChar + fmt::Display + fmt::Debug, 
    S: fmt::Display + fmt::Debug 
    {}