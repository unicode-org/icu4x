# ICU4X FFI crates

This folder contains various FFI wrappers for ICU4X.

The primary ICU4X FFI is generated via [Diplomat](https://github.com/rust-diplomat/diplomat/) and can be found under `ffi/capi`, alongside generated C, C++, JS, TS, and Dart bindings.

In C and C++, the bindings in `ffi/capi/bindings` can be directly used and linked against a compiled version of `icu_capi`. See our [C++ example](https://github.com/unicode-org/icu4x/blob/main/examples/cpp) for more info.

For JS/TS we provide an NPM package in `ffi/npm`. This is currently not published on NPM.

For Dart we provide a package in `ffi/dart`. This is not currently published on `pub.dev`.

For use in FreeRTOS, we provide a special wrapper in `ffi/freertos`. This uses the FreeRTOS allocator and can be linked directly with a FreeRTOS firmware.

For use with the GN build tool, see `examples/gn` for an example setup.
