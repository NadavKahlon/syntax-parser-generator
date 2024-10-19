//! Data structures for efficient management of handles and their associated data.
//!
//! This module provides various data structures that may be used to:
//!
//! * Manage instances of [Handled](super::Handled) types, automatically creating handles to them
//!     on the fly ([HandledVec], [HandledHashMap]).
//!
//! * Efficient management of collections of handles ([HandleMap], [HandleBitSet]).

pub use handle_bit_set::HandleBitSet;
pub use handle_map::HandleMap;
pub use handled_hash_map::HandledHashMap;
pub use handled_vec::HandledVec;

mod handle_bit_set;
mod handle_map;
mod handled_hash_map;
mod handled_vec;
