use std::ops::Index;
use crate::handle::{Handle, Handled};

pub struct HandledCollection<T>
where
    T: Handled,
{
    contents: Vec<T>,
}

impl<T> HandledCollection<T>
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

impl<T> Index<Handle<T>> for HandledCollection<T>
where
    T: Handled,
{
    type Output = T;

    fn index(&self, index: Handle<T>) -> &Self::Output {
        &self.contents[index.index()]
    }
}