//! This module provides [`Serialize`] and [`Serializer`], which are the
//! dyn-compatible version of [`serde::Serialize`] and [`serde::Serializer`].

use core::fmt::Display;
use core::mem;

use serde::ser;

use crate::error::Error;

/// The dyn-compatible version of [`serde::Serialize`].
///
/// One should avoid implementing [`Serialize`] and implement
/// [`serde::Serialize`] instead. Implementing [`serde::Serialize`]
/// automatically provides one with an implementation of [`Serialize`].
///
/// # Example
///
/// Values implementing [`serde::Serialize`] can be converted into `dyn
/// Serialize`, which also implements [`serde::Serialize`].
///
/// ```
/// use dyn_serde::Serialize;
///
/// let values: &[&dyn Serialize] = &[
///     &false,
///     &123u32,
///     &"Hello, world!",
/// ];
/// let json = serde_json::to_string(values).unwrap();
/// assert_eq!(json, "[false,123,\"Hello, world!\"]");
/// ```
pub trait Serialize {
    /// Serializes this value into the given dynamic serializer.
    ///
    /// # Errors
    ///
    /// This method would return an [`Error`] if the serialization implemented
    /// by [`serde::Serialize::serialize`] fails; Or the dynamic serializer has
    /// been consumed.
    fn serialize_dyn(&self, serializer: &mut dyn Serializer) -> Result<(), Error>;
}

/// The dyn-compatible version of [`serde::Serializer`].
///
/// # Example
///
/// To construct dynamic serializer, use `<dyn Serializer>::new` instead of
/// implementing manually.
///
/// ```
/// use dyn_serde::Serializer;
///
/// let stdout = std::io::stdout();
/// let mut serializer = serde_json::Serializer::new(stdout.lock());
/// let serializer = <dyn Serializer>::new(&mut serializer);
/// let serializer: Box<dyn Serializer> = Box::new(serializer);
/// # let _ = serializer;
/// ```
///
/// The trait object `dyn Serializer` also implements [`serde::Serializer`].
///
/// ```
/// use serde::Serialize;
/// use dyn_serde::Serializer;
///
/// // Serializes to stdout.
/// let stdout = std::io::stdout();
/// let mut serializer1 = serde_json::Serializer::new(stdout.lock());
/// let mut serializer1 = <dyn Serializer>::new(&mut serializer1);
///
/// // Serializes to buffer.
/// let mut buffer = Vec::with_capacity(64);
/// let mut serializer2 = serde_json::Serializer::new(std::io::Cursor::new(&mut buffer));
/// let mut serializer2 = <dyn Serializer>::new(&mut serializer2);
///
/// let mut serializers: [&mut dyn Serializer; 2] = [
///     &mut serializer1,
///     &mut serializer2,
/// ];
/// let value = "Hello, world!";
/// for serializer in serializers {
///     value.serialize(serializer).unwrap();
/// }
/// assert_eq!(buffer, b"\"Hello, world!\"");
/// ```
pub trait Serializer {
    /// Serializes a `bool` value.
    fn serialize_bool_dyn(&mut self, v: bool) -> Result<(), Error>;

    /// Serializes an `i8` value.
    fn serialize_i8_dyn(&mut self, v: i8) -> Result<(), Error>;

    /// Serializes an `i16` value.
    fn serialize_i16_dyn(&mut self, v: i16) -> Result<(), Error>;

    /// Serializes an `i32` value.
    fn serialize_i32_dyn(&mut self, v: i32) -> Result<(), Error>;

    /// Serializes an `i64` value.
    fn serialize_i64_dyn(&mut self, v: i64) -> Result<(), Error>;

    /// Serializes an `i128` value.
    fn serialize_i128_dyn(&mut self, v: i128) -> Result<(), Error>;

    /// Serializes a `u8` value.
    fn serialize_u8_dyn(&mut self, v: u8) -> Result<(), Error>;

    /// Serializes a `u16` value.
    fn serialize_u16_dyn(&mut self, v: u16) -> Result<(), Error>;

    /// Serializes a `u32` value.
    fn serialize_u32_dyn(&mut self, v: u32) -> Result<(), Error>;

    /// Serializes a `u64` value.
    fn serialize_u64_dyn(&mut self, v: u64) -> Result<(), Error>;

