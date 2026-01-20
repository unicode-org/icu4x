// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::trivially_copy_pass_by_ref,
    )
)]
#![warn(missing_docs)]

//! `icu_provider_fs` is one of the [`ICU4X`] components.
//!
//! It reads ICU4X data files from the filesystem in a given directory.
//!
//! # Examples
//!
//! ```
//! use icu_provider_fs::FsDataProvider;
//!
//! let provider = FsDataProvider::try_new("/path/to/data/directory".into())
//!     .expect_err("Specify a real directory in the line above");
//! ```
//!
//! # Directory Structure
//!
//! The ICU4X data directory has a file named `manifest.json` at the root, and a nested structure
//! with a data marker ([`DataMarkerInfo`](icu_provider::DataMarkerInfo)), and locale ([`DataLocale`](icu_provider::DataLocale))
//! as the leaf data files. For example, Arabic JSON data for cardinal plural rules lives at `plurals/cardinal@1/ar.json`.
//!
//! The exact form of the directory structure may change over time. ICU4X uses metadata from
//! `manifest.json` to dynamically interpret different versions of the directory structure.
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
//! * Postcard - Binary, small `#[no_std]` resource format
//! * Bincode - Binary, fast resource format
//!
//! The directory passed to the [`FsDataProvider`] constructor may contain either of them.
//!
//! *Notice:* In order for ICU4X to be able to *deserialize* the returned data, the corresponding
//! Cargo feature has to be activated on the [`icu_provider`] crate. See
//! [`AsDeserializingBufferProvider::as_deserializing`](icu_provider::buf::AsDeserializingBufferProvider).
//!
//! # Exporting data
//!
//! To generate the data required for [`FsDataProvider`], run the following:
//!
//! ```bash
//! icu4x-datagen --markers all --locales full --format fs
//! ```
//!
//! To export `postcard` format, use
//!
//! ```bash
//! icu4x-datagen --markers all --locales full --format fs --syntax postcard
//! ```
//!
//! [`ICU4X`]: ../icu/index.html

mod datapath;
mod fs_data_provider;
mod manifest;

#[cfg(feature = "export")]
pub mod export;

pub use fs_data_provider::FsDataProvider;
