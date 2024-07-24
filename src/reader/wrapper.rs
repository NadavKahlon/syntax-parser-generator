use std::marker::PhantomData;
use crate::reader::Reader;

pub struct WrapperReader<T, U, S, W>
where
    S: Reader<T>,
    W: Fn(T) -> U,
{
    reader: S,
    wrapper_function: W,
    phantom_data_t: PhantomData<T>,
    phantom_data_u: PhantomData<T>,
}


impl<T, U, S, W> WrapperReader<T, U, S, W>
where
    S: Reader<T>,
    W: Fn(T) -> U,
{
    pub fn new(reader: S, wrapper_function: W) -> Self {
        Self {
            reader,
            wrapper_function,
            phantom_data_t: Default::default(),
            phantom_data_u: Default::default(),
        }
    }
}

impl<T, U, S, W> Reader<U> for WrapperReader<T, U, S, W>
where
    S: Reader<T>,
    W: Fn(T) -> U,
{
    fn read_next(&mut self) -> U {
        (self.wrapper_function)(self.reader.read_next())
    }

    fn set_head(&mut self) {
        self.reader.set_head()
    }

    fn set_tail(&mut self) {
        self.reader.set_tail()
    }

    fn reset_to_tail(&mut self) {
        self.reader.reset_to_tail()
    }

    fn get_sequence(&self) -> impl Iterator<Item=U> {
        self.reader.get_sequence().map(&self.wrapper_function)
    }
}
