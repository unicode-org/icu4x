# core_maths [![crates.io](https://img.shields.io/crates/v/core_maths)](https://crates.io/crates/core_maths)

<!-- cargo-rdme start -->

Extension trait for full float functionality in `#[no_std]` backed by [`libm`].

Method signatures and documentation are the same as `std` as of 1.72.

## Usage
```rust
#[allow(unused_imports)] // will be unused on std targets
use core_maths::*;

3.9.floor();
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
