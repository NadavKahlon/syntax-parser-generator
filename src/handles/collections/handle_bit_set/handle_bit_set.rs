use std::cmp::max;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use derive_where::derive_where;

use crate::handles::{Handle, HandleCore, Handled};

/// An efficient set of handles.
///
/// This data-structure is an efficient implementation of a set of handles. Since handles actually
/// represents integer indices, a set of handles can be represented by a vector of "is-present"
/// bits. This implementation of a set of handles is based on such "bit-vectors".
#[derive_where(Clone)]
pub struct HandleBitSet<T>
where
    T: Handled,
{
    pub(super) bytes: Vec<u8>,
    pub(super) phantom_data: PhantomData<Handle<T>>,
}

impl<T> HandleBitSet<T>
where
    T: Handled,
{
    /// Creates a new, empty, set of handles.
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

    /// Checks whether a given handle is contained in the set.
    pub fn contains(&self, handle: Handle<T>) -> bool {
        let (byte_index, byte_offset) = self.locate(handle);
        match self.bytes.get(byte_index) {
            None => false,
            Some(byte) => (byte & (1 << byte_offset)) != 0,
        }
    }

    /// Insert a given handle to the set.
    ///
    /// Returns `true` if the handle wasn't previously in the set, and `false` otherwise.
    pub fn insert(&mut self, handle: Handle<T>) -> bool {
        if !self.contains(handle) {
            let (byte_index, byte_offset) = self.locate(handle);
            if self.bytes.len() <= byte_index {
                self.bytes.resize(byte_index + 1, 0);
            }
            self.bytes[byte_index] |= 1 << byte_offset;
            true
        } else {
            false
        }
    }

    /// Calculates the union of this set and another set of handles.
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

    /// Checks whether the set has no handles in it.
    pub fn is_empty(&self) -> bool {
        self.bytes.iter().all(|&byte| byte == 0)
    }
}

impl<T> Debug for HandleBitSet<T>
where
    T: Handled,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter().collect::<Vec<Handle<T>>>().fmt(f)
    }
}
