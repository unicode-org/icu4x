// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_cldr` contains implementations of the [`ICU4X`] [`DataProvider`] interface
//! based on the JSON files shipped by CLDR. Create a [`CldrPaths`] and then pass it into
//! [`CldrJsonDataProvider`].
//!
//! This crate contains two implementations of [`CldrPaths`]:
//!
//! - [`CldrPathsLocal`] points to local copies of the CLDR JSON repositories.
//! - `CldrPathsDownload` downloads and caches the CLDR JSON repositories. Requires the
//!   "download" feature.
//!
//! **Important:** This data provider implementation is not optimized for production use.
//! It is much more efficient if you use [`FsDataProvider`] instead.
//!
//! [`ICU4X`]: ../icu/index.html
//! [`DataProvider`]: icu_provider::prelude::DataProvider
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`CldrJsonDataProvider`]: transform::CldrJsonDataProvider

mod cldr_langid;
mod cldr_paths;
mod error;
mod reader;
mod support;

pub mod transform;

#[cfg(feature = "download")]
pub mod download;

pub use cldr_paths::CldrPaths;
pub use cldr_paths::CldrPathsAllInOne;
pub use cldr_paths::CldrPathsLocal;
pub use error::Error as CldrError;
pub use transform::get_all_cldr_keys;
pub use transform::CldrJsonDataProvider;
