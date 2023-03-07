// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![no_std]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        // Exhaustiveness and Debug is not required for Diplomat types
    )
)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::result_unit_err)]

//! This crate contains the source of truth for the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated
//! FFI bindings. This generates the C, C++, JavaScript, and TypeScript bindings. This crate also contains the `extern "C"`
//! FFI for ICU4X.
//!
//! While the types in this crate are public, APIs from this crate are *not intended to be used from Rust*
//! and as such this crate may unpredictably change its Rust API across compatible semver versions. The `extern "C"` APIs exposed
//! by this crate, while not directly documented, are stable within the same major semver version, as are the bindings exposed under
//! the `cpp/` and `js/` folders.
//!
//! This crate may still be explored for documentation on docs.rs, and there are generated language-specific docs available as well.
//! C++ has sphinx docs in `cpp/docs/`, and the header files also contain documentation comments. The JS version has sphinx docs under
//! `js/docs`, and the TypeScript sources in `js/include` are compatible with `tsdoc`.
//!
//! This crate is `no_std` and will not typically build as a staticlib on its own. If you wish to link to it you should prefer
//! using `icu_capi_staticlib`, or for more esoteric platforms you may write a shim crate depending on this crate that hooks in
//! an allocator and panic hook.
//!
//! More information on using ICU4X from C++ can be found in [our tutorial].
//!
//! [our tutorial]: https://github.com/unicode-org/icu4x/blob/main/docs/tutorials/cpp.md
// Renamed so you can't accidentally use it
#[cfg(target_arch = "wasm32")]
extern crate std as rust_std;

extern crate alloc;

pub mod bidi;
pub mod calendar;
pub mod collator;
pub mod common;
pub mod data_struct;
pub mod date;
pub mod datetime;
pub mod datetime_formatter;
pub mod decimal;
pub mod errors;
pub mod fallbacker;
pub mod fixed_decimal;
pub mod list;
pub mod locale;
pub mod locid_transform;
pub mod logging;
pub mod normalizer;
pub mod normalizer_properties;
pub mod pluralrules;
pub mod properties_maps;
pub mod properties_sets;
pub mod properties_unisets;
pub mod provider;
pub mod script;
pub mod segmenter_grapheme;
pub mod segmenter_line;
pub mod segmenter_sentence;
pub mod segmenter_word;
pub mod time;
pub mod timezone;
pub mod timezone_formatter;
pub mod week;
pub mod zoned_formatter;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;
