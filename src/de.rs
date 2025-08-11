//! For dynamic deserialization, see [`Deserializer`] and [`DeserializeSeed`].

use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use serde::de::VariantAccess as _;

/// The result type returned by [`dyn Deserializer`]'s methods.
///
/// [`dyn Deserializer`]: Deserializer
pub type DeserializeResult<T> = Result<T, DeserializeError>;

/// The result type returned by [`InplaceDeserializer`]'s methods.
pub type InplaceDeserializeResult<T> = Result<T, InplaceDeserializeError>;

/// The dyn-compatible version of [`serde::Deserializer`].
///
/// One should avoid implementing `Deserializer` manually and use
/// `<dyn Deserializer>::new` to construct an instance instead.
///
/// # Example
///
/// ```
/// # use serde::Deserialize as _;
/// # use dyn_serde::Deserializer;
/// #
/// let mut deserializer = serde_json::Deserializer::from_str("\"Hello, world!\"");
/// let mut deserializer = <dyn Deserializer<'_>>::new(&mut deserializer);
/// let deserializer = &mut deserializer as &mut dyn Deserializer<'_>;
///
/// let value = String::deserialize(deserializer).unwrap();
/// assert_eq!(value, "Hello, world!");
/// ```
#[diagnostic::on_unimplemented(note = "Consider using `<dyn Deserializer>::new`")]
pub trait Deserializer<'de> {
    /// Require the `Deserializer` to figure out how to drive the visitor based
    /// on what data type is in the input.
    fn dyn_deserialize_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `bool` value.
    fn dyn_deserialize_bool(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `i8` value.
    fn dyn_deserialize_i8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `i16` value.
    fn dyn_deserialize_i16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `i32` value.
    fn dyn_deserialize_i32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `i64` value.
    fn dyn_deserialize_i64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `i128` value.
    fn dyn_deserialize_i128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `u8` value.
    fn dyn_deserialize_u8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `u16` value.
    fn dyn_deserialize_u16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `u32` value.
    fn dyn_deserialize_u32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `u64` value.
    fn dyn_deserialize_u64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an `u128` value.
    fn dyn_deserialize_u128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `f32` value.
    fn dyn_deserialize_f32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `f64` value.
    fn dyn_deserialize_f64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a `char` value.
    fn dyn_deserialize_char(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a string value and does
    /// not benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    fn dyn_deserialize_str(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a string value and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    fn dyn_deserialize_string(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    fn dyn_deserialize_bytes(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a byte array and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    fn dyn_deserialize_byte_buf(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an optional value.
    fn dyn_deserialize_option(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a unit value.
    fn dyn_deserialize_unit(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn dyn_deserialize_unit_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn dyn_deserialize_newtype_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a sequence of values.
    fn dyn_deserialize_seq(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a sequence of values and
    /// knows how many values there are without looking at the serialized data.
    fn dyn_deserialize_tuple(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a tuple struct with a
    /// particular name and number of fields.
    fn dyn_deserialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
    fn dyn_deserialize_map(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    fn dyn_deserialize_struct(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn dyn_deserialize_enum(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field or the discriminant of an enum variant.
    fn dyn_deserialize_identifier(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    fn dyn_deserialize_ignored_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Determine whether `Deserialize` implementations should expect to
    /// deserialize their human-readable form.
    fn dyn_is_human_readable(&self) -> bool;
}

impl<'de> dyn Deserializer<'de> + '_ {
    /// A convenient way to construct an instance of [`dyn Deserializer<'de>`].
    ///
    /// This function is equivalent to [`InplaceDeserializer::Deserializer`].
    ///
    /// [`dyn Deserializer<'de>`]: Deserializer
    ///
    /// # Examples
    ///
    /// ```
    /// # use dyn_serde::Deserializer;
    /// #
    /// let mut deserializer = serde_json::Deserializer::from_str("false");
    /// let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
    /// let deserializer = &mut deserializer as &mut dyn Deserializer<'_>;
    /// # let _ = deserializer;
    /// ```
    #[must_use]
    pub fn new<D>(deserializer: D) -> InplaceDeserializer<'de, D>
    where
        D: serde::Deserializer<'de>,
    {
        InplaceDeserializer::Deserializer(deserializer)
    }
}

/// The dyn-compatible version of trait [`serde::de::DeserializeSeed`].
///
/// One should avoid implementing `Visitor` manually and use
/// [`InplaceDeserializeSeed::DeserializeSeed`] to construct an instance instead.
pub trait DeserializeSeed<'de> {
    /// Performs stateful deserialization with given dynamic deserializer.
    ///
    /// # Errors
    ///
    /// This method returns an error if the concrete deserialization fails;
    /// Or the given deserializer or the deserialize seed has been consumed.
    fn dyn_deserialize(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> DeserializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::de::Visitor`].
///
/// One should avoid implementing `Visitor` manually and use
/// [`InplaceVisitor::Visitor`] to construct an instance instead.
pub trait Visitor<'de> {
    /// Formats a message stating what data this Visitor expects to receive.
    fn dyn_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    /// The input contains a boolean.
    fn dyn_visit_bool(&mut self, v: bool) -> DeserializeResult<()>;

    /// The input contains an `i8`.
    fn dyn_visit_i8(&mut self, v: i8) -> DeserializeResult<()>;

    /// The input contains an `i16`.
    fn dyn_visit_i16(&mut self, v: i16) -> DeserializeResult<()>;

    /// The input contains an `i32`.
    fn dyn_visit_i32(&mut self, v: i32) -> DeserializeResult<()>;

    /// The input contains an `i64`.
    fn dyn_visit_i64(&mut self, v: i64) -> DeserializeResult<()>;

    /// The input contains a `i128`.
    fn dyn_visit_i128(&mut self, v: i128) -> DeserializeResult<()>;

    /// The input contains a `u8`.
    fn dyn_visit_u8(&mut self, v: u8) -> DeserializeResult<()>;

    /// The input contains a `u16`.
    fn dyn_visit_u16(&mut self, v: u16) -> DeserializeResult<()>;

    /// The input contains a `u32`.
    fn dyn_visit_u32(&mut self, v: u32) -> DeserializeResult<()>;

    /// The input contains a `u64`.
    fn dyn_visit_u64(&mut self, v: u64) -> DeserializeResult<()>;

    /// The input contains a `u128`.
    fn dyn_visit_u128(&mut self, v: u128) -> DeserializeResult<()>;

    /// The input contains an `f32`.
    fn dyn_visit_f32(&mut self, v: f32) -> DeserializeResult<()>;

    /// The input contains an `f64`.
    fn dyn_visit_f64(&mut self, v: f64) -> DeserializeResult<()>;

    /// The input contains a `char`.
    fn dyn_visit_char(&mut self, v: char) -> DeserializeResult<()>;

    /// The input contains a string. The lifetime of the string is ephemeral and
    /// it may be destroyed after this method returns.
    fn dyn_visit_str(&mut self, v: &str) -> DeserializeResult<()>;

    /// The input contains a string that lives at least as long as the
    /// `Deserializer`.
    fn dyn_visit_borrowed_str(&mut self, v: &'de str) -> DeserializeResult<()>;

    /// The input contains a string and ownership of the string is being given
    /// to the `Visitor`.
    fn dyn_visit_string(&mut self, v: String) -> DeserializeResult<()>;

    /// The input contains a byte array. The lifetime of the byte array is
    /// ephemeral and it may be destroyed after this method returns.
    fn dyn_visit_bytes(&mut self, v: &[u8]) -> DeserializeResult<()>;

    /// The input contains a byte array that lives at least as long as the
    /// `Deserializer`.
    fn dyn_visit_borrowed_bytes(&mut self, v: &'de [u8]) -> DeserializeResult<()>;

    /// The input contains a byte array and ownership of the byte array is being
    /// given to the `Visitor`.
    fn dyn_visit_byte_buf(&mut self, v: Vec<u8>) -> DeserializeResult<()>;

    /// The input contains an optional that is absent.
    fn dyn_visit_none(&mut self) -> DeserializeResult<()>;

    /// The input contains an optional that is present.
    fn dyn_visit_some(&mut self, deserializer: &mut dyn Deserializer<'de>)
        -> DeserializeResult<()>;

    /// The input contains a unit `()`.
    fn dyn_visit_unit(&mut self) -> DeserializeResult<()>;

    /// The input contains a newtype struct.
    fn dyn_visit_newtype_struct(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> DeserializeResult<()>;

    /// The input contains a sequence of elements.
    fn dyn_visit_seq(&mut self, access: &mut dyn SeqAccess<'de>) -> DeserializeResult<()>;

    /// The input contains a key-value map.
    fn dyn_visit_map(&mut self, access: &mut dyn MapAccess<'de>) -> DeserializeResult<()>;

    /// The input contains an enum.
    fn dyn_visit_enum(&mut self, access: &mut dyn EnumAccess<'de>) -> DeserializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::de::SeqAccess`].
///
/// The trait object is created by the [`Deserializer`] and passed to the
/// [`Visitor`] in order to deserialize the content of the sequence.
///
/// One should avoid implementing `SeqAccess` manually and use
/// [`InplaceSeqAccess::SeqAccess`] to construct an instance instead.
pub trait SeqAccess<'de> {
    /// This returns `Ok(Some(_))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    fn dyn_next_element(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<()>>;

    /// Returns the number of elements remaining in the sequence, if known.
    fn dyn_size_hint(&self) -> Option<usize>;
}

/// The dyn-compatible version of trait [`serde::de::MapAccess`].
///
/// The trait object is created by the [`Deserializer`] and passed to the
/// [`Visitor`] in order to deserialize the content of the map.
///
/// One should avoid implementing `MapAccess` manually and use
/// [`InplaceMapAccess::MapAccess`] to construct an instance instead.
pub trait MapAccess<'de> {
    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    fn dyn_next_key(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<()>>;

    /// This returns a `Ok(value)` for the next value in the map.
    fn dyn_next_value(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// This returns `Ok(Some(_))` for the next (key-value) pair in the map,
    /// or `Ok(None)` if there are no more remaining items.
    fn dyn_next_entry(
        &mut self,
        kseed: &mut dyn DeserializeSeed<'de>,
        vseed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<((), ())>>;

    /// Returns the number of entries remaining in the map, if known.
    fn dyn_size_hint(&self) -> Option<usize>;
}

/// The dyn-compatible version of trait [`serde::de::EnumAccess`].
///
/// The trait object is created by the [`Deserializer`] and passed to the
/// [`Visitor`] in order to identify which variant of an enum to deserialize.
///
/// One should avoid implementing `EnumAccess` manually and use
/// [`InplaceEnumAccess::EnumAccess`] to construct an instance instead.
pub trait EnumAccess<'de> {
    /// `variant` is called to identify which variant to deserialize.
    fn dyn_variant(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<&mut dyn VariantAccess<'de>>;
}

/// The dyn-compatible version of trait [`serde::de::VariantAccess`].
///
/// The trait object is returned by [`dyn_variant`] defined on trait
/// [`EnumAccess`].
///
/// One should avoid implementing `VariantAccess` manually and use
/// [`InplaceEnumAccess::VariantAccess`] to construct an instance instead.
///
/// [`dyn_variant`]: EnumAccess::dyn_variant
pub trait VariantAccess<'de> {
    /// Called when deserializing a variant with no values.
    fn dyn_unit_variant(&mut self) -> InplaceDeserializeResult<()>;

    /// Called when deserializing a variant with a single value.
    fn dyn_newtype_variant(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Called when deserializing a tuple-like variant.
    ///
    /// The `len` is the number of fields expected in the tuple variant.
    fn dyn_tuple_variant(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;

    /// Called when deserializing a struct-like variant.
    ///
    /// The `fields` are the names of the fields of the struct variant.
    fn dyn_struct_variant(
        &mut self,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()>;
}

// enum InplaceDeserializer
// ----------------------------------------------------------------------------
/// An implementation of the [`Deserializer`] trait which performs in-place
/// deserialization.
#[derive(Clone, Debug, Default)]
pub enum InplaceDeserializer<'de, D: serde::Deserializer<'de>> {
    /// The deserializer is not ready.
    #[default]
    None,
    /// The deserialization has done unsuccessfully.
    Error(D::Error),
    /// The deserializer is ready.
    Deserializer(D),
}

impl<'de, D: serde::Deserializer<'de>> InplaceDeserializer<'de, D> {
    fn into_result(self, result: DeserializeResult<()>) -> Result<(), D::Error> {
        result.map_err(|error| match self {
            InplaceDeserializer::Error(error) => error,
            _ => error.into_error(),
        })
    }

    fn deserialize_with<F>(&mut self, f: F) -> InplaceDeserializeResult<()>
    where
        F: FnOnce(D) -> Result<(), D::Error>,
    {
        if let InplaceDeserializer::Deserializer(_) = self {
            if let InplaceDeserializer::Deserializer(deserializer) = mem::take(self) {
                return (f)(deserializer).map_err(|error| {
                    *self = InplaceDeserializer::Error(error);
                    InplaceDeserializeError::Error
                });
            }
        }
        Err(InplaceDeserializeError::NotDeserializer)
    }
}

impl<'de, D: serde::Deserializer<'de>> Deserializer<'de> for InplaceDeserializer<'de, D> {
    fn dyn_deserialize_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_any(visitor))
    }

    fn dyn_deserialize_bool(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_bool(visitor))
    }

    fn dyn_deserialize_i8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_i8(visitor))
    }

    fn dyn_deserialize_i16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_i16(visitor))
    }

    fn dyn_deserialize_i32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_i32(visitor))
    }

    fn dyn_deserialize_i64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_i64(visitor))
    }

    fn dyn_deserialize_i128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_i128(visitor))
    }

    fn dyn_deserialize_u8(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_u8(visitor))
    }

    fn dyn_deserialize_u16(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_u16(visitor))
    }

    fn dyn_deserialize_u32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_u32(visitor))
    }

    fn dyn_deserialize_u64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_u64(visitor))
    }

    fn dyn_deserialize_u128(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_u128(visitor))
    }

    fn dyn_deserialize_f32(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_f32(visitor))
    }

    fn dyn_deserialize_f64(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_f64(visitor))
    }

    fn dyn_deserialize_char(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_char(visitor))
    }

    fn dyn_deserialize_str(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_str(visitor))
    }

    fn dyn_deserialize_string(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_string(visitor))
    }

    fn dyn_deserialize_bytes(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_bytes(visitor))
    }

    fn dyn_deserialize_byte_buf(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_byte_buf(visitor))
    }

    fn dyn_deserialize_option(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_option(visitor))
    }

    fn dyn_deserialize_unit(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_unit(visitor))
    }

    fn dyn_deserialize_unit_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_unit_struct(name, visitor))
    }

    fn dyn_deserialize_newtype_struct(
        &mut self,
        name: &'static str,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_newtype_struct(name, visitor))
    }

    fn dyn_deserialize_seq(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_seq(visitor))
    }

    fn dyn_deserialize_tuple(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_tuple(len, visitor))
    }

    fn dyn_deserialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_tuple_struct(name, len, visitor))
    }

    fn dyn_deserialize_map(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_map(visitor))
    }

    fn dyn_deserialize_struct(
        &mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_struct(name, fields, visitor))
    }

    fn dyn_deserialize_enum(
        &mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_enum(name, variants, visitor))
    }

    fn dyn_deserialize_identifier(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_identifier(visitor))
    }

    fn dyn_deserialize_ignored_any(
        &mut self,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.deserialize_with(|de| de.deserialize_ignored_any(visitor))
    }

    fn dyn_is_human_readable(&self) -> bool {
        if let InplaceDeserializer::Deserializer(deserializer) = self {
            deserializer.is_human_readable()
        } else {
            true
        }
    }
}

