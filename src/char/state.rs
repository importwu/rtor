use std::str::Chars;

#[derive(Debug, Clone)]
pub struct State<'a> {
    source: Chars<'a>,
    pos: Pos
}

impl<'a> State<'a> {

    pub fn new(str: &'a str) -> Self {
        Self {
            source: str.chars(),
            pos: Pos::new()
        }
    }
}

impl<'a> Iterator for State<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.source.next()?;
        self.pos.move_by(ch);
        Some(ch)
    }
}

impl<'a> Input for State<'a> {
    type Source = &'a str;
    type Pos = Pos;

    fn as_source(&self) -> Self::Source {
        self.source.as_str()
    }

    fn pos(&self) -> Self::Pos {
        self.pos
    }
}