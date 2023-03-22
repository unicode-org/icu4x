// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_blob` contains [`BlobDataProvider`], a [`BufferProvider`] implementation that
//! supports loading data from a single serialized blob.
//!
//! To build blob data, use the `--format blob` option of [`icu_datagen`]:
//!
//! ```bash
//! $ icu4x-datagen --keys all --locales full --format blob --out data.postcard
//! ```
//!
//! For examples, see the specific data providers.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`BufferProvider`]: icu_provider::BufferProvider
//! [`icu_datagen`]: ../icu_datagen/index.html

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod blob_data_provider;
mod blob_schema;

#[cfg(feature = "export")]
pub mod export;

pub use blob_data_provider::BlobDataProvider;
