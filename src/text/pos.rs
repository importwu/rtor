#[derive(Debug, Clone, Copy)]
pub struct Pos {
    line: usize,
    column: usize
}

impl Pos {
    fn new() -> Self {
        Self {
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
            _ => self.column += 1,
        }
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