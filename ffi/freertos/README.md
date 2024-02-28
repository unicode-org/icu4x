# icu_freertos [![crates.io](https://img.shields.io/crates/v/icu_freertos)](https://crates.io/crates/icu_freertos)

<!-- cargo-rdme start -->

This crate is a shim that enables one to use icu4x on Cortex-M + FreeRTOS by setting up the
relevant Rust runtime hooks.

Note that compiling to this platform needs Rust nightly, and this crate attempts to
build across multiple nightly versions.

This crate has a build script that will attempt to detect the nightly version and configure
things appropriately, where possible. Older nightlies will end up setting the
`--cfg needs_alloc_error_handler` flag: if using a custom build system and a nightly from
2022 or earlier, please set this flag.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
