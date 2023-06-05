use std::{
    str::Chars, 
    slice::Iter, 
    iter::{
        Copied,
        Cloned
    }
};

use crate::{
    iter::Many, 
    Parser
};

pub trait Input: Clone {
    type Token: Copy;
    type Tokens: Iterator<Item = Self::Token>;

    fn next(&mut self) -> Option<Self::Token>;

    fn peek(&mut self) -> Option<Self::Token>;

    fn diff(&self, other: &Self) -> Self;

    fn tokens(&self) -> Self::Tokens;

    fn many<P>(&mut self, parser: P) -> Many<Self, P> 
    where
        P: Parser<Self>
    {
        Many::new(self, parser)
    }
}

impl<'a> Input for &'a str {
    type Token = char;
    type Tokens = Chars<'a>;

    fn next(&mut self) -> Option<Self::Token> {
        let mut chars = self.chars();
        let ch = chars.next()?;
        *self = chars.as_str();
        Some(ch)
    }

    fn peek(&mut self) -> Option<Self::Token> {
        self.chars().next()
    }

    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }

    fn tokens(&self) -> Self::Tokens {
        self.chars()
    }
}

// impl<'a> Input for &'a [u8] {
//     type Token = u8;
//     type Tokens = Copied<Iter<'a, u8>>;

//     fn next(&mut self) -> Option<Self::Token> {
//         let mut iter = self.iter();
//         let item = *iter.next()?;
//         *self = iter.as_slice();
//         Some(item)
//     }

//     fn peek(&mut self) -> Option<Self::Token> {
//         self.iter().copied().next()
//     }

//     fn diff(&self, other: &Self) -> Self {
//         let offset = other.as_ptr() as usize - self.as_ptr() as usize;
//         &self[..offset]
//     }

//     fn tokens(&self) -> Self::Tokens {
//         self.iter().copied()
//     }
// }

impl<'a, T: Clone> Input for &'a [T] {
    type Token = T;
    type Tokens = Cloned<Iter<'a, T>>;
    
    fn next(&mut self) -> Option<Self::Token> {
        let mut iter = self.iter();
        let item = iter.cloned().next()?;
        *self = iter.as_slice();
        Some(item)
    }
    
    fn peek(&mut self) -> Option<Self::Token> {
        self.iter().cloned().next()
    }
    
    fn diff(&self, other: &Self) -> Self {
        let offset = other.as_ptr() as usize - self.as_ptr() as usize;
        &self[..offset]
    }
    
    fn tokens(&self) -> Self::Tokens {
        self.iter().cloned()
    }
}

