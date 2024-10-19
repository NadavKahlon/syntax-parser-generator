//! Streams of input text to be parsed.
//!
//! The first stage of the parsing pipeline - lexical analysis - requires a unique API to access the
//! input text. This module defines this API in the form of the [Reader] trait. It also contains
//! several implementations of the trait, which wrap standard sources of data with this required
//! API.

pub use address_based::{AddressBasedReader, AddressSpace};
pub use byte_array_reader::ByteArrayReader;
pub use reader::Reader;

mod reader;
mod address_based;
mod byte_array_reader;
