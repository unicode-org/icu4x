// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc, feature = "export")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
    )
)]
#![warn(missing_docs)]

//! `icu_provider_blob` contains [`BlobDataProvider`], a [`BufferProvider`] implementation that
//! supports loading data from a single serialized blob.
//!
//! To build blob data, use the `--format blob` option of [`icu_provider_export`]:
//!
//! ```bash
//! $ icu4x-datagen --markers all --locales full --format blob --out data.postcard
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [`BufferProvider`]: icu_provider::buf::BufferProvider
//! [`icu_provider_export`]: ../icu_provider_export/index.html

#[cfg(feature = "alloc")]
extern crate alloc;

mod blob_data_provider;
mod blob_schema;

#[cfg(feature = "export")]
pub mod export;

pub use blob_data_provider::BlobDataProvider;
