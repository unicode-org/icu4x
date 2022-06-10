// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_codepointtrie::CodePointTrie;
use icu_provider::{yoke, zerofrom};
use icu_uniset::UnicodeSet;
use zerovec::ZeroVec;

#[cfg(feature = "serde")]
use serde;

use crate::u24::U24;

#[icu_provider::data_struct(CanonicalDecompositionDataV1Marker = "normalizer/nfd@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionDataV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub decomposition_starts_with_non_starter: UnicodeSet<'data>,
}

#[icu_provider::data_struct(
    CompatibilityDecompositionSupplementV1Marker = "normalizer/nfkd@1",
    Uts46DecompositionSupplementV1Marker = "normalizer/uts46d@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionSupplementV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    pub flags: u8,
}

impl DecompositionSupplementV1<'_> {
    const HALF_WIDTH_VOICING_MARK_MASK: u8 = 1;
    const IOTA_SUBSCRIPT_MASK: u8 = (1 << 1);

    /// Whether half-width kana voicing marks decompose into non-starters
    /// (their full-width combining counterparts).
    pub fn half_width_voicing_marks_become_non_starters(&self) -> bool {
        (self.flags & DecompositionSupplementV1::HALF_WIDTH_VOICING_MARK_MASK) != 0
    }

    /// Whether U+0345 COMBINING GREEK YPOGEGRAMMENI decomposes into a
    /// starter (U+03B9 GREEK SMALL LETTER IOTA).
    pub fn iota_subscript_becomes_starter(&self) -> bool {
        (self.flags & DecompositionSupplementV1::IOTA_SUBSCRIPT_MASK) != 0
    }
}

#[icu_provider::data_struct(
    CanonicalDecompositionTablesV1Marker = "normalizer/nfdex@1",
    CompatibilityDecompositionTablesV1Marker = "normalizer/nfkdex@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionTablesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars16: ZeroVec<'data, u16>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars24: ZeroVec<'data, U24>,
}
