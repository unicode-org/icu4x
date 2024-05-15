// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON parentLocales.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/parentLocales.json>

use icu_locale::LanguageIdentifier;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Debug, Deserialize, PartialOrd, Ord, Eq)]
pub(in crate::provider) struct LocaleRule {
    #[serde(rename = "nonlikelyScript")]
    pub(in crate::provider) non_likely_scripts: String,
}

#[derive(PartialEq, Debug, Deserialize, PartialOrd, Ord, Eq)]
pub(in crate::provider) struct LocaleRules {
    #[serde(rename = "parentLocale")]
    pub(in crate::provider) parent_locale: Option<LocaleRule>,
    pub(in crate::provider) collations: Option<LocaleRule>,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct ParentLocales {
    #[serde(rename = "parentLocale")]
    pub(in crate::provider) parent_locale: HashMap<LanguageIdentifier, LanguageIdentifier>,
    pub(in crate::provider) collations: BTreeMap<String, LanguageIdentifier>,
    #[serde(rename = "_localeRules", default = "rules_backport")]
    pub(in crate::provider) rules: LocaleRules,
}

fn rules_backport() -> LocaleRules {
    LocaleRules {
        parent_locale: Some(LocaleRule {
            non_likely_scripts: "root".into(),
        }),
        collations: None,
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Supplemental {
    #[serde(rename = "parentLocales")]
    pub(in crate::provider) parent_locales: ParentLocales,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(in crate::provider) struct Resource {
    pub(in crate::provider) supplemental: Supplemental,
}
