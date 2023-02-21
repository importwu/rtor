use std::ops;
use std::ptr::NonNull;

use crate::traits::Input;

#[derive(Debug)]
pub struct CursorGuard<I: Input> {
    input: NonNull<I>,
    cursor: Cursor<I::Pos>,
    is_restore: bool
}

#[derive(Debug, Clone, Copy)]
pub struct Cursor<P>(P);

impl<P: Copy> Cursor<P> {
    #[inline]
    pub fn pos(&self) -> P {
        self.0
    } 
}

impl<P> ops::Deref for Cursor<P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> ops::DerefMut for Cursor<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: Input> CursorGuard<I> {

    pub fn new(state: &I, pos: I::Pos) -> Self {
        Self {
            input: NonNull::new(state as *const _ as *mut _).unwrap(),
            cursor: Cursor(pos),
            is_restore: false
        }
    }

    pub fn restore(&mut self) {
        unsafe {
            self.input.as_mut().restore_callback(self.cursor)
        }
        self.is_restore = true
    }
}

impl<I: Input> Drop for CursorGuard<I> {
    fn drop(&mut self) {
        if !self.is_restore {
            unsafe {
                self.input.as_mut().commit_callback(self.cursor)
            }
        }
    }
}