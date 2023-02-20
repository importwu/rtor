use std::io::{Read, self, ErrorKind};
use std::ptr::NonNull;
use std::alloc::{Layout, alloc, dealloc};
use std::slice;

struct Buffer {
    ptr: NonNull<u8>,
    filled: usize,
    cap: usize,
    pos: usize,
}

impl Buffer {
    fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<u8>(cap).unwrap();
        let ptr = unsafe { alloc(layout) };
    
        Self {
            ptr: NonNull::new(ptr).unwrap(),
            filled: 0,
            cap,
            pos: 0
        }
    }

    fn fill<R: Read>(&mut self, mut reader: R) -> io::Result<usize> {
        let n = reader.read(self.as_mut())?;
        self.filled = n;
        self.pos = 0;
        Ok(n)
    }

    fn consume(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None
        }

        let ret = self.buf_read(self.pos);
        self.pos += 1;
        Some(ret)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        return self.pos == self.filled
    }

    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.cap) }
    }

    #[inline]
    fn buf_read(&self, index: usize) -> u8 {
        unsafe {
            self.ptr.as_ptr().add(index).read()
        }
    }
}


impl Drop for Buffer {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.cap).unwrap();
        unsafe { dealloc(self.ptr.as_ptr(), layout) }
    }
}

pub struct ByteReader<R> {
    reader: R,
    buf: Buffer,
    is_eof: bool
}

impl<R: Read> ByteReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Buffer::with_capacity(1024),
            is_eof: false
        }
    }
}

impl<R: Read> Iterator for ByteReader<R> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        
        loop {
            if self.is_eof { return None }

            match self.buf.consume() {
                Some(b) => return Some(b),
                None => {
                    match self.buf.fill(&mut self.reader) {
                        Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                        Err(_) => {
                            self.is_eof = true;
                            return None
                        }
                        Ok(n) if n == 0 => {
                            self.is_eof = true;
                            return None
                        }
                        Ok(_) => continue
                    }
                }
            }
        }
    }
}


#[test]
fn test() {
    let bs = "abcdefg".as_bytes();
    let bytes = ByteReader::new(bs);

    for b in bytes {
        println!("{}", b)
    }
}
