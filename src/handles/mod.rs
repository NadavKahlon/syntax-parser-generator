//! Manage lightweight handles to arbitrary objects.
//!
//! This module provides an API for managing lightweight identifiers, called _handles_, that may
//! be associated with object of arbitrary types. This is used to make the objects' manipulation
//! more efficient (juggling lightweight identifiers instead of whole objects), and allows the crate
//! to operate on a more abstract level - using these generic handles, instead of ground instances.
//!
//! # Usage
//!
//! A [Handle] is defined by a single field known as its _core_. The core identifies the handled
//! object, and is an instance of a lightweight type that implements [HandleCore]. The module comes
//! with 2 built-in implementations of this trait: [u8] and [u16].
//!
//! In order to associate handles with instances of some arbitrary type `T`, we must first define
//! the corresponding [HandleCore] type that `T`'s handles will constitute of. This is done by
//! implementing the [Handled] trait for this `T`. The more instances of `T` we expect to keep
//! track of, the more bits we might want to use for `T`'s handles (i.e. the wider its [HandleCore]
//! should be).
//!
//! ```rust
//! # use syntax_parser_generator::handles::Handled;
//! struct MyStruct {
//!     data: Vec<u32>,
//!     name: String,
//! }
//! impl Handled for MyStruct {
//!     type HandleCoreType = u16;
//! }
//! ```
//!
//! We can now use [Handled::new_handle] to create handles to instances of `T`, identified by serial
//! numbers.
//!
//! ```rust
//! # use syntax_parser_generator::handles::Handled;
//! # struct NamedList {
//! #     data: Vec<u32>,
//! #     name: String,
//! # }
//! # impl Handled for NamedList {
//! #     type HandleCoreType = u16;
//! # }
//! let handle_of_first_named_list = NamedList::new_handle(0);
//! ```
//!
//! In practice, you won't need to manually create handles with [Handled::new_handle]. Instead, you
//! may use an existing API for doing so systematically. Check out [collections::HandledVec],
//! [collections::HandledHashMap], and [specials::AutomaticallyHandled] for that purpose.

pub use handle::{Handle, HandleCore, Handled};

mod handle;
mod known_cores;

pub mod collections;

pub mod specials;
