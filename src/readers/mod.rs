mod reader;
pub use reader::Reader;

mod address_based;
pub use address_based::{AddressSpace, AddressBasedReader};

mod byte_array_reader;
pub use byte_array_reader::ByteArrayReader;
