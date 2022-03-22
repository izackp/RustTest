
use core::ops::{self};
use core::slice::{self};
use std::vec::IntoIter;

pub struct DeferedVec<T> {
    pub container:Vec<T>,
    pub to_add:Vec<T>,
    pub to_remove:Vec<usize>
}

impl<T> DeferedVec<T> {
    pub const fn new() -> Self {
        DeferedVec { container: Vec::new(), to_add: Vec::new(), to_remove: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DeferedVec { container: Vec::with_capacity(capacity), to_add: Vec::new(), to_remove: Vec::new() }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn add(&mut self, value: T) {
        self.to_add.push(value)
    }

    #[inline]
    pub fn remove(&mut self, index:usize) {
        self.to_remove.push(index)
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn push(&mut self, value: T) {
        self.container.push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.container.pop()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.container.len()
    }

    pub fn process_changes(&mut self) {
        //Assumes to remove is in order
        while let Some(index) = self.to_remove.pop() {
            self.container.swap_remove(index);
        }

        while let Some(value) = self.to_add.pop() {
            self.container.push(value);
        }
    }
}

impl<T> ops::Deref for DeferedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.container.deref()
    }
}

impl<T> ops::DerefMut for DeferedVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.container.deref_mut()
    }
}

impl<T> IntoIterator for DeferedVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        self.container.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DeferedVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DeferedVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.container.iter_mut()
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for DeferedVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeferedVec")
         .field(&self.container)
         .field(&self.to_add)
         .field(&self.to_remove)
         .finish()
    }
}
