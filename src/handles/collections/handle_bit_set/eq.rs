use std::hash::{Hash, Hasher};

use crate::handles::collections::handle_bit_set::HandleBitSet;
use crate::handles::Handled;

impl<T> HandleBitSet<T>
where
    T: Handled,
{
    fn canonicalize(&self) -> Self {
        // Removes trailing zeros to get the set into its canonical form
        let mut bytes = self.bytes.clone();
        while let Some(&0) = bytes.last() {
            bytes.pop();
        }
        Self {
            bytes,
            phantom_data: Default::default(),
        }
    }
}

impl<T> PartialEq for HandleBitSet<T>
where
    T: Handled,
{
    fn eq(&self, other: &Self) -> bool {
        self.canonicalize().bytes == other.canonicalize().bytes
    }
}

impl<T: Handled> Eq for HandleBitSet<T> {}

impl<T: Handled> Hash for HandleBitSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalize().bytes.hash(state)
    }
}