// enum InplaceDeserializeSeed
// ----------------------------------------------------------------------------
/// An implementation of the [`DeserializeSeed`] trait which performs in-place
/// deserialization.
#[derive(Debug, Default)]
pub enum InplaceDeserializeSeed<'de, T: serde::de::DeserializeSeed<'de>> {
    /// The deserializer is not ready.
    #[default]
    None,
    /// The deserialization has done successfully.
    Value(T::Value),
    /// The deserialize seed is ready.
    DeserializeSeed(T),
}

impl<'de, T: serde::de::DeserializeSeed<'de>> InplaceDeserializeSeed<'de, T> {
    fn into_result(self, result: InplaceDeserializeResult<()>) -> DeserializeResult<T::Value> {
        match self {
            InplaceDeserializeSeed::Value(value) => Ok(value),
            // This never panics because `result` is `Ok(_)` if and only if
            // `self` is `Value(_)`. And we have checked that it is't because
            // of the above branch.
            _ => Err(DeserializeError::from(result.unwrap_err())),
        }
    }

    fn into_result_option(
        self,
        result: InplaceDeserializeResult<Option<impl Debug>>,
    ) -> DeserializeResult<Option<T::Value>> {
        match result {
            Ok(None) => Ok(None),
            result => match self {
                InplaceDeserializeSeed::Value(value) => Ok(Some(value)),
                // This never panics because `result` is `Ok(_)` if and only if
                // `self` is `Value(_)`. And we have checked that it is't
                // because of the above branch.
                _ => Err(DeserializeError::from(result.unwrap_err())),
            },
        }
    }
}

