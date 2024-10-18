use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use crate::handles::{Handle, Handled};

/// A collection of [Handled] instances with automatically-generated handles, supporting efficient
/// handle-to-instance lookup.
///
/// This data structure accumulates instances of some [Handled] type `T`, while automatically
/// generating handles to each of the contained instances, based on their order of insertion to the
/// vector. It supports efficient handle-to-instance lookup.
///
/// This usually serves for generating handles to instances of complex data types.
///
/// # Example
///
/// ```rust
/// # use syntax_parser_generator::handles::{Handle, Handled};
/// # use syntax_parser_generator::handles::collections::HandledVec;
/// struct LinkedListNode {
///     num: u32,
///     next: Option<Handle<LinkedListNode>>,
/// }
/// impl Handled for LinkedListNode { type HandleCoreType = u8; }
///
/// let mut nodes = HandledVec::new();
/// let tail_handle = nodes.insert(LinkedListNode { num: 1337, next: None });
/// let head_handle = nodes.insert(LinkedListNode { num: 42, next: Some(tail_handle) });
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HandledVec<T>
where
    T: Handled,
{
    contents: Vec<T>,
}

impl<T> HandledVec<T>
where
    T: Handled,
{
    /// Creates a new, empty, [HandledVec].
    pub fn new() -> Self {
        Self { contents: Vec::new() }
    }

    /// Adds a new `item` into the collection, and returns its newly generated handle.
    pub fn insert(&mut self, item: T) -> Handle<T> {
        self.contents.push(item);
        (self.contents.len() - 1).into()
    }

    /// Lists the handles to all available items in the collection.
    pub fn list_handles(&self) -> impl Iterator<Item=Handle<T>> {
        (0..self.contents.len())
            .map(|index| index.into())
    }

    /// Get an iterator of references to the items in the collection.
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.into_iter()
    }

    /// Get an iterator of mutable references to the items in the collection.
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.into_iter()
    }
}

impl<T> Index<Handle<T>> for HandledVec<T>
where
    T: Handled,
{
    type Output = T;

    /// Get a reference to the collection's item that's associated with the given handle.
    fn index(&self, index: Handle<T>) -> &Self::Output {
        &self.contents[index.index()]
    }
}

impl<T> IndexMut<Handle<T>> for HandledVec<T>
where
    T: Handled,
{
    /// Get a mutable reference to the collection's item that's associated with the given handle.
    fn index_mut(&mut self, index: Handle<T>) -> &mut Self::Output {
        &mut self.contents[index.index()]
    }
}

impl<'a, T> IntoIterator for &'a HandledVec<T>
where
    T: Handled,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter()
    }
}

impl<'a, T> FromIterator<T> for HandledVec<T>
where
    T: Handled,
{
    fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
        let mut result = Self::new();
        for item in iter {
            result.insert(item);
        }
        result
    }
}

impl<'a, T> IntoIterator for &'a mut HandledVec<T>
where
    T: Handled,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.iter_mut()
    }
}