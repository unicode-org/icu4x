# deduplicating_array [![crates.io](https://img.shields.io/crates/v/deduplicating_array)](https://crates.io/crates/deduplicating_array)

<!-- cargo-rdme start -->

A serde serialization strategy that uses `PartialEq` to reduce serialized size.

This create can be used with Serde derive like this:

```rust

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Foo {
    #[serde(with = "deduplicating_array")]
    data: [Bar; 12],
    // ...
}
```

`Bar`s that are equal to a `Bar`s that appears earlier in the array will not be serialized
(instead, the index of the first occurence is serialized). Deserialization clones the first
`Bar` into all the indices where it occurs (hence `Bar` has to implement `Clone`).

Human readable serialization represents skipped values as singleton arrays containing the
target index, e.g. the Rust array `["Foo", "Bar", "Foo"]` will serialize to JSON `["Foo", "Bar", [0]]`.

This implies that singleton integer arrays cannot be used as array elements (they do work in Bincode,
but there's really not much point in using them).

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
