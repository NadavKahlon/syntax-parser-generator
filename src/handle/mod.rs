use std::marker::PhantomData;
use serial_number::SerialNumber;

mod serial_number;

pub struct Handle<T, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    pub serial: SerialNumberType,
    phantom_data: PhantomData<T>,
}

impl<T, SerialNumberType> From<usize> for Handle<T, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    fn from(index: usize) -> Self {
        Self {
            serial: SerialNumberType::from_index(index),
            phantom_data: Default::default(),
        }
    }
}

impl<T, SerialNumberType> Into<usize> for Handle<T, SerialNumberType>
where
    SerialNumberType: SerialNumber,
{
    fn into(self) -> usize {
        self.serial.into_index()
    }
}
