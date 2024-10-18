use crate::readers::address_based::{AddressBasedReader, AddressSpace};

pub struct ByteArrayAddressSpace {
    data: Box<[u8]>,
}

impl ByteArrayAddressSpace {
    fn from_string(data: String) -> Self {
        ByteArrayAddressSpace { data: data.into_bytes().into_boxed_slice() }
    }
}

impl AddressSpace<u8> for ByteArrayAddressSpace {
    fn read_at(&self, address: usize) -> Option<u8> {
        self.data.get(address).copied()
    }
}

/// Implementation of the [Reader](crate::readers::Reader) interface for accessing an in-memory
/// array of bytes.
pub type ByteArrayReader = AddressBasedReader<u8, ByteArrayAddressSpace>;

impl ByteArrayReader {
    /// Creates a new [Reader](crate::readers::Reader) for accessing the sequence of bytes in a
    /// given [String].
    pub fn from_string(data: String) -> ByteArrayReader {
        let address_space = ByteArrayAddressSpace::from_string(data);
        AddressBasedReader::raw_new(address_space)
    }
}


#[cfg(test)]
mod tests {
    use crate::readers::Reader;
    use super::*;

    #[test]
    fn test_reading() {
        let mut reader = ByteArrayReader::from_string("Hi, this is data".to_string());
        assert_eq!(reader.read_next(), Some('H' as u8));
        assert_eq!(reader.read_next(), Some('i' as u8));
        assert_eq!(reader.read_next(), Some(',' as u8));
    }

    #[test]
    fn test_sequence_extraction() {
        let mut reader = ByteArrayReader::from_string("Hi, this is data".to_string());
        reader.read_next();
        reader.read_next();
        reader.read_next();
        reader.read_next();
        reader.set_head();
        reader.read_next();
        reader.read_next();
        reader.read_next();
        reader.read_next();
        reader.set_tail();
        assert_eq!(
            String::from_utf8(reader.get_sequence().collect()).unwrap(),
            "this".to_string(),
        );
    }

    #[test]
    fn test_reset_to_tail() {
        let mut reader = ByteArrayReader::from_string("Hi, this is data".to_string());
        reader.set_tail();
        reader.read_next();
        reader.restart_from_tail();
        assert_eq!(reader.read_next(), Some('H' as u8));
    }
}