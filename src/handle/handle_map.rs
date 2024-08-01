use std::marker::PhantomData;
use crate::handle::{Handle, Handled};

// TODO "complete map", where everything is known (no "Option<U>", just U). Why? to half tne space
pub struct HandleMap<T, U>
where
    T: Handled + ?Sized,
{
    contents: Vec<Option<U>>,
    phantom_data: PhantomData<Handle<T>>,
}

impl<T, U> HandleMap<T, U>
where
    T: Handled + ?Sized,
{
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn insert(&mut self, key: Handle<T>, item: U) -> bool {
        let result = self.get(key).is_none();
        if key.index() >= self.contents.len() {
            self.contents.resize_with(key.index() + 1, || None);
        }
        self.contents[key.index()] = Some(item);
        result
    }

    pub fn get(&self, key: Handle<T>) -> Option<&U> {
        self.contents.get(key.index())?.as_ref()
    }

    pub fn contains_key(&self, key: Handle<T>) -> bool {
        !self.get(key).is_none()
    }
}

impl<T, U> From<Vec<Option<U>>> for HandleMap<T, U>
where
    T: Handled + ?Sized,
{
    fn from(contents: Vec<Option<U>>) -> Self {
        Self {
            contents,
            phantom_data: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyHandled { }

    impl Handled for DummyHandled {
        type HandleCoreType = u16;
    }

    #[test]
    fn test_handle_map() {
        let mut map: HandleMap<DummyHandled, i32> = HandleMap::new();

        assert_eq!(map.insert(1.into(), 1), true);
        assert_eq!(map.insert(50.into(), 50), true);
        assert_eq!(map.insert(1.into(), 1), false);
        assert_eq!(map.get(2.into()), None);
        assert_eq!(map.get(1.into()), Some(&1));
    }
}
