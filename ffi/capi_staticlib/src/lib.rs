// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This exists as a separate crate to work around
//! cargo being unable to conditionally compile crate-types.
//!
//! https://github.com/rust-lang/cargo/issues/4881
//!
//! This leads to problems like emscripten being unable to link
//! because symbols like log_js are not defined even if the crate_type
//! is not actually desired. As a workaround, the capi_staticlib and capi_dylib
//! crates exist as endpoints to be built when those respective library types are needed.

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]


#![cfg_attr(
    all(feature = "x86tiny", not(feature = "internal_all_features_hack")),
    feature(alloc_error_handler)
)]

// Necessary to for symbols to be linked in
extern crate icu_capi;

extern crate alloc;


#[cfg(all(feature = "x86tiny", not(feature = "internal_all_features_hack")))]
mod x86tiny_glue;
