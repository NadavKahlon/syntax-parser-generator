use std::marker::PhantomData;
use crate::handle::Handle;
use crate::handle::serial_number::SerialNumber;

// TODO "complete map", where everything is known (no "Option<U>", just U). Why? to half tne space
pub struct HandleVectorMap<T, U, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    contents: Vec<Option<U>>,
    phantom_data: PhantomData<Handle<T, SerialNumberType>>,
}

impl<T, U, SerialNumberType> HandleVectorMap<T, U, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    pub fn new() -> Self {
        Vec::new().into()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity).into()
    }

    pub fn insert(&mut self, key: Handle<T, SerialNumberType>, item: U) -> bool {
        let result = self.contents[key.index()].is_none();
        self.contents[key.index()] = Some(item);
        result
    }

    pub fn get(&mut self, key: Handle<T, SerialNumberType>) -> &Option<U> {
        &self.contents[key.index()]
    }
}

impl<T, U, SerialNumberType> From<Vec<Option<U>>> for HandleVectorMap<T, U, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    fn from(contents: Vec<Option<U>>) -> Self {
        Self {
            contents,
            phantom_data: Default::default(),
        }
    }
}