//! This module provides [`Deserializer`], which is the dyn-compatible version
//! of [`serde::Deserializer`].

use core::fmt::{self, Formatter};
use core::mem;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

use serde::de::{self, VariantAccess};

use crate::error::Error;

/// The dyn-compatible version of [`serde::Deserializer`].
///
/// # Example
///
/// To construct dynamic deserializer, use `<dyn Serializer>::new` instead of
/// implementing manually.
///
/// ```
/// use dyn_serde::Deserializer;
///
/// let mut deserializer = serde_json::Deserializer::from_str("false");
/// let deserializer = <dyn Deserializer>::new(&mut deserializer);
/// let deserializer: Box<dyn Deserializer> = Box::new(deserializer);
/// # let _ = deserializer;
/// ```
///
/// The trait object `dyn Deserializer` also implements [`serde::Deserializer`].
///
/// ```
/// use serde::Deserialize;
/// use dyn_serde::Deserializer;
///
/// let mut deserializer = serde_json::Deserializer::from_str("false");
/// let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
/// let deserializer: &mut dyn Deserializer = &mut deserializer;
///
/// let value = bool::deserialize(deserializer).unwrap();
/// assert_eq!(value, false);
/// ```
pub trait Deserializer<'de> {
    /// Requires the deserializer to figure out how to drive the visitor
    /// based on what data type is in the input.
    fn deserialize_any_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `bool`.
    fn deserialize_bool_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `i8`.
    fn deserialize_i8_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `i16`.
    fn deserialize_i16_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `i32`.
    fn deserialize_i32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `i64`.
    fn deserialize_i64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `i128`.
    fn deserialize_i128_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `u8`.
    fn deserialize_u8_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `u16`.
    fn deserialize_u16_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `u32`.
    fn deserialize_u32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `u64`.
    fn deserialize_u64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a `u128`.
    fn deserialize_u128_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `f32`.
    fn deserialize_f32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an `f64`.
    fn deserialize_f64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a character.
    fn deserialize_char_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a string slice.
    fn deserialize_str_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a string.
    fn deserialize_string_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a byte slice.
    fn deserialize_bytes_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a byte array.
    fn deserialize_byte_buf_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains an optional value.
    fn deserialize_option_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a unit value.
    fn deserialize_unit_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a unit struct with a particular name.
    fn deserialize_unit_struct_dyn(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains a newtype struct with a particular name.
    fn deserialize_newtype_struct_dyn(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains a sequence.
    fn deserialize_seq_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a sequence and knows how many values
    /// there are without looking at the serialized data.
    fn deserialize_tuple_dyn(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains a tuple struct with a particular name
    /// and number of fields.
    fn deserialize_tuple_struct_dyn(
        &mut self,
        name: &'static str,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains a map.
    fn deserialize_map_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the input contains a struct with a particular name and
    /// fields.
    fn deserialize_struct_dyn(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains an enum value with a particular name and
    /// possible variants.
    fn deserialize_enum_dyn(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Hints that the input contains an identifier of a struct field or the
    /// discriminant of an enum variant.
    fn deserialize_identifier_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Hints that the deserializer needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    fn deserialize_ignored_any_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error>;

    /// Determine whether the deserializer should expect to deserialize their
    /// human-readable form.
    fn is_human_readable_dyn(&self) -> bool;
}

impl<'de> dyn Deserializer<'de> + '_ {
    /// Converts the concrete deserializer into dynamic deserializer.
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::Deserializer;
    ///
    /// let mut deserializer = serde_json::Deserializer::from_str("false");
    /// let deserializer = <dyn Deserializer>::new(&mut deserializer);
    /// # let _ = deserializer;
    /// ```
    #[inline]
    #[must_use]
    pub fn new<D>(deserializer: D) -> Option<D>
    where
        D: serde::Deserializer<'de>,
    {
        Some(deserializer)
    }

    /// Deserializes the value from the given dynamic deserializer.
    ///
    /// This is equivalent to [`T::deserialize(self)`].
    ///
    /// [`T::deserialize(self)`]: serde::Deserialize::deserialize
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::Deserializer;
    ///
    /// let mut deserializer = serde_json::Deserializer::from_str("false");
    /// let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
    /// let deserializer: &mut dyn Deserializer = &mut deserializer;
    ///
    /// let value = deserializer.deserialize::<bool>().unwrap();
    /// assert_eq!(value, false);
    /// ```
    #[inline]
    pub fn deserialize<T>(&mut self) -> Result<T, Error>
    where
        T: de::Deserialize<'de>,
    {
        T::deserialize(self)
    }

    /// Deserializes the value from the given dynamic deserializer.
    ///
    /// This is equivalent to [`seed.deserialize(self)`].
    ///
    /// [`seed.deserialize(self)`]: serde::de::DeserializeSeed::deserialize
    ///
    /// # Example
    ///
    /// ```
    /// use std::marker::PhantomData;
    /// use dyn_serde::Deserializer;
    ///
    /// let mut deserializer = serde_json::Deserializer::from_str("false");
    /// let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
    /// let deserializer: &mut dyn Deserializer = &mut deserializer;
    ///
    /// let value = deserializer.deserialize_with(PhantomData::<bool>).unwrap();
    /// assert_eq!(value, false);
    /// ```
    #[inline]
    pub fn deserialize_with<T>(&mut self, seed: T) -> Result<T::Value, Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

/// The dyn-compatible version of [`serde::de::DeserializeSeed`].
pub trait DeserializeSeed<'de> {
    /// Performs stateful deserialization with given dynamic deserializer.
    ///
    /// # Errors
    ///
    /// This method returns an error if the concrete deserialization fails;
    /// Or the given deserializer or the deserialize seed has been consumed.
    fn deserialize_dyn(&mut self, deserializer: &mut dyn Deserializer<'de>) -> Result<(), Error>;
}

/// The dyn-compatible version of [`serde::de::Visitor`].
pub trait Visitor<'de> {
    /// Formats a message stating what data this visitor expects to receive.
    fn expecting_dyn(&self, formatter: &mut Formatter<'_>) -> fmt::Result;

    /// Visits the input which contains a `bool`.
    fn visit_bool_dyn(&mut self, v: bool) -> Result<(), Error>;

    /// Visits the input which contains an `i8`.
    fn visit_i8_dyn(&mut self, v: i8) -> Result<(), Error>;

    /// Visits the input which contains an `i16`.
    fn visit_i16_dyn(&mut self, v: i16) -> Result<(), Error>;

    /// Visits the input which contains an `i32`.
    fn visit_i32_dyn(&mut self, v: i32) -> Result<(), Error>;

    /// Visits the input which contains an `i64`.
    fn visit_i64_dyn(&mut self, v: i64) -> Result<(), Error>;

    /// Visits the input which contains an `i128`.
    fn visit_i128_dyn(&mut self, v: i128) -> Result<(), Error>;

    /// Visits the input which contains a `u8`.
    fn visit_u8_dyn(&mut self, v: u8) -> Result<(), Error>;

    /// Visits the input which contains a `u16`.
    fn visit_u16_dyn(&mut self, v: u16) -> Result<(), Error>;

    /// Visits the input which contains a `u32`.
    fn visit_u32_dyn(&mut self, v: u32) -> Result<(), Error>;

    /// Visits the input which contains a `u64`.
    fn visit_u64_dyn(&mut self, v: u64) -> Result<(), Error>;

    /// Visits the input which contains a `u128`.
    fn visit_u128_dyn(&mut self, v: u128) -> Result<(), Error>;

    /// Visits the input which contains an `f32`.
    fn visit_f32_dyn(&mut self, v: f32) -> Result<(), Error>;

    /// Visits the input which contains an `f64`.
    fn visit_f64_dyn(&mut self, v: f64) -> Result<(), Error>;

    /// Visits the input which contains a character.
    fn visit_char_dyn(&mut self, v: char) -> Result<(), Error>;

    /// Visits the input which contains a string slice.
    fn visit_str_dyn(&mut self, v: &str) -> Result<(), Error>;

    /// Visits the input which contains a string slice which lives at least as
    /// long as the deserializer.
    fn visit_borrowed_str_dyn(&mut self, v: &'de str) -> Result<(), Error>;

    /// Visits the input which contains a string.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string_dyn(&mut self, v: String) -> Result<(), Error>;

    /// Visits the input which contains a byte slice.
    fn visit_bytes_dyn(&mut self, v: &[u8]) -> Result<(), Error>;

    /// Visits the input which contains a byte slice which lives at least as
    /// long as the deserializer.
    fn visit_borrowed_bytes_dyn(&mut self, v: &'de [u8]) -> Result<(), Error>;

    /// Visits the input which contains a byte array.
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf_dyn(&mut self, v: Vec<u8>) -> Result<(), Error>;

    /// Visits the input which contains an optional value that is absent.
    fn visit_none_dyn(&mut self) -> Result<(), Error>;

    /// Visits the input which contains an optional value that is present.
    fn visit_some_dyn(&mut self, deserializer: &mut dyn Deserializer<'de>) -> Result<(), Error>;

    /// Visits the input which contains a unit value.
    fn visit_unit_dyn(&mut self) -> Result<(), Error>;

    /// Visits the input which contains a newtype struct.
    fn visit_newtype_struct_dyn(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> Result<(), Error>;

    /// Visits the input which contains a sequence.
    fn visit_seq_dyn(&mut self, access: &mut dyn SeqAccess<'de>) -> Result<(), Error>;

    /// Visits the input which contains a map.
    fn visit_map_dyn(&mut self, access: &mut dyn MapAccess<'de>) -> Result<(), Error>;

    /// Visits the input which contains an enum value.
    fn visit_enum_dyn(&mut self, access: &mut dyn EnumAccess<'de>) -> Result<(), Error>;
}

/// The dyn-compatible version of [`serde::de::SeqAccess`].
pub trait SeqAccess<'de> {
    /// This returns the next value in the sequence, or `None` if there are no
    /// more remaining items.
    fn next_element_dyn(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> Result<Option<()>, Error>;

    /// Returns the number of elements remaining in the sequence, if known.
    fn size_hint_dyn(&self) -> Option<usize>;
}

/// The dyn-compatible version of [`serde::de::MapAccess`].
pub trait MapAccess<'de> {
    /// This returns the next key in the map, or `None` if there are no more
    /// remaining items.
    fn next_key_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<Option<()>, Error>;

    /// This returns the next value in the map, or `None` if there are no more
    /// remaining items.
    fn next_value_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error>;

    /// This returns the next entry in the map, or `None` if there are no more
    /// remaining items.
    fn next_entry_dyn(
        &mut self,
        kseed: &mut dyn DeserializeSeed<'de>,
        vseed: &mut dyn DeserializeSeed<'de>,
    ) -> Result<Option<()>, Error>;

    /// Returns the number of elements remaining in the sequence, if known.
    fn size_hint_dyn(&self) -> Option<usize>;
}

/// The dyn-compatible version of [`serde::de::EnumAccess`] and
/// [`serde::de::VariantAccess`].
pub trait EnumAccess<'de> {
    /// Identifies which variant to deserialize.
    fn variant_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error>;

    /// Deserializes a variant with no values.
    fn unit_variant_dyn(&mut self) -> Result<(), Error>;

    /// Deserializes a variant with a single value.
    fn newtype_variant_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error>;

    /// Deserializes a tuple-like variant.
    fn tuple_variant_dyn(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;

    /// Deserializes a struct-like variant.
    fn struct_variant_dyn(
        &mut self,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error>;
}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl<'de, D: de::Deserializer<'de>> Deserializer<'de> for Option<D> {
    fn deserialize_any_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_any(visitor).map_err(de::Error::custom)
    }

    fn deserialize_bool_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_bool(visitor).map_err(de::Error::custom)
    }

    fn deserialize_i8_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_i8(visitor).map_err(de::Error::custom)
    }

    fn deserialize_i16_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_i16(visitor).map_err(de::Error::custom)
    }

    fn deserialize_i32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_i32(visitor).map_err(de::Error::custom)
    }

    fn deserialize_i64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_i64(visitor).map_err(de::Error::custom)
    }

    fn deserialize_i128_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_i128(visitor).map_err(de::Error::custom)
    }

    fn deserialize_u8_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_u8(visitor).map_err(de::Error::custom)
    }

    fn deserialize_u16_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_u16(visitor).map_err(de::Error::custom)
    }

    fn deserialize_u32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_u32(visitor).map_err(de::Error::custom)
    }

    fn deserialize_u64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_u64(visitor).map_err(de::Error::custom)
    }

    fn deserialize_u128_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_u128(visitor).map_err(de::Error::custom)
    }

    fn deserialize_f32_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_f32(visitor).map_err(de::Error::custom)
    }

    fn deserialize_f64_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_f64(visitor).map_err(de::Error::custom)
    }

    fn deserialize_char_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_char(visitor).map_err(de::Error::custom)
    }

    fn deserialize_str_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_str(visitor).map_err(de::Error::custom)
    }

    fn deserialize_string_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_string(visitor).map_err(de::Error::custom)
    }

    fn deserialize_bytes_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_bytes(visitor).map_err(de::Error::custom)
    }

    fn deserialize_byte_buf_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_byte_buf(visitor).map_err(de::Error::custom)
    }

    fn deserialize_option_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_option(visitor).map_err(de::Error::custom)
    }

    fn deserialize_unit_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_unit(visitor).map_err(de::Error::custom)
    }

    fn deserialize_unit_struct_dyn(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_unit_struct(name, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_newtype_struct_dyn(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_newtype_struct(name, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_seq_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_seq(visitor).map_err(de::Error::custom)
    }

    fn deserialize_tuple_dyn(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_tuple(len, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_tuple_struct_dyn(
        &mut self,
        name: &'static str,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_tuple_struct(name, len, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_map_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_map(visitor).map_err(de::Error::custom)
    }

    fn deserialize_struct_dyn(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_struct(name, fields, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_enum_dyn(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_enum(name, variants, visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_identifier_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_identifier(visitor)
            .map_err(de::Error::custom)
    }

    fn deserialize_ignored_any_dyn(&mut self, visitor: &mut dyn Visitor<'de>) -> Result<(), Error> {
        let de = self.take().ok_or_else(Error::default)?;
        de.deserialize_ignored_any(visitor)
            .map_err(de::Error::custom)
    }

    fn is_human_readable_dyn(&self) -> bool {
        matches!(self, Some(de) if de.is_human_readable())
    }
}

impl<'de, A: de::SeqAccess<'de>> SeqAccess<'de> for A {
    #[inline]
    fn next_element_dyn(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> Result<Option<()>, Error> {
        self.next_element_seed(seed).map_err(de::Error::custom)
    }

    #[inline]
    fn size_hint_dyn(&self) -> Option<usize> {
        self.size_hint()
    }
}

impl<'de, A: de::MapAccess<'de>> MapAccess<'de> for A {
    #[inline]
    fn next_key_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<Option<()>, Error> {
        self.next_key_seed(seed).map_err(de::Error::custom)
    }

    #[inline]
    fn next_value_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error> {
        self.next_value_seed(seed).map_err(de::Error::custom)
    }

    fn next_entry_dyn(
        &mut self,
        kseed: &mut dyn DeserializeSeed<'de>,
        vseed: &mut dyn DeserializeSeed<'de>,
    ) -> Result<Option<()>, Error> {
        match self.next_entry_seed(kseed, vseed) {
            Ok(Some(_)) => Ok(Some(())),
            Ok(None) => Ok(None),
            Err(err) => Err(de::Error::custom(err)),
        }
    }

    #[inline]
    fn size_hint_dyn(&self) -> Option<usize> {
        self.size_hint()
    }
}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT OBJECT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl<'de> de::Deserializer<'de> for &mut (dyn Deserializer<'de> + '_) {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_any_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_bool_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_i8_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_i16_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_i32_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_i64_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_i128_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_u8_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_u16_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_u32_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_u64_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_u128_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_f32_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_f64_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_char_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_str_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_string_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_bytes_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_byte_buf_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_option_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_unit_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_unit_struct_dyn(name, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_newtype_struct_dyn(name, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_seq_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_tuple_dyn(len, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_tuple_struct_dyn(name, len, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_map_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_struct_dyn(name, fields, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_enum_dyn(name, variants, &mut visitor)?;
        visitor.expect()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_identifier_dyn(&mut visitor)?;
        visitor.expect()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.deserialize_ignored_any_dyn(&mut visitor)?;
        visitor.expect()
    }

    #[inline]
    fn is_human_readable(&self) -> bool {
        self.is_human_readable_dyn()
    }
}

impl<'de> de::DeserializeSeed<'de> for &mut dyn DeserializeSeed<'de> {
    type Value = ();

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut deserializer = <dyn Deserializer>::new(deserializer);
        self.deserialize_dyn(&mut deserializer)
            .map_err(Error::into_de_error)
    }
}

impl<'de> de::Visitor<'de> for &mut dyn Visitor<'de> {
    type Value = ();

    #[inline]
    fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.expecting_dyn(formatter)
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_bool_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i16_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i32_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i128_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u8_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u16_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u32_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u128_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f32_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f64_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_char_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_bytes_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes_dyn(v)
            .map_err(Error::into_de_error)
    }

    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_byte_buf_dyn(v).map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_none_dyn().map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut deserializer = <dyn Deserializer>::new(deserializer);
        self.visit_some_dyn(&mut deserializer)
            .map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_unit_dyn().map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut deserializer = <dyn Deserializer>::new(deserializer);
        self.visit_newtype_struct_dyn(&mut deserializer)
            .map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        self.visit_seq_dyn(&mut access)
            .map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        self.visit_map_dyn(&mut access)
            .map_err(Error::into_de_error)
    }

    #[inline]
    fn visit_enum<A>(self, access: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>,
    {
        let mut access = MakeEnumAccess::EnumAccess(access);
        self.visit_enum_dyn(&mut access)
            .map_err(Error::into_de_error)
    }
}

impl<'de> de::SeqAccess<'de> for &mut dyn SeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let mut seed = MakeDeserializeSeed::Seed(seed);
        match self.next_element_dyn(&mut seed)? {
            Some(()) => seed.expect().map(Some),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.size_hint_dyn()
    }
}

impl<'de> de::MapAccess<'de> for &mut dyn MapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let mut seed = MakeDeserializeSeed::Seed(seed);
        match self.next_key_dyn(&mut seed)? {
            Some(()) => seed.expect().map(Some),
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let mut seed = MakeDeserializeSeed::Seed(seed);
        self.next_value_dyn(&mut seed)?;
        seed.expect()
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Error>
    where
        K: de::DeserializeSeed<'de>,
        V: de::DeserializeSeed<'de>,
    {
        let mut kseed = MakeDeserializeSeed::Seed(kseed);
        let mut vseed = MakeDeserializeSeed::Seed(vseed);
        Ok(match self.next_entry_dyn(&mut kseed, &mut vseed)? {
            Some(()) => Some((kseed.expect()?, vseed.expect()?)),
            None => None,
        })
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.size_hint_dyn()
    }
}

impl<'de> de::EnumAccess<'de> for &mut dyn EnumAccess<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self), Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let mut seed = MakeDeserializeSeed::Seed(seed);
        self.variant_dyn(&mut seed)?;
        let value = seed.expect()?;
        Ok((value, self))
    }
}

impl<'de> de::VariantAccess<'de> for &mut dyn EnumAccess<'de> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Error> {
        self.unit_variant_dyn()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let mut seed = MakeDeserializeSeed::Seed(seed);
        self.newtype_variant_dyn(&mut seed)?;
        seed.expect()
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.tuple_variant_dyn(len, &mut visitor)?;
        visitor.expect()
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        let mut visitor = MakeVisitor::Visitor(visitor);
        self.struct_variant_dyn(fields, &mut visitor)?;
        visitor.expect()
    }
}

