// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `icu-cldr-json-data-provider` is one of the [`ICU4X`] components.
//!
//! It contains implementations of the [`DataProvider`] interface based on the JSON files
//! shipped by CLDR. You create a CldrPaths and then pass it into CldrJsonDataProvider.
//!
//! This crate contains two implementations of CldrPaths:
//!
//! - `CldrPathsLocal` points to local copies of the CLDR JSON repositories.
//! - `CldrPathsDownload` downloads and caches the CLDR JSON repositories. Requires the
//!   "download" feature.
//!
//! **Important:** This data provider implementation is not optimized for production use.
//! It is much more efficient if you use [`FsDataProvider`] instead.
//!
//! [`ICU4X`]: https://github.com/unicode-org/icu4x

mod cldr_langid;
mod cldr_paths;
mod error;
mod reader;
mod support;

pub mod transform;

#[cfg(feature = "download")]
pub mod download;

pub use cldr_paths::CldrPaths;
pub use cldr_paths::CldrPathsLocal;
pub use error::Error as CldrError;
pub use transform::CldrJsonDataProvider;
