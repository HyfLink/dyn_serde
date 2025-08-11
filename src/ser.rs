//! For dynamic serialization, see [`Serialize`] and [`Serializer`].

use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem;

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;

use serde::ser::SerializeMap as _;
use serde::ser::SerializeSeq as _;
use serde::ser::SerializeStruct as _;
use serde::ser::SerializeStructVariant as _;
use serde::ser::SerializeTuple as _;
use serde::ser::SerializeTupleStruct as _;
use serde::ser::SerializeTupleVariant as _;

/// The result type returned by [`dyn Serializer`]'s methods.
///
/// [`dyn Serializer`]: Serializer
pub type SerializeResult<T> = Result<T, SerializeError>;

/// The result type returned by [`InplaceSerializer`]'s methods.
pub type InplaceSerializeResult<T> = Result<T, InplaceSerializeError>;

/// The dyn-compatible version of trait [`serde::Serialize`].
///
/// One should avoid implementing `Serialize` manually and implement
/// [`serde::Serialize`] instead. Implementing [`serde::Serialize`]
/// automatically provides one with an implementation of [`Serialize`].
///
/// # Examples
///
/// Values implementing [`serde::Serialize`] can be converted into `dyn
/// Serialize`. The trait object also implements [`serde::Serialize`].
///
/// ```
/// # use serde::Serialize as _;
/// # use dyn_serde::Serialize;
/// #
/// let mut buf = Vec::new();
/// let writer = std::io::Cursor::new(&mut buf);
/// let mut serializer = serde_json::Serializer::new(writer);
///
/// // Let's serialize a heterogeneous array.
/// let value = [
///     &-3.1415926f64 as &dyn Serialize,
///     &"Hello, world!",
///     &false,
///     &b"\x01\x02\x03",
///     &Option::<String>::None,
/// ];
///
/// // Use `serde::Serialize`.
/// value.serialize(&mut serializer).unwrap();
///
/// assert_eq!(buf, b"[-3.1415926,\"Hello, world!\",false,[1,2,3],null]");
/// ```
#[diagnostic::on_unimplemented(note = "Consider implementing `serde::Serialize` for `{Self}`")]
pub trait Serialize {
    /// Serialize the `self` value with the given dynamic `serializer`.
    ///
    /// This function is equivalent to [`serde::Serialize::serialize`] thanks
    /// to the implementation of [`serde::Serializer`] on `&mut dyn Serializer`.
    fn dyn_serialize(&self, serializer: &mut dyn Serializer) -> SerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::Serializer`].
///
/// One should avoid implementing `Serializer` manually and use
/// `<dyn Serializer>::new` to construct an instance instead.
///
/// # Examples
///
/// ```
/// # use serde::Serializer as _;
/// # use serde::ser::SerializeSeq as _;
/// # use dyn_serde::Serializer;
/// #
/// let mut buf = Vec::new();
/// let mut serializer = serde_json::Serializer::new(std::io::Cursor::new(&mut buf));
/// let mut inplace_serializer = <dyn Serializer>::new(&mut serializer);
/// let serializer = &mut inplace_serializer as &mut dyn Serializer;
///
/// // Serializes a heterogeneous array using `serde::Serializer` and
/// // `serde::ser::SerializeSeq`.
/// let mut seq = serializer.serialize_seq(Some(5)).unwrap();
/// seq.serialize_element(&-3.1415926f64).unwrap();
/// seq.serialize_element(&"Hello, world!").unwrap();
/// seq.serialize_element(&false).unwrap();
/// seq.serialize_element(&b"\x01\x02\x03").unwrap();
/// seq.serialize_element(&Option::<String>::None).unwrap();
/// let result = seq.end();
/// assert!(result.is_ok());
///
/// // Now `inplace_serializer` becomes `InplaceSerializer::Ok(_)` or `Error(_)`.
/// // And continue to use `serializer` would cause an error.
/// assert!(serializer.serialize_i32(0).is_err());
/// assert!(serializer.serialize_bool(true).is_err());
///
/// assert_eq!(buf, b"[-3.1415926,\"Hello, world!\",false,[1,2,3],null]");
/// ```
#[diagnostic::on_unimplemented(note = "Consider using `<dyn Serializer>::new`")]
pub trait Serializer {
    /// Serialize a `bool` value.
    fn dyn_serialize_bool(&mut self, v: bool) -> InplaceSerializeResult<()>;

    /// Serialize an `i8` value.
    fn dyn_serialize_i8(&mut self, v: i8) -> InplaceSerializeResult<()>;

    /// Serialize an `i16` value.
    fn dyn_serialize_i16(&mut self, v: i16) -> InplaceSerializeResult<()>;

    /// Serialize an `i32` value.
    fn dyn_serialize_i32(&mut self, v: i32) -> InplaceSerializeResult<()>;

    /// Serialize an `i64` value.
    fn dyn_serialize_i64(&mut self, v: i64) -> InplaceSerializeResult<()>;

