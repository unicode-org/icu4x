# icu_capi [![crates.io](https://img.shields.io/crates/v/icu_capi)](https://crates.io/crates/icu_capi)

This module contains the source of truth for the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated
FFI bindings. This generates the C, C++ and Wasm bindings. This module also contains the C
FFI for ICU4X.

To re-generate the bindings run:

```sh
cargo make diplomat-install
cargo make diplomat-gen
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