// /////////////////////////////////////////////////////////////////////////////
// PRIVATE
// /////////////////////////////////////////////////////////////////////////////

/// This `enum` is the only implementation of [`DeserializeSeed`].
enum MakeDeserializeSeed<'de, T: de::DeserializeSeed<'de>> {
    None,
    Seed(T),
    Value(T::Value),
}

impl<'de, V: de::DeserializeSeed<'de>> MakeDeserializeSeed<'de, V> {
    fn take(&mut self) -> Result<V, Error> {
        match mem::replace(self, MakeDeserializeSeed::None) {
            MakeDeserializeSeed::Seed(visitor) => Ok(visitor),
            MakeDeserializeSeed::None | MakeDeserializeSeed::Value(_) => Err(Error::default()),
        }
    }

    fn write(&mut self, value: V::Value) {
        *self = MakeDeserializeSeed::Value(value);
    }

    fn expect(self) -> Result<V::Value, Error> {
        match self {
            MakeDeserializeSeed::Value(value) => Ok(value),
            MakeDeserializeSeed::None | MakeDeserializeSeed::Seed(_) => Err(Error::default()),
        }
    }
}

impl<'de, T: de::DeserializeSeed<'de>> DeserializeSeed<'de> for MakeDeserializeSeed<'de, T> {
    fn deserialize_dyn(&mut self, deserializer: &mut dyn Deserializer<'de>) -> Result<(), Error> {
        self.take()?
            .deserialize(deserializer)
            .map(|value| self.write(value))
    }
}

