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
pub(crate) enum TransformAlias {
    Bcp47(Locale),
    LegacyId(String),
}

impl<'de> Deserialize<'de> for TransformAlias {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(Ok(locale)) = s.contains("-t-").then(|| s.parse::<Locale>()) {
            Ok(Self::Bcp47(locale))
        } else {
            Ok(Self::LegacyId(s))
        }
    }
}

impl Display for TransformAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bcp47(locale) => locale.fmt(f),
            Self::LegacyId(s) => s.fmt(f),
        }
    }
}

// cldr-transforms-full/main/<lang>/metadata.json
#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) direction: Direction,
    #[serde(default)]
    pub(crate) visibility: Visibility,
    pub(crate) source: String,
    pub(crate) target: String,
    #[serde(default)]
    pub(crate) variant: Option<String>,
    #[serde(default)]
    pub(crate) alias: Vec<TransformAlias>,
    #[serde(default)]
    #[serde(rename = "backwardAlias")]
    pub(crate) backward_alias: Vec<TransformAlias>,
}
