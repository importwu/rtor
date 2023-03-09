use crate::traits::Input;
// use crate::cursor::{Cursor, CursorGuard};

use super::position::Position;

pub struct StaticInput<'a> {
    str: &'a str,
    pos: Position,
    offset: usize,
    msgs: Vec<String>
 }
 
 impl<'a> StaticInput<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            pos: Position::start(),
            offset: 0,
            msgs: Vec::new()
        }
     }

     pub fn pos(&self) -> Position {
        self.pos
     }
 }


 impl<'a> Iterator for StaticInput<'a> {
     type Item = char;
 
     fn next(&mut self) -> Option<Self::Item> {
        let ch = self.str[self.offset..].chars().next()?;
        self.offset += char::len_utf8(ch);

        self.pos.forward(ch);

        Some(ch)
     }
 }
 
 type Offset = usize;

//  impl<'a> Input for StaticInput<'a> {
 
//     type Pos = (Offset, Position);
//     type Msg = String;

//     fn cursor(&mut self) -> CursorGuard<Self> {
//         CursorGuard::new(self, (self.offset, self.pos))
//     }
     
//     fn restore_callback(&mut self, cursor: Cursor<Self::Pos>) {
//         (self.offset, self.pos) = cursor.pos();
//     }
    
//     fn commit_callback(&mut self, _cursor: Cursor<Self::Pos>) {}

//     fn report(&mut self, msg: Self::Msg) {
//         self.msgs.push(msg)
//     }

//     fn finish(self) -> Vec<Self::Msg> {
//         self.msgs
//     }
//  }


use crate::combinators::{between, sepby, pure};
use crate::text::{char, digit, token, string};
use crate::Parser;
 #[test]
pub fn test() {

  
}
