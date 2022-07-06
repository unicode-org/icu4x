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
        clippy::panic
    )
)]
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

// Renamed so you can't accidentally use it
#[cfg(target_arch = "wasm32")]
extern crate std as rust_std;

extern crate alloc;

pub mod bidi;
pub mod calendar;
pub mod data_struct;
pub mod datetime;
pub mod decimal;
pub mod errors;
pub mod fixed_decimal;
pub mod locale;
pub mod locale_canonicalizer;
pub mod pluralrules;
pub mod properties_maps;
pub mod properties_sets;
pub mod provider;
pub mod segmenter_grapheme;
pub mod segmenter_line;
pub mod segmenter_sentence;
pub mod segmenter_word;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;
