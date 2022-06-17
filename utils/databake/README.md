# databake [![crates.io](https://img.shields.io/crates/v/databake)](https://crates.io/crates/databake)

This crate allows data to write itself into Rust code (bake itself in).

Types that implement the `Bake` trait can be written into Rust expressions,
which allows using Rust code itself as a zero-overhead "serialization" strategy.

## Example
```rust
use databake::*;
use alloc::borrow::Cow;

let data = [Some((18, Cow::Borrowed("hi")))];
assert_eq!(
    data.bake(&Default::default()).to_string(),
    r#"[Some ((18i32 , :: alloc :: borrow :: Cow :: Borrowed ("hi")))]"#,
);
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
