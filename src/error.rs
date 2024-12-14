//! This module provides a non-generic error type for the failure of the dynamic
//! serialization or deserialization.

use core::error;
use core::fmt::{self, Debug, Display, Formatter};

use serde::{de, ser};

/// Type alias that is a [result][core::result::Result] type with an error type
/// of [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// An error that is returned on the failure of the dynamic serialization or
/// deserialization.
///
/// # Representation
///
/// This `struct` has two representations, controlled by feature `error`.
///
/// | Feature `error` | Memory Layout of `Error` |
/// | --------------- | ------------------------ |
/// | Yes             | same as a pointer        |
/// | No              | same as unit `()`        |
///
/// * If feature `error` is enabled (default), error messages provided by
///   concrete (de)serializer are boxed and held by this struct;
///
/// * If feature `error` is disabled, the `struct Error` is just a marker
///   indicating the failure of the dynamic (de)serialization. Error messages
///   provided by concrete (de)serializer are discarded;
pub struct Error {
    repr: repr::Repr,
}

impl Error {
    /// Converts this error into [`serde::de::Error`].
    #[cold]
    #[must_use]
    #[inline(never)]
    pub fn into_de_error<E: de::Error>(self) -> E {
        self.repr.into_de_error()
    }

    /// Converts this error into [`serde::ser::Error`].
    #[cold]
    #[must_use]
    #[inline(never)]
    pub fn into_ser_error<E: ser::Error>(self) -> E {
        E::custom(self)
    }
}

impl Default for Error {
    #[cold]
    #[inline(never)]
    fn default() -> Self {
        Error {
            #[cfg(feature = "error")]
            repr: repr::Repr::default(),
            #[cfg(not(feature = "error"))]
            repr: repr::Repr,
        }
    }
}

impl Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <repr::Repr as Debug>::fmt(&self.repr, f)
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <repr::Repr as Display>::fmt(&self.repr, f)
    }
}

impl error::Error for Error {}

impl ser::Error for Error {
    #[cold]
    #[inline(never)]
    fn custom<T: Display>(msg: T) -> Self {
        Error {
            repr: repr::Repr::custom(msg),
        }
    }
}

impl de::Error for Error {
    #[cold]
    #[inline(never)]
    fn custom<T: Display>(msg: T) -> Self {
        Error {
            repr: repr::Repr::custom(msg),
        }
    }

    #[cold]
    #[inline(never)]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error {
            repr: repr::Repr::invalid_type(unexp, exp),
        }
    }

    #[cold]
    #[inline(never)]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error {
            repr: repr::Repr::invalid_type(unexp, exp),
        }
    }

    #[cold]
    #[inline(never)]
    fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self {
        Error {
            repr: repr::Repr::invalid_length(len, exp),
        }
    }

    #[cold]
    #[inline(never)]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Error {
            repr: repr::Repr::unknown_variant(variant, expected),
        }
    }

    #[cold]
    #[inline(never)]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Error {
            repr: repr::Repr::unknown_field(field, expected),
        }
    }

    #[cold]
    #[inline(never)]
    fn missing_field(field: &'static str) -> Self {
        Error {
            repr: repr::Repr::missing_field(field),
        }
    }

    #[cold]
    #[inline(never)]
    fn duplicate_field(field: &'static str) -> Self {
        Error {
            repr: repr::Repr::duplicate_field(field),
        }
    }
}

#[cfg(not(feature = "error"))]
mod repr {
    use core::error;
    use core::fmt::{self, Debug, Display, Formatter};

    use serde::{de, ser};

    pub struct Repr;

    impl Repr {
        pub fn into_de_error<E: de::Error>(self) -> E {
            E::custom(self)
        }
    }

    impl Debug for Repr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str("Error")
        }
    }

    impl Display for Repr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.write_str("an error occurred during dynamic (de)serialization")
        }
    }

    impl error::Error for Repr {}

    impl ser::Error for Repr {
        #[inline]
        fn custom<T: Display>(_: T) -> Self {
            Repr
        }
    }

    impl de::Error for Repr {
        #[inline]
        fn custom<T: Display>(_: T) -> Self {
            Repr
        }

        #[inline]
        fn invalid_type(_: de::Unexpected, _: &dyn de::Expected) -> Self {
            Repr
        }

        #[inline]
        fn invalid_value(_: de::Unexpected, _: &dyn de::Expected) -> Self {
            Repr
        }

        #[inline]
        fn invalid_length(_: usize, _: &dyn de::Expected) -> Self {
            Repr
        }

        #[inline]
        fn unknown_variant(_: &str, _: &'static [&'static str]) -> Self {
            Repr
        }

        #[inline]
        fn unknown_field(_: &str, _: &'static [&'static str]) -> Self {
            Repr
        }

        #[inline]
        fn missing_field(_: &'static str) -> Self {
            Repr
        }

        #[inline]
        fn duplicate_field(_: &'static str) -> Self {
            Repr
        }
    }
}

