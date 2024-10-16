use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use derive_where::derive_where;

// TODO remove Ord traits from here, create a complete module for ordered handles
pub trait HandleCore: Clone + Copy + Eq + PartialEq + Hash + Debug + Ord + PartialOrd {
    fn into_index(self) -> usize;
    fn from_index(index: usize) -> Self;
}

#[derive_where(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T>
where
    T: Handled + ?Sized,
{
    pub core: T::HandleCoreType,
}

pub trait Handled {
    type HandleCoreType: HandleCore;

    fn new_handle(index: usize) -> Handle<Self> {
        index.into()
    }
}

impl<T> Handle<T>
where
    T: Handled + ?Sized,
{
    pub(super) fn index(&self) -> usize {
        self.core.into_index()
    }

    // Create a handles to "nothing", different from all other handles in the given set
    pub fn mock(existing_handles: &Vec<Handle<T>>) -> Handle<T> {
        let existing_indices: std::collections::HashSet<usize> = existing_handles
            .iter()
            .map(|handle| handle.index())
            .collect();

        let mut index = 0;
        while existing_indices.contains(&index) {
            index += 1;
        }

        index.into()
    }
}

impl<T> Debug for Handle<T>
where
    T: Handled,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle({:?})", self.core.into_index())
    }
}

impl<T> From<usize> for Handle<T>
where
    T: Handled + ?Sized,
{
    fn from(index: usize) -> Self {
        Self {
            core: T::HandleCoreType::from_index(index),
        }
    }
}

impl<T> Into<usize> for Handle<T>
where
    T: Handled + ?Sized,
{
    fn into(self) -> usize {
        self.core.into_index()
    }
}
