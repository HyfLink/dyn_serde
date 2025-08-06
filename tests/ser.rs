//! Test `serde_dyn::Serialize` and `serde_dyn::Serializer`.

use dyn_serde::{Serialize, Serializer};

macro_rules! make_serializer {
    ($buf:ident, $serializer:ident) => {
        let mut $buf = Vec::with_capacity(100);
        let $serializer = std::io::Cursor::new(&mut $buf);
        let mut $serializer = serde_json::Serializer::new($serializer);
        let mut $serializer = <dyn Serializer>::new(&mut $serializer);
        let $serializer = &mut $serializer as &mut dyn Serializer;
    };
}

#[test]
fn test_serializer() {
    #[derive(serde::Serialize)]
    enum Enum {
        UnitVariant,
        NewtypeVariant(NewtypeStruct),
        TupleVariant(Vec<i32>, String),
        StructVariant { x: TupleStruct, y: Struct },
    }

    #[derive(serde::Serialize)]
    struct NewtypeStruct(String);

    #[derive(serde::Serialize)]
    struct TupleStruct(Vec<i32>, String);

    #[derive(serde::Serialize)]
    struct Struct {
        x: Vec<i32>,
        y: String,
    }

    let value = vec![
        Enum::UnitVariant,
        Enum::NewtypeVariant(NewtypeStruct("Foo".to_string())),
        Enum::TupleVariant(vec![3, 1, 4], "Bar".to_string()),
        Enum::StructVariant {
            x: TupleStruct(vec![1, 5, 9], "Baz".to_string()),
            y: Struct {
                x: vec![2, 6, 5],
                y: "Qux".to_string(),
            },
        },
    ];

    make_serializer!(buf, serializer);
    value.dyn_serialize(serializer).unwrap();
    value.dyn_serialize(serializer).unwrap_err();
    10i32.dyn_serialize(serializer).unwrap_err();
    false.dyn_serialize(serializer).unwrap_err();
    assert_eq!(
        buf,
        b"[\"UnitVariant\",{\"NewtypeVariant\":\"Foo\"},{\"TupleVariant\":[[3,1,4],\"Bar\"]},{\"StructVariant\":{\"x\":[[1,5,9],\"Baz\"],\"y\":{\"x\":[2,6,5],\"y\":\"Qux\"}}}]"
    );
}

#[test]
fn test_serialize_bool() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_bool(true).unwrap();
    assert_eq!(buf, b"true");
}

#[test]
fn test_serialize_i8() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_i8(-128).unwrap();
    assert_eq!(buf, b"-128");
}

#[test]
fn test_serialize_i16() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_i16(i16::MAX).unwrap();
    assert_eq!(buf, b"32767");
}

#[test]
fn test_serialize_i32() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_i32(i32::MAX).unwrap();
    assert_eq!(buf, b"2147483647");
}

#[test]
fn test_serialize_i64() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_i64(i64::MAX).unwrap();
    assert_eq!(buf, b"9223372036854775807");
}

#[test]
fn test_serialize_i128() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_i128(i128::MAX).unwrap();
    assert_eq!(buf, b"170141183460469231731687303715884105727");
}

#[test]
fn test_serialize_u8() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_u8(u8::MAX).unwrap();
    assert_eq!(buf, b"255");
}

#[test]
fn test_serialize_u16() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_u16(u16::MAX).unwrap();
    assert_eq!(buf, b"65535");
}

#[test]
fn test_serialize_u32() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_u32(u32::MAX).unwrap();
    assert_eq!(buf, b"4294967295");
}

#[test]
fn test_serialize_u64() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_u64(u64::MAX).unwrap();
    assert_eq!(buf, b"18446744073709551615");
}

#[test]
fn test_serialize_u128() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_u128(u128::MAX).unwrap();
    assert_eq!(buf, b"340282366920938463463374607431768211455");
}

#[test]
fn test_serialize_f32() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_serialize_f32(3.14159265358979323846264338327950288)
        .unwrap();
    assert_eq!(buf, b"3.1415927");
}

#[test]
fn test_serialize_f64() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_serialize_f64(3.14159265358979323846264338327950288)
        .unwrap();
    assert_eq!(buf, b"3.141592653589793");
}

#[test]
fn test_serialize_char() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_char('\u{fe0f}').unwrap();
    assert_eq!(buf, b"\"\xEF\xB8\x8F\"");
}

#[test]
fn test_serialize_str() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_str("♥️").unwrap();
    assert_eq!(buf, "\"♥️\"".as_bytes());
}

#[test]
fn test_serialize_none() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_none().unwrap();
    assert_eq!(buf, b"null");
}

