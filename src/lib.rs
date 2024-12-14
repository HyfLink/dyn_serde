//! TODO: crate-level documentation
//!
//!
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod de;
pub mod error;
pub mod ser;

// re-exports
pub use crate::de::Deserializer;
pub use crate::ser::{Serialize, Serializer};