    /// Serialize an `i128` value.
    fn dyn_serialize_i128(&mut self, v: i128) -> InplaceSerializeResult<()>;

    /// Serialize a `u8` value.
    fn dyn_serialize_u8(&mut self, v: u8) -> InplaceSerializeResult<()>;

    /// Serialize a `u16` value.
    fn dyn_serialize_u16(&mut self, v: u16) -> InplaceSerializeResult<()>;

    /// Serialize a `u32` value.
    fn dyn_serialize_u32(&mut self, v: u32) -> InplaceSerializeResult<()>;

    /// Serialize a `u64` value.
    fn dyn_serialize_u64(&mut self, v: u64) -> InplaceSerializeResult<()>;

    /// Serialize a `u128` value.
    fn dyn_serialize_u128(&mut self, v: u128) -> InplaceSerializeResult<()>;

    /// Serialize an `f32` value.
    fn dyn_serialize_f32(&mut self, v: f32) -> InplaceSerializeResult<()>;

    /// Serialize an `f64` value.
    fn dyn_serialize_f64(&mut self, v: f64) -> InplaceSerializeResult<()>;

    /// Serialize a character.
    fn dyn_serialize_char(&mut self, v: char) -> InplaceSerializeResult<()>;

    /// Serialize a `&str`.
    fn dyn_serialize_str(&mut self, v: &str) -> InplaceSerializeResult<()>;

    /// Serialize a chunk of raw byte data.
    fn dyn_serialize_bytes(&mut self, v: &[u8]) -> InplaceSerializeResult<()>;

    /// Serialize a `None` value.
    fn dyn_serialize_none(&mut self) -> InplaceSerializeResult<()>;

