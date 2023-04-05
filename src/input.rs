use std::{
    str::Chars,
    iter::Copied,
    slice::Iter
};



pub trait Input: Clone {
    type Item: Copy;
    // type Items: Iterator<Item = Self::Item>;
    type Inner;

    fn next(&mut self) -> Option<Self::Item>;

    fn peek(&mut self) -> Option<Self::Item>;

    fn diff(&self, other: &Self) -> Self;
    
    // fn items(&self) -> Self::Items;

    fn as_inner(&self) -> Self::Inner;
}


impl<'a> Input for &'a str {
    type Item = char;
    // type Items = Chars<'a>;
    type Inner = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.chars();
        let ch = chars.next()?;
        *self = chars.as_str();
        Some(ch)
    }

    fn peek(&mut self) -> Option<Self::Item> {
        self.chars().next()
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    fn as_inner(&self) -> Self::Inner {
        self
    }
    // fn items(&self) -> Self::Items {
    //     self.chars()
    // }
}

impl<'a> Input for &'a [u8] {
    type Item = u8;
    // type Items = Copied<Iter<'a, u8>>;
    type Inner = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.iter();
        let item = *iter.next()?;
        *self = iter.as_slice();
        Some(item)
    }

    fn peek(&mut self) -> Option<Self::Item> {
        self.iter().copied().next()
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    // fn items(&self) -> Self::Items {
    //     self.iter().copied()
    // }

    fn as_inner(&self) -> Self::Inner {
        self
    }
}


mod test {
    use super::*;

    #[test]
    fn test() {
        let mut a = "abcdef";
        let b = a;
        a.next();
        a.next();
        a.next();

        println!("{}", b.diff(&a));
        println!("{}", a);

    }
}