/// This `enum` is the only implementation of [`Visitor`].
enum MakeVisitor<'de, V: de::Visitor<'de>> {
    None,
    Visitor(V),
    Value(V::Value),
}

impl<'de, V: de::Visitor<'de>> MakeVisitor<'de, V> {
    fn take(&mut self) -> Result<V, Error> {
        match mem::replace(self, MakeVisitor::None) {
            MakeVisitor::Visitor(visitor) => Ok(visitor),
            MakeVisitor::None | MakeVisitor::Value(_) => Err(Error::default()),
        }
    }

    fn write(&mut self, value: V::Value) {
        *self = MakeVisitor::Value(value);
    }

    fn expect(self) -> Result<V::Value, Error> {
        match self {
            MakeVisitor::Value(value) => Ok(value),
            MakeVisitor::None | MakeVisitor::Visitor(_) => Err(Error::default()),
        }
    }
}

impl<'de, T: de::Visitor<'de>> Visitor<'de> for MakeVisitor<'de, T> {
    fn expecting_dyn(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if let MakeVisitor::Visitor(ref visitor) = self {
            visitor.expecting(formatter)
        } else {
            formatter.write_str("nothing (the visitor has been consumed)")
        }
    }

    fn visit_bool_dyn(&mut self, v: bool) -> Result<(), Error> {
        self.take()?.visit_bool(v).map(|value| self.write(value))
    }

