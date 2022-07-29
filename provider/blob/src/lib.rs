// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_blob` contains implementations of the [`ICU4X`] [`BufferProvider`] interface
//! that load data from a single blob.
//!
//! There are two exports:
//!
//! 1. [`BlobDataProvider`] supports data blobs loaded dynamically at runtime.
//! 2. [`StaticDataProvider`] supports data blobs baked into the binary at compile time.
//!
//! To build blob data, use the `--format blob` option of [`icu_datagen`]. For example, to build
//! "hello world" data, run:
//!
//! ```bash
//! $ cargo run --features bin -p icu_datagen -- \
//!     --format blob \
//!     --hello-world-key \
//!     --all-locales \
//!     --out hello_world.postcard
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
        // TODO(#2266): enable missing_debug_implementations,
    )
)]

extern crate alloc;

mod blob_data_provider;
mod blob_schema;
mod static_data_provider;

#[cfg(feature = "export")]
pub mod export;

pub use blob_data_provider::BlobDataProvider;
pub use static_data_provider::StaticDataProvider;