#[cfg(feature = "error")]
mod repr {
    use core::error;
    use core::fmt::{self, Debug, Display, Formatter};

    use serde::{de, ser};

    #[cfg(not(feature = "std"))]
    use alloc::{boxed::Box, string::ToString};
    #[cfg(feature = "std")]
    use std::{boxed::Box, string::ToString};

    pub struct Repr {
        repr: Option<Box<ErrorCode>>,
    }

    impl Repr {
        pub fn into_de_error<E: de::Error>(self) -> E {
            match self.repr {
                Some(repr) => match *repr {
                    ErrorCode::Custom(msg) => E::custom(msg),
                    ErrorCode::InvalidType(ref unexp, exp) => {
                        E::invalid_type(de::Unexpected::from(unexp), &&*exp)
                    }
                    ErrorCode::InvalidValue(ref unexp, exp) => {
                        E::invalid_value(de::Unexpected::from(unexp), &&*exp)
                    }
                    ErrorCode::InvalidLength(len, exp) => E::invalid_length(len, &&*exp),
                    ErrorCode::UnknownVariant(ref variant, expected) => {
                        E::unknown_variant(variant, expected)
                    }
                    ErrorCode::UnknownField(ref field, expected) => {
                        E::unknown_field(field, expected)
                    }
                    ErrorCode::MissingField(field) => E::missing_field(field),
                    ErrorCode::DuplicateField(field) => E::duplicate_field(field),
                },
                None => E::custom("an error occurred during dynamic (de)serialization"),
            }
        }
    }

    impl Default for Repr {
        #[inline]
        fn default() -> Self {
            Repr { repr: None }
        }
    }

