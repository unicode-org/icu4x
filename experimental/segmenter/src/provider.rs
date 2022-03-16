// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::{ZeroSlice, ZeroVec};

/// Pre-processed Unicode data in the form of tables to be used for rule-based breaking.
#[icu_provider::data_struct(
    LineBreakDataV1Marker = "segmenter/line@1",
    WordBreakDataV1Marker = "segmenter/word@1",
    GraphemeClusterBreakDataV1Marker = "segmenter/grapheme@1",
    SentenceBreakDataV1Marker = "segmenter/sentence@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct RuleBreakDataV1<'data> {
    /// Property table for rule-based breaking.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub property_table: RuleBreakPropertyTable<'data>,

    /// Break state table for rule-based breaking.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub break_state_table: RuleBreakStateTable<'data>,

    /// Number of properties; should be the square root of the length of [`Self::break_state_table`].
    pub property_count: u8,

    pub last_codepoint_property: i8,
    pub sot_property: u8,
    pub eot_property: u8,
    pub complex_property: u8,
}

/// Property table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct RuleBreakPropertyTable<'data>(
    #[cfg_attr(feature = "serialize", serde(borrow))] pub ZeroVec<'data, u8>,
);

/// Break state table for rule-based breaking.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct RuleBreakStateTable<'data>(
    #[cfg_attr(feature = "serialize", serde(borrow))] pub ZeroVec<'data, i8>,
);

/// char16trie data for dictionary break
#[icu_provider::data_struct(UCharDictionaryBreakDataV1Marker = "segmenter/char16trie@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize))]
pub struct UCharDictionaryBreakDataV1<'data> {
    /// Dictionary data of char16trie.
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub trie_data: ZeroVec<'data, u16>,
}

impl<'data> Default for UCharDictionaryBreakDataV1<'data> {
    fn default() -> Self {
        // Test data of thai dictionary
        const THAI_DICTIONARY: &ZeroSlice<u16> =
            match ZeroSlice::<u16>::try_from_bytes(include_bytes!("../tests/testdata/thai.dict")) {
                Ok(s) => s,
                Err(_) => panic!("invalid dictionary data"),
            };
        Self {
            trie_data: THAI_DICTIONARY.as_zerovec(),
        }
    }
}

pub const ALL_KEYS: [ResourceKey; 4] = [
    LineBreakDataV1Marker::KEY,
    GraphemeClusterBreakDataV1Marker::KEY,
    WordBreakDataV1Marker::KEY,
    SentenceBreakDataV1Marker::KEY,
];
