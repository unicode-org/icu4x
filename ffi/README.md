# ICU4X FFI crates

This folder contains various FFI crates for ICU4X.

The primary ICU4X FFI API is generated via [diplomat](https://github.com/rust-diplomat/diplomat/) and can be found under `ffi/capi` (crate name `icu_capi`), alongside generated C, C++, and WASM bindings.

Due to [cargo #4881](https://github.com/rust-lang/cargo/issues/4881), it's not possible for `crate-type`s to be selectively built, and this can cause problems when designing FFI crates to be built on different platforms. For that reason, the `ffi/capi` `icu_capi` crate only builds as an `rlib` and should not be used directly when performing FFI. Instead, the `icu_capi_staticlib` (`ffi/capi_staticlib`) or `icu_capi_cdylib` (`ffi/capi_cdylib`) crates should be built to get the appropriate staticlib or cdylib target.

Some platforms and build systems will have special bindings: currently we have one such case: FreeRTOS. If you wish to build ICU4X for FreeRTOS, use the `ffi/freertos` (`icu_freertos`) crate.

The `experimental/ecma402` (`icu4x_ecma402`) crate contains an implementation of the [`ecma402_traits`](https://docs.rs/ecma402_traits) traits, enabling compatibility testing against ICU.