    fn visit_i8_dyn(&mut self, v: i8) -> Result<(), Error> {
        self.take()?.visit_i8(v).map(|value| self.write(value))
    }

    fn visit_i16_dyn(&mut self, v: i16) -> Result<(), Error> {
        self.take()?.visit_i16(v).map(|value| self.write(value))
    }

    fn visit_i32_dyn(&mut self, v: i32) -> Result<(), Error> {
        self.take()?.visit_i32(v).map(|value| self.write(value))
    }

    fn visit_i64_dyn(&mut self, v: i64) -> Result<(), Error> {
        self.take()?.visit_i64(v).map(|value| self.write(value))
    }

    fn visit_i128_dyn(&mut self, v: i128) -> Result<(), Error> {
        self.take()?.visit_i128(v).map(|value| self.write(value))
    }

    fn visit_u8_dyn(&mut self, v: u8) -> Result<(), Error> {
        self.take()?.visit_u8(v).map(|value| self.write(value))
    }

    fn visit_u16_dyn(&mut self, v: u16) -> Result<(), Error> {
        self.take()?.visit_u16(v).map(|value| self.write(value))
    }

    fn visit_u32_dyn(&mut self, v: u32) -> Result<(), Error> {
        self.take()?.visit_u32(v).map(|value| self.write(value))
    }

