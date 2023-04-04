use std::{
    str::Chars,
    iter::Copied,
    slice::Iter
};

pub trait Input: Clone {
    type Item: Copy;
    type Items: Iterator<Item = Self::Item>;

    fn next(&mut self) -> Option<Self::Item>;

    fn diff(&self, other: &Self) -> Self;
    
    fn items(&self) -> Self::Items;
}


impl<'a> Input for &'a str {
    type Item = char;
    type Items = Chars<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.chars();
        let ch = chars.next()?;
        *self = chars.as_str();
        Some(ch)
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    fn items(&self) -> Self::Items {
        self.chars()
    }
}

impl<'a> Input for &'a [u8] {
    type Item = u8;
    type Items = Copied<Iter<'a, u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iter = self.iter();
        let item = *iter.next()?;
        *self = iter.as_slice();
        Some(item)
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    fn items(&self) -> Self::Items {
        self.iter().copied()
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
        println!("{}", a)
    }
}