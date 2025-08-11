//! Dynamic serialization and deserialization based on [`serde`].
//!
//! This crate provides a way to serialize and deserialize data dynamically at
//! runtime. This is useful when you need to serialize or deserialize data of
//! unknown type.
//!
//! * [`Serialize`]
//! * [`Serializer`]
//! * [`Deserializer`]

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