    /// Serialize a `Some(_)` value.
    fn dyn_serialize_some(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Serialize a `()` value.
    fn dyn_serialize_unit(&mut self) -> InplaceSerializeResult<()>;

    /// Serialize a unit struct like `struct Unit` or `PhantomData<T>`.
    fn dyn_serialize_unit_struct(&mut self, name: &'static str) -> InplaceSerializeResult<()>;

    /// Serialize a unit variant like `E::A` in `enum E { A, B }`.
    ///
    /// The `name` is the name of the enum, the `variant_index` is the index of
    /// this variant within the enum, and the `variant` is the name of the
    /// variant.
    fn dyn_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> InplaceSerializeResult<()>;

    /// Serialize a newtype struct like `struct Millimeters(u8)`.
    fn dyn_serialize_newtype_struct(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()>;

    /// Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.
    ///
    /// The `name` is the name of the enum, the `variant_index` is the index of
    /// this variant within the enum, and the `variant` is the name of the
    /// variant. The `value` is the data contained within this newtype variant.
    fn dyn_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()>;

    /// Begin to serialize a variably sized sequence. This call must be
    /// followed by zero or more calls to `dyn_serialize_element`, then a call
    /// to `dyn_end`.
    ///
    /// The argument is the number of elements in the sequence, which may or may
    /// not be computable before the sequence is iterated. Some serializers only
    /// support sequences whose length is known up front.
    fn dyn_serialize_seq(
        &mut self,
        len: Option<usize>,
    ) -> InplaceSerializeResult<&mut dyn SerializeSeq>;

    /// Begin to serialize a statically sized sequence whose length will be
    /// known at deserialization time without looking at the serialized data.
    /// This call must be followed by zero or more calls to
    /// `dyn_serialize_element`, then a call to `dyn_end`.
    fn dyn_serialize_tuple(
        &mut self,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTuple>;

    /// Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`. This
    /// call must be followed by zero or more calls to `dyn_serialize_field`,
    /// then a call to `dyn_end`.
    ///
    /// The `name` is the name of the tuple struct and the `len` is the number
    /// of data fields that will be serialized.
    fn dyn_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTupleStruct>;

    /// Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8)
    /// }`. This call must be followed by zero or more calls to
    /// `dyn_serialize_field`, then a call to `dyn_end`.
    ///
    /// The `name` is the name of the enum, the `variant_index` is the index of
    /// this variant within the enum, the `variant` is the name of the variant,
    /// and the `len` is the number of data fields that will be serialized.
    fn dyn_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTupleVariant>;

    /// Begin to serialize a map. This call must be followed by zero or more
    /// calls to `dyn_serialize_key` and `dyn_serialize_value`, then a call to
    /// `dyn_end`.
    ///
    /// The argument is the number of elements in the map, which may or may not
    /// be computable before the map is iterated. Some serializers only support
    /// maps whose length is known up front.
    fn dyn_serialize_map(
        &mut self,
        len: Option<usize>,
    ) -> InplaceSerializeResult<&mut dyn SerializeMap>;

    /// Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.
    /// This call must be followed by zero or more calls to
    /// `dyn_serialize_field`, then a call to `dyn_end`.
    ///
    /// The `name` is the name of the struct and the `len` is the number of
    /// data fields that will be serialized. `len` does not include fields
    /// which are skipped with [`SerializeStruct::dyn_skip_field`].
    fn dyn_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeStruct>;

    /// Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,
    /// g: u8, b: u8 } }`. This call must be followed by zero or more calls to
    /// `dyn_serialize_field`, then a call to `dyn_end`.
    ///
    /// The `name` is the name of the enum, the `variant_index` is the index of
    /// this variant within the enum, the `variant` is the name of the variant,
    /// and the `len` is the number of data fields that will be serialized.
    /// `len` does not include fields which are skipped with
    /// [`SerializeStructVariant::dyn_skip_field`].
    fn dyn_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeStructVariant>;

    /// Serialize a string produced by an implementation of [`Display`].
    fn dyn_collect_str(&mut self, value: &dyn Display) -> InplaceSerializeResult<()>;

    /// Determine whether `Serialize` implementations should serialize in
    /// human-readable form.
    ///
    /// See [`is_human_readable`] defined on the [`serde::Serializer`] trait
    /// for more information.
    ///
    /// [`is_human_readable`]: serde::Serializer::is_human_readable
    fn dyn_is_human_readable(&self) -> bool;
}

impl dyn Serializer + '_ {
    /// A convenient way to construct an instance of [`dyn Serializer`].
    ///
    /// This function is equivalent to [`InplaceSerializer::Serializer`].
    ///
    /// [`dyn Serializer`]: Serializer
    ///
    /// # Examples
    ///
    /// ```
    /// # use dyn_serde::Serializer;
    /// #
    /// let stdout = std::io::stdout();
    /// let mut serializer = serde_json::Serializer::new(stdout.lock());
    /// let mut serializer = <dyn Serializer>::new(&mut serializer);
    /// let serializer = &mut serializer as &mut dyn Serializer;
    /// # let _ = serializer;
    /// ```
    #[must_use]
    pub fn new<S>(serializer: S) -> InplaceSerializer<S>
    where
        S: serde::Serializer,
    {
        InplaceSerializer::Serializer(serializer)
    }
}

/// The dyn-compatible version of trait [`serde::ser::SerializeSeq`].
///
/// The trait object is returned by [`dyn_serialize_seq`] defined on trait
/// [`Serializer`].
///
/// One should avoid implementing `SerializeSeq` manually and use
/// [`InplaceSerializer::SerializeSeq`] to construct an instance instead.
///
/// [`dyn_serialize_seq`]: Serializer::dyn_serialize_seq
pub trait SerializeSeq {
    /// Serialize a sequence element.
    fn dyn_serialize_element(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Finish serializing a sequence.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeTuple`].
///
/// The trait object is returned by [`dyn_serialize_tuple`] defined on trait
/// [`Serializer`].
///
/// One should avoid implementing `SerializeTuple` manually and use
/// [`InplaceSerializer::SerializeTuple`] to construct an instance instead.
///
/// [`dyn_serialize_tuple`]: Serializer::dyn_serialize_tuple
pub trait SerializeTuple {
    /// Serialize a tuple element.
    fn dyn_serialize_element(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Finish serializing a tuple.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeTupleStruct`].
///
/// The trait object is returned by [`dyn_serialize_tuple_struct`] defined on
/// trait [`Serializer`].
///
/// One should avoid implementing `SerializeTupleStruct` manually and use
/// [`InplaceSerializer::SerializeTupleStruct`] to construct an instance
/// instead.
///
/// [`dyn_serialize_tuple_struct`]: Serializer::dyn_serialize_tuple_struct
pub trait SerializeTupleStruct {
    /// Serialize a tuple struct field.
    fn dyn_serialize_field(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Finish serializing a tuple struct.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeTupleVariant`].
///
/// The trait object is returned by [`dyn_serialize_tuple_variant`] defined on
/// trait [`Serializer`].
///
/// One should avoid implementing `SerializeTupleVariant` manually and use
/// [`InplaceSerializer::SerializeTupleVariant`] to construct an instance
/// instead.
///
/// [`dyn_serialize_tuple_variant`]: Serializer::dyn_serialize_tuple_variant
pub trait SerializeTupleVariant {
    /// Serialize a tuple variant field.
    fn dyn_serialize_field(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Finish serializing a tuple variant.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeMap`].
///
/// The trait object is returned by [`dyn_serialize_map`] defined on trait
/// [`Serializer`].
///
/// One should avoid implementing `SerializeMap` manually and use
/// [`InplaceSerializer::SerializeMap`] to construct an instance instead.
///
/// [`dyn_serialize_map`]: Serializer::dyn_serialize_map
pub trait SerializeMap {
    /// Serialize a map key.
    fn dyn_serialize_key(&mut self, key: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Serialize a map value.
    fn dyn_serialize_value(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()>;

    /// Serialize a map entry consisting of a key and a value.
    fn dyn_serialize_entry(
        &mut self,
        key: &dyn Serialize,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()>;

    /// Finish serializing a map.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeStruct`].
///
/// The trait object is returned by [`dyn_serialize_struct`] defined on trait
/// [`Serializer`].
///
/// One should avoid implementing `SerializeStruct` manually and use
/// [`InplaceSerializer::SerializeStruct`] to construct an instance instead.
///
/// [`dyn_serialize_struct`]: Serializer::dyn_serialize_struct
pub trait SerializeStruct {
    /// Serialize a struct field.
    fn dyn_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()>;

    /// Indicate that a struct field has been skipped.
    fn dyn_skip_field(&mut self, key: &'static str) -> InplaceSerializeResult<()>;

    /// Finish serializing a struct.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// The dyn-compatible version of trait [`serde::ser::SerializeStructVariant`].
///
/// The trait object is returned by [`dyn_serialize_struct_variant`] defined on
/// trait [`Serializer`].
///
/// One should avoid implementing `SerializeStructVariant` manually and use
/// [`InplaceSerializer::SerializeStructVariant`] to construct an instance
/// instead.
///
/// [`dyn_serialize_struct_variant`]: Serializer::dyn_serialize_struct_variant
pub trait SerializeStructVariant {
    /// Serialize a struct field.
    fn dyn_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()>;

    /// Indicate that a struct variant field has been skipped.
    fn dyn_skip_field(&mut self, key: &'static str) -> InplaceSerializeResult<()>;

    /// Finish serializing a struct variant.
    fn dyn_end(&mut self) -> InplaceSerializeResult<()>;
}

/// An implementation of the [`Serializer`] trait which performs
/// in-place serialization so that the result type is unified to
/// [`InplaceSerializeResult<_>`].
#[derive(Clone, Default, Debug)]
pub enum InplaceSerializer<S>
where
    S: serde::Serializer,
{
    /// The in-place serializer is not ready.
    #[default]
    None,
    /// The in-place serialization has done successfually.
    Ok(S::Ok),
    /// The in-place serialization has done unsuccessfually.
    Error(S::Error),
    /// The in-place serializer is ready.
    Serializer(S),
    /// The in-place serializer is ready to serialize the content of the
    /// sequence.
    SerializeSeq(S::SerializeSeq),
    /// The in-place serializer is ready to serialize the content of the tuple.
    SerializeTuple(S::SerializeTuple),
    /// The in-place serializer is ready to serialize the content of the tuple
    /// struct.
    SerializeTupleStruct(S::SerializeTupleStruct),
    /// The in-place serializer is ready to serialize the content of the tuple
    /// variant.
    SerializeTupleVariant(S::SerializeTupleVariant),
    /// The in-place serializer is ready to serialize the content of the map.
    SerializeMap(S::SerializeMap),
    /// The in-place serializer is ready to serialize the content of the
    /// struct.
    SerializeStruct(S::SerializeStruct),
    /// The in-place serializer is ready to serialize the content of the
    /// struct variant.
    SerializeStructVariant(S::SerializeStructVariant),
}

impl<S> InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn take(&mut self) -> InplaceSerializeResult<S> {
        if let InplaceSerializer::Serializer(_) = self {
            if let InplaceSerializer::Serializer(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializer)
    }

    fn take_seq(&mut self) -> InplaceSerializeResult<S::SerializeSeq> {
        if let InplaceSerializer::SerializeSeq(_) = self {
            if let InplaceSerializer::SerializeSeq(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeSeq)
    }

    fn take_tuple(&mut self) -> InplaceSerializeResult<S::SerializeTuple> {
        if let InplaceSerializer::SerializeTuple(_) = self {
            if let InplaceSerializer::SerializeTuple(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeTuple)
    }

    fn take_tuple_struct(&mut self) -> InplaceSerializeResult<S::SerializeTupleStruct> {
        if let InplaceSerializer::SerializeTupleStruct(_) = self {
            if let InplaceSerializer::SerializeTupleStruct(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeTupleStruct)
    }

    fn take_tuple_variant(&mut self) -> InplaceSerializeResult<S::SerializeTupleVariant> {
        if let InplaceSerializer::SerializeTupleVariant(_) = self {
            if let InplaceSerializer::SerializeTupleVariant(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeTupleVariant)
    }

    fn take_map(&mut self) -> InplaceSerializeResult<S::SerializeMap> {
        if let InplaceSerializer::SerializeMap(_) = self {
            if let InplaceSerializer::SerializeMap(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeMap)
    }

    fn take_struct(&mut self) -> InplaceSerializeResult<S::SerializeStruct> {
        if let InplaceSerializer::SerializeStruct(_) = self {
            if let InplaceSerializer::SerializeStruct(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeStruct)
    }

    fn take_struct_variant(&mut self) -> InplaceSerializeResult<S::SerializeStructVariant> {
        if let InplaceSerializer::SerializeStructVariant(_) = self {
            if let InplaceSerializer::SerializeStructVariant(serializer) = mem::take(self) {
                return Ok(serializer);
            }
        }
        Err(InplaceSerializeError::NotSerializeStructVariant)
    }

    fn get_seq(&mut self) -> InplaceSerializeResult<&mut S::SerializeSeq> {
        if let InplaceSerializer::SerializeSeq(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeSeq)
        }
    }

    fn get_tuple(&mut self) -> InplaceSerializeResult<&mut S::SerializeTuple> {
        if let InplaceSerializer::SerializeTuple(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeTuple)
        }
    }

    fn get_tuple_struct(&mut self) -> InplaceSerializeResult<&mut S::SerializeTupleStruct> {
        if let InplaceSerializer::SerializeTupleStruct(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeTupleStruct)
        }
    }

    fn get_tuple_variant(&mut self) -> InplaceSerializeResult<&mut S::SerializeTupleVariant> {
        if let InplaceSerializer::SerializeTupleVariant(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeTupleVariant)
        }
    }

    fn get_map(&mut self) -> InplaceSerializeResult<&mut S::SerializeMap> {
        if let InplaceSerializer::SerializeMap(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeMap)
        }
    }

    fn get_struct(&mut self) -> InplaceSerializeResult<&mut S::SerializeStruct> {
        if let InplaceSerializer::SerializeStruct(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeStruct)
        }
    }

    fn get_struct_variant(&mut self) -> InplaceSerializeResult<&mut S::SerializeStructVariant> {
        if let InplaceSerializer::SerializeStructVariant(serializer) = self {
            Ok(serializer)
        } else {
            Err(InplaceSerializeError::NotSerializeStructVariant)
        }
    }

    fn serialize_with<T, U>(
        &mut self,
        take: impl FnOnce(&mut Self) -> InplaceSerializeResult<T>,
        then: impl FnOnce(U) -> Self,
        serialize: impl FnOnce(T) -> Result<U, S::Error>,
    ) -> InplaceSerializeResult<()> {
        let serializer = (take)(self)?;
        match (serialize)(serializer) {
            #[allow(clippy::unit_arg)]
            Ok(ok) => Ok(*self = (then)(ok)),
            Err(error) => {
                *self = InplaceSerializer::Error(error);
                Err(InplaceSerializeError::Error)
            }
        }
    }

    fn serialize_with_mut<T>(
        &mut self,
        take: impl FnOnce(&mut Self) -> InplaceSerializeResult<&mut T>,
        serialize: impl FnOnce(&mut T) -> Result<(), S::Error>,
    ) -> InplaceSerializeResult<()> {
        let serializer = (take)(self)?;
        match (serialize)(serializer) {
            Ok(ok) => Ok(ok),
            Err(error) => {
                *self = InplaceSerializer::Error(error);
                Err(InplaceSerializeError::Error)
            }
        }
    }
}

impl<S> Serializer for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_bool(&mut self, v: bool) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_bool(v)
        })
    }

    fn dyn_serialize_i8(&mut self, v: i8) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_i8(v)
        })
    }

    fn dyn_serialize_i16(&mut self, v: i16) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_i16(v)
        })
    }

    fn dyn_serialize_i32(&mut self, v: i32) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_i32(v)
        })
    }

