// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_blob` contains implementations of the [`ICU4X`] [`DataProvider`] interface
//! that load data from a single blob.
//!
//! Currently, this crate supports only static blobs, but it will soon support blobs loaded
//! dynamically at runtime (see [#848](https://github.com/unicode-org/icu4x/issues/848)).
//!
//! To build blob data, use the `--format blob` option of [`icu4x-datagen`]. For example, to build
//! "hello world" data, run:
//!
//! ```bash
//! $ cargo run --bin=icu4x-datagen -- \
//!     -v \
//!     --format blob \
//!     --hello-world-key \
//!     --all-locales \
//!     --out hello_world.bincode
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
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: icu_provider::prelude::DataProvider
//! [`icu4x-datagen`]: https://github.com/unicode-org/icu4x/tree/main/tools/datagen#readme

mod blob_schema;
mod path_util;
mod static_data_provider;

#[cfg(feature = "export")]
pub mod export;

pub use static_data_provider::StaticDataProvider;
