# icu_capi [![crates.io](https://img.shields.io/crates/v/icu_capi)](https://crates.io/crates/icu_capi)

This crate contains the source of truth for the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated
FFI bindings. This generates the C, C++ and Wasm bindings. This crate also contains the `extern "C"`
FFI for ICU4X.

While the types in this crate are public, APIs from this crate are *not intended to be used from Rust*
and as such this crate may unpredictably change its Rust API across compatible semver versions. The `extern "C"` APIs exposed
by this crate, while not directly documented, are stable within the same major semver version, as are the bindings exposed under
the `cpp/` and `wasm/` folders.

This crate may still be explored for documentation on docs.rs, and there are generated language-specific docs available as well.
C++ has sphinx docs in `cpp/docs/`, and the header files also contain documentation comments. The JS/WASM port has sphinx docs under
`wasm/icu4x/docs`, and the TypeScript sources in `wasm/icu4x/lib` are compatible with `tsdoc`.

This crate is `no_std` and will not typically build as a staticlib on its own, if you wish to link to it you should prefer
using `icu_capi_staticlib`, or for more esoteric platforms you may write a shim crate depending on this crate that hooks in
an allocator and panic hook.

To re-generate the bindings run:

```sh
cargo make diplomat-install
cargo make diplomat-gen
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