impl<'de, T: serde::de::DeserializeSeed<'de>> DeserializeSeed<'de>
    for InplaceDeserializeSeed<'de, T>
{
    fn dyn_deserialize(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> DeserializeResult<()> {
        if let InplaceDeserializeSeed::DeserializeSeed(_) = self {
            if let InplaceDeserializeSeed::DeserializeSeed(seed) = mem::take(self) {
                *self = InplaceDeserializeSeed::Value(seed.deserialize(deserializer)?);
                return Ok(());
            }
        }
        Err(DeserializeError::from(
            InplaceDeserializeError::NotDeserializeSeed,
        ))
    }
}

// enum InplaceVisitor
// ----------------------------------------------------------------------------
/// An implementation of the [`Visitor`] trait which performs in-place
/// deserialization.
#[derive(Debug, Default)]
pub enum InplaceVisitor<'de, V: serde::de::Visitor<'de>> {
    /// The visitor is not ready.
    #[default]
    None,
    /// The deserialization has done successfully.
    Value(V::Value),
    /// The visitor is ready.
    Visitor(V),
}

impl<'de, V: serde::de::Visitor<'de>> InplaceVisitor<'de, V> {
    fn into_result(self, result: InplaceDeserializeResult<()>) -> DeserializeResult<V::Value> {
        match self {
            InplaceVisitor::Value(value) => Ok(value),
            // This never panics because `result` is `Ok(_)` if and only if
            // `self` is `Value(_)`. And we have checked that it is't because
            // of the above branch.
            _ => Err(DeserializeError::from(result.unwrap_err())),
        }
    }

    fn visit_with<F>(&mut self, f: F) -> DeserializeResult<()>
    where
        F: FnOnce(V) -> DeserializeResult<V::Value>,
    {
        if let InplaceVisitor::Visitor(_) = self {
            if let InplaceVisitor::Visitor(visitor) = mem::take(self) {
                *self = InplaceVisitor::Value((f)(visitor)?);
                return Ok(());
            }
        }
        Err(DeserializeError::from(InplaceDeserializeError::NotVisitor))
    }
}

