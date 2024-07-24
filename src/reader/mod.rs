use crate::reader::wrapper::WrapperReader;

mod address_based;
mod string_reader;
mod wrapper;

pub trait Reader<T> {
    fn read_next(&mut self) -> T;
    fn set_head(&mut self);
    fn set_tail(&mut self);
    fn reset_to_tail(&mut self);  // The next item read is the one after tail
    fn get_sequence(&self) -> impl Iterator<Item=T>;
    fn wrap<U, W>(self, wrapper_function: W) -> impl Reader<U>
    where
        W: Fn(T) -> U,
        Self: Sized,
    {
        WrapperReader::new(self, wrapper_function)
    }
}
