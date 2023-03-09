use std::fs::File;
use std::io::{Write, Read, BufReader};
use std::mem::forget;
use std::ops::{Deref, DerefMut, Index};

use rtor::stream::{Utf8Stream, Utf8StreamError};


fn main() {

    let mut input = InStream::new("aaaabc\nde\nfgh".as_bytes());

    // let mut input = InStr::new("aaaabc\nde\nfgh");

    let mut cursor = input.cursor();

    // cursor.consume_while(|ch| *ch != 'a');

    cursor.consume_while(|ch| {
        match ch {
            Ok(ch) => *ch != 'a',
            Err(_) => false
        }
    });

    println!("{:?}", cursor.next());
    println!("{:?}", cursor.next());

    cursor.rollback();

    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());

    println!("{:?}", input.pos());

}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    line: usize,
    column: usize
}

impl Pos {
    fn new() -> Self {
        Self {
            line: 1,
            column: 1
        }
    }

    fn advance(&mut self, v: char) {
        match v {
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            _ => self.column += 1,
        }
    }

    #[inline]
    fn line(&self) -> usize {
        self.line
    }

    #[inline]
    fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CursorState<Pos> {
    Rollback(Pos),
    Commit
}

pub trait Input {

    type Item;
    type Pos: Copy;

    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized;

    #[doc(hidden)]
    fn update(&mut self, cursor: CursorState<Self::Pos>) where Self: Sized;

    fn pos(&self) -> Self::Pos;

    fn next(&mut self) -> Option<Self::Item>;

    fn consume_while<F>(&mut self, mut pred: F) 
        where F: FnMut(&Self::Item) -> bool,
            Self: Sized
    {
        loop {
            let mut cursor = self.cursor();
            match cursor.next() {
                Some(t) => {
                    if !pred(&t) {
                        continue;
                    }
                    cursor.rollback();
                    return;
                }
                None => return
            }
        }
    }
}

struct InStr<'a> {
    src: &'a str,
    pos: Pos,
    offset: usize
}

impl<'a> InStr<'a> {

    fn new(src: &'a str) -> Self {
        Self { 
            src,
            pos: Pos::new(),
            offset: 0
        }
    }
}

type Offset = usize;

impl<'a> Input for InStr<'a> {

    type Item = char;
    type Pos = (Offset, Pos);

    #[inline]
    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized {
        CursorGuard::new(self)
    }

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.src[self.offset..].chars().next()?;
        self.pos.advance(ch);
        self.offset += ch.len_utf8();
        Some(ch)
    }

    #[inline]
    fn update(&mut self, cursor: CursorState<Self::Pos>) {
        match cursor {
            CursorState::Rollback((offset, pos)) => {
                self.offset = offset;
                self.pos = pos;
            }
            _ => ()
        }        
    }

    #[inline]
    fn pos(&self) -> Self::Pos {
        (self.offset, self.pos)
    }
}


struct InStream<R> {
    src: Utf8Stream<R>,
    pos: Pos,
    offset: usize,
    cursor_count: usize
}

impl<R: Read> InStream<R> {

    fn new(reader: R) -> Self {
        Self {
            src: Utf8Stream::new(reader),
            pos: Pos::new(),
            offset: 0,
            cursor_count: 0
        }
    }

}

impl<R: Read> Input for InStream<R> {

    type Item = Result<char, Utf8StreamError>;
    type Pos = (Offset, Pos);

    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized {
        self.cursor_count += 1;
        CursorGuard::new(self)
    }

    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.cursor_count == 0 {
            self.src.next()
        }else {
            self.src.peek(self.offset)
        }?;

        match r {
            Ok(t) => {
                self.pos.advance(t);
                self.offset += 1;
                Some(Ok(t))
            }
            e@Err(Utf8StreamError::DecodeUtf8Error(_)) => {
                self.pos.advance('\u{FFFD}');
                self.offset += 1;
                Some(e)
            }   
            Err(e) => Some(Err(e))
        }
    }

    fn update(&mut self, cursor: CursorState<Self::Pos>) {
        self.cursor_count -= 1;
        match cursor {
            CursorState::Rollback((offset, pos)) => {
                self.pos = pos;
                self.offset = offset;

                if self.cursor_count == 0 {
                    self.src.peeked_mut().consume(offset);
                    self.offset = 0;
                }
            }
            CursorState::Commit => {
                if self.cursor_count == 0 { 
                    self.src.peeked_mut().consume(self.offset);
                    self.offset = 0;
                }
            }   
        } 
    }

    #[inline]
    fn pos(&self) -> Self::Pos {
        (self.offset, self.pos)
    }
}

pub struct CursorGuard<'a, I: Input> {
    input: &'a mut I,
    pos: I::Pos
}

impl<'a, I> CursorGuard<'a, I> 
    where I: Input,
        I::Pos: Copy
{
    fn new(input: &'a mut I) -> Self {
        Self {
            pos: input.pos(),
            input
        }
    }

    #[inline]
    pub fn rollback(self) {
        self.input.update(CursorState::Rollback(self.pos));
        forget(self);
    }

}

impl<'a, I: Input> Drop for CursorGuard<'a, I> {
    #[inline]
    fn drop(&mut self) {
        self.input.update(CursorState::Commit)
    }
}

impl<'a, I: Input> Deref for CursorGuard<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.input
    }
}

impl<'a, I: Input> DerefMut for CursorGuard<'a, I> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.input
    }
}