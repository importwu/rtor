use crate::traits::Input;
use crate::cursor::{Cursor, CursorGuard};

use super::position::Position;

pub struct StaticInput<'a> {
    str: &'a str,
    pos: Position,
    errors: Vec<()>
 }
 
 impl<'a> StaticInput<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            pos: Position::start(),
            errors: Vec::new()
        }
     }

     pub fn line(&self) -> usize {
        self.pos.line
     }

     pub fn column(&self) -> usize {
        self.pos.column
     }
 }


 impl<'a> Iterator for StaticInput<'a> {
     type Item = char;
 
     fn next(&mut self) -> Option<Self::Item> {
        let ch = self.str[self.pos.offset..].chars().next()?;

        self.pos.offset += char::len_utf8(ch);

        if ch == '\n' {
            self.pos.line += 1;
            self.pos.column = 1;
        }else {
            self.pos.column += 1;
        }

        Some(ch)
     }
 }
 

 impl<'a> Input for StaticInput<'a> {
 
    type Pos = Position;
    type Err = ();
    type Errs = Vec<()>;

    fn cursor(&mut self) -> CursorGuard<Self> {
        CursorGuard::new(self, self.pos)
    }
     
    fn restore_callback(&mut self, cursor: Cursor<Self::Pos>) {
        self.pos = cursor.pos();
    }
    
    fn commit_callback(&mut self, _cursor: Cursor<Self::Pos>) {}

    fn report_err(&mut self, err: Self::Err) {
        self.errors.push(err)
    }

    fn finish(self) -> Result<(), Self::Errs> {
        if self.errors.is_empty() {
            Ok(())
        }else {
            Err(self.errors)
        }
    }
 }


 #[test]
pub fn test() {
    let mut input = StaticInput::new("as\r\nd");
    let mut cursor = input.cursor();

    println!("ch {:?}, ln {},col {}", input.next(), input.line(), input.column());
    println!("ch {:?}, ln {},col {}", input.next(), input.line(), input.column());
    println!("ch {:?}, ln {},col {}", input.next(), input.line(), input.column());
    println!("ch {:?}, ln {},col {}", input.next(), input.line(), input.column());
    cursor.restore();
    println!("ch {:?}, ln {},col {}", input.next(), input.line(), input.column());
}
