#[derive(Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize
}

impl Position {
    pub fn start() -> Self {
        Self {
            line: 1,
            column: 0,
            offset: 0
        }
    }
}