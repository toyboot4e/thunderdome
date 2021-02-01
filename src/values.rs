use std::iter::{Enumerate, ExactSizeIterator, FusedIterator};
use std::slice;

use crate::arena::Entry;

/// See [`Arena::values`](crate::Arena::values).
pub struct ValuesIter<'a, T> {
    pub(crate) len: u32,
    pub(crate) inner: Enumerate<slice::Iter<'a, Entry<T>>>,
}

impl<'a, T> Iterator for ValuesIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.len == 0 {
                return None;
            }

            match self.inner.next()? {
                (_, Entry::Empty(_)) => continue,
                (_slot, Entry::Occupied(occupied)) => {
                    self.len = self
                        .len
                        .checked_sub(1)
                        .unwrap_or_else(|| unreachable!("Underflowed u32 trying to iterate Arena"));

                    return Some(&occupied.value);
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len as usize, Some(self.len as usize))
    }
}

impl<'a, T> FusedIterator for ValuesIter<'a, T> {}
impl<'a, T> ExactSizeIterator for ValuesIter<'a, T> {}
