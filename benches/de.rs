//! Deserialization benchmark on different libraries.
//!
//! Data 'twitter.json' is from [nativejson-benchmark].
//!
//! [nativejson-benchmark]: <https://github.com/miloyip/nativejson-benchmark>

use std::hint::black_box;

use criterion::Criterion;

use serde::Deserialize;
use serde_json::Value;

fn main() {
    let json = std::fs::read("benches/twitter.json").unwrap();

    Criterion::default()
        .configure_from_args()
        .bench_function("serde-json", |bench| {
            bench.iter(|| {
                let de = std::io::Cursor::new(&json);
                let mut de = serde_json::Deserializer::from_reader(de);
                let de = &mut de;

                let value = black_box(Value::deserialize(black_box(de)));
                assert!(value.is_ok());
            })
        })
        .bench_function("dyn-serde", |bench| {
            bench.iter(|| {
                use dyn_serde::Deserializer;

                let de = std::io::Cursor::new(&json);
                let mut de = serde_json::Deserializer::from_reader(de);
                let mut de = <dyn Deserializer>::new(&mut de);
                let de: &mut dyn Deserializer = &mut de;

                let value = black_box(Value::deserialize(black_box(de)));
                assert!(value.is_ok());
            })
        })
        .bench_function("erased-serde", |bench| {
            bench.iter(|| {
                use erased_serde::Deserializer;

                let de = std::io::Cursor::new(&json);
                let mut de = serde_json::Deserializer::from_reader(de);
                let mut de = <dyn Deserializer>::erase(&mut de);
                let de: &mut dyn Deserializer = &mut de;

                let value = black_box(Value::deserialize(black_box(de)));
                assert!(value.is_ok());
            })
        })
        .final_summary();
}
