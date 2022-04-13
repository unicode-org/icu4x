// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
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
        self.cldr_json_root.clone().join("cldr-core")
    }

    /// Path to checkout of cldr-dates:
    /// <https://github.com/unicode-cldr/cldr-dates-full>
    pub fn cldr_dates_gregorian(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-dates-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-cal-buddhist:
    /// <https://github.com/unicode-cldr/cldr-cal-buddhist-full>
    pub fn cldr_dates_buddhist(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-cal-buddhist-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-cal-japanese:
    /// <https://github.com/unicode-cldr/cldr-cal-japanese-full>
    pub fn cldr_dates_japanese(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-cal-japanese-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-cal-coptic:
    /// <https://github.com/unicode-cldr/cldr-cal-coptic-full>
    pub fn cldr_dates_coptic(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-cal-coptic-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-cal-indian:
    /// <https://github.com/unicode-cldr/cldr-cal-indian-full>
    pub fn cldr_dates_indian(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-cal-indian-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-numbers:
    /// <https://github.com/unicode-cldr/cldr-numbers-full>
    pub fn cldr_numbers(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-numbers-{}", self.locale_subset))
    }
    /// Path to checkout of cldr-misc:
    /// <https://github.com/unicode-cldr/cldr-misc-full>
    pub fn cldr_misc(&self) -> PathBuf {
        self.cldr_json_root
            .clone()
            .join(format!("cldr-misc-{}", self.locale_subset))
    }

    /// Path to checkout of cldr-bcp47:
    /// <https://github.com/unicode-org/cldr-json/tree/main/cldr-json/cldr-bcp47>
    pub fn cldr_bcp47(&self) -> PathBuf {
        self.cldr_json_root.clone().join("cldr-bcp47")
    }

    /// Path to checkout of CLDR dates repository for given calendar
    ///
    /// `cal` should be a BCP-47 calendar identifier
    pub fn cldr_dates(&self, cal: &str) -> Result<PathBuf, DatagenError> {
        match cal {
            "gregory" => Ok(self.cldr_dates_gregorian()),
            "buddhist" => Ok(self.cldr_dates_buddhist()),
            "japanese" => Ok(self.cldr_dates_japanese()),
            "coptic" => Ok(self.cldr_dates_coptic()),
            "indian" => Ok(self.cldr_dates_indian()),
            _ => Err(DatagenError::Custom(
                format!("Unsupported calendar {}", cal),
                None,
            )),
        }
    }

    /// Returns a list of (CLDR name, BCP name, path) for each supported calendar
    pub fn cldr_dates_all(&self) -> Vec<(&'static str, &'static str, PathBuf)> {
        vec![
            ("gregorian", "gregory", self.cldr_dates_gregorian()),
            ("buddhist", "buddhist", self.cldr_dates_buddhist()),
            ("japanese", "japanese", self.cldr_dates_japanese()),
            ("coptic", "coptic", self.cldr_dates_coptic()),
            ("indian", "indian", self.cldr_dates_indian()),
            // TODO Japanese is not yet fully supported (#1116)
            // more calendars here
        ]
    }
}