impl<'de, V: serde::de::Visitor<'de>> Visitor<'de> for InplaceVisitor<'de, V> {
    fn dyn_expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InplaceVisitor::None => formatter.write_str("nothing (the visitor is not ready)"),
            InplaceVisitor::Value(_) => {
                formatter.write_str("nothing (the deserialization has done successfully)")
            }
            InplaceVisitor::Visitor(visitor) => visitor.expecting(formatter),
        }
    }

    fn dyn_visit_bool(&mut self, v: bool) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_bool(v))
    }

    fn dyn_visit_i8(&mut self, v: i8) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_i8(v))
    }

    fn dyn_visit_i16(&mut self, v: i16) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_i16(v))
    }

    fn dyn_visit_i32(&mut self, v: i32) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_i32(v))
    }

    fn dyn_visit_i64(&mut self, v: i64) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_i64(v))
    }

    fn dyn_visit_i128(&mut self, v: i128) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_i128(v))
    }

    fn dyn_visit_u8(&mut self, v: u8) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_u8(v))
    }

    fn dyn_visit_u16(&mut self, v: u16) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_u16(v))
    }

    fn dyn_visit_u32(&mut self, v: u32) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_u32(v))
    }

    fn dyn_visit_u64(&mut self, v: u64) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_u64(v))
    }

    fn dyn_visit_u128(&mut self, v: u128) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_u128(v))
    }

    fn dyn_visit_f32(&mut self, v: f32) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_f32(v))
    }

    fn dyn_visit_f64(&mut self, v: f64) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_f64(v))
    }

    fn dyn_visit_char(&mut self, v: char) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_char(v))
    }

    fn dyn_visit_str(&mut self, v: &str) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_str(v))
    }

    fn dyn_visit_borrowed_str(&mut self, v: &'de str) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_borrowed_str(v))
    }

    fn dyn_visit_string(&mut self, v: String) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_string(v))
    }

    fn dyn_visit_bytes(&mut self, v: &[u8]) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_bytes(v))
    }

    fn dyn_visit_borrowed_bytes(&mut self, v: &'de [u8]) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_borrowed_bytes(v))
    }

    fn dyn_visit_byte_buf(&mut self, v: Vec<u8>) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_byte_buf(v))
    }

    fn dyn_visit_none(&mut self) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_none())
    }

    fn dyn_visit_some(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_some(deserializer))
    }

    fn dyn_visit_unit(&mut self) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_unit())
    }

    fn dyn_visit_newtype_struct(
        &mut self,
        deserializer: &mut dyn Deserializer<'de>,
    ) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_newtype_struct(deserializer))
    }

    fn dyn_visit_seq(&mut self, access: &mut dyn SeqAccess<'de>) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_seq(access))
    }

    fn dyn_visit_map(&mut self, access: &mut dyn MapAccess<'de>) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_map(access))
    }

    fn dyn_visit_enum(&mut self, access: &mut dyn EnumAccess<'de>) -> DeserializeResult<()> {
        self.visit_with(|visitor| visitor.visit_enum(access))
    }
}

