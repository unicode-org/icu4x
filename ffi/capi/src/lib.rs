// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(target_os = "none", feature(alloc_error_handler))]
#![allow(clippy::upper_case_acronyms)]

#![cfg_attr(not(target_arch = "wasm32"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod custom_writeable;
pub mod decimal;
pub mod fixed_decimal;
pub mod locale;
pub mod locale_canonicalizer;
pub mod pluralrules;
pub mod provider;

#[cfg(target_arch = "wasm32")]
mod wasm_glue;

// Assume "none" is FreeRTOS for now
#[cfg(target_os = "none")]
mod freertos_glue;