    fn dyn_serialize_i64(&mut self, v: i64) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_i64(v)
        })
    }

    fn dyn_serialize_i128(&mut self, v: i128) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_i128(v)
        })
    }

    fn dyn_serialize_u8(&mut self, v: u8) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_u8(v)
        })
    }

    fn dyn_serialize_u16(&mut self, v: u16) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_u16(v)
        })
    }

    fn dyn_serialize_u32(&mut self, v: u32) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_u32(v)
        })
    }

    fn dyn_serialize_u64(&mut self, v: u64) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_u64(v)
        })
    }

    fn dyn_serialize_u128(&mut self, v: u128) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_u128(v)
        })
    }

    fn dyn_serialize_f32(&mut self, v: f32) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_f32(v)
        })
    }

    fn dyn_serialize_f64(&mut self, v: f64) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_f64(v)
        })
    }

    fn dyn_serialize_char(&mut self, v: char) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_char(v)
        })
    }

    fn dyn_serialize_str(&mut self, v: &str) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_str(v)
        })
    }

    fn dyn_serialize_bytes(&mut self, v: &[u8]) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_bytes(v)
        })
    }

    fn dyn_serialize_none(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_none()
        })
    }

    fn dyn_serialize_some(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_some(value)
        })
    }

    fn dyn_serialize_unit(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_unit()
        })
    }

    fn dyn_serialize_unit_struct(&mut self, name: &'static str) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_unit_struct(name)
        })
    }

    fn dyn_serialize_unit_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_unit_variant(name, variant_index, variant)
        })
    }

    fn dyn_serialize_newtype_struct(
        &mut self,
        name: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_newtype_struct(name, value)
        })
    }

    fn dyn_serialize_newtype_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.serialize_newtype_variant(name, variant_index, variant, value)
        })
    }

    fn dyn_serialize_seq(
        &mut self,
        len: Option<usize>,
    ) -> InplaceSerializeResult<&mut dyn SerializeSeq> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeSeq,
            |ser| ser.serialize_seq(len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_tuple(
        &mut self,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTuple> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeTuple,
            |ser| ser.serialize_tuple(len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_tuple_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTupleStruct> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeTupleStruct,
            |ser| ser.serialize_tuple_struct(name, len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_tuple_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeTupleVariant> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeTupleVariant,
            |ser| ser.serialize_tuple_variant(name, variant_index, variant, len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_map(
        &mut self,
        len: Option<usize>,
    ) -> InplaceSerializeResult<&mut dyn SerializeMap> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeMap,
            |ser| ser.serialize_map(len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_struct(
        &mut self,
        name: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeStruct> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeStruct,
            |ser| ser.serialize_struct(name, len),
        )?;
        Ok(self)
    }

    fn dyn_serialize_struct_variant(
        &mut self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> InplaceSerializeResult<&mut dyn SerializeStructVariant> {
        self.serialize_with(
            InplaceSerializer::take,
            InplaceSerializer::SerializeStructVariant,
            |ser| ser.serialize_struct_variant(name, variant_index, variant, len),
        )?;
        Ok(self)
    }

    fn dyn_collect_str(&mut self, value: &dyn Display) -> InplaceSerializeResult<()> {
        self.serialize_with(InplaceSerializer::take, InplaceSerializer::Ok, |ser| {
            ser.collect_str(value)
        })
    }

    fn dyn_is_human_readable(&self) -> bool {
        if let InplaceSerializer::Serializer(serializer) = self {
            serializer.is_human_readable()
        } else {
            true
        }
    }
}

impl<S> SerializeSeq for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_element(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_seq, |ser| {
            ser.serialize_element(value)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_seq,
            InplaceSerializer::Ok,
            S::SerializeSeq::end,
        )
    }
}

impl<S> SerializeTuple for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_element(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_tuple, |ser| {
            ser.serialize_element(value)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_tuple,
            InplaceSerializer::Ok,
            S::SerializeTuple::end,
        )
    }
}

impl<S> SerializeTupleStruct for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_field(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_tuple_struct, |ser| {
            ser.serialize_field(value)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_tuple_struct,
            InplaceSerializer::Ok,
            S::SerializeTupleStruct::end,
        )
    }
}

impl<S> SerializeTupleVariant for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_field(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_tuple_variant, |ser| {
            ser.serialize_field(value)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_tuple_variant,
            InplaceSerializer::Ok,
            S::SerializeTupleVariant::end,
        )
    }
}

impl<S> SerializeMap for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_key(&mut self, key: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_map, |ser| ser.serialize_key(key))
    }

    fn dyn_serialize_value(&mut self, value: &dyn Serialize) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_map, |ser| ser.serialize_value(value))
    }

    fn dyn_serialize_entry(
        &mut self,
        key: &dyn Serialize,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_map, |ser| {
            ser.serialize_entry(key, value)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_map,
            InplaceSerializer::Ok,
            S::SerializeMap::end,
        )
    }
}

