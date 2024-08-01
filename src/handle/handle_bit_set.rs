use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::handle::{Handle, Handled};
use crate::handle::core::HandleCore;

pub struct HandleBitSet<T>
where
    T: Handled,
{
    bytes: Vec<u8>,
    phantom_data: PhantomData<Handle<T>>,
}

impl<T> Debug for HandleBitSet<T>
where
    T: Handled,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{:?}}}", self.iter().collect::<Vec<Handle<T>>>())
    }
}

impl<T> HandleBitSet<T>
where
    T: Handled,
{
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            phantom_data: Default::default(),
        }
    }

    fn locate(&self, handle: Handle<T>) -> (usize, usize) {
        let core = handle.core.into_index();
        (core / 8, core % 8)
    }

    pub fn contains(&self, handle: Handle<T>) -> bool {
        let (byte_index, byte_offset) = self.locate(handle);
        match self.bytes.get(byte_index) {
            None => false,
            Some(byte) => (byte & (1 << byte_offset)) != 0,
        }
    }

    pub fn insert(&mut self, handle: Handle<T>) {
        let (byte_index, byte_offset) = self.locate(handle);
        if self.bytes.len() <= byte_index {
            self.bytes.resize(byte_index + 1, 0);
        }
        self.bytes[byte_index] |= 1 << byte_offset;
    }

    pub fn union(&self, other: &Self) -> Self {
        let max_len = max(self.bytes.len(), other.bytes.len());
        let mut result_bytes = vec![0; max_len];

        for i in 0..self.bytes.len() {
            result_bytes[i] |= self.bytes[i];
        }
        for i in 0..other.bytes.len() {
            result_bytes[i] |= other.bytes[i];
        }

        Self {
            bytes: result_bytes,
            phantom_data: Default::default(),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    fn canonicalize(&self) -> Self {
        // Removes trailing zeros to get the set into its canonical form
        let mut bytes = self.bytes.clone();
        while let Some(&0) = bytes.last() {
            bytes.pop();
        }
        Self { bytes, phantom_data: Default::default() }
    }
}

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
            set.insert(item)
        }
        set
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
    fn new(set: &'a HandleBitSet<T>) -> Self {
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::handle::auto::AutomaticallyHandled;
    use crate::handle::handle_bit_set::HandleBitSet;

    enum T {
        T1,
        T2,
        T3,
        T4,
    }
    impl AutomaticallyHandled for T {
        type HandleCoreType = u8;
        fn serial(&self) -> usize {
            match self {
                T::T1 => 0,
                T::T2 => 89,
                T::T3 => 1000,
                T::T4 => 3,
            }
        }
    }

    #[test]
    fn test_insert() {
        let mut set = HandleBitSet::new();

        assert_eq!(set.contains(T::T1.handle()), false);

        set.insert(T::T1.handle());
        assert_eq!(set.contains(T::T1.handle()), true);

        set.insert(T::T2.handle());
        assert_eq!(set.contains(T::T2.handle()), true);
        assert_eq!(set.contains(T::T3.handle()), false);

        set.insert(T::T3.handle());
        assert_eq!(set.contains(T::T2.handle()), true);
        assert_eq!(set.contains(T::T3.handle()), true);
    }

    #[test]
    fn test_union() {
        let set1: HandleBitSet<T> = vec![T::T1.handle(), T::T4.handle()].into_iter().collect();
        let set2: HandleBitSet<T> = vec![T::T2.handle()].into_iter().collect();

        assert_eq!(
            set1.union(&set2),
            vec![T::T4.handle(), T::T1.handle(), T::T2.handle()].into_iter().collect()
        )
    }

    #[test]
    fn test_hash() {
        let set_hash_set: HashSet<HandleBitSet<T>> = vec![
            vec![T::T2.handle(), T::T3.handle()]
                .into_iter().collect(),
            vec![T::T3.handle(), T::T2.handle()]
                .into_iter().collect(),
            vec![T::T3.handle()]
                .into_iter().collect(),
            vec![T::T1.handle(), T::T3.handle()]
                .into_iter().collect(),
        ].into_iter().collect();
        assert_eq!(set_hash_set.len(), 3)
    }

    #[test]
    fn test_canonicalize() {
        let set1: HandleBitSet<T> = HandleBitSet {
            bytes: vec![1, 3, 255, 0, 5, 0, 6, 0],
            phantom_data: Default::default(),
        };
        let set2: HandleBitSet<T> = HandleBitSet {
            bytes: vec![1, 3, 255, 0, 5, 0, 6, 0, 0],
            phantom_data: Default::default(),
        };
        assert_eq!(set1, set2)
    }
}