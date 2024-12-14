# Dyn Serde

This crate provides support for dynamic serialization and deserialization based on [`serde`].

[`serde`]: https://serde.rs/

## Overview

This crate provides three dyn-compatible version of the `serde` traits:

1. `Serialize` - dyn-compatible version of `serde::Serialize`:

   - Implementing `serde::Serialize` will automatically provides an implementation of `Serialize`;
   - The trait object `dyn Serialize` also implements `serde::Serialize`;

1. `Serializer` - dyn-compatible version of `serde::Serializer`:

   - Instance of `Serializer` can be converted from instance of `serde::Serializer` by method `<dyn Serializer>::new`;
   - The trait object `dyn Serializer` also implements `serde::Serializer`;

1. `Deserializer` - dyn-compatible version of `serde::Deserializer`:

   - Instance of `Deserializer` can be converted from instance of `serde::Deserializer` by method `<dyn Deserializer>::new`;
   - The trait object `dyn Deserializer` also implements `serde::Deserializer`;

## Examples

### Serialization

Following example demonstrates how to perform serialization with a dynamic serializer:

```rust
use serde::Serialize
use dyn_serde::Serializer;

// 1. Creates dynamic serializer using `<dyn Serializer>::new`.
let mut buffer = Vec::with_capacity(64);
let mut serializer = serde_json::Serializer::new(std::io::Cursor::new(&mut buffer));
let mut serializer = <dyn Serializer>::new(&mut serializer);
let serializer: &mut dyn Serializer = &mut serializer;

// 2. Just uses the dynamic serializer as a `serde::Serializer`.
let value = "Hello, world!";
value.serialize(serializer).unwrap();
assert_eq!(buffer, b"\"Hello, world!\"");
```

### Deserialization

Following example demonstrates how to perform deserialization with a dynamic deserializer, which is basically the same as serialization:

```rust
use serde::Deserialize;
use dyn_serde::Deserializer;

// 1. Creates dynamic deserializer using `<dyn Deserializer>::new`.
let mut deserializer = serde_json::Deserializer::from_str("\"Hello, world!\"");
let mut deserializer = <dyn Deserializer>::new(&mut deserializer);
let deserializer: &mut dyn Deserializer = &mut deserializer;

// 2. Just uses the dynamic deserializer as a `serde::Deserializer`.
let value = String::deserialize(deserializer).unwrap();
assert_eq!(value, "Hello, world!");
```

## Comparison with erased-serde

This crate (`dyn_serde`) is aimed at learning [`erased_serde`], which is licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a>.

And `dyn_serde` has made many improvements:

1. `dyn_serde` supports "no-std" and "no-alloc".

1. In the deserialization `erased_serde` uses a carefully maintained object `Any` to represent the deserialized value, which may need to perform memory management (and produces a lot of unsafe codes). But `dyn_serde` performs in-place deserialization and does not need any memory allocation.

1. Based to the above improvements, `dyn_serde` gets rid of all unsafe codes.

1. &hellip;&hellip;

Also, there are still some unimplemented functionalities:

1. The serialization result (`Serializer::Ok`) cannot be accessed by the dynamic serializer.

   Note, if the serializer is known to be `MakeSerializer<S>`, the method `expect` could help.

2. `erased_serde` provides macro `serialize_trait_object!` for implementing `serde::Serialize` for trait objects.

3. `dyn_serde` needs more detailed error messages.

4. &hellip;&hellip;

[`erased_serde`]: https://github.com/dtolnay/erased-serde/

## Performance

`bench/de.rs` is a simple benchmark that tests the deserialization performance of `serde_json` (version 0.4.5, default features), `erased-serde` (version 1.0.133, default features) and `dyn_serde` (version 1.0.0, default features).

- benchmark tool: [`criterion.rs`].

- data ([twiter.json], ~620 KiB) is from [nativejson-benchmark].

- cargo/rustc: version 1.83.0.

Following output shows, the dynamic deserializer defined in `dyn_serde` has much lower overhead (`4.0915 / 3.6208 = 1.123`) than `erased_serde` (`6.4764 / 3.6208 = 1.789`).

```text
$ cargo bench de

serde-json              time:   [3.5854 ms 3.6208 ms 3.6587 ms]
                        change: [-0.8127% +0.5305% +1.9817%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

dyn-serde               time:   [4.0693 ms 4.0915 ms 4.1174 ms]
                        change: [-0.4532% +0.3426% +1.1811%] (p = 0.40 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

erased-serde            time:   [6.4466 ms 6.4764 ms 6.5231 ms]
                        change: [-0.3880% +0.1163% +0.9587%] (p = 0.77 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
```

[`criterion.rs`]: https://github.com/bheisler/criterion.rs
[twiter.json]: https://github.com/miloyip/nativejson-benchmark/blob/master/data/twitter.json
[nativejson-benchmark]: https://github.com/miloyip/nativejson-benchmark

## No-std support

To opt off the Rust standard library, you can disable feature `std`:

```toml
dyn_serde = { version = "*", default-features = false }
```

If the memory allocation is needed (for example, `deserialize_byte_buf` uses `Vec<u8>`), you can enable feature `alloc`:

```toml
dyn_serde = { version = "*", default-features = false, features = ["alloc"] }
```

NOTE: if both `std` and `alloc` are disabled, the (de)serialization error would not provide useful error messages!

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
