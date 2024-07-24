use std::marker::PhantomData;
use crate::reader::Reader;

pub trait AddressSpace<T> {
    fn read_at(&self, address: usize) -> T;
    fn size(&self) -> usize;
}

pub struct AddressBasedReader<T, U>
where
    U: AddressSpace<T>,
{
    address_space: U,
    head_address: usize,
    tail_address: usize,
    cursor_address: usize,
    phantom_data: PhantomData<T>,
}

impl<T, U> AddressBasedReader<T, U>
where
    U: AddressSpace<T>,
{
    pub fn raw_new(address_space: U) -> Self {
        Self {
            address_space,
            head_address: 0,
            tail_address: 0,
            cursor_address: 0,
            phantom_data: Default::default(),
        }
    }
}

impl<T, U> Reader<T> for AddressBasedReader<T, U>
where
    U: AddressSpace<T>,
{
    fn read_next(&mut self) -> Option<T> {
        if self.cursor_address < self.address_space.size() {
            let result = self.address_space.read_at(self.cursor_address);
            self.cursor_address += 1;
            result
        } else {
            None
        }
    }

    fn set_head(&mut self) {
        self.head_address = self.cursor_address;
    }

    fn set_tail(&mut self) {
        self.tail_address = self.cursor_address;
    }

    fn reset_to_tail(&mut self) {
        self.cursor_address = self.tail_address;
    }

    fn get_sequence(&self) -> impl Iterator<Item=T> {
        (self.head_address..self.tail_address)
            .map(|address| self.address_space.read_at(address))
    }
}
