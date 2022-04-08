// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_datagen::segmenter` contains implementations of the [`ICU4X`] [data provider] interface
//! based on Unicode properties and TOML files implementing [Unicode Standard Annex #14][UAX14] and
//! [Unicode Standard Annex #29][UAX29] breaking rules.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::segmenter::SegmenterRuleProvider;
//! let provider = SegmenterRuleProvider::try_new(
//!     "/path/to/segmenter/data/directory",
//!     "/path/to/uprops/data/directory",
//! )
//! .expect_err("Specify a real directory in the line above");
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

use icu_provider::datagen::OmnibusDatagenProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fork::by_key::MultiForkByKeyProvider;
use std::path::{Path, PathBuf};

mod transform;

pub use transform::SegmenterRuleProvider;

/// Returns the absolute path to the directory containing the segmenter raw data.
pub fn segmenter_data_root() -> PathBuf {
    PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data")
}

#[macro_export]
macro_rules! create_segmenter_provider {
    ($segmenter_data_root:expr, $uprops_root:expr) => {{
        $crate::segmenter::SegmenterRuleProvider::try_new($segmenter_data_root, $uprops_root)?
    }};
}

pub fn create_exportable_provider<T: DataMarker>(
    segmenter_data_root: &Path,
    uprops_root: &Path,
) -> Result<MultiForkByKeyProvider<Box<dyn OmnibusDatagenProvider<T> + Sync>>, DataError>
where
    SegmenterRuleProvider: OmnibusDatagenProvider<T>,
{
    Ok(MultiForkByKeyProvider {
        providers: vec![Box::new(SegmenterRuleProvider::try_new(
            segmenter_data_root,
            uprops_root,
        )?)],
    })
}
