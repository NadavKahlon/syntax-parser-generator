use std::marker::PhantomData;
use derive_where::derive_where;
use crate::handle::{Handle, Handled};

// TODO "complete map", where everything is known (no "Option<U>", just U). Why? to half tne space
#[derive_where(Debug, PartialEq, Eq; U)]
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

    pub fn get_mut(&mut self, key: Handle<T>) -> Option<&mut U> {
        self.contents.get_mut(key.index())?.as_mut()
    }

    pub fn contains_key(&self, key: Handle<T>) -> bool {
        !self.get(key).is_none()
    }

    pub fn iter(&self) -> Iter<T, U> {
        (&self).into_iter()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item=Handle<T>> + 'a {
        self.iter().map(|(key, _)| key)
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

impl<'a, T, U> IntoIterator for &'a HandleMap<T, U>
where
    T: Handled + ?Sized,
{
    type Item = (Handle<T>, &'a U);
    type IntoIter = Iter<'a, T, U>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

pub struct Iter<'a, T, U>
where
    T: Handled + ?Sized,
{
    map: &'a HandleMap<T, U>,
    curr_index: usize,
}

impl<'a, T, U> Iter<'a, T, U>
where
    T: Handled + ?Sized,
{
    fn new(map: &'a HandleMap<T, U>) -> Self {
        Self { map, curr_index: 0 }
    }
}

impl<'a, T, U> Iterator for Iter<'a, T, U>
where
    T: Handled + ?Sized,
{
    type Item = (Handle<T>, &'a U);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.contents.get(self.curr_index)? {
                None => {
                    self.curr_index += 1
                }
                Some(content) => {
                    let handle: Handle<T> = self.curr_index.into();
                    self.curr_index += 1;
                    break Some((handle, content));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct DummyHandled {}

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

    #[test]
    fn test_into_iter() {
        let mut map: HandleMap<DummyHandled, i32> = HandleMap::new();
        map.insert(1.into(), 1);
        map.insert(50.into(), 32);
        map.insert(2.into(), 2);


        assert_eq!(
            map.into_iter().collect::<Vec<(Handle<DummyHandled>, &i32)>>(),
            vec![
                (1.into(), &1),
                (2.into(), &2),
                (50.into(), &32),
            ]
        )
    }
}