impl<S> SerializeStruct for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_struct, |ser| {
            ser.serialize_field(key, value)
        })
    }

    fn dyn_skip_field(&mut self, key: &'static str) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_struct, |ser| ser.skip_field(key))
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_struct,
            InplaceSerializer::Ok,
            S::SerializeStruct::end,
        )
    }
}

impl<S> SerializeStructVariant for InplaceSerializer<S>
where
    S: serde::Serializer,
{
    fn dyn_serialize_field(
        &mut self,
        key: &'static str,
        value: &dyn Serialize,
    ) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_struct_variant, |ser| {
            ser.serialize_field(key, value)
        })
    }

    fn dyn_skip_field(&mut self, key: &'static str) -> InplaceSerializeResult<()> {
        self.serialize_with_mut(InplaceSerializer::get_struct_variant, |ser| {
            ser.skip_field(key)
        })
    }

    fn dyn_end(&mut self) -> InplaceSerializeResult<()> {
        self.serialize_with(
            InplaceSerializer::take_struct_variant,
            InplaceSerializer::Ok,
            S::SerializeStructVariant::end,
        )
    }
}

/// An error returned by [`InplaceSerializer`] when the in-place serialization
/// has done unsuccessfully.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InplaceSerializeError {
    /// The in-place serialization has done unsuccessfually.
    Error,
    /// The in-place serializer is not ready.
    NotSerializer,
    /// The in-place serializer is not ready to serialize the content of the
    /// sequence.
    NotSerializeSeq,
    /// The in-place serializer is not ready to serialize the content of the
    /// tuple.
    NotSerializeTuple,
    /// The in-place serializer is not ready to serialize the content of the
    /// tuple struct.
    NotSerializeTupleStruct,
    /// The in-place serializer is not ready to serialize the content of the
    /// tuple variant.
    NotSerializeTupleVariant,
    /// The in-place serializer is not ready to serialize the content of the
    /// map.
    NotSerializeMap,
    /// The in-place serializer is not ready to serialize the content of the
    /// struct.
    NotSerializeStruct,
    /// The in-place serializer is not ready to serialize the content of the
    /// struct variant.
    NotSerializeStructVariant,
}

