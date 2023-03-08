use std::ptr::{NonNull, copy_nonoverlapping};
use std::ops;
use std::io::Read;
use std::alloc::{Layout, dealloc, alloc};
use crate::traits::Input;
use crate::cursor::{Cursor, CursorGuard};
use crate::decode::DecodeUtf8Lossy;

use super::position::Position;


impl<T: Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        let (ptr, layout) = self.current_memory();
        unsafe { dealloc(ptr.as_ptr().cast(), layout) }
    }
}

impl<T: Copy + std::fmt::Debug> std::fmt::Debug for Buffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let xs = unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.cap) };
        f.write_fmt(format_args!("{:?}\n", xs)).unwrap();
        f.write_fmt(format_args!("cap = {}\nlen = {}\nhead = {}\ntail = {}\n", self.cap, self.len, self.head, self.tail))
    }
}

impl<T: Copy> ops::Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "index of bound");
        let index = (self.head + index) % self.cap;
        unsafe {
            self.ptr.as_ptr().add(index).as_ref().unwrap_unchecked()
        }
    }
}

pub struct Buffer<T: Copy> {
    ptr: NonNull<T>,
    cap: usize,
    head: usize,
    tail: usize,
    len: usize,
}

impl<T: Copy> Buffer<T> {
    fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<T>(cap).unwrap();
        let ptr = unsafe { alloc(layout).cast::<T>() };

        Self {
            ptr: NonNull::new(ptr).unwrap(),
            cap,
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn is_full(&self) -> bool {
        return self.len == self.cap
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    fn push_back(&mut self, value: T) {
        if self.is_full() { self.grow() }

        self.buf_write(self.tail, value);
        let tail = self.tail + 1;
        self.tail = tail % self.cap;
        self.len += 1;
    }

    fn truncate_front(&mut self, end: usize) {
        assert!(end <= self.len, "truncate too long");
        self.head = (self.head + end) % self.cap;
        self.len -= end;
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() { return None }

        let value = self.buf_read(self.head);
        let head = self.head + 1;
        self.head = head % self.cap;
        self.len -= 1;

        Some(value)
    }

    fn grow(&mut self) {
        let (old_ptr, old_layout) = self.current_memory();

        let new_cap = self.cap * 2;
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let new_ptr = unsafe { alloc(new_layout).cast::<T>() };

        let count = self.cap - self.head;

        unsafe {
            copy_nonoverlapping(
                old_ptr.as_ptr().add(self.head), 
                new_ptr, 
                count
            );

            copy_nonoverlapping(
                old_ptr.as_ptr(), 
                new_ptr.add(count), 
                self.tail
            );
        }

        unsafe { dealloc(old_ptr.as_ptr().cast(), old_layout) }

        self.ptr = NonNull::new(new_ptr).unwrap();
        self.head = 0;
        self.tail = self.cap;
        self.cap = new_cap;
    }

    fn current_memory(&self) -> (NonNull<T>, Layout) {
        let layout = Layout::array::<T>(self.cap).unwrap();
        (self.ptr, layout)
    }

    #[inline]
    fn buf_write(&mut self, index: usize, value: T) {
        unsafe {
            self.ptr.as_ptr().add(index).write(value);
        }
    }

    #[inline]
    fn buf_read(&mut self, index: usize) -> T{
        unsafe {
            self.ptr.as_ptr().add(index).read()
        }
    }

}

pub struct StreamInput<R: Read> {
    decoder: DecodeUtf8Lossy<R>,
    pub buf: Buffer<char>,
    pos: Position,
    offset: usize,
    cursor_count: usize,
    msgs: Vec<String>
}

impl<R: Read> StreamInput<R> {
    pub fn new(reader: R) -> Self {
        Self {
            decoder: DecodeUtf8Lossy::new(reader),
            buf: Buffer::with_capacity(5),
            pos: Position::start(),
            offset: 0,
            cursor_count: 0,
            msgs: Vec::new()
        }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }
}

impl<R: Read> Iterator for StreamInput<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {

        let ch = {
            if self.cursor_count == 0 {
                match self.buf.pop_front() {
                    Some(ch) => ch,
                    None => self.decoder.next()?
                }
            }else {
                if self.offset == self.buf.len() {
                    let ch = self.decoder.next()?;
                    self.buf.push_back(ch);
                    self.offset += 1;
                    ch
                }else {
                    let ch = self.buf[self.offset];
                    self.offset += 1;
                    ch
                }
            }
        };

        self.pos.forward(ch);

        Some(ch)
    }
}

type Offset = usize;

impl<R: Read> Input for StreamInput<R> {
    type Pos = (Offset, Position);
    type Msg = String;

    fn cursor(&mut self) -> CursorGuard<Self> where Self: Sized {
        self.cursor_count += 1;
        CursorGuard::new(self, (self.offset, self.pos))
    }

    fn restore_callback(&mut self, cursor: Cursor<Self::Pos>) {
        (self.offset, self.pos) = cursor.pos();
        self.cursor_count -= 1;
        if self.cursor_count == 0 {
            self.buf.truncate_front(self.offset);
            self.offset = 0;
        }
    }

    fn commit_callback(&mut self, _cursor: Cursor<Self::Pos>) {
        self.cursor_count -= 1;
        if self.cursor_count == 0 {
            self.buf.truncate_front(self.offset);
            self.offset = 0;
        }
    }

    fn report(&mut self, msg: Self::Msg) {
        self.msgs.push(msg)
    }

    fn finish(self) -> Vec<Self::Msg> {
        self.msgs
    }
}


use crate::combinators::{between, sepby};
use crate::text::{char, digit};
use crate::Parser;
#[test]
fn test() {
    let mut input = StreamInput::new("asd".as_bytes());

    let mut c = input.cursor();

    drop(input);

}
