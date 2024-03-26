// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON parentLocales.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/parentLocales.json>

use icu_locid::LanguageIdentifier;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Debug, Deserialize)]
pub struct ParentLocales {
    #[serde(rename = "parentLocale")]
    pub parent_locale: HashMap<LanguageIdentifier, LanguageIdentifier>,
    pub collations: BTreeMap<String, LanguageIdentifier>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Supplemental {
    #[serde(rename = "parentLocales")]
    pub parent_locales: ParentLocales,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Resource {
    pub supplemental: Supplemental,
}
