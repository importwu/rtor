use std::fs::File;
use std::io::{Write, Read, BufReader};
use std::mem::forget;
use std::ops::{Deref, DerefMut, Index};

use rtor::utf8_stream::Utf8Stream;


fn main() {

    // let mut input = InStream::new("aaaasdasd".as_bytes());

    let mut input = InStr::new("aaaabc\nde\nfgh");

    let mut cursor = input.cursor();

    cursor.consume_while(|ch| *ch != 'a');

    println!("{:?}", cursor.next());
    println!("{:?}", cursor.next());

    drop(cursor);

    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());
    println!("{:?}", input.next());

    println!("{:?}", input.pos());


}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    offset: usize,
    line: usize,
    column: usize
}

impl Pos {
    fn new() -> Self {
        Self {
            offset: 0,
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
            _ => {
                self.column += 1;
            }
        }

        self.offset += v.len_utf8();
    }

    #[inline]
    fn offset(&self) -> usize {
        self.offset
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
    Restore(Pos),
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
                Some(ch) => {
                    if !pred(&ch) {
                        continue;
                    }
                    cursor.restore();
                    return;
                }
                None => return
            }
        }
    }
}

struct InStr<'a> {
    src: &'a str,
    pos: Pos
}

impl<'a> InStr<'a> {

    fn new(src: &'a str) -> Self {
        Self { 
            src,
            pos: Pos::new()
        }
    }
}

impl<'a> Input for InStr<'a> {

    type Item = char;
    type Pos = Pos;

    #[inline]
    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized {
        CursorGuard::new(self)
    }

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.src[self.pos.offset()..].chars().next()?;
        self.pos.advance(ch);
        Some(ch)
    }

    #[inline]
    fn update(&mut self, cursor: CursorState<Self::Pos>) {
        match cursor {
            CursorState::Restore(pos) => {
                self.pos = pos;
            }
            _ => {}
        }        
    }

    #[inline]
    fn pos(&self) -> Self::Pos {
        self.pos
    }
}


struct InStream<R: Read> {
    src: Utf8Stream<R>,
    pos: Pos,
    cursor_count: usize
}

impl<R: Read> InStream<R> {

    fn new(reader: R) -> Self {
        Self::with_capacity(5, reader)
    }

    fn with_capacity(cap: usize, reader: R) -> Self {
        Self {
            src: Utf8Stream::new(reader),
            pos: Pos::new(),
            cursor_count: 0
        }
    }

}

// impl<R: Read> Input for InStream<R> {

//     type Item = char;
//     type Pos = Pos;

//     fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized {
//         self.cursor_count += 1;
//         CursorGuard::new(self)
//     }

//     fn next(&mut self) -> Option<Self::Item> {
//         let ch = self.src.offset(self.pos.offset()).next()?;
//         self.pos.advance(ch);
//         Some(ch)
//     }

//     fn update(&mut self, cursor: CursorState<Self::Pos>) {
//         self.cursor_count -= 1;
//         match cursor {
//             CursorState::Restore(pos) => {
//                 self.pos = pos;
//                 if self.cursor_count == 0 {
//                     self.src.offset(pos.offset()).compact();
//                 }
//             }
//             CursorState::Commit => {
//                 if self.cursor_count == 0 { 
//                     self.src.offset(self.pos.offset()).compact()
//                 }
//             }   
//         } 
//     }

//     #[inline]
//     fn pos(&self) -> Self::Pos {
//         self.pos
//     }
// }

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
    pub fn restore(self) {
        self.input.update(CursorState::Restore(self.pos));
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