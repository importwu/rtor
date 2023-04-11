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

pub type ParseResult<O, I> = Result<(O, I), Error<<I as Input>::Item>>;

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

pub trait FindItem<T> {

    fn find_item(&self, item: T) -> bool;
}

impl<'a> FindItem<char> for &'a str {
    fn find_item(&self, item: char) -> bool {
        self.chars().any(|x| x == item)
    }
}

impl<'a> FindItem<u8> for &'a str {
    fn find_item(&self, item: u8) -> bool {
        self.chars().any(|x| x == item as char)
    }
}

impl<const N: usize> FindItem<u8> for [u8; N] {
    fn find_item(&self, item: u8) -> bool {
        self.iter().any(|x| *x == item)
    }
}

impl<'a, const N: usize> FindItem<u8> for &'a [u8; N] {
    fn find_item(&self, item: u8) -> bool {
        self.iter().any(|x| *x == item)
    }
}