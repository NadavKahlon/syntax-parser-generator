use std::marker::PhantomData;
use crate::readers::Reader;

pub trait AddressSpace<T> {
    fn read_at(&self, address: usize) -> Option<T>;
    fn is_available(&self, address: usize) -> bool;
}

pub struct AddressBasedReader<T, AddressSpaceType>
where
    AddressSpaceType: AddressSpace<T>,
{
    address_space: AddressSpaceType,
    head_address: usize,
    tail_address: usize,
    cursor_address: usize,
    phantom_data: PhantomData<T>,
}

impl<T, AddressSpaceType> AddressBasedReader<T, AddressSpaceType>
where
    AddressSpaceType: AddressSpace<T>,
{
    pub fn raw_new(address_space: AddressSpaceType) -> Self {
        Self {
            address_space,
            head_address: 0,
            tail_address: 0,
            cursor_address: 0,
            phantom_data: Default::default(),
        }
    }
}

impl<T, AddressSpaceType> Reader<T> for AddressBasedReader<T, AddressSpaceType>
where
    AddressSpaceType: AddressSpace<T>,
{
    fn is_available(&self) -> bool {
        self.address_space.is_available(self.cursor_address)
    }

    fn read_next(&mut self) -> Option<T> {
        let result = self.address_space.read_at(self.cursor_address)?;
        self.cursor_address += 1;
        Some(result)
    }

    fn set_head(&mut self) {
        self.head_address = self.cursor_address;
    }

    fn set_tail(&mut self) {
        self.tail_address = self.cursor_address;
    }

    fn move_cursor_to_tail(&mut self) {
        self.cursor_address = self.tail_address;
    }

    fn get_sequence(&self) -> impl Iterator<Item=T> {
        (self.head_address..self.tail_address)
            .map(|address| self.address_space.read_at(address))
            .map(|optional_item| optional_item.expect(
                "Sequence of Reader items between head & tail should exist"
            ))
    }
}
