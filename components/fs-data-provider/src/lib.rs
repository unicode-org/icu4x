// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `icu-fs-data-provider` is one of the [`ICU4X`] components.
//!
//! It reads ICU4X data files from the filesystem in a given directory. It can also export data to
//! the filesystem via an iterable data provider (see the `export` module).
//!
//! # Examples
//!
//! ```
//! use icu_fs_data_provider::FsDataProvider;
//!
//! let provider = FsDataProvider::try_new("/path/to/data/directory")
//!     .expect_err("Specify a real directoroy in the line above");
//! ```

mod error;
mod fs_data_provider;
pub mod manifest;

pub use error::Error as FsDataError;
pub use fs_data_provider::FsDataProvider;

#[cfg(feature = "export")]
pub mod export;
