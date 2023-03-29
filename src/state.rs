use std::str::Chars;

use crate::Pos;


#[derive(Debug)]
pub struct State<'a> {
    inner: Chars<'a>,
    pos: Pos,
}

impl<'a> State<'a> {

    pub fn new(str: &'a str) -> Self {
        Self { 
            inner: str.chars(), 
            pos: Pos::new()
        }
    }

    #[inline]
    pub fn pos(&self) -> Pos {
        self.pos
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}


impl<'a> Clone for State<'a> {

    fn clone(&self) -> Self {
        Self { 
            inner: self.inner.clone(), 
            pos: self.pos
        }
    }
}

impl<'a> Iterator for State<'a> {

    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inner.next()?;
        self.pos.move_by(ch);
        Some(ch)
    }
}
