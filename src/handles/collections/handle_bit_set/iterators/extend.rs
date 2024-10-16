use crate::handles::{Handle, Handled};
use crate::handles::collections::handle_bit_set::HandleBitSet;

impl<T> Extend<Handle<T>> for HandleBitSet<T>
where
    T: Handled,
{
    fn extend<I: IntoIterator<Item=Handle<T>>>(&mut self, iter: I) {
        for index in iter {
            self.insert(index);
        }
    }
}

impl<'a, T> Extend<&'a Handle<T>> for HandleBitSet<T>
where
    T: Handled,
{
    fn extend<I: IntoIterator<Item=&'a Handle<T>>>(&mut self, iter: I) {
        for handle in iter {
            self.insert(*handle);
        }
    }
}
