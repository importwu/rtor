use std::str::Chars;

use crate::Pos;


#[derive(Debug)]
pub struct State<'a, U> {
    inner: Chars<'a>,
    pos: Pos,
    pub udata: U
}

impl<'a, U> State<'a, U> {

    pub fn with_udata(str: &'a str, udata: U) -> Self {
        Self { 
            inner: str.chars(), 
            pos: Pos::new(), 
            udata
        }
    }

    #[inline]
    pub fn pos(&self) -> Pos {
        self.pos
    }
}

impl<'a> State<'a, ()> {

    pub fn new(str: &'a str) -> Self {
        Self::with_udata(str, ())
    }
}


impl<'a, U: Clone> Clone for State<'a, U> {

    fn clone(&self) -> Self {
        Self { 
            inner: self.inner.clone(), 
            pos: self.pos, 
            udata: self.udata.clone() 
        }
    }
}

impl<'a, U> Iterator for State<'a, U> {

    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.inner.next()?;
        self.pos.move_by(ch);
        Some(ch)
    }
}
