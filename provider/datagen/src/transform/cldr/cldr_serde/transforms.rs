// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Display;

use icu_locid::Locale;
use serde::{Deserialize, Deserializer};

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(in crate::provider) enum Direction {
    Forward,
    Backward,
    Both,
}

#[derive(PartialEq, Debug, Default, Copy, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(in crate::provider) enum Visibility {
    Internal,
    #[default]
    External,
}

#[derive(PartialEq, Debug, Clone)]
pub(in crate::provider) enum TransformAlias {
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
pub(in crate::provider) struct Resource {
    pub(in crate::provider) direction: Direction,
    #[serde(default)]
    pub(in crate::provider) visibility: Visibility,
    pub(in crate::provider) source: String,
    pub(in crate::provider) target: String,
    #[serde(default)]
    pub(in crate::provider) variant: Option<String>,
    #[serde(default)]
    pub(in crate::provider) alias: Vec<TransformAlias>,
    #[serde(default)]
    #[serde(rename = "backwardAlias")]
    pub(in crate::provider) backward_alias: Vec<TransformAlias>,
}
