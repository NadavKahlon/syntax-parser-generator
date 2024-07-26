use serial_number::SerialNumber;

mod serial_number;
pub mod handle_map;
pub mod handled_collection;

pub trait Handled {
    type SerialNumberType: SerialNumber;

    fn new_handle(index: usize) -> Handle<Self> {
        index.into()
    }
}

pub struct Handle<T>
where
    T: Handled + ?Sized,
{
    pub serial: T::SerialNumberType,
}

impl<T> Clone for Handle<T>
where
    T: ?Sized + Handled,
{
    fn clone(&self) -> Self {
        Self::from(self.index())
    }
}

impl<T> Copy for Handle<T>
where
    T: Handled + ?Sized,
{

}


impl<T> Handle<T>
where
    T: Handled + ?Sized,
{
    fn index(&self) -> usize {
        self.serial.into_index()
    }
}

impl<T> From<usize> for Handle<T>
where
    T: Handled + ?Sized,
{
    fn from(index: usize) -> Self {
        Self {
            serial: T::SerialNumberType::from_index(index),
        }
    }
}

impl<T> Into<usize> for Handle<T>
where
    T: Handled + ?Sized,
{
    fn into(self) -> usize {
        self.serial.into_index()
    }
}
