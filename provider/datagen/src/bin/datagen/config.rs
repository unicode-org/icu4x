// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub use icu_datagen::{CollationHanDatabase, CoverageLevel, FallbackMode, TrieType};
pub use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub keys: KeyInclude,
    pub fallback: FallbackMode,
    pub locales: LocaleInclude,
    #[serde(default, skip_serializing_if = "is_default")]
    pub collations: HashSet<String>,
    #[serde(default, skip_serializing_if = "is_default", rename = "segmenterModels")]
    pub segmenter_models: SegmenterModelInclude,

    #[serde(default)]
    pub cldr: PathOrTag,
    #[serde(default, rename = "icuExport")]
    pub icu_export: PathOrTag,
    #[serde(default, rename = "segmenterLstm")]
    pub segmenter_lstm: PathOrTag,
    #[serde(default, skip_serializing_if = "is_default", rename = "trieType")]
    pub trie_type: TrieType,
    #[serde(default, skip_serializing_if = "is_default", rename = "collationHanDatabase")]
    pub collation_han_database: CollationHanDatabase,

    pub export: Export,
    #[serde(default, skip_serializing_if = "is_default")]
    pub overwrite: bool,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum KeyInclude {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "explicit")]
    Explicit(#[serde(with = "data_key_as_str")] HashSet<DataKey>),
    #[serde(rename = "forBinary")]
    ForBinary(PathBuf),
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
            .map(|s| icu_datagen::key(&s).ok_or(s))
            .collect::<Result<_, _>>()
            .map_err(|s| D::Error::custom(format!("Unknown key {s}")))
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocaleInclude {
    #[serde(rename = "recommended")]
    Recommended,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "explicit")]
    Explicit(#[serde(serialize_with = "ordered_set")] HashSet<LanguageIdentifier>),
    #[serde(rename = "cldrSet")]
    CldrSet(HashSet<CoverageLevel>),
}

pub fn ordered_set<S: serde::Serializer>(selff: &HashSet<LanguageIdentifier>, ser: S) -> Result<S::Ok, S::Error> {
    use serde::Serialize;
    let mut sorted = selff.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|l| l.to_string());
    sorted.serialize(ser)
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum SegmenterModelInclude {
    #[default]
    /// Set this data driver to generate the recommended set of segmenter models. This will cover
    /// all languages supported by ICU4X: Thai, Burmese, Khmer, Lao, Chinese, and Japanese.
    /// Both dictionary and LSTM models will be included, to the extent required by the chosen data keys.
    #[serde(rename = "recommended")]
    Recommended,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "explicit")]
    Explicit(Vec<String>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub enum PathOrTag {
    #[serde(rename = "path")]
    Path(PathBuf),
    #[serde(rename = "tag")]
    Tag(String),
    #[default]
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Export {
    #[serde(rename = "fileSystem")]
    Fs {
        path: PathBuf,
        syntax: FsSyntax,
        #[serde(default, skip_serializing_if = "is_default")]
        fingerprint: bool,
    },
    #[serde(rename = "blob")]
    Blob {
        path: PathBuf,
    },
    #[serde(rename = "baked")]
    Baked {
        path: PathBuf,
        #[serde(default, skip_serializing_if = "is_default")]
        pretty: bool,
        #[serde(default, skip_serializing_if = "is_default", rename = "useSeparateCrates")]
        use_separate_crates: bool,
        #[doc(hidden)] // we don't want this on the JSON API, but the CLI API goes through this struct
        #[serde(default, skip_serializing, skip_deserializing)]
        insert_feature_gates: bool,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum FsSyntax {
    #[serde(rename = "postcard")]
    Postcard,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "bincode")]
    Bincode,
    #[serde(rename = "jsonPretty")]
    JsonPretty,
}
