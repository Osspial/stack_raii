use std::{ptr, mem};

pub struct StackBase<T> {
    stack: Vec<T>
}

pub struct StackElement<'a, T: 'a> {
    stack: &'a mut Vec<T>,
    el_index: usize
}

impl<T> StackBase<T> {
    #[inline]
    pub fn new() -> StackBase<T> {
        StackBase {
            stack: Vec::new()
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> StackBase<T> {
        StackBase {
            stack: Vec::with_capacity(cap)
        }
    }
}

impl<'a, T> StackElement<'a, T> {
    pub fn top(&self) -> &T {
        unsafe{ self.stack.get_unchecked(self.stack.len() - 1) }
    }

    pub fn top_mut(&mut self) -> &mut T {
        let top_index = self.stack.len() - 1;
        unsafe{ self.stack.get_unchecked_mut(top_index) }
    }

    pub fn pop(self) -> T {
        unsafe {
            let top = self.top() as *const _;
            let top_removed = ptr::read(top);

            let new_len = self.stack.len() - 1;
            self.stack.set_len(new_len);

            mem::forget(self);

            top_removed
        }
    }
}

pub trait Stack {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
    fn push(&mut self) -> StackElement<Self::Item>;
}

impl<'a, T> Drop for StackElement<'a, T> {
    fn drop(&mut self) {
        self.stack.pop();
        debug_assert_eq!(self.el_index, self.stack.len());
    }
}
