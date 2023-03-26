use crate::Pos;

use std::{
    cmp::Ordering,
    fmt
};

#[derive(Debug)]
pub struct ParseError {
    pub pos: Pos,
    pub unexpect: Option<String>,
    pub expect: Vec<String>
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


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "parse error at line: {}, column: {}.", self.pos.line(), self.pos.column())?;

        let unexpect = self.unexpect.as_deref().unwrap_or("<eof>");

        match self.expect.len() {
            0 => writeln!(f, "expect \"{}\", but found \"{}\".", "<unknow>", unexpect),
            1 => writeln!(f, "expect \"{}\", but found \"{}\".", self.expect[0], unexpect),
            n => {
                let mut expect = self.expect[..n - 1].join(",");
                expect.push_str(" or ");
                expect.push_str(&self.expect[n - 1]);
                writeln!(f, "expect \"{}\", but found \"{}\".", expect, unexpect)
            }
        }
    }
}
