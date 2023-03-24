// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options bag

pub use crate::transform::cldr::source::CoverageLevel;

use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use std::collections::HashSet;

/// Defines options for datagen
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    /// Defines the keys to include
    pub keys: KeyInclude,
    /// Defines the locales to include
    pub locales: LocaleInclude,
    /// Set this to true if `WordSegmenter::try_new_dictionary` is used. If only
    /// `try_new_auto` is used, we don't need dictionary data for languages that
    /// use LSTM.
    pub dictionary_segmenter: bool,
    /// Whether to optimize tries for speed or size
    pub trie_type: IcuTrieType,
    /// Which Han collation to use
    pub collation_han_database: CollationHanDatabase,
    /// The collation types to include
    pub collations: HashSet<String>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            keys: Default::default(),
            locales: Default::default(),
            dictionary_segmenter: Default::default(),
            trie_type: IcuTrieType::Small,
            collation_han_database: CollationHanDatabase::Implicit,
            collations: Default::default(),
        }
    }
}

impl crate::SourceData {
    // This should be on Options and take a &mut SourceData, but because of aliasing the call source.options.resolve(&source) doesn't compile.
    pub(crate) fn resolve(&mut self) -> Result<(), DataError> {
        self.options.locales = match core::mem::take(&mut self.options.locales) {
            LocaleInclude::None => LocaleInclude::Explicit(Default::default()),
            LocaleInclude::CldrSet(levels) => LocaleInclude::Explicit(
                self.locales(levels.iter().copied().collect::<Vec<_>>().as_slice())?
                    .into_iter()
                    .collect(),
            ),
            s => s,
        };
        Ok(())
    }
}

impl Options {
    pub(crate) fn resolved_keys(&self) -> HashSet<DataKey> {
        match &self.keys {
            KeyInclude::None => Default::default(),
            KeyInclude::All => crate::all_keys().into_iter().collect(),
            KeyInclude::AllWithExperimental => {
                crate::all_keys_with_experimental().into_iter().collect()
            }
            KeyInclude::Explicit(set) => set.clone(),
        }
    }
}

/// Defines the keys that will be generated
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum KeyInclude {
    /// No keys
    None,
    /// All stable keys
    All,
    /// All stable and experimental keys
    AllWithExperimental,
    /// An explicit set of keys.
    Explicit(HashSet<DataKey>),
}

impl Default for KeyInclude {
    fn default() -> Self {
        Self::All
    }
}

#[allow(dead_code)] // serde not in use yet
mod data_key_as_str {
    use super::*;
    use serde::{de::*, ser::*};
    use std::borrow::Cow;

    pub fn serialize<S: Serializer>(selff: &HashSet<DataKey>, ser: S) -> Result<S::Ok, S::Error> {
        selff
            .iter()
            .map(|k| k.path().get())
            .collect::<HashSet<_>>()
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

/// Defines the locaes to include
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum LocaleInclude {
    /// All locales
    All,
    /// No locales
    None,
    /// An explicit set of locales
    Explicit(HashSet<LanguageIdentifier>),
    /// All locales with the given CLDR coverage levels
    CldrSet(HashSet<CoverageLevel>),
}

impl Default for LocaleInclude {
    fn default() -> Self {
        Self::All
    }
}

/// Specifies the collation Han database to use.
///
/// Unihan is more precise but significantly increases data size. See
/// <https://github.com/unicode-org/icu/blob/main/docs/userguide/icu_data/buildtool.md#collation-ucadata>
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CollationHanDatabase {
    /// Implicit
    Implicit,
    /// Unihan
    Unihan,
}

impl std::fmt::Display for CollationHanDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CollationHanDatabase::Implicit => write!(f, "implicithan"),
            CollationHanDatabase::Unihan => write!(f, "unihan"),
        }
    }
}

/// Specifies the trie type to use.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum IcuTrieType {
    /// Fast tries are optimized for speed
    Fast,
    /// Small tries are optimized for size
    Small,
}

impl IcuTrieType {
    pub(crate) fn to_internal(self) -> icu_collections::codepointtrie::TrieType {
        match self {
            IcuTrieType::Fast => icu_collections::codepointtrie::TrieType::Fast,
            IcuTrieType::Small => icu_collections::codepointtrie::TrieType::Small,
        }
    }
}

impl std::fmt::Display for IcuTrieType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            IcuTrieType::Fast => write!(f, "fast"),
            IcuTrieType::Small => write!(f, "small"),
        }
    }
}
