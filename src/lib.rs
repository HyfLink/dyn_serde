//! This crate provides the dyn-compatible version of the [`serde`] traits:
//!
//! 1. [`Serialize`] - dyn-compatible version of [`serde::Serialize`]:
//!
//!   * Implementing `serde::Serialize` will automatically provides an
//!     implementation of `Serialize`;
//!
//!   * The trait object `dyn Serialize` also implements `serde::Serialize`;
//!
//! 1. [`Serializer`] - dyn-compatible version of [`serde::Serializer`]:
//!
//!   * Instance of `Serializer` can be converted from instance of
//!     `serde::Serializer` by method [`<dyn Serializer>::new`];
//!
//!   * The trait object `dyn Serializer` also implements `serde::Serializer`;
//!
//! 1. [`Deserializer`] - dyn-compatible version of [`serde::Deserializer`]:
//!
//!   * Instance of `Deserializer` can be converted from instance of
//!     `serde::Deserializer` by method [`<dyn Deserializer>::new`];
//!
//!   * The trait object `dyn Deserializer` also implements `serde::Deserializer`;
//!
//! This crate is has basically the same functionality with `erased_serde`. For
//! more information, see [`README.md`].
//!
//! [`README.md`]: https://crates.io/crates/dyn_serde
//! [`<dyn Serializer>::new`]: trait.Serializer.html#method.new
//! [`<dyn Deserializer>::new`]: trait.Deserializer.html#method.new
//!
//! # Example
//!
//! ## Serialization
//!
//! ```
//! use serde::Serialize;
//! use dyn_serde::Serializer;
//!
//! // 1. Creates dynamic serializer using `<dyn Serializer>::new`.
//! let mut buffer = Vec::with_capacity(64);
//! let mut serializer = serde_json::Serializer::new(std::io::Cursor::new(&mut buffer));
//! let mut serializer = <dyn Serializer>::new(&mut serializer);
//! let serializer: &mut dyn Serializer = &mut serializer;
//!
//! // 2. Just uses the dynamic serializer as a `serde::Serializer`.
//! let value = "Hello, world!";
//! value.serialize(serializer).unwrap();
//! assert_eq!(buffer, b"\"Hello, world!\"");
//! ```
//!
//! ## Deserialization
//!
//! ```
//! use serde::Deserialize;
//! use dyn_serde::Deserializer;
//!
//! // 1. Creates dynamic deserializer using `<dyn Deserializer>::new`.
//! let mut deserializer = serde_json::Deserializer::from_str("\"Hello, world!\"");
//! let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
//! let deserializer: &mut dyn Deserializer = &mut deserializer;
//!
//! // 2. Just uses the dynamic deserializer as a `serde::Deserializer`.
//! let value = String::deserialize(deserializer).unwrap();
//! assert_eq!(value, "Hello, world!");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod de;
pub mod ser;

// re-exports
pub use crate::de::Deserializer;
pub use crate::ser::{Serialize, Serializer};
