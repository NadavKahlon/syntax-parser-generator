use std::fmt::{Debug, Formatter};
use derive_where::derive_where;
use core::HandleCore;

mod core;
pub mod handle_map;
pub mod handled_vec;
pub mod handled_hash_map;
pub mod auto;
pub mod handle_bit_set;
pub mod order;
pub mod mock;

pub trait Handled {
    type HandleCoreType: HandleCore;

    fn new_handle(index: usize) -> Handle<Self> {
        index.into()
    }
}

#[derive_where(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T>
where
    T: Handled + ?Sized,
{
    pub core: T::HandleCoreType,
}

impl<T> Handle<T>
where
    T: Handled + ?Sized,
{
    fn index(&self) -> usize {
        self.core.into_index()
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
