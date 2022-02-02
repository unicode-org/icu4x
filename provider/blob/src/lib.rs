// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_blob` contains implementations of the [`ICU4X`] [`DataProvider`] interface
//! that load data from a single blob.
//!
//! There are two exports:
//!
//! 1. [`BlobDataProvider`] supports data blobs loaded dynamically at runtime.
//! 2. [`StaticDataProvider`] supports data blobs baked into the binary at compile time.
//!
//! To build blob data, use the `--format blob` option of [`icu4x-datagen`]. For example, to build
//! "hello world" data, run:
//!
//! ```bash
//! $ cargo run --bin=icu4x-datagen -- \
//!     --format blob \
//!     --hello-world-key \
//!     --all-locales \
//!     --out hello_world.postcard
//! ```
//!
//! # Example
//!
//! Create a [`StaticDataProvider`] from pre-built test data:
//!
//! ```
//! let _ = icu_testdata::get_static_provider();
//! ```
//!
//! For more examples, see the specific data providers.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: icu_provider::prelude::DataProvider
//! [`icu4x-datagen`]: https://github.com/unicode-org/icu4x/tree/main/tools/datagen#readme

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod blob_data_provider;
mod blob_schema;
mod static_data_provider;

#[cfg(feature = "export")]
pub mod export;

pub use blob_data_provider::BlobDataProvider;
pub use static_data_provider::StaticDataProvider;
