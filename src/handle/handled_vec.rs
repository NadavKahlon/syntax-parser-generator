use std::ops::{Index, IndexMut};
use std::slice::Iter;
use crate::handle::{Handle, Handled};

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
    pub fn new() -> Self {
        Self { contents: Vec::new() }
    }

    pub fn insert(&mut self, item: T) -> Handle<T> {
        self.contents.push(item);
        (self.contents.len() - 1).into()
    }
}

impl<T> Index<Handle<T>> for HandledVec<T>
where
    T: Handled,
{
    type Output = T;

    fn index(&self, index: Handle<T>) -> &Self::Output {
        &self.contents[index.index()]
    }
}

impl<T> IndexMut<Handle<T>> for HandledVec<T>
where
    T: Handled,
{
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
