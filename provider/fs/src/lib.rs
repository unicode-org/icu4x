// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_fs_data_provider` is one of the [`ICU4X`] components.
//!
//! It reads ICU4X data files from the filesystem in a given directory. It can also export data to
//! the filesystem via an iterable data provider (see the `export` module).
//!
//! # Examples
//!
//! ```
//! use icu_provider_fs::FsDataProvider;
//!
//! let provider = FsDataProvider::try_new("/path/to/data/directory")
//!     .expect_err("Specify a real directory in the line above");
//! ```
//!
//! # Directory Structure
//!
//! The ICU4X data directory has a file named *manifest.json* at the root, and a nested structure
//! with category (ResourceCategory), subcategory@version, optional variant, and language identifier
//! as the leaf data files. For example, Arabic JSON data for cardinal plurals lives at
//! *plurals/cardinal@1/ar.json*.
//!
//! The exact form of the directory structure may change over time. ICU4X uses metadata from
//! *manifest.json* to dynamically interpret different versions of the directory structure.
//!
//! ```text
//! ├── manifest.json
//! ├── dates
//! │   └── gregory@1
//! │       ├── ar-EG.json
//! │       ├── ar.json
//! │       ├── be.json
//! │       ⋮
//! │       └── und.json
//! └── plurals
//!     ├── cardinal@1
//!     │   ├── ar.json
//!     │   ├── be.json
//!     │   ⋮
//!     │   └── und.json
//!     └── ordinal@1
//!         ├── ar.json
//!         ├── be.json
//!         ⋮
//!         └── und.json
//! ```
//!
//! # Resource Formats
//!
//! `ICU4X` data can be stored in different formats. At the moment there are:
//!
//! * JSON - Textual format, easy to read
//! * Bincode - Binary, fast resource format
//!
//! The directory passed to the [`FsDataProvider`] constructor may contain either of them.
//!
//! # Exporting data
//!
//! To generate the data required for [`FsDataProvider`], run the following from the top level:
//!
//! ```text
//! cargo run             \
//!   --bin icu4x-datagen \
//!   --                  \
//!   --cldr-tag 39.0.0   \
//!   --out ./icu4x-data  \
//!   --all-keys          \
//!   --all-locales
//! ```
//!
//! To export `bincode` format, use
//!
//! ```text
//! cargo run             \
//!   --bin icu4x-datagen \
//!   --                  \
//!   --cldr-tag 39.0.0   \
//!   --out ./icu4x-data  \
//!   --all-keys          \
//!   --all-locales       \
//!   -s bincode
//! ```
//!
//! *Notice:* In order to use `bincode` encoded data in production, [`icu_provider_fs`](crate) has to be
//! added with `bincode` feature.
//!
//! [`ICU4X`]: ../icu/index.html

mod deserializer;
mod error;
mod fs_data_provider;
pub mod manifest;

#[cfg(feature = "export")]
pub mod export;

pub use error::Error as FsDataError;
pub use fs_data_provider::FsDataProvider;