    /// Serializes a `u128` value.
    fn serialize_u128_dyn(&mut self, v: u128) -> Result<(), Error>;

    /// Serializes an `f32` value.
    fn serialize_f32_dyn(&mut self, v: f32) -> Result<(), Error>;

    /// Serializes an `f64` value.
    fn serialize_f64_dyn(&mut self, v: f64) -> Result<(), Error>;

    /// Serializes a character.
    fn serialize_char_dyn(&mut self, v: char) -> Result<(), Error>;

    /// Serializes a string slice.
    fn serialize_str_dyn(&mut self, v: &str) -> Result<(), Error>;

    /// Serializes a byte slice.
    fn serialize_bytes_dyn(&mut self, v: &[u8]) -> Result<(), Error>;

    /// Serializes an optional value that is absent.
    fn serialize_none_dyn(&mut self) -> Result<(), Error>;

    /// Serializes an optional value that is present.
    fn serialize_some_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Serializes a unit value.
    fn serialize_unit_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a unit struct.
    fn serialize_unit_struct_dyn(&mut self, name: &'static str) -> Result<(), Error>;

    /// Serializes a unit variant.
    fn serialize_unit_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error>;

    /// Serializes a newtype struct.
    fn serialize_newtype_struct_dyn(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error>;

    /// Serializes a newtype variant.
    fn serialize_newtype_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error>;

    /// Begins to serialize a variably sized sequence.
    fn serialize_seq_dyn(&mut self, len: Option<usize>) -> Result<(), Error>;

    /// Begins to serialize a statically sized sequence.
    fn serialize_tuple_dyn(&mut self, len: usize) -> Result<(), Error>;

    /// Begins to serialize a tuple struct.
    fn serialize_tuple_struct_dyn(&mut self, name: &'static str, len: usize) -> Result<(), Error>;

    /// Begins to serialize a tuple variant.
    fn serialize_tuple_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<(), Error>;

    /// Begins to serialize a map.
    fn serialize_map_dyn(&mut self, len: Option<usize>) -> Result<(), Error>;

    /// Begins to serialize a struct.
    fn serialize_struct_dyn(&mut self, name: &'static str, len: usize) -> Result<(), Error>;

    /// Begins to serialize a struct variant.
    fn serialize_struct_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<(), Error>;

    /// Serializes a string produced by an implementation of `Display`.
    fn collect_str_dyn(&mut self, value: &dyn Display) -> Result<(), Error>;

    /// Determine whether `Serialize` implementations should serialize in
    /// human-readable form.
    ///
    /// This method always returns `false` if the serializer is unusable.
    ///
    /// See the documentation of [`serde::Serializer::is_human_readable`].
    fn is_human_readable_dyn(&self) -> bool;

    /// Serializes a sequence element.
    fn seq_serialize_element_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Finishes serializing a sequence.
    fn seq_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a tuple element.
    fn tuple_serialize_element_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Finishes serializing a tuple.
    fn tuple_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a tuple struct field.
    fn tuple_struct_serialize_field_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Finishes serializing a tuple struct.
    fn tuple_struct_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a tuple variant field.
    fn tuple_variant_serialize_field_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Finishes serializing a tuple variant.
    fn tuple_variant_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a map key.
    fn map_serialize_key_dyn(&mut self, key: &dyn Serialize) -> Result<(), Error>;

    /// Serializes a map value.
    fn map_serialize_value_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error>;

    /// Serializes a map entry consisting of a key and a value.
    fn map_serialize_entry_dyn(
        &mut self,
        k: &dyn Serialize,
        v: &dyn Serialize,
    ) -> Result<(), Error>;

    /// Finishes serializing a map.
    ///
    /// This method will consume the dynamic serializer. And later, serializing
    /// operations on this serializer will always return an error.
    fn map_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a struct field.
    fn struct_serialize_field_dyn(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error>;

    /// Indicates that a struct field has been skipped.
    fn struct_skip_field_dyn(&mut self, key: &'static str) -> Result<(), Error>;

    /// Finishes serializing a struct.
    fn struct_end_dyn(&mut self) -> Result<(), Error>;

    /// Serializes a struct variant field.
    fn struct_variant_serialize_field_dyn(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error>;

    /// Indicates that a struct variant field has been skipped.
    fn struct_variant_skip_field_dyn(&mut self, key: &'static str) -> Result<(), Error>;

    /// Finishes serializing a struct variant.
    fn struct_variant_end_dyn(&mut self) -> Result<(), Error>;
}

impl dyn Serializer {
    /// Converts the concrete serializer into dynamic serializer.
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::Serializer;
    ///
    /// let stdout = std::io::stdout();
    /// let mut serializer = serde_json::Serializer::new(stdout.lock());
    /// let serializer = <dyn Serializer>::new(&mut serializer);
    /// # let _ = serializer;
    /// ```
    #[inline]
    #[must_use]
    pub fn new<S>(serializer: S) -> MakeSerializer<S>
    where
        S: serde::Serializer,
    {
        MakeSerializer::Serializer(serializer)
    }
}

/// A serializer adaptor that performs in-place serialization.
///
/// This `enum` is the only implementation of the [`Serializer`] trait. To
/// construct dynamic serializers, it's supposed to use method [`new`] defined
/// on type `dyn Serializer`.
///
/// This serializer is implemented by a finite state machine:
///
/// * At the beginning, this serializer is initialized to `Serializer`;
///
/// * After serialization, this serializer is either `Value`;
///
/// * And after the result has been taken, the serializer becomes `None`;
///
/// [`new`]: trait.Serializer.html#method.new
#[derive(Clone, Debug, Default)]
pub enum MakeSerializer<S: ser::Serializer> {
    /// The serialization result has been taken.
    #[default]
    None,
    /// The serialization has done successfully.
    Value(S::Ok),
    /// The serializer is ready to perform serialization.
    Serializer(S),
    /// The serializer is serializing a sequence.
    SerializeSeq(S::SerializeSeq),
    /// The serializer is serializing a tuple.
    SerializeTuple(S::SerializeTuple),
    /// The serializer is serializing a tuple struct.
    SerializeTupleStruct(S::SerializeTupleStruct),
    /// The serializer is serializing a tuple variant.
    SerializeTupleVariant(S::SerializeTupleVariant),
    /// The serializer is serializing a map.
    SerializeMap(S::SerializeMap),
    /// The serializer is serializing a struct.
    SerializeStruct(S::SerializeStruct),
    /// The serializer is serializing a struct variant.
    SerializeStructVariant(S::SerializeStructVariant),
}

impl<S: ser::Serializer> MakeSerializer<S> {
    /// Consumes the serializer, returning the concrete serialization result.
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::{Serialize, Serializer};
    ///
    /// let mut buffer = Vec::with_capacity(64);
    /// let mut serializer = serde_json::Serializer::new(std::io::Cursor::new(&mut buffer));
    /// let mut serializer = <dyn Serializer>::new(&mut serializer);
    ///
    /// let value = "Hello, world!";
    /// value.serialize_dyn(&mut serializer).unwrap();
    ///
    /// // `serde_json::Serializer` returns `()`.
    /// serializer.expect().unwrap();
    /// assert_eq!(buffer, b"\"Hello, world!\"");
    pub fn expect(self) -> Option<S::Ok> {
        if let MakeSerializer::Value(value) = self {
            Some(value)
        } else {
            None
        }
    }
}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl<T: serde::Serialize> Serialize for T {
    #[inline]
    fn serialize_dyn(&self, serializer: &mut dyn Serializer) -> Result<(), Error> {
        self.serialize(serializer)
    }
}

#[allow(clippy::unit_arg)]
impl<S: serde::Serializer> Serializer for MakeSerializer<S> {
    fn serialize_bool_dyn(&mut self, v: bool) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_bool(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_i8_dyn(&mut self, v: i8) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_i8(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_i16_dyn(&mut self, v: i16) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_i16(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_i32_dyn(&mut self, v: i32) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_i32(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_i64_dyn(&mut self, v: i64) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_i64(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_i128_dyn(&mut self, v: i128) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_i128(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_u8_dyn(&mut self, v: u8) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_u8(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_u16_dyn(&mut self, v: u16) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_u16(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_u32_dyn(&mut self, v: u32) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_u32(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_u64_dyn(&mut self, v: u64) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_u64(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_u128_dyn(&mut self, v: u128) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_u128(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_f32_dyn(&mut self, v: f32) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_f32(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_f64_dyn(&mut self, v: f64) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_f64(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_char_dyn(&mut self, v: char) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_char(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_str_dyn(&mut self, v: &str) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_str(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_bytes_dyn(&mut self, v: &[u8]) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_bytes(v) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_none_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_none() {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_some_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_some(value) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_unit_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_unit() {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_unit_struct_dyn(&mut self, name: &'static str) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_unit_struct(name) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_unit_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_unit_variant(name, variant_index, variant) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_newtype_struct_dyn(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_newtype_struct(name, value) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_newtype_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_newtype_variant(name, variant_index, variant, value) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_seq_dyn(&mut self, len: Option<usize>) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_seq(len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeSeq(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_tuple_dyn(&mut self, len: usize) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_tuple(len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeTuple(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_tuple_struct_dyn(&mut self, name: &'static str, len: usize) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_tuple_struct(name, len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeTupleStruct(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_tuple_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_tuple_variant(name, variant_index, variant, len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeTupleVariant(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_map_dyn(&mut self, len: Option<usize>) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_map(len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeMap(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_struct_dyn(&mut self, name: &'static str, len: usize) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_struct(name, len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeStruct(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn serialize_struct_variant_dyn(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.serialize_struct_variant(name, variant_index, variant, len) {
                Ok(ok) => Ok(*self = MakeSerializer::SerializeStructVariant(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn collect_str_dyn(&mut self, value: &dyn Display) -> Result<(), Error> {
        if let MakeSerializer::Serializer(ser) = mem::take(self) {
            match ser.collect_str(value) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn is_human_readable_dyn(&self) -> bool {
        matches!(self, MakeSerializer::Serializer(ser) if ser.is_human_readable())
    }

    fn seq_serialize_element_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeSeq(ser) = self {
            ser::SerializeSeq::serialize_element(ser, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn seq_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeSeq(ser) = mem::take(self) {
            match ser::SerializeSeq::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn tuple_serialize_element_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeTuple(ser) = self {
            ser::SerializeTuple::serialize_element(ser, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn tuple_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeTuple(ser) = mem::take(self) {
            match ser::SerializeTuple::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn tuple_struct_serialize_field_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeTupleStruct(ser) = self {
            ser::SerializeTupleStruct::serialize_field(ser, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn tuple_struct_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeTupleStruct(ser) = mem::take(self) {
            match ser::SerializeTupleStruct::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn tuple_variant_serialize_field_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeTupleVariant(ser) = self {
            ser::SerializeTupleVariant::serialize_field(ser, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn tuple_variant_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeTupleVariant(ser) = mem::take(self) {
            match ser::SerializeTupleVariant::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn map_serialize_key_dyn(&mut self, key: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeMap(ser) = self {
            ser::SerializeMap::serialize_key(ser, key).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn map_serialize_value_dyn(&mut self, value: &dyn Serialize) -> Result<(), Error> {
        if let MakeSerializer::SerializeMap(ser) = self {
            ser::SerializeMap::serialize_value(ser, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn map_serialize_entry_dyn(
        &mut self,
        key: &dyn Serialize,
        value: &dyn Serialize,
    ) -> Result<(), Error> {
        if let MakeSerializer::SerializeMap(ser) = self {
            ser::SerializeMap::serialize_entry(ser, key, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn map_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeMap(ser) = mem::take(self) {
            match ser::SerializeMap::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn struct_serialize_field_dyn(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error> {
        if let MakeSerializer::SerializeStruct(ser) = self {
            ser::SerializeStruct::serialize_field(ser, key, value).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn struct_skip_field_dyn(&mut self, key: &'static str) -> Result<(), Error> {
        if let MakeSerializer::SerializeStruct(ser) = self {
            ser::SerializeStruct::skip_field(ser, key).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn struct_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeStruct(ser) = mem::take(self) {
            match ser::SerializeStruct::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }

    fn struct_variant_serialize_field_dyn(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> Result<(), Error> {
        if let MakeSerializer::SerializeStructVariant(ser) = self {
            ser::SerializeStructVariant::serialize_field(ser, key, value)
                .map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn struct_variant_skip_field_dyn(&mut self, key: &'static str) -> Result<(), Error> {
        if let MakeSerializer::SerializeStructVariant(ser) = self {
            ser::SerializeStructVariant::skip_field(ser, key).map_err(ser::Error::custom)
        } else {
            Err(Error::default())
        }
    }

    fn struct_variant_end_dyn(&mut self) -> Result<(), Error> {
        if let MakeSerializer::SerializeStructVariant(ser) = mem::take(self) {
            match ser::SerializeStructVariant::end(ser) {
                Ok(ok) => Ok(*self = MakeSerializer::Value(ok)),
                Err(err) => Err(ser::Error::custom(err)),
            }
        } else {
            Err(Error::default())
        }
    }
}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT OBJECT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl serde::Serialize for dyn Serialize + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = <dyn Serializer>::new(serializer);
        match self.serialize_dyn(&mut serializer) {
            Ok(()) => match serializer.expect() {
                Some(ok) => Ok(ok),
                None => Err(ser::Error::custom(Error::default())),
            },
            Err(err) => Err(err.into_ser_error()),
        }
    }
}

impl serde::Serializer for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.serialize_bool_dyn(v)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.serialize_i8_dyn(v)
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.serialize_i16_dyn(v)
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.serialize_i32_dyn(v)
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.serialize_i64_dyn(v)
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<(), Error> {
        self.serialize_i128_dyn(v)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.serialize_u8_dyn(v)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.serialize_u16_dyn(v)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.serialize_u32_dyn(v)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.serialize_u64_dyn(v)
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<(), Error> {
        self.serialize_u128_dyn(v)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<(), Error> {
        self.serialize_f32_dyn(v)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<(), Error> {
        self.serialize_f64_dyn(v)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.serialize_char_dyn(v)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<(), Error> {
        self.serialize_str_dyn(v)
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<(), Error> {
        self.serialize_bytes_dyn(v)
    }

    #[inline]
    fn serialize_none(self) -> Result<(), Error> {
        self.serialize_none_dyn()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_some_dyn(&value)
    }

    #[inline]
    fn serialize_unit(self) -> Result<(), Error> {
        self.serialize_unit_dyn()
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<(), Error> {
        self.serialize_unit_struct_dyn(name)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.serialize_unit_variant_dyn(name, variant_index, variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_newtype_struct_dyn(name, &value)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.serialize_newtype_variant_dyn(name, variant_index, variant, &value)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self, Error> {
        self.serialize_seq_dyn(len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self, Error> {
        self.serialize_tuple_dyn(len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self, Error> {
        self.serialize_tuple_struct_dyn(name, len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self, Error> {
        self.serialize_tuple_variant_dyn(name, variant_index, variant, len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self, Error> {
        self.serialize_map_dyn(len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self, Error> {
        self.serialize_struct_dyn(name, len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self, Error> {
        self.serialize_struct_variant_dyn(name, variant_index, variant, len)?;
        Ok(self)
    }

    #[inline]
    fn collect_str<T>(self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Display,
    {
        self.collect_str_dyn(&value)
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable_dyn()
    }
}

impl ser::SerializeSeq for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.seq_serialize_element_dyn(&value)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.seq_end_dyn()
    }
}

impl ser::SerializeTuple for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.tuple_serialize_element_dyn(&value)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.tuple_end_dyn()
    }
}

impl ser::SerializeTupleStruct for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.tuple_struct_serialize_field_dyn(&value)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.tuple_struct_end_dyn()
    }
}

impl ser::SerializeTupleVariant for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.tuple_variant_serialize_field_dyn(&value)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.tuple_variant_end_dyn()
    }
}

impl ser::SerializeMap for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.map_serialize_key_dyn(&key)
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.map_serialize_value_dyn(&value)
    }

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Error>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        self.map_serialize_entry_dyn(&key, &value)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.map_end_dyn()
    }
}

impl ser::SerializeStruct for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.struct_serialize_field_dyn(key, &value)
    }

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.struct_skip_field_dyn(key)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.struct_end_dyn()
    }
}

impl ser::SerializeStructVariant for &mut dyn Serializer {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.struct_variant_serialize_field_dyn(key, &value)
    }

    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Error> {
        self.struct_variant_skip_field_dyn(key)
    }

    #[inline]
    fn end(self) -> Result<(), Error> {
        self.struct_variant_end_dyn()
    }
}
