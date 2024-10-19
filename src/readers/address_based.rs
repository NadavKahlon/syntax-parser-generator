use std::marker::PhantomData;

use crate::readers::Reader;

/// A sequence of items that can be accessed by conceptual addresses.
///
/// The first item's address should be 0.
pub trait AddressSpace<T> {
    /// Get the item in a given address.
    ///
    /// If no item is available at that address, [None] is returned.
    fn read_at(&self, address: usize) -> Option<T>;
}

/// A [Reader] implementation that's based on an [AddressSpace] of the read items.
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
    /// Create a new reader over the items in the given address space.
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
            .map(|optional_item| {
                optional_item.expect("Sequence of Reader items between head & tail should exist")
            })
    }
}
