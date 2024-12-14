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

1. In the deserialization `erased_serde` uses a carefully maintained object `Any` to represent the deserialized value, which may need to perform memory management (and produces a lot of unsafe codes). But `dyn_serde` performs in-place deserialization and does not need any memory allocation.

2. Based to the above improvements, `dyn_serde` gets rid of all unsafe codes.

3. &hellip;&hellip;

Also, there are still some unimplemented functionalities:

1. The serialization result (`Serializer::Ok`) cannot be accessed by the dynamic serializer.

   Note, if the serializer is known to be `MakeSerializer<S>`, the method `expect` could help.

2. `erased_serde` provides macro `serialize_trait_object!` for implementing `serde::Serialize` for trait objects.

   Currently no plan to do this.

3. `dyn_serde` needs more detailed error messages.

4. &hellip;&hellip;

[`erased_serde`]: https://github.com/dtolnay/erased-serde/

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
