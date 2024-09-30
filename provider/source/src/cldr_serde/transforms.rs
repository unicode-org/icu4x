// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Display;

use icu::locale::Locale;
use serde::{Deserialize, Deserializer};

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(PartialEq, Debug, Default, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Visibility {
    Internal,
    #[default]
    External,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct LocaleFromString(pub(crate) Locale);

impl<'de> Deserialize<'de> for LocaleFromString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        Ok(Self(
            String::deserialize(deserializer)?
                .parse()
                .map_err(|_| D::Error::custom("invalid bcp47"))?,
        ))
    }
}

impl Display for LocaleFromString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// cldr-transforms/transforms/<lang>.json
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    #[serde(rename = "_rulesFile")]
    pub(crate) rules_file: String,
    #[serde(rename = "_direction")]
    pub(crate) direction: Direction,
    #[serde(rename = "_visibility", default)]
    pub(crate) visibility: Visibility,
    #[serde(rename = "_source")]
    pub(crate) source: String,
    #[serde(rename = "_target")]
    pub(crate) target: String,
    #[serde(rename = "_alias", default)]
    pub(crate) alias: Option<String>,
    #[serde(rename = "_backwardAlias", default)]
    pub(crate) backward_alias: Option<String>,
    #[serde(rename = "_aliasBcp47", default)]
    pub(crate) alias_bcp47: Option<String>,
    #[serde(rename = "_backwardAliasBcp47", default)]
    pub(crate) backward_alias_bcp47: Option<String>,
}