    fn visit_u64_dyn(&mut self, v: u64) -> Result<(), Error> {
        self.take()?.visit_u64(v).map(|value| self.write(value))
    }

    fn visit_u128_dyn(&mut self, v: u128) -> Result<(), Error> {
        self.take()?.visit_u128(v).map(|value| self.write(value))
    }

    fn visit_f32_dyn(&mut self, v: f32) -> Result<(), Error> {
        self.take()?.visit_f32(v).map(|value| self.write(value))
    }

    fn visit_f64_dyn(&mut self, v: f64) -> Result<(), Error> {
        self.take()?.visit_f64(v).map(|value| self.write(value))
    }

    fn visit_char_dyn(&mut self, v: char) -> Result<(), Error> {
        self.take()?.visit_char(v).map(|value| self.write(value))
    }

    fn visit_str_dyn(&mut self, v: &str) -> Result<(), Error> {
        self.take()?.visit_str(v).map(|value| self.write(value))
    }

    fn visit_borrowed_str_dyn(&mut self, v: &'de str) -> Result<(), Error> {
        self.take()?
            .visit_borrowed_str(v)
            .map(|value| self.write(value))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_string_dyn(&mut self, v: String) -> Result<(), Error> {
        self.take()?.visit_string(v).map(|value| self.write(value))
    }

    fn visit_bytes_dyn(&mut self, v: &[u8]) -> Result<(), Error> {
        self.take()?.visit_bytes(v).map(|value| self.write(value))
    }

    fn visit_borrowed_bytes_dyn(&mut self, v: &'de [u8]) -> Result<(), Error> {
        self.take()?
            .visit_borrowed_bytes(v)
            .map(|value| self.write(value))
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf_dyn(&mut self, v: Vec<u8>) -> Result<(), Error> {
        self.take()?
            .visit_byte_buf(v)
            .map(|value| self.write(value))
    }

    fn visit_none_dyn(&mut self) -> Result<(), Error> {
        self.take()?.visit_none().map(|value| self.write(value))
    }

    fn visit_some_dyn(&mut self, deserializer: &mut dyn Deserializer<'de>) -> Result<(), Error> {
        self.take()?
            .visit_some(deserializer)
            .map(|value| self.write(value))
    }

    fn visit_unit_dyn(&mut self) -> Result<(), Error> {
        self.take()?.visit_unit().map(|value| self.write(value))
    }

    fn visit_newtype_struct_dyn(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> Result<(), Error> {
        self.take()?
            .visit_newtype_struct(deserializer)
            .map(|value| self.write(value))
    }

    fn visit_seq_dyn(&mut self, access: &mut dyn SeqAccess<'de>) -> Result<(), Error> {
        self.take()?
            .visit_seq(access)
            .map(|value| self.write(value))
    }

    fn visit_map_dyn(&mut self, access: &mut dyn MapAccess<'de>) -> Result<(), Error> {
        self.take()?
            .visit_map(access)
            .map(|value| self.write(value))
    }

    fn visit_enum_dyn(&mut self, access: &mut dyn EnumAccess<'de>) -> Result<(), Error> {
        self.take()?
            .visit_enum(access)
            .map(|value| self.write(value))
    }
}

/// This `enum` is the only implementation of [`EnumAccess`].
enum MakeEnumAccess<'de, A: de::EnumAccess<'de>> {
    None,
    EnumAccess(A),
    VariantAccess(A::Variant),
}

impl<'de, A: de::EnumAccess<'de>> MakeEnumAccess<'de, A> {
    fn enum_access(&mut self) -> Result<A, Error> {
        match mem::replace(self, MakeEnumAccess::None) {
            MakeEnumAccess::EnumAccess(access) => Ok(access),
            MakeEnumAccess::None | MakeEnumAccess::VariantAccess(_) => Err(Error::default()),
        }
    }

    fn variant_access(&mut self) -> Result<A::Variant, Error> {
        match mem::replace(self, MakeEnumAccess::None) {
            MakeEnumAccess::VariantAccess(access) => Ok(access),
            MakeEnumAccess::None | MakeEnumAccess::EnumAccess(_) => Err(Error::default()),
        }
    }
}

impl<'de, A: de::EnumAccess<'de>> EnumAccess<'de> for MakeEnumAccess<'de, A> {
    fn variant_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error> {
        match self.enum_access()?.variant_seed(seed) {
            #[allow(clippy::unit_arg)]
            Ok(((), access)) => Ok(*self = MakeEnumAccess::VariantAccess(access)),
            Err(err) => Err(de::Error::custom(err)),
        }
    }

    fn unit_variant_dyn(&mut self) -> Result<(), Error> {
        self.variant_access()?
            .unit_variant()
            .map_err(de::Error::custom)
    }

    fn newtype_variant_dyn(&mut self, seed: &mut dyn DeserializeSeed<'de>) -> Result<(), Error> {
        self.variant_access()?
            .newtype_variant_seed(seed)
            .map_err(de::Error::custom)
    }

    fn tuple_variant_dyn(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        self.variant_access()?
            .tuple_variant(len, visitor)
            .map_err(de::Error::custom)
    }

    fn struct_variant_dyn(
        &mut self,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> Result<(), Error> {
        self.variant_access()?
            .struct_variant(fields, visitor)
            .map_err(de::Error::custom)
    }
}
