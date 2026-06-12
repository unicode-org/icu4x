// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON locale display pattern files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-localenames-full/main/en/localeDisplayNames.json>

use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LocaleDisplayPattern {
    #[serde(rename = "localePattern")]
    pub(crate) locale_pattern: String,
    #[serde(rename = "localeSeparator")]
    pub(crate) locale_separator: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LocaleDisplayNames {
    #[serde(rename = "localeDisplayPattern")]
    pub(crate) locale_display_pattern: LocaleDisplayPattern,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct PatternDisplayNames {
    #[serde(rename = "localeDisplayNames")]
    pub(crate) localedisplaynames: LocaleDisplayNames,
}

pub(crate) type Resource = super::super::LocaleResource<PatternDisplayNames>;
