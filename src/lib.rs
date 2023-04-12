mod parser;

mod error;

pub mod primitive;

pub mod combine;

mod iter;

mod input;

pub use self::{
    error::Error,
    parser::Parser,
    input::Input
};

pub type ParseResult<O, I> = Result<(O, I), Error<<I as Input>::Token>>;

pub trait AsChar {
    fn as_char(&self) -> char;
}

impl AsChar for u8 {
    #[inline]
    fn as_char(&self) -> char {
        *self as char
    }
}

impl AsChar for char {
    #[inline]
    fn as_char(&self) -> char {
        *self
    }
}

pub trait FindToken<T> {

    fn find_token(&self, token: T) -> bool;
}

impl<'a> FindToken<char> for &'a str {
    fn find_token(&self, token: char) -> bool {
        self.chars().any(|x| x == token)
    }
}

impl<'a> FindToken<u8> for &'a str {
    fn find_token(&self, token: u8) -> bool {
        self.chars().any(|x| x == token as char)
    }
}

impl<const N: usize> FindToken<u8> for [u8; N] {
    fn find_token(&self, token: u8) -> bool {
        self.iter().any(|x| *x == token)
    }
}

impl<'a, const N: usize> FindToken<u8> for &'a [u8; N] {
    fn find_token(&self, token: u8) -> bool {
        self.iter().any(|x| *x == token)
    }
}