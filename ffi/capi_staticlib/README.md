# icu_capi_staticlib [![crates.io](https://img.shields.io/crates/v/icu_capi_staticlib)](https://crates.io/crates/icu_capi_staticlib)

This exists as a separate crate to work around
cargo being [unable to conditionally compile crate-types](https://github.com/rust-lang/cargo/issues/4881).

This leads to problems like emscripten being unable to link
because symbols like log_js are not defined even if the crate_type
is not actually desired. As a workaround, the `icu_capi_staticlib` and
`icu_capi_cdylib` crates exist as endpoints to be built when those
respective library types are needed.

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
