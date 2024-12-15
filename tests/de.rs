use dyn_serde::Deserializer;
use serde::Deserialize as _;
use serde_json::Value;

#[test]
fn deserialize() {
    let value = br#"{"a":false,"b":true,"c":null,"d":0,"e":31415926,"f":3.1415926,"g":[false,null,3.1415926,{"x":1.2,"y":1.6,"z":2.0}],"h":{"a":true,"b":null,"c":"foo"},"i":"bar"}"#;
    let mut deserializer = serde_json::Deserializer::from_slice(value);
    let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
    let deserializer: &mut dyn Deserializer = &mut deserializer;

    let value = Value::deserialize(deserializer).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
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
        })
    )
}
