use core::slice;
use std::str::Chars;
use std::iter::Copied;

pub trait Input: Clone {
    type Item: Copy;
    type Items: Iterator<Item = Self::Item>;

    fn consume(&mut self) -> Option<Self::Item>;

    fn take_while<F>(&mut self, pred: F) -> Self where F: FnMut(&Self::Item) -> bool;
    
    fn items(&self) -> Self::Items;
}

impl<'a> Input for &'a str {
    type Item = char;
    type Items = Chars<'a>;

    fn consume(&mut self) -> Option<Self::Item> {
        let mut chars = self.chars();
        let ch = chars.next()?;
        *self = chars.as_str();
        Some(ch)
    }

    fn take_while<F>(&mut self, mut pred: F) -> Self 
    where
        F: FnMut(&Self::Item) -> bool
    {
        let mut s = *self;
        let mut len = 0;

        loop {
            match s.consume() {
                None => break,
                Some(t) if pred(&t) => {
                    len += t.len_utf8();
                    continue
                },
                Some(_) => break
            }
        }

        let r = &self[..len];

        *self = &self[len..];

        r
    }

    fn items(&self) -> Self::Items {
        self.chars()
    }

}

impl<'a, I> Input for &'a [I] where I: Copy{
    type Item = I;
    type Items = Copied<slice::Iter<'a, Self::Item>>;

    fn consume(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None
        }

        let b = self[0];
        *self = &self[1..];
        Some(b)
    }

    fn take_while<F>(&mut self, mut pred: F) -> Self 
    where
        F: FnMut(&Self::Item) -> bool
    {
        let mut b = *self;
        let mut len = 0;

        loop {
            match b.consume() {
                None => break,
                Some(t) if pred(&t) => {
                    len += 1;
                    continue
                },
                Some(_) => break
            }
        }

        let r = &self[..len];

        *self = &self[len..];

        r
    }

    fn items(&self) -> Self::Items {
        self.iter().copied()
    }
}


mod test {
    use super::*;

    #[test]
    fn test() {
        let mut bs = "   abc";
        let i = bs.take_while(|x| *x == ' ');
        println!("{:?}", String::from_iter(i.items()));
    }
}