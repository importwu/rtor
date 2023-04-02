use crate::Input;

#[derive(Debug)]
pub enum Error<I: Input> {
    Unexpected(I::Item),
    Eoi
}