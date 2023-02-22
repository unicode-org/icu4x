// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use icu_provider::prelude::*;
use std::collections::{HashSet, BTreeSet};

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Options {
    #[serde(with = "data_key_as_str")]
    pub keys: HashSet<DataKey>,
    pub locales: LocaleOptions,
    pub fallback: FallbackOptions,
}

mod data_key_as_str {
    use serde::{de::*, ser::*};
    use super::*;
    use std::borrow::Cow;

    pub fn serialize<S: Serializer>(selff: &HashSet<DataKey>, ser: S) -> Result<S::Ok, S::Error> {
        selff.iter().map(|k| k.path().get()).collect::<BTreeSet<_>>().serialize(ser)
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
pub struct LocaleOptions {
    pub locales: LocaleInclude,
    pub regions: RegionInclude,
    pub include_root: bool,
    pub some_segmenter_flag: bool,
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LocaleInclude {
    All,
    None,
    Explicit(Vec<LanguageIdentifier>),
    CldrSet(Vec<CoverageLevel>),
}

impl LocaleInclude {
    pub(crate) fn resolved(
        self,
        sources: &crate::SourceData,
    ) -> Result<Self, DataError> {
        Ok(match self {
            LocaleInclude::CldrSet(levels) => LocaleInclude::Explicit(sources.locales(&levels)?),
            s => s
        })
    }
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RegionInclude {
    All,
    Explicit(Vec<Region>),
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FallbackOptions {
    pub variant: FallbackVariant,
}

#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FallbackVariant {
    None,
    Naive,
    Full,
}
