use std::{ptr, mem};
use std::ops::Deref;

pub struct BBStackBase<T> {
    stack: Vec<T>
}

pub struct BBStackElement<'a, T: 'a> {
    stack: &'a mut Vec<T>,
    el_index: usize
}

impl<T> BBStackBase<T> {
    #[inline]
    pub fn new() -> BBStackBase<T> {
        BBStackBase {
            stack: Vec::new()
        }
    }

    #[inline]
    pub fn with_capacity(cap: usize) -> BBStackBase<T> {
        BBStackBase {
            stack: Vec::with_capacity(cap)
        }
    }

    #[inline]
    pub fn push(&mut self, item: T) -> BBStackElement<T> {
        self.stack.push(item);
        BBStackElement {
            stack: &mut self.stack,
            el_index: 0
        }
    }
}

impl<'a, T> BBStackElement<'a, T> {
    #[inline]
    pub fn push(&mut self, item: T) -> BBStackElement<T> {
        self.stack.push(item);
        BBStackElement {
            stack: &mut self.stack,
            el_index: self.el_index + 1
        }
    }

    #[inline]
    pub fn top(&self) -> &T {
        unsafe{ self.stack.get_unchecked(self.stack.len() - 1) }
    }

    #[inline]
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

impl<'a, T> Deref for BBStackElement<'a, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &*self.stack
    }
}

impl<'a, T> AsRef<[T]> for BBStackElement<'a, T> {
    fn as_ref(&self) -> &[T] {
        &*self
    }
}

impl<'a, T> Drop for BBStackElement<'a, T> {
    fn drop(&mut self) {
        self.stack.pop();
        debug_assert_eq!(self.el_index, self.stack.len());
    }
}
