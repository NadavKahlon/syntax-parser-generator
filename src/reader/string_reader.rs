use crate::reader::address_based::{AddressBasedReader, AddressSpace};

pub struct ByteAddressSpace {
    data: Box<[u8]>,
}

impl ByteAddressSpace {
    fn from_string(data: String) -> Self {
        ByteAddressSpace { data: data.into_bytes().into_boxed_slice() }
    }
}

impl AddressSpace<u8> for ByteAddressSpace {
    fn read_at(&self, address: usize) -> u8 {
        self.data[address]
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

pub type ByteReader = AddressBasedReader<u8, ByteAddressSpace>;

impl ByteReader {
    pub fn from_string(data: String) -> ByteReader {
        let address_space = ByteAddressSpace::from_string(data);
        AddressBasedReader::raw_new(address_space)
    }
}


#[cfg(test)]
mod tests {
    use crate::reader::Reader;
    use super::*;

    #[test]
    fn test_reading() {
        let mut reader = ByteReader::from_string("Hi, this is data".to_string());
        assert_eq!(reader.read_next(), 'H' as u8);
        assert_eq!(reader.read_next(), 'i' as u8);
        assert_eq!(reader.read_next(), ',' as u8);
    }

    #[test]
    fn test_sequence_extraction() {
        let mut reader = ByteReader::from_string("Hi, this is data".to_string());
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
        let mut reader = ByteReader::from_string("Hi, this is data".to_string());
        reader.set_tail();
        reader.read_next();
        reader.reset_to_tail();
        assert_eq!(reader.read_next(), 'H' as u8);
    }
}