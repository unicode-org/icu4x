// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON dayPeriods.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/dayPeriods.json>

use icu::locale::LanguageIdentifier;
use serde::Deserialize;
use std::collections::HashMap;

/// A single day period rule.
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct DayPeriodRule {
    /// The start hour (inclusive), e.g. "06:00".
    #[serde(rename = "_from")]
    pub(crate) from: Option<String>,
    /// The end hour (exclusive), e.g. "12:00".
    #[serde(rename = "_before")]
    pub(crate) before: Option<String>,
    /// The specific time, e.g. "00:00" for midnight.
    #[serde(rename = "_at")]
    pub(crate) at: Option<String>,
}

/// Rules for a set of locales.
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Rules(pub(crate) HashMap<LanguageIdentifier, HashMap<String, DayPeriodRule>>);

/// Supplemental data.
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    /// The day period rules.
    #[serde(rename = "dayPeriodRuleSet")]
    pub(crate) day_period_rules: Rules,
}

/// The resource.
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    /// The supplemental data.
    pub(crate) supplemental: Supplemental,
}
