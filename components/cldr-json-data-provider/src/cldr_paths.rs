// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::{Error, MissingSourceError};
use std::default::Default;
use std::path::PathBuf;

/// Trait returning filesystem paths to CLDR JSON resource directories.
/// The fields should be Ok if present. They default to Err when not present.
pub trait CldrPaths: std::fmt::Debug {
    /// Path to checkout of cldr-core:
    /// https://github.com/unicode-cldr/cldr-core
    fn cldr_core(&self) -> Result<PathBuf, Error>;

    /// Path to checkout of cldr-dates:
    /// https://github.com/unicode-cldr/cldr-dates-full
    fn cldr_dates(&self) -> Result<PathBuf, Error>;
}

/// Implementation of CldrPaths for data directories already downloaded.
///
/// # Example
///
/// ```
/// use icu_cldr_json_data_provider::CldrPathsLocal;
/// use icu_cldr_json_data_provider::CldrJsonDataProvider;
/// use std::path::PathBuf;
///
/// let mut paths = CldrPathsLocal::default();
/// paths.cldr_core = Ok(PathBuf::from("/path/to/cldr-core"));
/// // fill in other paths as necessary
///
/// let data_provider = CldrJsonDataProvider::new(&paths);
/// ```
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct CldrPathsLocal {
    pub cldr_core: Result<PathBuf, MissingSourceError>,
    pub cldr_dates: Result<PathBuf, MissingSourceError>,
}

impl CldrPaths for CldrPathsLocal {
    fn cldr_core(&self) -> Result<PathBuf, Error> {
        self.cldr_core.clone().map_err(|e| e.into())
    }
    fn cldr_dates(&self) -> Result<PathBuf, Error> {
        self.cldr_dates.clone().map_err(|e| e.into())
    }
}

impl Default for CldrPathsLocal {
    fn default() -> CldrPathsLocal {
        CldrPathsLocal {
            cldr_core: Err(MissingSourceError { src: "cldr-core" }),
            cldr_dates: Err(MissingSourceError { src: "cldr-dates" }),
        }
    }
}