#[test]
fn test_serialize_some() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_some(&"Some(_)").unwrap();
    assert_eq!(buf, b"\"Some(_)\"");
}

#[test]
fn test_serialize_unit() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_unit().unwrap();
    assert_eq!(buf, b"null");
}

#[test]
fn test_serialize_unit_struct() {
    make_serializer!(buf, serializer);
    serializer.dyn_serialize_unit_struct("unit_struct").unwrap();
    assert_eq!(buf, b"null");
}

#[test]
fn test_serialize_unit_variant() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_serialize_unit_variant("Enum", 0, "unit_variant")
        .unwrap();
    assert_eq!(buf, b"\"unit_variant\"");
}

#[test]
fn test_serialize_newtype_struct() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_serialize_newtype_struct("newtype_struct", &"Value")
        .unwrap();
    assert_eq!(buf, b"\"Value\"");
}

#[test]
fn test_serialize_newtype_variant() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_serialize_newtype_variant("Enum", 0, "newtype_variant", &"Value")
        .unwrap();
    assert_eq!(buf, b"{\"newtype_variant\":\"Value\"}");
}

#[test]
fn test_serialize_seq() {
    make_serializer!(buf, serializer);
    let seq = serializer.dyn_serialize_seq(Some(3)).unwrap();
    seq.dyn_serialize_element(&1_i32).unwrap();
    seq.dyn_serialize_element(&2.0_f64).unwrap();
    seq.dyn_serialize_element(&"3").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"[1,2.0,\"3\"]");
}

#[test]
fn test_serialize_tuple() {
    make_serializer!(buf, serializer);
    let seq = serializer.dyn_serialize_tuple(3).unwrap();
    seq.dyn_serialize_element(&1_i32).unwrap();
    seq.dyn_serialize_element(&2.0_f64).unwrap();
    seq.dyn_serialize_element(&"3").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"[1,2.0,\"3\"]");
}

#[test]
fn test_serialize_tuple_struct() {
    make_serializer!(buf, serializer);
    let seq = serializer
        .dyn_serialize_tuple_struct("tuple_struct", 3)
        .unwrap();
    seq.dyn_serialize_field(&1_i32).unwrap();
    seq.dyn_serialize_field(&2.0_f64).unwrap();
    seq.dyn_serialize_field(&"3").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"[1,2.0,\"3\"]");
}

#[test]
fn test_serialize_tuple_variant() {
    make_serializer!(buf, serializer);
    let seq = serializer
        .dyn_serialize_tuple_variant("Enum", 0, "tuple_variant", 3)
        .unwrap();
    seq.dyn_serialize_field(&1_i32).unwrap();
    seq.dyn_serialize_field(&2.0_f64).unwrap();
    seq.dyn_serialize_field(&"3").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"{\"tuple_variant\":[1,2.0,\"3\"]}");
}

#[test]
fn test_serialize_map() {
    make_serializer!(buf, serializer);
    let seq = serializer.dyn_serialize_map(Some(3)).unwrap();
    seq.dyn_serialize_key(&"A").unwrap();
    seq.dyn_serialize_value(&"aaa").unwrap();
    seq.dyn_serialize_entry(&"B", &"bbb").unwrap();
    seq.dyn_serialize_entry(&"C", &"ccc").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"{\"A\":\"aaa\",\"B\":\"bbb\",\"C\":\"ccc\"}");
}

#[test]
fn test_serialize_struct() {
    make_serializer!(buf, serializer);
    let seq = serializer.dyn_serialize_struct("struct", 3).unwrap();
    seq.dyn_serialize_field("A", &"aaa").unwrap();
    seq.dyn_serialize_field("B", &"bbb").unwrap();
    seq.dyn_serialize_field("C", &"ccc").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(buf, b"{\"A\":\"aaa\",\"B\":\"bbb\",\"C\":\"ccc\"}");
}

#[test]
fn test_serialize_struct_variant() {
    make_serializer!(buf, serializer);
    let seq = serializer
        .dyn_serialize_struct_variant("Enum", 0, "struct_variant", 3)
        .unwrap();
    seq.dyn_serialize_field("A", &"aaa").unwrap();
    seq.dyn_serialize_field("B", &"bbb").unwrap();
    seq.dyn_serialize_field("C", &"ccc").unwrap();
    seq.dyn_end().unwrap();
    assert_eq!(
        buf,
        b"{\"struct_variant\":{\"A\":\"aaa\",\"B\":\"bbb\",\"C\":\"ccc\"}}"
    );
}

#[test]
fn test_collect_str() {
    make_serializer!(buf, serializer);
    serializer
        .dyn_collect_str(&std::io::ErrorKind::NotFound)
        .unwrap();
    assert_eq!(buf, b"\"entity not found\"");
}
