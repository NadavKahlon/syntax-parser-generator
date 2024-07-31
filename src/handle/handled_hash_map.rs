use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use crate::handle::{Handle, Handled};

pub struct HandledHashMap<T>
where
    T: Handled + Eq + PartialEq + Hash,
{
    contents: HashMap<T, Handle<T>>,
}

impl<T> HandledHashMap<T>
where
    T: Handled + Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Self { contents: HashMap::new() }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.contents.contains_key(item)
    }

    pub fn insert(&mut self, item: T) {
        if !self.contains(&item) {
            let new_handle = self.contents.len().into();
            self.contents.insert(item, new_handle);
        }
    }

    pub fn get_handle(&self, item: &T) -> Option<&Handle<T>> {
        self.contents.get(item)
    }
}

impl<T> FromIterator<T> for HandledHashMap<T>
where
    T: Handled + Eq + PartialEq + Hash,
{
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let mut set = Self::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl<T> Index<T> for HandledHashMap<T>
where
    T: Handled + Eq + PartialEq + Hash,
{
    type Output = Handle<T>;

    fn index(&self, index: T) -> &Self::Output {
        self.get_handle(&index).expect("No handle is associated with index item")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Handled for u32 {
        type HandleCoreType = u8;
    }

    #[test]
    fn test() {
        let mut set = HandledHashMap::from_iter(1..10);
        assert_eq!(set.contains(&1), true);
        assert_eq!(set.contains(&0), false);
        assert_eq!(set.contains(&5), true);

        set.insert(1);
        assert_eq!(set.contains(&1), true);

        assert_eq!(set.contains(&12), false);
        set.insert(12);
        assert_eq!(set.contains(&12), true);

        assert_eq!(set.contains(&15), false);
        set.insert(15);
        assert_eq!(set.contains(&15), true);

        assert_ne!(set.get_handle(&12), None);
        assert_eq!(set.get_handle(&5412), None);

        assert_eq!(set.get_handle(&25), None);
        set.insert(25);
        let handle_of_25 = set.get_handle(&25).copied();
        assert_ne!(set.get_handle(&25), None);
        set.insert(25);
        let handle_of_25_again = set.get_handle(&25).copied();
        assert_eq!(handle_of_25, handle_of_25_again);
    }
}