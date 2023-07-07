mod parser;
mod error;
pub mod char;
pub mod combinator;
pub mod token;
mod input;

pub use self::{
    error::{
        SimpleError,
        MultiError,
        ParseError
    },
    parser::Parser,
    input::{
        Input,
        Pos,
        State
    },

};

pub type ParseResult<O, I, E = SimpleError<<I as Input>::Token>> = Result<(O, I), E>;

pub trait AsChar {
    fn as_char(&self) -> char;
}

impl AsChar for u8 {
    fn as_char(&self) -> char {
        *self as char
    }
}

impl AsChar for char {
    fn as_char(&self) -> char {
        *self
    }
}

pub trait FindToken<T> {

    fn find_token(&self, token: &T) -> bool;
}

impl<'a> FindToken<char> for &'a str {
    fn find_token(&self, token: &char) -> bool {
        self.chars().any(|x| x == *token)
    }
}

impl<'a> FindToken<u8> for &'a str {
    fn find_token(&self, token: &u8) -> bool {
        self.chars().any(|x| x == *token as char)
    }
}

impl<T: PartialEq, const N: usize> FindToken<T> for [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a, T: PartialEq, const N: usize> FindToken<T> for &'a [T; N] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

impl<'a,  T: PartialEq> FindToken<T> for &'a [T] {
    fn find_token(&self, token: &T) -> bool {
        self.iter().any(|x| x == token)
    }
}

pub trait Alt<I, E> 
where
    I: Input,
    E: ParseError<I>
{
    type Output;
    
    fn choice(&mut self, input: I) -> Result<(Self::Output, I), E>;
}