    impl Debug for Repr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self.repr.as_ref().map(Box::as_ref) {
                Some(ErrorCode::Custom(msg)) => write!(f, "Custom({msg:?})"),
                Some(ErrorCode::InvalidType(unexp, exp)) => {
                    write!(f, "InvalidType({unexp:?}, {exp:?})")
                }
                Some(ErrorCode::InvalidValue(unexp, exp)) => {
                    write!(f, "InvalidValue({unexp:?}, {exp:?})")
                }
                Some(ErrorCode::InvalidLength(len, exp)) => {
                    write!(f, "InvalidLength({len}, {exp:?})")
                }
                Some(ErrorCode::UnknownVariant(variant, expected)) => {
                    write!(f, "UnknownVariant({variant:?}, {expected:?})")
                }
                Some(ErrorCode::UnknownField(field, expected)) => {
                    write!(f, "UnknownField({field:?}, {expected:?})")
                }
                Some(ErrorCode::MissingField(field)) => {
                    write!(f, "MissingField({field:?})")
                }
                Some(ErrorCode::DuplicateField(field)) => {
                    write!(f, "DuplicateField({field:?})")
                }
                None => f.write_str("Error"),
            }
        }
    }

    impl Display for Repr {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self.repr.as_ref().map(Box::as_ref) {
                Some(ErrorCode::Custom(msg)) => write!(f, "Custom({msg:?})"),
                Some(ErrorCode::InvalidType(unexp, exp)) => {
                    write!(f, "invalid type: {unexp}, expected {exp}")
                }
                Some(ErrorCode::InvalidValue(unexp, exp)) => {
                    write!(f, "invalid value: {unexp}, expected {exp}")
                }
                Some(ErrorCode::InvalidLength(len, exp)) => {
                    write!(f, "invalid length: {len}, expected {exp}")
                }
                Some(ErrorCode::UnknownVariant(variant, expected)) => match expected {
                    [] => write!(f, "unknown variant: {variant}, there are no variants"),
                    [a] => write!(f, "unknown variant: {variant}, expected `{a}`"),
                    [a, b] => write!(f, "unknown variant: {variant}, expected `{a}` or `{b}`"),
                    [a, b @ ..] => {
                        write!(f, "unknown variant: {variant}, expected one of `{a}`")?;
                        for x in b {
                            write!(f, ", `{x}`")?;
                        }
                        Ok(())
                    }
                },
                Some(ErrorCode::UnknownField(field, expected)) => match expected {
                    [] => write!(f, "unknown field: {field}, there are no variants"),
                    [a] => write!(f, "unknown field: {field}, expected `{a}`"),
                    [a, b] => write!(f, "unknown field: {field}, expected `{a}` or `{b}`"),
                    [a, b @ ..] => {
                        write!(f, "unknown field: {field}, expected one of `{a}`")?;
                        for x in b {
                            write!(f, ", `{x}`")?;
                        }
                        Ok(())
                    }
                },
                Some(ErrorCode::MissingField(field)) => {
                    write!(f, "missing field `{field}`")
                }
                Some(ErrorCode::DuplicateField(field)) => {
                    write!(f, "duplicate field `{field}`")
                }
                None => f.write_str("an error occurred during dynamic (de)serialization"),
            }
        }
    }

    impl error::Error for Repr {}

    impl ser::Error for Repr {
        fn custom<T: Display>(msg: T) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::Custom(
                    msg.to_string().into_boxed_str(),
                ))),
            }
        }
    }

    impl de::Error for Repr {
        fn custom<T: Display>(msg: T) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::Custom(
                    msg.to_string().into_boxed_str(),
                ))),
            }
        }

        fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::InvalidType(
                    Unexpected::from(unexp),
                    exp.to_string().into_boxed_str(),
                ))),
            }
        }

        fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::InvalidValue(
                    Unexpected::from(unexp),
                    exp.to_string().into_boxed_str(),
                ))),
            }
        }

        fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::InvalidLength(
                    len,
                    exp.to_string().into_boxed_str(),
                ))),
            }
        }

        fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::UnknownVariant(
                    Box::from(variant),
                    expected,
                ))),
            }
        }

        fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::UnknownField(
                    Box::from(field),
                    expected,
                ))),
            }
        }

        fn missing_field(field: &'static str) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::MissingField(field))),
            }
        }

        fn duplicate_field(field: &'static str) -> Self {
            Repr {
                repr: Some(Box::new(ErrorCode::DuplicateField(field))),
            }
        }
    }

    /// A non-generic version of [`serde::de::Error`].
    pub enum ErrorCode {
        /// A general error that is returned when deserializing some value.
        Custom(Box<str>),
        /// A error that is returned when receiving an unexpected type.
        InvalidType(Unexpected, Box<str>),
        /// A error that is returned when receiving an unexpected value of the
        /// right type.
        InvalidValue(Unexpected, Box<str>),
        /// A error that is returned when deserializing a sequence or map but the
        /// input data contains too many or too few elements.
        InvalidLength(usize, Box<str>),
        /// A error that is returned when receiving an enum variant with an
        /// unrecognized name.
        UnknownVariant(Box<str>, &'static [&'static str]),
        /// A error that is returned when receiving a struct field with an
        /// unrecognized name.
        UnknownField(Box<str>, &'static [&'static str]),
        /// A error that is returned when a struct expected to receive a required
        /// field with a particular name but that field was not present in the
        /// input.
        MissingField(&'static str),
        /// A error that is returned when a struct received more than one of the
        /// same field.
        DuplicateField(&'static str),
    }

    /// An owned version of [`serde::de::Unexpected`].
    pub enum Unexpected {
        /// The input contained a boolean value that was not expected.
        Bool(bool),
        /// The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
        /// was not expected.
        Unsigned(u64),
        /// The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
        /// was not expected.
        Signed(i64),
        /// The input contained a floating point `f32` or `f64` that was not
        /// expected.
        Float(f64),
        /// The input contained a `char` that was not expected.
        Char(char),
        /// The input contained a `&str` or `String` that was not expected.
        Str(Box<str>),
        /// The input contained a `&[u8]` or `Vec<u8>` that was not expected.
        Bytes(Box<[u8]>),
        /// The input contained a unit `()` that was not expected.
        Unit,
        /// The input contained an `Option<T>` that was not expected.
        Option,
        /// The input contained a newtype struct that was not expected.
        NewtypeStruct,
        /// The input contained a sequence that was not expected.
        Seq,
        /// The input contained a map that was not expected.
        Map,
        /// The input contained an enum that was not expected.
        Enum,
        /// The input contained a unit variant that was not expected.
        UnitVariant,
        /// The input contained a newtype variant that was not expected.
        NewtypeVariant,
        /// The input contained a tuple variant that was not expected.
        TupleVariant,
        /// The input contained a struct variant that was not expected.
        StructVariant,
        /// A message stating what uncategorized thing the input contained that was
        /// not expected.
        ///
        /// The message should be a noun or noun phrase, not capitalized and without
        /// a period. An example message is "unoriginal superhero".
        Other(Box<str>),
    }

    impl Debug for Unexpected {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Unexpected::Bool(v) => write!(f, "Bool({v:?})"),
                Unexpected::Unsigned(v) => write!(f, "Unsigned({v:?})"),
                Unexpected::Signed(v) => write!(f, "Signed({v:?})"),
                Unexpected::Float(v) => write!(f, "Float({v:?})"),
                Unexpected::Char(v) => write!(f, "Char({v:?})"),
                Unexpected::Str(v) => write!(f, "Str({v:?})"),
                Unexpected::Bytes(v) => write!(f, "Bytes({v:?})"),
                Unexpected::Unit => f.write_str("Unit"),
                Unexpected::Option => f.write_str("Option"),
                Unexpected::NewtypeStruct => f.write_str("NewtypeStruct"),
                Unexpected::Seq => f.write_str("Seq"),
                Unexpected::Map => f.write_str("Map"),
                Unexpected::Enum => f.write_str("Enum"),
                Unexpected::UnitVariant => f.write_str("UnitVariant"),
                Unexpected::NewtypeVariant => f.write_str("NewtypeVariant"),
                Unexpected::TupleVariant => f.write_str("TupleVariant"),
                Unexpected::StructVariant => f.write_str("StructVariant"),
                Unexpected::Other(v) => write!(f, "Other({v:?})"),
            }
        }
    }

    impl Display for Unexpected {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Unexpected::Bool(v) => write!(f, "boolean `{v}`"),
                Unexpected::Unsigned(v) => write!(f, "integer `{v}`"),
                Unexpected::Signed(v) => write!(f, "integer `{v}`"),
                Unexpected::Float(v) => write!(f, "floating point `{v}`"),
                Unexpected::Char(v) => write!(f, "character `{v}`"),
                Unexpected::Str(v) => write!(f, "string {v:?}"),
                Unexpected::Bytes(_) => f.write_str("byte array"),
                Unexpected::Unit => f.write_str("unit value"),
                Unexpected::Option => f.write_str("Option value"),
                Unexpected::NewtypeStruct => f.write_str("newtype struct"),
                Unexpected::Seq => f.write_str("sequence"),
                Unexpected::Map => f.write_str("map"),
                Unexpected::Enum => f.write_str("enum"),
                Unexpected::UnitVariant => f.write_str("unit variant"),
                Unexpected::NewtypeVariant => f.write_str("newtype variant"),
                Unexpected::TupleVariant => f.write_str("tuple variant"),
                Unexpected::StructVariant => f.write_str("struct variant"),
                Unexpected::Other(v) => f.write_str(v),
            }
        }
    }

    impl<'a> From<de::Unexpected<'a>> for Unexpected {
        fn from(value: de::Unexpected<'a>) -> Self {
            match value {
                de::Unexpected::Bool(v) => Unexpected::Bool(v),
                de::Unexpected::Unsigned(v) => Unexpected::Unsigned(v),
                de::Unexpected::Signed(v) => Unexpected::Signed(v),
                de::Unexpected::Float(v) => Unexpected::Float(v),
                de::Unexpected::Char(v) => Unexpected::Char(v),
                de::Unexpected::Str(v) => Unexpected::Str(Box::from(v)),
                de::Unexpected::Bytes(v) => Unexpected::Bytes(Box::from(v)),
                de::Unexpected::Unit => Unexpected::Unit,
                de::Unexpected::Option => Unexpected::Option,
                de::Unexpected::NewtypeStruct => Unexpected::NewtypeStruct,
                de::Unexpected::Seq => Unexpected::Seq,
                de::Unexpected::Map => Unexpected::Map,
                de::Unexpected::Enum => Unexpected::Enum,
                de::Unexpected::UnitVariant => Unexpected::UnitVariant,
                de::Unexpected::NewtypeVariant => Unexpected::NewtypeVariant,
                de::Unexpected::TupleVariant => Unexpected::TupleVariant,
                de::Unexpected::StructVariant => Unexpected::StructVariant,
                de::Unexpected::Other(v) => Unexpected::Other(Box::from(v)),
            }
        }
    }

    impl<'a> From<&'a Unexpected> for de::Unexpected<'a> {
        fn from(value: &'a Unexpected) -> Self {
            match value {
                Unexpected::Bool(v) => de::Unexpected::Bool(*v),
                Unexpected::Unsigned(v) => de::Unexpected::Unsigned(*v),
                Unexpected::Signed(v) => de::Unexpected::Signed(*v),
                Unexpected::Float(v) => de::Unexpected::Float(*v),
                Unexpected::Char(v) => de::Unexpected::Char(*v),
                Unexpected::Str(v) => de::Unexpected::Str(v),
                Unexpected::Bytes(v) => de::Unexpected::Bytes(v),
                Unexpected::Unit => de::Unexpected::Unit,
                Unexpected::Option => de::Unexpected::Option,
                Unexpected::NewtypeStruct => de::Unexpected::NewtypeStruct,
                Unexpected::Seq => de::Unexpected::Seq,
                Unexpected::Map => de::Unexpected::Map,
                Unexpected::Enum => de::Unexpected::Enum,
                Unexpected::UnitVariant => de::Unexpected::UnitVariant,
                Unexpected::NewtypeVariant => de::Unexpected::NewtypeVariant,
                Unexpected::TupleVariant => de::Unexpected::TupleVariant,
                Unexpected::StructVariant => de::Unexpected::StructVariant,
                Unexpected::Other(v) => de::Unexpected::Other(v),
            }
        }
    }
}
