use std::io::Read;
use std::fmt;

use crate::bytereader::ByteReader;

#[derive(Clone, Copy)]
pub struct DecodeUtf8Error {
    bytes: [u8; 4],
    error_len: u8
}

impl DecodeUtf8Error {
    fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.error_len as usize]
    }
}

impl fmt::Display for DecodeUtf8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("invalid utf8 byte sequence {:?}", self.bytes()))
    }
}

impl fmt::Debug for DecodeUtf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}   

type DecodeUtf8Result = Result<char, DecodeUtf8Error>;

pub struct DecodeUtf8<R: Read> {
    bytes: ByteReader<R>,
    buf: Option<u8>
}

impl<R: Read> DecodeUtf8<R> {
    pub fn new(reader: R) -> Self {
        Self {
            bytes: ByteReader::new(reader),
            buf: None
        }
    }
}

impl<R: Read> Iterator for DecodeUtf8<R> {
    type Item = DecodeUtf8Result;

    fn next(&mut self) -> Option<Self::Item> {

        let x = match self.buf.take() {
            Some(b) => b,
            None => self.bytes.next()?
        };

        let ret_next_cont_byte = || {

            let mut bytes = [x, 0, 0, 0];

            let mut index = 1;

            move || {
                match self.bytes.next() {
                    Some(b) => {

                        if b < 0x80 || b > 0xBF {
                            self.buf = Some(b);
                            return  Err(DecodeUtf8Error { bytes, error_len: index as u8 })
                        }

                        bytes[index] = b;

                        index += 1;

                        Ok(b)
                    },
                    None => Err(DecodeUtf8Error { bytes, error_len: index as u8 })
                }
            }
        };

        let mut next_cont_byte = ret_next_cont_byte();  

        macro_rules! try_cont_byte {
            ($res: expr) => {
                match $res {
                    Ok(b) => b,
                    Err(e) => return Some(Err(e))
                }
            };
        }

        match x {
            (0..=0x7F) => Some(Ok(x as char)),
            (0xC2..=0xDF) => {
                let point = (x & 0x1F) as u32;

                let y = try_cont_byte!(next_cont_byte());
                
                let point = acc_codepoint(point, y);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            (0xE0..=0xEF) => {
                let point = (x & 0xF) as u32;

                let y = try_cont_byte!(next_cont_byte());
                
                let point = acc_codepoint(point, y);

                let z = try_cont_byte!(next_cont_byte());
                
                let point = acc_codepoint(point, z);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            (0xF0..=0xF4) => {
                let point = (x & 0x7) as u32;

                let y = try_cont_byte!(next_cont_byte());

                let point = acc_codepoint(point, y);
                
                let z = try_cont_byte!(next_cont_byte());

                let point = acc_codepoint(point, z);

                let w = try_cont_byte!(next_cont_byte());

                let point = acc_codepoint(point, w);

                Some(Ok(unsafe { char::from_u32_unchecked(point) }))
            }
            _ => Some(Err(DecodeUtf8Error { bytes: [x, 0, 0, 0], error_len: 1 }))
        }

    }
}

#[inline]
fn acc_codepoint(point: u32, cont: u8) -> u32 {
    (point << 6) | (cont as u32 & 0x3F)
}

pub struct DecodeUtf8Lossy<R: Read> {
    decoder: DecodeUtf8<R>
}

impl<R: Read> DecodeUtf8Lossy<R> {
    pub fn new(reader: R) -> Self {
        Self { decoder: DecodeUtf8::new(reader) }
    }
}

impl<R: Read> Iterator for DecodeUtf8Lossy<R> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.decoder.next()? {
            Ok(ch) => Some(ch),
            Err(_) => Some('\u{FFFD}')
        }
    }
}


#[test]
fn test() {
    // let a = b"Hello \xF0\x90\x80World";
    // let stream = decode_utf8(&a[..]);
    
    // for c in stream {
    //     println!("{:?}", c)
    // }


    
}