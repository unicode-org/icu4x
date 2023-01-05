// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::DataError;
use itertools::Itertools;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use tzif::data::tzif::TzifData;
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct TzifDir<'a>(&'a TzdbPaths, String);

impl<'a> TzifDir<'a> {
    pub(crate) fn read_and_parse(&self) -> Result<Vec<(String, TzifData)>, DataError> {
        self.0.read_and_parse_tzifs(&self.1)
    }
}

#[derive(Debug)]
pub(crate) struct TzdbPaths(PathBuf);

impl TzdbPaths {
    pub(crate) fn new<T: Into<PathBuf>>(root: T) -> Self {
        Self(root.into())
    }

    pub(crate) fn tzif(&self) -> TzifDir<'_> {
        TzifDir(self, "tzif".to_string())
    }

    pub(crate) fn read_and_parse_tzifs(
        &self,
        path: &str,
    ) -> Result<Vec<(String, TzifData)>, DataError> {
        self.transitive_file_list(path)
            .into_iter()
            .map(|path| {
                tzif::parse_tzif_file(self.0.join(&path))
                    .map(|data| (tzid_from_path(&path), data))
                    .map_err(|e| {
                        DataError::custom("Tzdb error")
                            .with_display_context(&e)
                            .with_path_context(&path)
                    })
            })
            .collect()
    }

    fn transitive_file_list(&self, path: &str) -> Vec<PathBuf> {
        WalkDir::new(self.0.join(path))
            .follow_links(true)
            .into_iter()
            .flatten()
            .filter(|entry| entry.file_type().is_file())
            .map(|file| file.into_path())
            .collect()
    }
}

/// The data from the IANA Time Zone Database is structured in a way that the file paths
/// themselves are the Time Zone Identifiers (TZIDs).
///
/// The [TzifDir] struct stores these paths in a root directory called `tzif` which is
/// not included in the TZID. This function walks the file path components backwards until
/// it locates the directory called `tzif` and then constructs the TZID from the path components.
fn tzid_from_path(path: &Path) -> String {
    // TODO(issue#) We are currently calling Itertools::intersperse to avoid ambiguity with the
    // experimental intersperse API in the standard library.
    //
    // https://github.com/rust-lang/rust/issues/79524
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
    //
    // We should replace Itertools::intersperse with std intersperse if/when the API stabilizes.
    // Or, if the API never stabilizes, remove the fully qualified function call and invoke as an
    // iterator adaptor.
    Itertools::intersperse(
        path.components()
            .into_iter()
            .map(|component| component.as_os_str().to_string_lossy())
            .rev()
            .take_while(|component| component != "tzif"),
        "/".into(),
    )
    // TakeWhile is not reversible so unfortunately we must collect to reverse.
    // Fortunately this is not performance-critical code, nor is it a large list of paths.
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect()
}