// enum InplaceSeqAccess
// ----------------------------------------------------------------------------
/// An implementation of the [`SeqAccess`] trait which performs in-place
/// deserialization.
#[derive(Clone, Debug)]
pub enum InplaceSeqAccess<'de, A: serde::de::SeqAccess<'de>> {
    /// The deserialization has done successfully.
    Error(A::Error),
    /// The deserializer is ready to deserialize the sequence.
    SeqAccess(A),
}

impl<'de, A: serde::de::SeqAccess<'de>> InplaceSeqAccess<'de, A> {
    fn into_result(self, result: DeserializeResult<()>) -> Result<(), A::Error> {
        result.map_err(|error| match self {
            InplaceSeqAccess::Error(error) => error,
            _ => error.into_error(),
        })
    }

    fn next_with<T, F>(&mut self, f: F) -> InplaceDeserializeResult<T>
    where
        F: FnOnce(&mut A) -> Result<T, A::Error>,
    {
        if let InplaceSeqAccess::SeqAccess(access) = self {
            (f)(access).map_err(|error| {
                *self = InplaceSeqAccess::Error(error);
                InplaceDeserializeError::Error
            })
        } else {
            Err(InplaceDeserializeError::NotSeqAccess)
        }
    }
}

impl<'de, A: serde::de::SeqAccess<'de>> SeqAccess<'de> for InplaceSeqAccess<'de, A> {
    fn dyn_next_element(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<()>> {
        self.next_with(|access| access.next_element_seed(seed))
    }

    fn dyn_size_hint(&self) -> Option<usize> {
        if let InplaceSeqAccess::SeqAccess(access) = self {
            access.size_hint()
        } else {
            None
        }
    }
}

// enum InplaceMapAccess
// ----------------------------------------------------------------------------
/// An implementation of the [`MapAccess`] trait which performs in-place
/// deserialization.
#[derive(Clone, Debug)]
pub enum InplaceMapAccess<'de, A: serde::de::MapAccess<'de>> {
    /// The deserialization has done successfully.
    Error(A::Error),
    /// The deserializer is ready to deserialize the map.
    MapAccess(A),
}

impl<'de, A: serde::de::MapAccess<'de>> InplaceMapAccess<'de, A> {
    fn into_result(self, result: DeserializeResult<()>) -> Result<(), A::Error> {
        result.map_err(|error| match self {
            InplaceMapAccess::Error(error) => error,
            _ => error.into_error(),
        })
    }

