# zerovec [![crates.io](http://meritbadge.herokuapp.com/zerovec)](https://crates.io/crates/zerovec)

Zero-copy vector abstractions over byte arrays.

This crate has two main types:

- `ZeroVec<T>` for fixed-width types like `u32`
- `VarZeroVec<T>` for variable-width types like `str`

Both are intended as drop-in replacements for `Vec<T>` in Serde structs serialized with a
format supporting a borrowed byte buffer, like Bincode.

## Features

- `serde`: enables Serde Serialize/Deserialize impls for ZeroVec and VarZeroVec.

## Example

```rust
use zerovec::ZeroVec;

// This example requires the "serde" feature
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataStruct<'s> {
    #[serde(borrow)]
    nums: ZeroVec<'s, u32>,
}

let data = DataStruct {
    nums: ZeroVec::from_aligned(&[211, 281, 421, 461]),
};
let bincode_bytes = bincode::serialize(&data)
    .expect("Serialization should be successful");
assert_eq!(24, bincode_bytes.len());

let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");
assert_eq!(Some(211), deserialized.nums.first());
assert!(matches!(deserialized.nums, ZeroVec::Borrowed(_)));
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