impl Display for InplaceSerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            InplaceSerializeError::Error => "the in-place serialization has done unsuccessfually",
            InplaceSerializeError::NotSerializer => "the in-place serializer is not ready",
            InplaceSerializeError::NotSerializeSeq => "the in-place serializer is not ready to serialize the content of the sequence",
            InplaceSerializeError::NotSerializeTuple => "the in-place serializer is not ready to serialize the content of the tuple",
            InplaceSerializeError::NotSerializeTupleStruct => "the in-place serializer is not ready to serialize the content of the tuple struct",
            InplaceSerializeError::NotSerializeTupleVariant => "the in-place serializer is not ready to serialize the content of the tuple variant",
            InplaceSerializeError::NotSerializeMap => "the in-place serializer is not ready to serialize the content of the map",
            InplaceSerializeError::NotSerializeStruct => "the in-place serializer is not ready to serialize the content of the struct",
            InplaceSerializeError::NotSerializeStructVariant => "the in-place serializer is not ready to serialize the content of the struct variant",
        })
    }
}

impl Error for InplaceSerializeError {}

/// An error returned by [`dyn Serializer`] when the dynamic serialization has
/// done unsuccessfully.
///
/// [`dyn Serializer`]: Serializer
#[repr(transparent)]
// OPTIMIZE: use a more memory-effective representation.
pub struct SerializeError(InplaceSerializeResult<Box<str>>);