    fn next_with<T, F>(&mut self, f: F) -> InplaceDeserializeResult<T>
    where
        F: FnOnce(&mut A) -> Result<T, A::Error>,
    {
        if let InplaceMapAccess::MapAccess(access) = self {
            (f)(access).map_err(|error| {
                *self = InplaceMapAccess::Error(error);
                InplaceDeserializeError::Error
            })
        } else {
            Err(InplaceDeserializeError::NotMapAccess)
        }
    }
}

impl<'de, A: serde::de::MapAccess<'de>> MapAccess<'de> for InplaceMapAccess<'de, A> {
    fn dyn_next_key(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<()>> {
        self.next_with(|access| access.next_key_seed(seed))
    }

    fn dyn_next_value(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.next_with(|access| access.next_value_seed(seed))
    }

    fn dyn_next_entry(
        &mut self,
        kseed: &mut dyn DeserializeSeed<'de>,
        vseed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<Option<((), ())>> {
        self.next_with(|access| access.next_entry_seed(kseed, vseed))
    }

    fn dyn_size_hint(&self) -> Option<usize> {
        if let InplaceMapAccess::MapAccess(access) = self {
            access.size_hint()
        } else {
            None
        }
    }
}

// enum InplaceEnumAccess
// ----------------------------------------------------------------------------
/// An implementation of the [`EnumAccess`] trait which performs in-place
/// deserialization.
#[derive(Clone, Debug, Default)]
pub enum InplaceEnumAccess<'de, A: serde::de::EnumAccess<'de>> {
    /// The deserializer is not ready.
    #[default]
    None,
    /// The deserialization has done successfully.
    Error(A::Error),
    /// The deserializer is ready to deserialize the enum.
    EnumAccess(A),
    /// The deserializer is ready to deserialize the enum variant.
    VariantAccess(A::Variant),
}

impl<'de, A: serde::de::EnumAccess<'de>> InplaceEnumAccess<'de, A> {
    fn into_result(self, result: DeserializeResult<()>) -> Result<(), A::Error> {
        result.map_err(|error| match self {
            InplaceEnumAccess::Error(error) => error,
            _ => error.into_error(),
        })
    }

    fn next_variant_with<F>(&mut self, f: F) -> InplaceDeserializeResult<()>
    where
        F: FnOnce(A::Variant) -> Result<(), A::Error>,
    {
        if let InplaceEnumAccess::VariantAccess(_) = self {
            if let InplaceEnumAccess::VariantAccess(access) = mem::take(self) {
                return (f)(access).map_err(|error| {
                    *self = InplaceEnumAccess::Error(error);
                    InplaceDeserializeError::Error
                });
            }
        }
        Err(InplaceDeserializeError::NotVariantAccess)
    }
}

impl<'de, A: serde::de::EnumAccess<'de>> EnumAccess<'de> for InplaceEnumAccess<'de, A> {
    fn dyn_variant(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<&mut dyn VariantAccess<'de>> {
        if let InplaceEnumAccess::EnumAccess(_) = self {
            if let InplaceEnumAccess::EnumAccess(access) = mem::take(self) {
                return match access.variant_seed(seed) {
                    Ok((_, variant)) => {
                        *self = InplaceEnumAccess::VariantAccess(variant);
                        Ok(self)
                    }
                    Err(error) => {
                        *self = InplaceEnumAccess::Error(error);
                        Err(InplaceDeserializeError::Error)
                    }
                };
            }
        }
        Err(InplaceDeserializeError::NotEnumAccess)
    }
}

impl<'de, A: serde::de::EnumAccess<'de>> VariantAccess<'de> for InplaceEnumAccess<'de, A> {
    fn dyn_unit_variant(&mut self) -> InplaceDeserializeResult<()> {
        self.next_variant_with(|access| access.unit_variant())
    }

    fn dyn_newtype_variant(
        &mut self,
        seed: &mut dyn DeserializeSeed<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.next_variant_with(|access| access.newtype_variant_seed(seed))
    }

    fn dyn_tuple_variant(
        &mut self,
        len: usize,
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.next_variant_with(|access| access.tuple_variant(len, visitor))
    }

    fn dyn_struct_variant(
        &mut self,
        fields: &'static [&'static str],
        visitor: &mut dyn Visitor<'de>,
    ) -> InplaceDeserializeResult<()> {
        self.next_variant_with(|access| access.struct_variant(fields, visitor))
    }
}

