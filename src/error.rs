use crate::Pos;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct ParseError {
    pub(crate) pos: Pos,
    pub(crate) unexpect: Option<String>,
    pub(crate) expect: Vec<String>
}

impl ParseError {
    pub fn merge(mut self, mut other: Self) -> Self {
        if self.expect.is_empty() && !other.expect.is_empty() {
            return other
        }
        if !self.expect.is_empty() && other.expect.is_empty() {
            return self
        }
        match self.pos.cmp(&other.pos) {
            Ordering::Equal => {
                self.expect.append(&mut other.expect);
                self
            },
            Ordering::Greater => self,
            Ordering::Less => other
        }
    }
}
