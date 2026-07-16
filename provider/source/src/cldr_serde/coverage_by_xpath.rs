// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// Serde wrapper for `coverageByXPath.json` and locale-specific `coverageByXPath/{locale}.json` files.
#[derive(Deserialize, Debug)]
pub(crate) struct CoverageByXPathResource {
    /// Mapping from locale identifier (or `"root"`) to its corresponding coverage levels object.
    #[serde(rename = "coverageByXPath")]
    pub(crate) coverage_by_xpath: BTreeMap<String, CoverageByXPathLevels>,
}

/// Representation of coverage levels (`basic`, `core`, `moderate`, `modern`) containing sets of XPaths.
#[derive(Deserialize, Debug)]
pub(crate) struct CoverageByXPathLevels {
    /// XPaths classified under the `basic` coverage level.
    #[serde(default)]
    pub(crate) basic: BTreeSet<String>,
    /// XPaths classified under the `core` coverage level.
    #[serde(default)]
    pub(crate) core: BTreeSet<String>,
    /// XPaths classified under the `moderate` coverage level.
    #[serde(default)]
    pub(crate) moderate: BTreeSet<String>,
    /// XPaths classified under the `modern` coverage level.
    #[serde(default)]
    pub(crate) modern: BTreeSet<String>,
}
