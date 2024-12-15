use dyn_serde::Serializer;
use serde::Serialize as _;

#[test]
fn serialize() {
    let mut buffer = Vec::with_capacity(256);
    let mut serializer = serde_json::Serializer::new(std::io::Cursor::new(&mut buffer));
    let mut serializer = <dyn Serializer>::new(&mut serializer);
    let serializer: &mut dyn Serializer = &mut serializer;

    let value = serde_json::json!({
        "a": false,
        "b": true,
        "c": null,
        "d": 0,
        "e": 31415926,
        "f": 3.1415926,
        "g": [false, null, 3.1415926, {
            "x": 1.2,
            "y": 1.6,
            "z": 2.0,
        }],
        "h": {
            "a": true,
            "b": null,
            "c": "foo",
        },
        "i": "bar",
    });

    value.serialize(serializer).unwrap();
    assert_eq!(
        buffer,
        br#"{"a":false,"b":true,"c":null,"d":0,"e":31415926,"f":3.1415926,"g":[false,null,3.1415926,{"x":1.2,"y":1.6,"z":2.0}],"h":{"a":true,"b":null,"c":"foo"},"i":"bar"}"#
    )
}
