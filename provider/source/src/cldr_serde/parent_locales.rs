// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON parentLocales.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/parentLocales.json>

use icu::locale::LanguageIdentifier;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(PartialEq, Debug, Deserialize, PartialOrd, Ord, Eq)]
pub(crate) struct LocaleRule {
    #[serde(rename = "nonlikelyScript")]
    pub(crate) non_likely_scripts: String,
}

#[derive(PartialEq, Debug, Deserialize, PartialOrd, Ord, Eq)]
pub(crate) struct LocaleRules {
    #[serde(rename = "parentLocale")]
    pub(crate) parent_locale: Option<LocaleRule>,
    pub(crate) collations: Option<LocaleRule>,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct ParentLocales {
    #[serde(rename = "parentLocale")]
    pub(crate) parent_locale: HashMap<LanguageIdentifier, LanguageIdentifier>,
    pub(crate) collations: BTreeMap<String, LanguageIdentifier>,
    #[serde(rename = "_localeRules", default = "rules_backport")]
    pub(crate) rules: LocaleRules,
}

fn rules_backport() -> LocaleRules {
    LocaleRules {
        parent_locale: Some(LocaleRule {
            non_likely_scripts: "root".into(),
        }),
        collations: None,
    }
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Supplemental {
    #[serde(rename = "parentLocales")]
    pub(crate) parent_locales: ParentLocales,
}

#[derive(PartialEq, Debug, serde_derive::Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
