# shortvec [![crates.io](https://img.shields.io/crates/v/shortvec)](https://crates.io/crates/shortvec)

<!-- cargo-rdme start -->

This crate includes variable-length data types that are const-constructible for single
values and overflow to the heap.

## Why?

This crate is far from the first stack-or-heap vector in the Rust ecosystem. It was created
with the following value proposition:

1. Enable safe const construction of stack collections.
2. Avoid stack size penalties common with stack-or-heap collections.

As of this writing, `heapless` and `tinyvec` don't support const construction except
for empty vectors, and `smallvec` supports it on unstable.

Additionally, [`ShortBoxSlice`] has a smaller stack size than any of these:

```rust
use core::mem::size_of;

// NonZeroU64 has a niche that this crate utilizes
use core::num::NonZeroU64;

// ShortBoxSlice is the same size as `Box<[]>` for small or nichey values
assert_eq!(16, size_of::<shortvec::ShortBoxSlice::<NonZeroU64>>());

// Note: SmallVec supports pushing and therefore has a capacity field
assert_eq!(24, size_of::<smallvec::SmallVec::<[NonZeroU64; 1]>>());

// Note: heapless doesn't support spilling to the heap
assert_eq!(16, size_of::<heapless::Vec::<NonZeroU64, 1>>());

// Note: TinyVec only supports types that implement `Default`
assert_eq!(24, size_of::<tinyvec::TinyVec::<[u64; 1]>>());
```

The crate is `no_std` with `alloc`.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
