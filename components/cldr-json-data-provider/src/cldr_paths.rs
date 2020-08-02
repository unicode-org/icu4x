use crate::error::MissingSourceError;
use std::default::Default;
use std::path::PathBuf;

/// Struct containing filesystem paths to the CLDR JSON resource directories.
/// The fields should be Ok if present. They default to Err when not present.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct CldrPaths {
    /// Path to checkout of cldr-core:
    /// https://github.com/unicode-cldr/cldr-core
    pub cldr_core: Result<PathBuf, MissingSourceError>,
}

impl Default for CldrPaths {
    fn default() -> CldrPaths {
        CldrPaths {
            cldr_core: Err(MissingSourceError { src: "cldr-core" }),
        }
    }
}
