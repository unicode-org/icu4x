// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub use icu_datagen::{CollationHanDatabase, CoverageLevel, FallbackMode, TrieType};
pub use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub keys: KeyInclude,
    pub fallback: FallbackMode,
    pub locales: LocaleInclude,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        serialize_with = "sorted_set"
    )]
    pub collations: HashSet<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub segmenter_models: SegmenterModelInclude,

    #[serde(default)]
    pub cldr: PathOrTag,
    #[serde(default)]
    pub icu_export: PathOrTag,
    #[serde(default)]
    pub segmenter_lstm: PathOrTag,
    #[serde(default, skip_serializing_if = "is_default")]
    pub trie_type: TrieType,
    #[serde(default, skip_serializing_if = "is_default")]
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
#[serde(rename_all = "camelCase")]
pub enum KeyInclude {
    None,
    All,
    Explicit(#[serde(with = "data_key_as_str")] HashSet<DataKey>),
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
#[serde(rename_all = "camelCase")]
pub enum LocaleInclude {
    Recommended,
    All,
    None,
    Explicit(#[serde(serialize_with = "sorted_set")] HashSet<LanguageIdentifier>),
    CldrSet(#[serde(serialize_with = "sorted_set")] HashSet<CoverageLevel>),
}

pub fn sorted_set<S: serde::Serializer>(
    selff: &HashSet<impl core::fmt::Debug + serde::Serialize>,
    ser: S,
) -> Result<S::Ok, S::Error> {
    use serde::Serialize;
    let mut sorted = selff.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|l| format!("{l:?}"));
    sorted.serialize(ser)
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SegmenterModelInclude {
    #[default]
    /// Set this data driver to generate the recommended set of segmenter models. This will cover
    /// all languages supported by ICU4X: Thai, Burmese, Khmer, Lao, Chinese, and Japanese.
    /// Both dictionary and LSTM models will be included, to the extent required by the chosen data keys.
    Recommended,
    None,
    Explicit(Vec<String>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub enum PathOrTag {
    Path(PathBuf),
    Tag(String),
    #[default]
    Latest,
    None,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Export {
    FileSystem {
        path: PathBuf,
        syntax: FsSyntax,
        #[serde(default, skip_serializing_if = "is_default")]
        fingerprint: bool,
    },
    Blob {
        path: PathBuf,
    },
    Baked {
        path: PathBuf,
        #[serde(default, skip_serializing_if = "is_default")]
        pretty: bool,
        #[serde(
            default,
            skip_serializing_if = "is_default",
            rename = "useSeparateCrates"
        )]
        use_separate_crates: bool,
        #[doc(hidden)] // we don't want this on the JSON API, but the CLI API goes through this struct
        #[serde(default, skip_serializing, skip_deserializing)]
        insert_feature_gates: bool,
    },
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FsSyntax {
    Postcard,
    Json,
    Bincode,
    JsonPretty,
}
