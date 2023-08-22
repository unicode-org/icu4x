// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use serde::Deserialize;
use std::collections::HashMap;

// cldr-core/coverageLevels.json
#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "coverageLevels")]
    pub coverage_levels: HashMap<icu_locid::LanguageIdentifier, crate::CoverageLevel>,
}
