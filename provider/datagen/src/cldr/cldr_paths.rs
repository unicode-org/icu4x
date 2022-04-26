// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::path::PathBuf;

/// Struct encapsulating a CLDR data tree.
///
/// # Examples
///
/// ```
/// use icu_datagen::cldr::CldrPaths;
/// use std::path::PathBuf;
///
/// let paths = CldrPaths {
///     cldr_json_root: PathBuf::from("/path/to/cldr-json"),
///     locale_subset: "full".to_string(),
/// };
///
/// assert_eq!(paths.cldr_misc(), PathBuf::from("/path/to/cldr-json/cldr-misc-full"))
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct CldrPaths {
    /// Path to the CLDR JSON root directory
    pub cldr_json_root: PathBuf,
    /// CLDR JSON directory suffix: probably either "modern" or "full"
    pub locale_subset: String,
}

impl CldrPaths {
    /// Path to checkout of cldr-core:
    /// <https://github.com/unicode-cldr/cldr-core>
    pub fn cldr_core(&self) -> PathBuf {
        self.cldr_json_root.join("cldr-core")
    }

    /// Path to checkout of cldr-numbers:
    /// <https://github.com/unicode-cldr/cldr-numbers-full>
    pub fn cldr_numbers(&self) -> PathBuf {
        self.cldr_json_root
            .join(format!("cldr-numbers-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-misc:
    /// <https://github.com/unicode-cldr/cldr-misc-full>
    pub fn cldr_misc(&self) -> PathBuf {
        self.cldr_json_root
            .join(format!("cldr-misc-{}", self.locale_subset))
    }

    /// Path to checkout of cldr-bcp47:
    /// <https://github.com/unicode-org/cldr-json/tree/main/cldr-json/cldr-bcp47>
    pub fn cldr_bcp47(&self) -> PathBuf {
        self.cldr_json_root.join("cldr-bcp47")
    }

    /// Path to checkout of CLDR dates repository for given calendar
    ///
    /// `cal` should be a *CLDR* (not BCP-47) calendar identifier
    pub fn cldr_dates(&self, cal: &str) -> PathBuf {
        if cal == "gregorian" {
            self.cldr_json_root
                .join(format!("cldr-dates-{}", self.locale_subset))
        } else {
            self.cldr_json_root
                .join(format!("cldr-cal-{}-{}", cal, self.locale_subset))
        }
    }
}
