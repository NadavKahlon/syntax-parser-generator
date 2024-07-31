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

#[derive(Eq, PartialEq, Hash, Debug)]
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
{

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
