use crate::handle::{Handle, Handled};
use crate::handle::handle_bit_set::HandleBitSet;

impl<T> FromIterator<Handle<T>> for HandleBitSet<T>
where
    T: Handled,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item=Handle<T>>,
    {
        let mut set = Self::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}


impl<'a, T> FromIterator<&'a Handle<T>> for HandleBitSet<T>
where
    T: Handled,
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item=&'a Handle<T>>,
    {
        let mut set = Self::new();
        for item in iter {
            set.insert(*item);
        }
        set
    }
}
