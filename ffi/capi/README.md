# icu_capi [![crates.io](http://meritbadge.herokuapp.com/icu_capi)](https://crates.io/crates/icu_capi)

This module contains the C FFI for ICU4X. Currently it is also used as the source
of truth for the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated FFI bindings
for C++ and WASM. To re-generate the bindings run:

```sh
cargo make diplomat-install
cargo make diplomat-gen-c
```

Or re-generate all of the bindings:

```sh
cargo make diplomat-gen
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
