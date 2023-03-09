use std::mem::forget;
use std::ops::{Deref, DerefMut};

use crate::Input;

#[derive(Debug, Clone, Copy)]
pub enum CursorState<Pos> {
    Rollback(Pos),
    Commit
}

pub struct CursorGuard<'a, I: Input> {
    input: &'a mut I,
    pos: I::Pos
}

impl<'a, I: Input> CursorGuard<'a, I> {
    fn new(input: &'a mut I) -> Self {
        Self {
            pos: input.pos(),
            input
        }
    }

    #[inline]
    pub fn rollback(self) {
        self.input.update(CursorState::Rollback(self.pos));
        forget(self);
    }

}

impl<'a, I: Input> Drop for CursorGuard<'a, I> {
    #[inline]
    fn drop(&mut self) {
        self.input.update(CursorState::Commit)
    }
}

impl<'a, I: Input> Deref for CursorGuard<'a, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.input
    }
}

impl<'a, I: Input> DerefMut for CursorGuard<'a, I> { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.input
    }
}