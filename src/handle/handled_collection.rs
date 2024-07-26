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

    // Assuming failing accesses will never happen, as `handle` should be collected from `insert`
    pub fn get(&self, key: Handle<T>) -> &T {
        &self.contents[key.index()]
    }
}