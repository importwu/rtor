#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    column: usize
}

impl Position {
    pub fn start() -> Self {
        Self {
            line: 1,
            column: 1
        }
    }

    pub fn forward(&mut self, split: char) {
        if split == '\n' {
            self.line += 1;
            self.column = 1;
        }else {
            self.column += 1;
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.line
    }

}