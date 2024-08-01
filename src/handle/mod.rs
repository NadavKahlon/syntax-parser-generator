use std::any::type_name;
use std::fmt::{Debug, Formatter};
use core::HandleCore;

mod core;
pub mod handle_map;
pub mod handled_vec;
pub mod handled_hash_map;
pub mod auto;

pub trait Handled {
    type HandleCoreType: HandleCore;

    fn new_handle(index: usize) -> Handle<Self> {
        index.into()
    }
}

#[derive(Hash)]
pub struct Handle<T>
where
    T: Handled + ?Sized,
{
    pub core: T::HandleCoreType,
}

impl<T> Clone for Handle<T>
where
    T: ?Sized + Handled,
{
    fn clone(&self) -> Self {
        Self::from(self.index())
    }
}

impl<T> Copy for Handle<T>
where
    T: Handled + ?Sized,
{}

impl<T> PartialEq<Handle<T>> for Handle<T>
where
    T: Handled,
{
    fn eq(&self, other: &Handle<T>) -> bool {
        self.core == other.core
    }
}

impl<T> Debug for Handle<T>
where
    T: Handled
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handle<{:?}>({:?})", type_name::<T>(), self.core.into_index())
    }
}


impl<T> Handle<T>
where
    T: Handled + ?Sized,
{
    fn index(&self) -> usize {
        self.core.into_index()
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
