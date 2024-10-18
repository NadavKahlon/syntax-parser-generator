//! Streams of input text to be parsed.
//!
//! The first stage of the parsing pipeline - lexical analysis - requires a unique API to access the
//! input text. This module defines this API in the form of the [Reader] trait. It also contains
//! several implementations of the trait, which wrap standard sources of data with this required
//! API.

mod reader;
pub use reader::Reader;

mod address_based;
pub use address_based::{AddressSpace, AddressBasedReader};

mod byte_array_reader;
pub use byte_array_reader::ByteArrayReader;
