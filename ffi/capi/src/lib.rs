// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    all(target_os = "none", feature = "freertos"),
    feature(alloc_error_handler)
)]
#![allow(clippy::upper_case_acronyms)]
#![cfg_attr(target_os = "none", no_std)]

extern crate alloc;

pub mod custom_writeable;
#[cfg(feature = "decimal")]
pub mod decimal;
#[cfg(feature = "decimal")]
pub mod fixed_decimal;
pub mod locale;
#[cfg(feature = "locale_canonicalizer")]
pub mod locale_canonicalizer;
#[cfg(feature = "plurals")]
pub mod pluralrules;
pub mod provider;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;

// Assume "none" is FreeRTOS for now
// https://github.com/unicode-org/icu4x/issues/891
#[cfg(all(target_os = "none", feature = "freertos"))]
mod freertos_glue;
