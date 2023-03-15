// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use icu_provider::prelude::*;
use std::collections::{BTreeSet, HashSet};

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Options {
    pub keys: KeyInclude,
    pub locales: LocaleInclude,
    pub regions: RegionInclude,
    pub fallback: Fallback,
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum KeyInclude {
    All,
    AllWithExperimental,
    Explicit(#[serde(with = "data_key_as_str")] HashSet<DataKey>),
}

impl Default for KeyInclude {
    fn default() -> Self {
        Self::All
    }
}

impl KeyInclude {
    pub(crate) fn resolved(&self) -> HashSet<DataKey> {
        match self {
            Self::All => crate::all_keys().into_iter().collect(),
            Self::AllWithExperimental => crate::all_keys_with_experimental().into_iter().collect(),
            Self::Explicit(set) => set.clone(),
        }
    }
}

mod data_key_as_str {
    use super::*;
    use serde::{de::*, ser::*};
    use std::borrow::Cow;

    pub fn serialize<S: Serializer>(selff: &HashSet<DataKey>, ser: S) -> Result<S::Ok, S::Error> {
        selff
            .iter()
            .map(|k| k.path().get())
            .collect::<BTreeSet<_>>()
            .serialize(ser)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<HashSet<DataKey>, D::Error> {
        HashSet::<Cow<'de, str>>::deserialize(de)?
            .into_iter()
            .map(|s| crate::key(&s).ok_or(s))
            .collect::<Result<_, _>>()
            .map_err(|s| D::Error::custom(format!("Unknown key {s}")))
    }
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LocaleInclude {
    All,
    None,
    Explicit(Vec<LanguageIdentifier>),
    CldrSet(Vec<CoverageLevel>),
}

impl Default for LocaleInclude {
    fn default() -> Self {
        Self::All
    }
}

impl LocaleInclude {
    pub(crate) fn resolved(self, sources: &crate::SourceData) -> Result<Self, DataError> {
        Ok(match self {
            LocaleInclude::CldrSet(levels) => LocaleInclude::Explicit(sources.locales(&levels)?),
            s => s,
        })
    }
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RegionInclude {
    All,
    // TODO: Add region selection
}

impl Default for RegionInclude {
    fn default() -> Self {
        Self::All
    }
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Fallback {
    None,
    // TODO: Add full and naive
}

impl Default for Fallback {
    fn default() -> Self {
        Self::None
    }
}