impl SerializeError {
    fn into_error<E>(self) -> E
    where
        E: serde::ser::Error,
    {
        match self.0 {
            Ok(error) => E::custom(error.into_string()),
            Err(error) => E::custom(error),
        }
    }
}

impl Debug for SerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Ok(ref error) => f.write_str(error),
            Err(ref error) => Display::fmt(error, f),
        }
    }
}

impl Error for SerializeError {}

impl From<InplaceSerializeError> for SerializeError {
    #[cold]
    #[inline(never)]
    fn from(value: InplaceSerializeError) -> Self {
        SerializeError(Err(value))
    }
}

impl serde::ser::Error for SerializeError {
    #[cold]
    #[inline(never)]
    fn custom<T: Display>(msg: T) -> Self {
        SerializeError(Ok(msg.to_string().into_boxed_str()))
    }
}

// TRAIT IMPLEMENTATION
// ----------------------------------------------------------------------------
impl<T: serde::Serialize> Serialize for T {
    fn dyn_serialize(&self, serializer: &mut dyn Serializer) -> SerializeResult<()> {
        self.serialize(serializer)
    }
}

impl serde::Serialize for dyn Serialize + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = InplaceSerializer::Serializer(serializer);
        let result = self.dyn_serialize(&mut serializer);
        match serializer {
            InplaceSerializer::Ok(ok) => Ok(ok),
            InplaceSerializer::Error(error) => Err(error),
            // The `unwrap_err` never panics becasue `result` is `Ok(_)` if and
            // only if the `serializer` is `Ok(_)`. And we have checked that it
            // isn't because of the above branch.
            _ => Err(result.unwrap_err().into_error()),
        }
    }
}

