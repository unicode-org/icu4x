# Tiny FixedDecimal FFI Demo

This example contains tooling to build a size-optimized binary using FixedDecimal and FixedDecimalFormatter in C over FFI.

Prerequisites: `clang` and `lld`, which must be compatible with the Rust toolchain. `apt-get install clang lld` *might* work, but if you run into errors, refer to the following thread for tips:

https://github.com/rust-lang/rust/issues/60059

You also need to install the correct toolchains:

```bash
$ rustup install nightly-2022-04-05
$ rustup component add --toolchain nightly-2022-04-05 rust-src
$ rustup target add x86_64-unknown-linux-gnu --toolchain nightly-2022-04-05
```
