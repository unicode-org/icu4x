# zerovec [![crates.io](https://img.shields.io/crates/v/zerovec)](https://crates.io/crates/zerovec)

Zero-copy vector abstractions over byte arrays.

`zerovec` enable vectors of multibyte types to be backed by a byte array, abstracting away
issues including memory alignment and endianness.

This crate has four main types:

- [`ZeroVec<T>`](ZeroVec) for fixed-width types like `u32`
- [`VarZeroVec<T>`](VarZeroVec) for variable-width types like `str`
- [`ZeroMap<K, V>`](ZeroMap) to map from `K` to `V`
- [`ZeroMap2d<K0, K1, V>`](ZeroMap2d) to map from the pair `(K0, K1)` to `V`

The first two are intended as drop-in replacements for `Vec<T>` in Serde structs serialized
with a format supporting a borrowed byte buffer, like Bincode. The third and fourth are
intended as a replacement for `HashMap` or `LiteMap`.

Clients upgrading to `zerovec` benefit from zero heap allocations when deserializing
read-only data.

This crate has two optional features: `serde` and `yoke`. `serde` allows serializing and deserializing
`zerovec`'s abstractions via [`serde`](https://docs.rs/serde), and `yoke` enables implementations of `Yokeable`
from the [`yoke`](https://docs.rs/yoke/) crate.

## Performance

`zerovec` is designed for fast deserialization from byte buffers with zero memory allocations
while minimizing performance regressions for common vector operations.

Benchmark results on x86_64:

| Operation | `Vec<T>` | `zerovec` |
|---|---|---|
| Deserialize vec of 100 `u32` | 233.18 ns | 14.120 ns |
| Compute sum of vec of 100 `u32` (read every element) | 8.7472 ns | 10.775 ns |
| Binary search vec of 1000 `u32` 50 times | 442.80 ns | 472.51 ns |
| Deserialize vec of 100 strings | 7.3740 μs\* | 1.4495 μs |
| Count chars in vec of 100 strings (read every element) | 747.50 ns | 955.28 ns |
| Binary search vec of 500 strings 10 times | 466.09 ns | 790.33 ns |

\* *This result is reported for `Vec<String>`. However, Serde also supports deserializing to `Vec<&str>`; this gives 1.8420 μs, much faster than `Vec<String>` but a bit slower than `zerovec`.*

| Operation | `HashMap<K,V>`  | `LiteMap<K,V>` | `ZeroMap<K,V>` |
|---|---|---|---|
| Deserialize a small map | 2.72 μs | 1.28 μs | 480 ns |
| Deserialize a large map | 50.5 ms | 18.3 ms | 3.74 ms |
| Look up from a small deserialized map | 49 ns | 42 ns | 54 ns |
| Look up from a large deserialized map | 51 ns | 155 ns | 213 ns |

Small = 16 elements, large = 131,072 elements. Maps contain `<String, String>`.

The benches used to generate the above table can be found in the `benches` directory in the project repository.
`zeromap` benches are named by convention, e.g. `zeromap/deserialize/small`, `zeromap/lookup/large`. The type
is appended for baseline comparisons, e.g. `zeromap/lookup/small/hashmap`.

## Features

- `serde`: enables Serde Serialize/Deserialize impls for ZeroVec and VarZeroVec.

## Examples

Serialize and deserialize a struct with ZeroVec and VarZeroVec with Bincode:

```rust
use zerovec::{ZeroVec, VarZeroVec};

// This example requires the "serde" feature
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataStruct<'data> {
    #[serde(borrow)]
    nums: ZeroVec<'data, u32>,
    #[serde(borrow)]
    strs: VarZeroVec<'data, str>,
}

let data = DataStruct {
    nums: ZeroVec::from_slice(&[211, 281, 421, 461]),
    strs: VarZeroVec::from(&["hello", "world"]),
};
let bincode_bytes = bincode::serialize(&data)
    .expect("Serialization should be successful");
assert_eq!(54, bincode_bytes.len());

let deserialized: DataStruct = bincode::deserialize(&bincode_bytes)
    .expect("Deserialization should be successful");
assert_eq!(Some(211), deserialized.nums.first());
assert_eq!(Some("world"), deserialized.strs.get(1));
assert!(matches!(deserialized.nums, ZeroVec::Borrowed(_)));
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