/// An error returned by [`InplaceDeserializer`] when the in-place deserialization
/// has done unsuccessfully.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InplaceDeserializeError {
    /// The deserialization has done successfully.
    Ok,
    /// The deserialization has done unsuccessfully.
    Error,
    /// The deserializer is not ready.
    NotDeserializer,
    /// The deserialize seed is not ready.
    NotDeserializeSeed,
    /// The visitor is not ready.
    NotVisitor,
    /// The visitor is not ready to deserialize the contents of the sequence.
    NotSeqAccess,
    /// The visitor is not ready to deserialize the contents of the map.
    NotMapAccess,
    /// The visitor is not ready to deserialize the contents of the enum.
    NotEnumAccess,
    /// The visitor is not ready to deserialize the contents of the enum variant.
    NotVariantAccess,
}

impl Display for InplaceDeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            InplaceDeserializeError::Ok => "the deserialization has done successfully",
            InplaceDeserializeError::Error => "the deserialization has done unsuccessfully",
            InplaceDeserializeError::NotDeserializer => "the deserializer is not ready",
            InplaceDeserializeError::NotDeserializeSeed => "the deserialize seed is not ready",
            InplaceDeserializeError::NotVisitor => "the visitor is not ready",
            InplaceDeserializeError::NotSeqAccess => {
                "the visitor is not ready to deserialize the contents of the sequence"
            }
            InplaceDeserializeError::NotMapAccess => {
                "the visitor is not ready to deserialize the contents of the map"
            }
            InplaceDeserializeError::NotEnumAccess => {
                "the visitor is not ready to deserialize the contents of the enum"
            }
            InplaceDeserializeError::NotVariantAccess => {
                "the visitor is not ready to deserialize the contents of the enum variant"
            }
        })
    }
}

impl Error for InplaceDeserializeError {}

/// An error returned by [`dyn Deserializer`] when the dynamic deserialization has
/// done unsuccessfully.
///
/// [`dyn Deserializer`]: Deserializer
#[repr(transparent)]
// OPTIMIZE: use a more memory-effective representation.
pub struct DeserializeError(InplaceDeserializeResult<Box<str>>);

impl DeserializeError {
    fn into_error<E>(self) -> E
    where
        E: serde::de::Error,
    {
        match self.0 {
            Ok(error) => E::custom(error.into_string()),
            Err(error) => E::custom(error),
        }
    }
}

impl Debug for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Ok(ref error) => f.write_str(error),
            Err(ref error) => Display::fmt(error, f),
        }
    }
}

impl Error for DeserializeError {}

impl From<InplaceDeserializeError> for DeserializeError {
    #[cold]
    #[inline(never)]
    fn from(value: InplaceDeserializeError) -> Self {
        DeserializeError(Err(value))
    }
}

impl serde::de::Error for DeserializeError {
    #[cold]
    #[inline(never)]
    fn custom<T: Display>(msg: T) -> Self {
        DeserializeError(Ok(msg.to_string().into_boxed_str()))
    }
}

