macro_rules! make_deserializer {
    ($deserializer:ident = $str:literal) => {
        let mut $deserializer = serde_json::Deserializer::from_str($str);
        let mut $deserializer = <dyn dyn_serde::Deserializer>::new(&mut $deserializer);
        let $deserializer = &mut $deserializer as &mut dyn dyn_serde::Deserializer<'_>;
    };
}

#[test]
fn test_deserialize_bool() {
    make_deserializer!(deserializer = "false");
    let value = <bool as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, false);
}

#[test]
fn test_deserialize_i8() {
    make_deserializer!(deserializer = "127");
    let value = <i8 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, i8::MAX);
}

#[test]
fn test_deserialize_i16() {
    make_deserializer!(deserializer = "32767");
    let value = <i16 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, i16::MAX);
}

#[test]
fn test_deserialize_i32() {
    make_deserializer!(deserializer = "2147483647");
    let value = <i32 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, i32::MAX);
}

#[test]
fn test_deserialize_i64() {
    make_deserializer!(deserializer = "9223372036854775807");
    let value = <i64 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, i64::MAX);
}

#[test]
fn test_deserialize_u8() {
    make_deserializer!(deserializer = "255");
    let value = <u8 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, u8::MAX);
}

#[test]
fn test_deserialize_u16() {
    make_deserializer!(deserializer = "65535");
    let value = <u16 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, u16::MAX);
}

#[test]
fn test_deserialize_u32() {
    make_deserializer!(deserializer = "4294967295");
    let value = <u32 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, u32::MAX);
}

#[test]
fn test_deserialize_u64() {
    make_deserializer!(deserializer = "18446744073709551615");
    let value = <u64 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, u64::MAX);
}

#[test]
fn test_serialize_f32() {
    make_deserializer!(deserializer = "3.1415927");
    let value = <f32 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, 3.1415927);
}

#[test]
fn test_serialize_f64() {
    make_deserializer!(deserializer = "3.141592653589793");
    let value = <f64 as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, 3.141592653589793);
}

#[test]
fn test_serialize_char() {
    make_deserializer!(deserializer = "\"A\"");
    let value = <char as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, 'A');
}

#[test]
fn test_serialize_str() {
    make_deserializer!(deserializer = "\"♥️\"");
    let value = <String as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, "♥️");
}

#[test]
fn test_serialize_none() {
    make_deserializer!(deserializer = "null");
    let value = <Option<String> as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, None);
}

#[test]
fn test_serialize_some() {
    make_deserializer!(deserializer = "\"♥️\"");
    let value = <Option<String> as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value.as_deref(), Some("♥️"));
}

#[test]
fn test_serialize_unit() {
    make_deserializer!(deserializer = "null");
    let value = <() as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, ());
}

#[test]
fn test_serialize_unit_struct() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct UnitStruct;

    make_deserializer!(deserializer = "null");
    let value = <UnitStruct as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, UnitStruct);
}

#[test]
fn test_serialize_unit_variant() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Enum {
        UnitVariant,
    }

    make_deserializer!(deserializer = "\"UnitVariant\"");
    let value = <Enum as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, Enum::UnitVariant);
}

#[test]
fn test_serialize_newtype_struct() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct NewtypeStruct(bool);

    make_deserializer!(deserializer = "false");
    let value = <NewtypeStruct as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, NewtypeStruct(false));
}

#[test]
fn test_serialize_newtype_variant() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Enum {
        NewtypeVariant(bool),
    }

    make_deserializer!(deserializer = "{\"NewtypeVariant\":false}");
    let value = <Enum as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, Enum::NewtypeVariant(false));
}

#[test]
fn test_serialize_seq() {
    make_deserializer!(deserializer = "[3,1,4,1,5,9]");
    let value = <Vec<i32> as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, &[3, 1, 4, 1, 5, 9]);
}

#[test]
fn test_serialize_tuple() {
    make_deserializer!(deserializer = "[true,null,255]");
    let value = <(bool, (), u8) as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, (true, (), 255));
}

#[test]
fn test_serialize_tuple_struct() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct TupleStruct(bool, (), u8);

    make_deserializer!(deserializer = "[true,null,255]");
    let value = <TupleStruct as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, TupleStruct(true, (), 255));
}

#[test]
fn test_serialize_tuple_variant() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Enum {
        TupleVariant(bool, (), u8),
    }

    make_deserializer!(deserializer = "{\"TupleVariant\":[true,null,255]}");
    let value = <Enum as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, Enum::TupleVariant(true, (), 255));
}

#[test]
fn test_serialize_struct() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    struct Struct {
        a: bool,
    }

    make_deserializer!(deserializer = "{\"a\":true}");
    let value = <Struct as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, Struct { a: true });
}

#[test]
fn test_serialize_struct_variant() {
    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    enum Enum {
        StructVariant { a: bool },
    }

    make_deserializer!(deserializer = "{\"StructVariant\":{\"a\":true}}");
    let value = <Enum as serde::Deserialize<'_>>::deserialize(deserializer).unwrap();
    assert_eq!(value, Enum::StructVariant { a: true });
}
