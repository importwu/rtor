use std::io::{Error, Read, ErrorKind};
use std::fmt;
use std::ptr::copy_nonoverlapping;
use std::ops::Index;

#[derive(Debug)]
pub struct Buffer {
    buf: Vec<Result<char, DecodeUtf8Error>>,
    cap: usize,
    head: usize,
    tail: usize,
    len: usize
}

impl Buffer {
    fn with_capacity(cap: usize) -> Self {
        let mut buf = Vec::with_capacity(cap);
        unsafe { buf.set_len(cap) }

        Self {
            buf,
            cap,
            head: 0,
            tail: 0,
            len: 0
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    fn push_back(&mut self, value: Result<char, DecodeUtf8Error>) {
        if self.len == self.cap { self.grow() }

        self.buf[self.tail] = value;
        self.tail = (self.tail + 1) % self.cap;
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<Result<char, DecodeUtf8Error>> {
        if self.is_empty() { return None }

        let value = self.buf[self.head];
        self.head = (self.head + 1) % self.cap;
        self.len -= 1;

        Some(value)
    }

    #[inline]
    fn consume(&mut self, count: usize) {
        let count = if count >= self.len { self.len } else { count };

        self.head = (self.head + count) % self.cap;
        self.len -= count;
    }

    fn grow(&mut self) {
        let new_cap = if self.cap < 1024 {
            self.cap * 2
        }else {
            self.cap + self.cap / 4
        };

        let mut new_buf = Vec::with_capacity(new_cap);
        unsafe { new_buf.set_len(new_cap) }

        let count = self.cap - self.head;
        
        unsafe {
            copy_nonoverlapping(
                self.buf.as_ptr().add(self.head), 
                new_buf.as_mut_ptr(), 
                count
            );

            copy_nonoverlapping(
                self.buf.as_ptr(), 
                new_buf.as_mut_ptr().add(count), 
                self.tail
            );
        }

        self.buf = new_buf;
        self.head = 0;
        self.tail = self.cap;
        self.cap = new_cap;
    }
}

impl Index<usize> for Buffer {
    type Output = Result<char, DecodeUtf8Error>;

    fn index(&self, index: usize) -> &Self::Output {
        let index = (self.head + index) % self.cap;
        &self.buf[index]
    }
}

pub struct Utf8Stream<R> {
    decode_utf8: DecodeUtf8<R>,
    peeked: Buffer
}

impl<R: Read> Utf8Stream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            decode_utf8: DecodeUtf8::new(reader),
            peeked: Buffer::with_capacity(1)
        }
    }

    pub fn peeked(&self) -> &Buffer{
        &self.peeked
    }

    pub fn peeked_mut(&mut self) -> &mut Buffer{
        &mut self.peeked
    }

    pub fn peek(&mut self, count: usize) -> Option<Result<char, Utf8StreamError>> {
        let peeked_len = self.peeked.len();

        if count < peeked_len {
            match self.peeked[count] {
                Ok(t) => return Some(Ok(t)),
                Err(e) => return Some(Err(Utf8StreamError::DecodeUtf8Error(e)))
            }
        }

        for _ in peeked_len..count + 1 {
            let r = self.decode_utf8.next()?;
            match r {
                Ok(t) => {
                    self.peeked.push_back(Ok(t));
                    continue;
                },
                Err(Utf8StreamError::DecodeUtf8Error(e)) => {
                    self.peeked.push_back(Err(e));
                    continue;
                }
                Err(e) => return Some(Err(e))
            }
        }

        match self.peeked[count] {
            Ok(t) => return Some(Ok(t)),
            Err(e) => return Some(Err(Utf8StreamError::DecodeUtf8Error(e)))
        }
    }
}

impl<R: Read> Iterator for Utf8Stream<R> {
    type Item = Result<char, Utf8StreamError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_empty() { 
            return self.decode_utf8.next();
        }

        let peeked = self.peeked.pop_front()?;

        match peeked {
            Ok(t) => Some(Ok(t)),
            Err(e) => Some(Err(Utf8StreamError::DecodeUtf8Error(e)))
        }
    }
}

struct Bytes<R> {
    reader: R,
    buf: Vec<u8>,
    filled: usize,
    pos: usize
}

impl<R: Read> Bytes<R> {

