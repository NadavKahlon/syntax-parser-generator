use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use derive_where::derive_where;

/// A type that may serve as the internal representation of a [Handle].
///
/// A [Handle] is defined by a single field known as its _core_. The core identifies the handled
/// object, and is an instance of a lightweight type that implements [HandleCore]. It should
/// be convertible from and to a `usize`, which represents the handled object's serial-number.
// TODO remove Ord traits from here, create a complete module for ordered handles
pub trait HandleCore: Clone + Copy + Eq + PartialEq + Hash + Debug + Ord + PartialOrd {
    /// Get the serial number represented by this [HandleCore].
    fn into_index(self) -> usize;

    /// Transform a serial number into an identifying [HandleCore].
    fn from_index(index: usize) -> Self;
}

/// A lightweight identifier for instances of an arbitrary type `T`.
#[derive_where(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T>
where
    T: Handled + ?Sized,
{
    /// The internal representation of the identifier.
    pub core: T::HandleCoreType,
}

/// A type whose instances may be identified by handles.
pub trait Handled {
    /// The type of the internal representation of our handles.
    type HandleCoreType: HandleCore;

    /// Creates handle associated with an object identified by a certain `serial_number`.
    fn new_handle(serial_number: usize) -> Handle<Self> {
        serial_number.into()
    }
}

impl<T> Handle<T>
where
    T: Handled + ?Sized,
{
    pub(super) fn index(&self) -> usize {
        self.core.into_index()
    }

    /// Creates a handle different from all handles in a given list of `existing_handles`.
    ///
    /// This may serve as "a handle to nothing", a "mock handle".
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
    /// Converts a serial number of an object to a matching handle.
    fn from(serial_number: usize) -> Self {
        Self {
            core: T::HandleCoreType::from_index(serial_number),
        }
    }
}

impl<T> Into<usize> for Handle<T>
where
    T: Handled + ?Sized,
{
    /// Converts a handle to the serial number it identifies.
    fn into(self) -> usize {
        self.core.into_index()
    }
}
