# Dyn Serde

This crate provides support for dynamic serialization and deserialization based on [`serde`] with better performance and without `unsafe` codes.

[`serde`]: https://serde.rs/

## Overview

This crate provides three dyn-compatible version of the `serde` traits:

1. `Serialize` - dyn-compatible version of `serde::Serialize`:

1. `Serializer` - dyn-compatible version of `serde::Serializer`:

1. `Deserializer` - dyn-compatible version of `serde::Deserializer`:

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

## No-std support

To opt off the Rust standard library, you can disable feature `std`:

```toml
dyn_serde = { version = "*", default-features = false }
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
