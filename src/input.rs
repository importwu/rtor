pub trait Input: Clone + Iterator {
    type Source;
    type Pos;

    fn as_source(&self) -> Self::Source;

    fn pos(&self) -> Self::Pos;
}