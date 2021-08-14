// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    all(target_os = "none", feature = "freertos"),
    feature(alloc_error_handler)
)]
#![no_std]
#![allow(clippy::upper_case_acronyms)]

// Use Dlmalloc to remove the system allocator dependency
#[cfg(feature = "rust_global_allocator")]
#[global_allocator]
static ALLOCATOR: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

// Needed to be able to build cdylibs/etc
//
// Renamed so you can't accidentally use it
#[cfg(not(target_os = "none"))]
extern crate std as rust_std;

extern crate alloc;

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
// https://github.com/unicode-org/icu4x/issues/891
#[cfg(all(target_os = "none", feature = "freertos"))]
mod freertos_glue;
