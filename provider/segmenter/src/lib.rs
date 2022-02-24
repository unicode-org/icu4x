// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_provider_segmenter` contains implementations of the [`ICU4X`] [data provider] interface
//! based on Unicode properties and TOML files implementing [Unicode Standard Annex #14][UAX14] and
//! [Unicode Standard Annex #29][UAX29] breaking rules.
//!
//! **Important:** This provider implementation is not optimized for production use. It is much more
//! efficient if you use [`FsDataProvider`] or [`StaticDataProvider`] instead.
//!
//! # Examples
//!
//! ```
//! use icu_provider_segmenter::SegmenterRuleProvider;
//! let provider = SegmenterRuleProvider::try_new("/path/to/data/directory")
//!     .expect_err("Specify a real directory in the line above");
//! ```
//!
//! # Exporting data
//!
//! To generate the data required by the segmenters, run `cargo make testdata` from the top level.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/
//! [`FsDataProvider`]: ../icu_provider_fs/struct.FsDataProvider.html
//! [`StaticDataProvider`]: ../icu_provider_blob/struct.StaticDataProvider.html

use icu_provider::fork::by_key::MultiForkByKeyProvider;
use icu_provider::iter::IterableDynProvider;
use icu_provider::prelude::*;
use std::path::{Path, PathBuf};

mod transform;

pub use crate::transform::SegmenterRuleProvider;

/// Returns the absolute path to the directory containing rule break data.
pub fn break_data_root() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
}

pub fn create_exportable_provider<T: DataMarker>(
    data_root: &Path,
) -> Result<MultiForkByKeyProvider<Box<dyn IterableDynProvider<T> + Sync>>, DataError>
where
    SegmenterRuleProvider: IterableDynProvider<T>,
{
    Ok(MultiForkByKeyProvider {
        providers: vec![Box::new(SegmenterRuleProvider::try_new(data_root)?)],
    })
}