    fn new(reader: R) -> Self {
        let mut buf = Vec::with_capacity(1024);
        unsafe { buf.set_len(1024) }

        Self {
            reader,
            buf,
            filled: 0,
            pos: 0
        }
    }
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos < self.filled {
                break;
            }

            match self.reader.read(&mut self.buf) {
                Ok(0) => return None,
                Ok(n) => {
                    self.filled = n;
                    self.pos = 0;
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => return Some(Err(e))
            }
        }

        let v = self.buf[self.pos];
        self.pos += 1;
        Some(Ok(v))
    }
}

#[derive(Clone, Copy)]
pub struct DecodeUtf8Error {
    invalid_bytes: [u8; 4],
    err_len: u8
}

impl DecodeUtf8Error {
    fn invalid_bytes(&self) -> &[u8] {
        &self.invalid_bytes[0..self.err_len as usize]
    }
}

impl fmt::Debug for DecodeUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("invalid utf8 byte sequence {:?}", self.invalid_bytes()))
    }
}   

#[derive(Debug)]
pub enum Utf8StreamError {
    DecodeUtf8Error(DecodeUtf8Error),
    IoError(Error)
}

struct DecodeUtf8<R> {
    bytes: Bytes<R>,
    buf: Option<u8>
}

impl<R: Read> DecodeUtf8<R> {
    fn new(reader: R) -> Self {
        Self {
            bytes: Bytes::new(reader),
            buf: None
        }
    }
}

impl<R: Read> Iterator for DecodeUtf8<R> {
    type Item = Result<char, Utf8StreamError>;

    fn next(&mut self) -> Option<Self::Item> {
        let x = match self.buf.take() {
            Some(t) => t,
            None => 
                match self.bytes.next() {
                    Some(r) => match r {
                        Ok(t) => t,
                        Err(e) => return Some(Err(Utf8StreamError::IoError(e)))
                    }
                    None => return None
                }
        };
                   
        let next_cont_byte = || {

            let mut invalid_bytes = [x, 0, 0, 0];
    
            let mut index = 1;

            move || match self.bytes.next() {
                Some(r) => match r {
                    Ok(t) if t < 0x80 || t > 0xBF  => {
                        self.buf = Some(t);
                        return  Err(Utf8StreamError::DecodeUtf8Error(DecodeUtf8Error {invalid_bytes, err_len: index as u8 }))
                    }
                    Ok(t) => {
                        invalid_bytes[index] = t;
                        
                        index += 1;
                        
                        Ok(t)
                    },
                    Err(e) => Err(Utf8StreamError::IoError(e))
                }
                None => Err(Utf8StreamError::DecodeUtf8Error(DecodeUtf8Error {invalid_bytes, err_len: index as u8 }))
            }
        };

        let mut next_cont_byte = next_cont_byte();  

        macro_rules! try_cont_byte {
            () => {
                match next_cont_byte() {
                    Ok(b) => b,
                    Err(e) => return Some(Err(e))
                }
            };
        }

        match x {
            (0..=0x7F) => Some(Ok(x as char)),
            (0xC2..=0xDF) => {
                let point = (x & 0x1F) as u32;

                let y = try_cont_byte!();
                
                let point = acc_codepoint(point, y);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            (0xE0..=0xEF) => {
                let point = (x & 0xF) as u32;

                let y = try_cont_byte!();
                
                let point = acc_codepoint(point, y);

                let z = try_cont_byte!();
                
                let point = acc_codepoint(point, z);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            (0xF0..=0xF4) => {
                let point = (x & 0x7) as u32;

                let y = try_cont_byte!();

                let point = acc_codepoint(point, y);
                
                let z = try_cont_byte!();

                let point = acc_codepoint(point, z);

                let w = try_cont_byte!();

                let point = acc_codepoint(point, w);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            _ => Some(Err(Utf8StreamError::DecodeUtf8Error(DecodeUtf8Error {invalid_bytes: [x, 0, 0, 0], err_len: 1 })))
        }
    }
}

#[inline]
fn acc_codepoint(point: u32, cont: u8) -> u32 {
    (point << 6) | (cont as u32 & 0x3F)
}

#[test]
fn test() {

    let mut stream = Utf8Stream::new(b"Hello \xF0\x90\x80World".as_slice());

    println!("{:?}", stream.peek(1));
    println!("{:?}", stream.peeked_mut().consume(1));
    println!("{:?}", stream.peek(1));


    println!("{:?}", stream.peeked())
}