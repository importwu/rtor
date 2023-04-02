mod parser;

mod error;

pub mod primitive;

pub mod combine;

mod iter;

mod input;

pub use self::{
    error::Error,
    parser::Parser,
    iter::iterator,
    input::Input
};

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
        str::find(self, item).is_some()
    }
}

impl<I, const N: usize> FindItem<I> for [I; N] 
where
    I: PartialEq
{
    fn find_item(&self, item: I) -> bool {
        self.iter().find(|x| **x == item).is_some()
    }
}

impl<'a, I, const N: usize> FindItem<I> for &'a [I; N] 
where
    I: PartialEq
{
    fn find_item(&self, item: I) -> bool {
        self.iter().find(|x| **x == item).is_some()
    }
}