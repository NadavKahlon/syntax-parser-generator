use crate::handle::handle_bit_set::HandleBitSet;
use crate::handle::{Handle, Handled};
use crate::handle::core::HandleCore;

impl<T> HandleBitSet<T>
where
    T: Handled,
{
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a HandleBitSet<T>
where
    T: Handled
{
    type Item = Handle<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter<'a, T>
where
    T: Handled,
{
    set: &'a HandleBitSet<T>,
    curr_index: usize,
}

impl<'a, T> Iter<'a, T>
where
    T: Handled,
{
    pub fn new(set: &'a HandleBitSet<T>) -> Self {
        Self { set, curr_index: 0 }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
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