impl<'a> serde::Serializer for &'a mut (dyn Serializer + '_) {
    type Ok = ();
    type Error = SerializeError;
    type SerializeSeq = &'a mut dyn SerializeSeq;
    type SerializeTuple = &'a mut dyn SerializeTuple;
    type SerializeTupleStruct = &'a mut dyn SerializeTupleStruct;
    type SerializeTupleVariant = &'a mut dyn SerializeTupleVariant;
    type SerializeMap = &'a mut dyn SerializeMap;
    type SerializeStruct = &'a mut dyn SerializeStruct;
    type SerializeStructVariant = &'a mut dyn SerializeStructVariant;

    fn serialize_bool(self, v: bool) -> SerializeResult<()> {
        self.dyn_serialize_bool(v).map_err(SerializeError::from)
    }

    fn serialize_i8(self, v: i8) -> SerializeResult<()> {
        self.dyn_serialize_i8(v).map_err(SerializeError::from)
    }

    fn serialize_i16(self, v: i16) -> SerializeResult<()> {
        self.dyn_serialize_i16(v).map_err(SerializeError::from)
    }

    fn serialize_i32(self, v: i32) -> SerializeResult<()> {
        self.dyn_serialize_i32(v).map_err(SerializeError::from)
    }

    fn serialize_i64(self, v: i64) -> SerializeResult<()> {
        self.dyn_serialize_i64(v).map_err(SerializeError::from)
    }

    fn serialize_u8(self, v: u8) -> SerializeResult<()> {
        self.dyn_serialize_u8(v).map_err(SerializeError::from)
    }

    fn serialize_u16(self, v: u16) -> SerializeResult<()> {
        self.dyn_serialize_u16(v).map_err(SerializeError::from)
    }

    fn serialize_u32(self, v: u32) -> SerializeResult<()> {
        self.dyn_serialize_u32(v).map_err(SerializeError::from)
    }

    fn serialize_u64(self, v: u64) -> SerializeResult<()> {
        self.dyn_serialize_u64(v).map_err(SerializeError::from)
    }

    fn serialize_f32(self, v: f32) -> SerializeResult<()> {
        self.dyn_serialize_f32(v).map_err(SerializeError::from)
    }

    fn serialize_f64(self, v: f64) -> SerializeResult<()> {
        self.dyn_serialize_f64(v).map_err(SerializeError::from)
    }

    fn serialize_char(self, v: char) -> SerializeResult<()> {
        self.dyn_serialize_char(v).map_err(SerializeError::from)
    }

    fn serialize_str(self, v: &str) -> SerializeResult<()> {
        self.dyn_serialize_str(v).map_err(SerializeError::from)
    }

    fn serialize_bytes(self, v: &[u8]) -> SerializeResult<()> {
        self.dyn_serialize_bytes(v).map_err(SerializeError::from)
    }

    fn serialize_none(self) -> SerializeResult<()> {
        self.dyn_serialize_none().map_err(SerializeError::from)
    }

    fn serialize_some<T>(self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_some(&value)
            .map_err(SerializeError::from)
    }

    fn serialize_unit(self) -> SerializeResult<()> {
        self.dyn_serialize_unit().map_err(SerializeError::from)
    }

    fn serialize_unit_struct(self, name: &'static str) -> SerializeResult<()> {
        self.dyn_serialize_unit_struct(name)
            .map_err(SerializeError::from)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> SerializeResult<()> {
        self.dyn_serialize_unit_variant(name, variant_index, variant)
            .map_err(SerializeError::from)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_newtype_struct(name, &value)
            .map_err(SerializeError::from)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_newtype_variant(name, variant_index, variant, &value)
            .map_err(SerializeError::from)
    }

    fn serialize_seq(self, len: Option<usize>) -> SerializeResult<Self::SerializeSeq> {
        self.dyn_serialize_seq(len).map_err(SerializeError::from)
    }

    fn serialize_tuple(self, len: usize) -> SerializeResult<Self::SerializeTuple> {
        self.dyn_serialize_tuple(len).map_err(SerializeError::from)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> SerializeResult<Self::SerializeTupleStruct> {
        self.dyn_serialize_tuple_struct(name, len)
            .map_err(SerializeError::from)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> SerializeResult<Self::SerializeTupleVariant> {
        self.dyn_serialize_tuple_variant(name, variant_index, variant, len)
            .map_err(SerializeError::from)
    }

    fn serialize_map(self, len: Option<usize>) -> SerializeResult<Self::SerializeMap> {
        self.dyn_serialize_map(len).map_err(SerializeError::from)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> SerializeResult<Self::SerializeStruct> {
        self.dyn_serialize_struct(name, len)
            .map_err(SerializeError::from)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> SerializeResult<Self::SerializeStructVariant> {
        self.dyn_serialize_struct_variant(name, variant_index, variant, len)
            .map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeSeq for &mut (dyn SerializeSeq + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_element(&value)
            .map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeTuple for &mut (dyn SerializeTuple + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_element(&value)
            .map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeTupleStruct for &mut (dyn SerializeTupleStruct + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_field(&value)
            .map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeTupleVariant for &mut (dyn SerializeTupleVariant + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_field(&value)
            .map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeMap for &mut (dyn SerializeMap + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_key<T>(&mut self, key: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_key(&key).map_err(SerializeError::from)
    }

    fn serialize_value<T>(&mut self, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_value(&value)
            .map_err(SerializeError::from)
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> SerializeResult<()>
    where
        K: ?Sized + serde::Serialize,
        V: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_entry(&key, &value)
            .map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeStruct for &mut (dyn SerializeStruct + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_field(key, &value)
            .map_err(SerializeError::from)
    }

    fn skip_field(&mut self, key: &'static str) -> SerializeResult<()> {
        self.dyn_skip_field(key).map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}

impl serde::ser::SerializeStructVariant for &mut (dyn SerializeStructVariant + '_) {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> SerializeResult<()>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dyn_serialize_field(key, &value)
            .map_err(SerializeError::from)
    }

    fn skip_field(&mut self, key: &'static str) -> SerializeResult<()> {
        self.dyn_skip_field(key).map_err(SerializeError::from)
    }

    fn end(self) -> SerializeResult<()> {
        self.dyn_end().map_err(SerializeError::from)
    }
}
