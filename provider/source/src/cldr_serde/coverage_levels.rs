// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

// cldr-core/coverageLevels.json
#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Resource {
    #[serde(rename = "coverageLevels")]
    pub(crate) coverage_levels: HashMap<icu::locale::LanguageIdentifier, crate::CoverageLevel>,
}