// TRAIT IMPLEMENTATION
// ----------------------------------------------------------------------------
impl<'de> serde::Deserializer<'de> for &mut (dyn Deserializer<'de> + '_) {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_any(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_bool<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_any(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_i8<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_i8(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_i16<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_i16(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_i32<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_i32(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_i64<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_i64(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_u8<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_u8(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_u16<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_u16(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_u32<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_u32(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_u64<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_u64(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_f32<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_f32(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_f64<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_f64(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_char<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_char(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_str<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_str(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_string<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_string(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_bytes(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_byte_buf(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_option<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_option(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_unit<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_unit(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_unit_struct(name, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_newtype_struct(name, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_seq<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_seq(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_tuple(len, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_tuple_struct(name, len, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_map<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_map(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_struct(name, fields, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_enum(name, variants, &mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_identifier(&mut visitor);
        visitor.into_result(result)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_deserialize_ignored_any(&mut visitor);
        visitor.into_result(result)
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for &mut (dyn DeserializeSeed<'de> + '_) {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut deserializer = InplaceDeserializer::Deserializer(deserializer);
        let result = self.dyn_deserialize(&mut deserializer);
        deserializer.into_result(result)
    }
}

impl<'de> serde::de::Visitor<'de> for &mut (dyn Visitor<'de> + '_) {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.dyn_expecting(formatter)
    }

    fn visit_bool<E>(self, v: bool) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_bool(v).map_err(E::custom)
    }

    fn visit_i8<E>(self, v: i8) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_i8(v).map_err(E::custom)
    }

    fn visit_i16<E>(self, v: i16) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_i16(v).map_err(E::custom)
    }

    fn visit_i32<E>(self, v: i32) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_i32(v).map_err(E::custom)
    }

    fn visit_i64<E>(self, v: i64) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_i64(v).map_err(E::custom)
    }

    fn visit_i128<E>(self, v: i128) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_i128(v).map_err(E::custom)
    }

    fn visit_u8<E>(self, v: u8) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_u8(v).map_err(E::custom)
    }

    fn visit_u16<E>(self, v: u16) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_u16(v).map_err(E::custom)
    }

    fn visit_u32<E>(self, v: u32) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_u32(v).map_err(E::custom)
    }

    fn visit_u64<E>(self, v: u64) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_u64(v).map_err(E::custom)
    }

    fn visit_u128<E>(self, v: u128) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_u128(v).map_err(E::custom)
    }

    fn visit_f32<E>(self, v: f32) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_f32(v).map_err(E::custom)
    }

    fn visit_f64<E>(self, v: f64) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_f64(v).map_err(E::custom)
    }

    fn visit_char<E>(self, v: char) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_char(v).map_err(E::custom)
    }

    fn visit_str<E>(self, v: &str) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_str(v).map_err(E::custom)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_borrowed_str(v).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_string(v).map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_bytes(v).map_err(E::custom)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_borrowed_bytes(v).map_err(E::custom)
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_byte_buf(v).map_err(E::custom)
    }

    fn visit_none<E>(self) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_none().map_err(E::custom)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut deserializer = InplaceDeserializer::Deserializer(deserializer);
        let result = self.dyn_visit_some(&mut deserializer);
        deserializer.into_result(result)
    }

    fn visit_unit<E>(self) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        self.dyn_visit_unit().map_err(E::custom)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut deserializer = InplaceDeserializer::Deserializer(deserializer);
        let result = self.dyn_visit_newtype_struct(&mut deserializer);
        deserializer.into_result(result)
    }

    fn visit_seq<A>(self, access: A) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut access = InplaceSeqAccess::SeqAccess(access);
        let result = self.dyn_visit_seq(&mut access);
        access.into_result(result)
    }

    fn visit_map<A>(self, access: A) -> Result<(), A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut access = InplaceMapAccess::MapAccess(access);
        let result = self.dyn_visit_map(&mut access);
        access.into_result(result)
    }

    fn visit_enum<A>(self, access: A) -> Result<(), A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let mut access = InplaceEnumAccess::EnumAccess(access);
        let result = self.dyn_visit_enum(&mut access);
        access.into_result(result)
    }
}

impl<'de> serde::de::SeqAccess<'de> for &mut (dyn SeqAccess<'de> + '_) {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> DeserializeResult<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let mut seed = InplaceDeserializeSeed::DeserializeSeed(seed);
        let result = self.dyn_next_element(&mut seed);
        seed.into_result_option(result)
    }

    fn size_hint(&self) -> Option<usize> {
        self.dyn_size_hint()
    }
}

impl<'de> serde::de::MapAccess<'de> for &mut (dyn MapAccess<'de> + '_) {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> DeserializeResult<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        let mut seed = InplaceDeserializeSeed::DeserializeSeed(seed);
        let result = self.dyn_next_key(&mut seed);
        seed.into_result_option(result)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let mut seed = InplaceDeserializeSeed::DeserializeSeed(seed);
        let result = self.dyn_next_value(&mut seed);
        seed.into_result(result)
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> DeserializeResult<Option<(K::Value, V::Value)>>
    where
        K: serde::de::DeserializeSeed<'de>,
        V: serde::de::DeserializeSeed<'de>,
    {
        let mut kseed = InplaceDeserializeSeed::DeserializeSeed(kseed);
        let mut vseed = InplaceDeserializeSeed::DeserializeSeed(vseed);

        let result = self.dyn_next_entry(&mut kseed, &mut vseed);
        vseed.into_result_option(result).map(|opt_value| {
            opt_value.map(|value| match kseed {
                InplaceDeserializeSeed::Value(key) => (key, value),
                // This is unreachable because we have checked that the value
                // has been successfully deserialized. Thus the key must be
                // deserialized successfully before the value.
                _ => unreachable!(),
            })
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.dyn_size_hint()
    }
}

impl<'a, 'de> serde::de::EnumAccess<'de> for &'a mut (dyn EnumAccess<'de> + '_) {
    type Error = DeserializeError;
    type Variant = &'a mut dyn VariantAccess<'de>;

    fn variant_seed<V>(self, seed: V) -> DeserializeResult<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let mut seed = InplaceDeserializeSeed::DeserializeSeed(seed);
        let result = self.dyn_variant(&mut seed);
        match (seed, result) {
            (InplaceDeserializeSeed::Value(value), Ok(variant)) => Ok((value, variant)),
            (_, Err(error)) => Err(DeserializeError::from(error)),
            // This is unreachable because `result` is `Ok(_)` if and only if
            // the `seed` is `Value(_)`. Thus the two branches would cover all
            // cases.
            (_, Ok(_)) => unreachable!(),
        }
    }
}

impl<'de> serde::de::VariantAccess<'de> for &mut (dyn VariantAccess<'de> + '_) {
    type Error = DeserializeError;

    fn unit_variant(self) -> DeserializeResult<()> {
        self.dyn_unit_variant().map_err(DeserializeError::from)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> DeserializeResult<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let mut seed = InplaceDeserializeSeed::DeserializeSeed(seed);
        let result = self.dyn_newtype_variant(&mut seed);
        seed.into_result(result)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_tuple_variant(len, &mut visitor);
        visitor.into_result(result)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> DeserializeResult<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut visitor = InplaceVisitor::Visitor(visitor);
        let result = self.dyn_struct_variant(fields, &mut visitor);
        visitor.into_result(result)
    }
}
