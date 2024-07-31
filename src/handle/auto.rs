use crate::handle::{Handle, Handled};
use crate::handle::core::HandleCore;

pub trait AutomaticallyHandled: Sized {
    type HandleCoreType: HandleCore;
    fn serial(&self) -> usize;
    fn handle(&self) -> Handle<Self> {
        self.serial().into()
    }
}

impl<T> Handled for T
where
    T: AutomaticallyHandled,
{ type HandleCoreType = T::HandleCoreType; }
