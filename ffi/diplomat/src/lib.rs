// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    any(
        all(feature = "freertos", not(feature = "x86tiny")),
        all(feature = "x86tiny", not(feature = "freertos")),
    ),
    feature(alloc_error_handler)
)]
#![no_std]
#![allow(clippy::upper_case_acronyms)]

//! This module contains the source of truth for the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated
//! FFI bindings. This generates the C, C++ and Wasm bindings. This module also contains the C
//! FFI for ICU4X.
//!
//! To re-generate the bindings run:
//!
//! ```sh
//! cargo make diplomat-install
//! cargo make diplomat-gen
//! ```
//!

// Needed to be able to build cdylibs/etc
//
// Renamed so you can't accidentally use it
#[cfg(all(not(feature = "freertos"), not(feature = "x86tiny")))]
extern crate std as rust_std;

extern crate alloc;

pub mod custom_writeable;
pub mod decimal;
pub mod fixed_decimal;
pub mod locale;
pub mod locale_canonicalizer;
pub mod pluralrules;
pub mod properties_maps;
pub mod properties_sets;
pub mod provider;
pub mod segmenter_line;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;

#[cfg(all(feature = "freertos", not(feature = "x86tiny")))]
mod freertos_glue;

#[cfg(all(feature = "x86tiny", not(feature = "freertos")))]
mod x86tiny_glue;
