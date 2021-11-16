// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::{Error, MissingSourceError};
use std::default::Default;
use std::path::PathBuf;

/// Trait returning filesystem paths to CLDR JSON resource directories.
/// The fields should be [`Ok`] if present. They default to [`Err`] when not present.
pub trait CldrPaths: std::fmt::Debug {
    /// Path to checkout of cldr-core:
    /// <https://github.com/unicode-cldr/cldr-core>
    fn cldr_core(&self) -> Result<PathBuf, Error>;

    /// Path to checkout of cldr-dates:
    /// <https://github.com/unicode-cldr/cldr-dates-full>
    fn cldr_dates_gregorian(&self) -> Result<PathBuf, Error>;

    /// Path to checkout of cldr-numbers:
    /// <https://github.com/unicode-cldr/cldr-numbers-full>
    fn cldr_numbers(&self) -> Result<PathBuf, Error>;

    /// Path to checkout of cldr-misc
    /// <https://github.com/unicode-cldr/cldr-misc-full>
    fn cldr_misc(&self) -> Result<PathBuf, Error>;

    /// Path to checkout of CLDR dates repository for given calendar
    ///
    /// `cal` should be a BCP-47 calendar identifier
    fn cldr_dates(&self, cal: &str) -> Result<PathBuf, Error> {
        if cal == "gregory" {
            self.cldr_dates_gregorian()
        } else {
            return Err(Error::Custom(format!("Unsupported calendar {}", cal), None));
        }
    }

    /// Returns a list of (CLDR name, BCP name, path) for each supported calendar
    fn cldr_dates_all(&self) -> Vec<(&'static str, &'static str, PathBuf)> {
        let mut vec = Vec::new();
        if let Ok(greg) = self.cldr_dates_gregorian() {
            vec.push(("gregorian", "gregory", greg));
        }
        // more calendars here
        vec
    }
}

/// An implementation of [`CldrPaths`] for multiple separate local CLDR JSON directories per
/// component.
///
/// # Examples
///
/// ```
/// use icu_provider_cldr::CldrPathsLocal;
/// use icu_provider_cldr::CldrJsonDataProvider;
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
    pub cldr_dates_gregorian: Result<PathBuf, MissingSourceError>,
    pub cldr_numbers: Result<PathBuf, MissingSourceError>,
    pub cldr_misc: Result<PathBuf, MissingSourceError>,
}

impl CldrPaths for CldrPathsLocal {
    fn cldr_core(&self) -> Result<PathBuf, Error> {
        self.cldr_core.clone().map_err(|e| e.into())
    }
    fn cldr_dates_gregorian(&self) -> Result<PathBuf, Error> {
        self.cldr_dates_gregorian.clone().map_err(|e| e.into())
    }
    fn cldr_numbers(&self) -> Result<PathBuf, Error> {
        self.cldr_numbers.clone().map_err(|e| e.into())
    }
    fn cldr_misc(&self) -> Result<PathBuf, Error> {
        self.cldr_misc.clone().map_err(|e| e.into())
    }
}

impl Default for CldrPathsLocal {
    fn default() -> Self {
        Self {
            cldr_core: Err(MissingSourceError { src: "cldr-core" }),
            cldr_dates_gregorian: Err(MissingSourceError { src: "cldr-dates" }),
            cldr_numbers: Err(MissingSourceError {
                src: "cldr-numbers",
            }),
            cldr_misc: Err(MissingSourceError { src: "cldr-misc" }),
        }
    }
}

/// An implementation of [`CldrPaths`] for one combined local CLDR JSON directory.
///
/// # Examples
///
/// ```
/// use icu_provider_cldr::CldrPathsAllInOne;
/// use icu_provider_cldr::CldrJsonDataProvider;
/// use std::path::PathBuf;
///
/// let paths = CldrPathsAllInOne {
///     cldr_json_root: PathBuf::from("/path/to/cldr-json"),
///     locale_subset: "full".to_string(),
/// };
///
/// let data_provider = CldrJsonDataProvider::new(&paths);
/// ```
#[derive(Debug, PartialEq)]
pub struct CldrPathsAllInOne {
    /// Path to the CLDR JSON root directory
    pub cldr_json_root: PathBuf,
    /// CLDR JSON directory suffix: probably either "modern" or "full"
    pub locale_subset: String,
}

impl CldrPaths for CldrPathsAllInOne {
    fn cldr_core(&self) -> Result<PathBuf, Error> {
        Ok(self.cldr_json_root.clone().join("cldr-core"))
    }
    fn cldr_dates_gregorian(&self) -> Result<PathBuf, Error> {
        Ok(self
            .cldr_json_root
            .clone()
            .join(format!("cldr-dates-{}", self.locale_subset)))
    }
    fn cldr_numbers(&self) -> Result<PathBuf, Error> {
        Ok(self
            .cldr_json_root
            .clone()
            .join(format!("cldr-numbers-{}", self.locale_subset)))
    }
    fn cldr_misc(&self) -> Result<PathBuf, Error> {
        Ok(self
            .cldr_json_root
            .clone()
            .join(format!("cldr-misc-{}", self.locale_subset)))
    }
}

#[cfg(test)]
pub(crate) fn for_test() -> CldrPathsAllInOne {
    CldrPathsAllInOne {
        cldr_json_root: icu_testdata::paths::cldr_json_root(),
        locale_subset: "full".to_string(),
    }
}
