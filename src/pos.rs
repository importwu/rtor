#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    line: usize,
    column: usize
}

impl Pos {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
        }
    }

    pub fn move_by(&mut self, ch: char) {
        self.column += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }
}