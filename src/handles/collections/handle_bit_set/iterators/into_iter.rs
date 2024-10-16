use crate::handles::collections::handle_bit_set::HandleBitSet;
use crate::handles::{Handle, HandleCore, Handled};

impl<T> IntoIterator for HandleBitSet<T>
where
    T: Handled,
{
    type Item = Handle<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IntoIter<T>
where
    T: Handled,
{
    set: HandleBitSet<T>,
    curr_index: usize,
}

impl<T> IntoIter<T>
where
    T: Handled,
{
    fn new(set: HandleBitSet<T>) -> Self {
        Self { set, curr_index: 0 }
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Handled,
{
    type Item = Handle<T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr_index >= (self.set.bytes.len() * 8) {
                break None;
            }

            let handle = Handle { core: T::HandleCoreType::from_index(self.curr_index) };
            self.curr_index += 1;

            if self.set.contains(handle) {
                break Some(handle);
            }
        }
    }
}