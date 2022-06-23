// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs for the normalizer

use icu_char16trie::char16trie::Char16Trie;
use icu_codepointtrie::CodePointTrie;
use icu_provider::{yoke, zerofrom};
use icu_uniset::UnicodeSet;
use zerovec::ZeroVec;

#[cfg(feature = "serde")]
use serde;

use crate::u24::U24;

/// Main data for NFD
#[icu_provider::data_struct(CanonicalDecompositionDataV1Marker = "normalizer/nfd@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionDataV1<'data> {
    /// Trie for NFD decomposition.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    /// Set containing the characters whose NFD decomposition
    /// starts with a non-starter
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub decomposition_starts_with_non_starter: UnicodeSet<'data>,
}

/// Data that either NFKD or the decomposed form of UTS 46 needs
/// _in addition to_ the NFD data.
#[icu_provider::data_struct(
    CompatibilityDecompositionSupplementV1Marker = "normalizer/nfkd@1",
    Uts46DecompositionSupplementV1Marker = "normalizer/uts46d@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionSupplementV1<'data> {
    /// Trie for the decompositions that differ from NFD.
    /// Getting a zero from this trie means that you need
    /// to make another lookup from `DecompositionDataV1::trie`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    /// Flags that indicate how the set of characters whose
    /// decompositions starts with a non-starter differs from
    /// the set for NFD.
    ///
    /// Bit 0: Whether half-width kana voicing marks decompose
    ///        into non-starters (their full-width combining
    ///        counterparts).
    /// Bit 1: Whether U+0345 COMBINING GREEK YPOGEGRAMMENI
    ///        decomposes into a starter (U+03B9 GREEK SMALL
    ///        LETTER IOTA).
    /// (Other bits unused.)
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

/// The expansion tables for cases where the decomposition isn't
/// contained in the trie value
#[icu_provider::data_struct(
    CanonicalDecompositionTablesV1Marker = "normalizer/nfdex@1",
    CompatibilityDecompositionTablesV1Marker = "normalizer/nfkdex@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionTablesV1<'data> {
    /// Decompositions that are fully within the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars16: ZeroVec<'data, u16>,
    /// Decompositions with at least one character outside
    /// the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars24: ZeroVec<'data, U24>,
}

/// Non-Hangul canonical compositions
#[icu_provider::data_struct(CanonicalCompositionsV1Marker = "normalizer/comp@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CanonicalCompositionsV1<'data> {
    /// Trie keys are two-`char` strings with the second
    /// character coming first. The value, if any, is the
    /// (non-Hangul) canonical composition.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub canonical_compositions: Char16Trie<'data>,
}

/// Passthrough set
#[icu_provider::data_struct(
    CanonicalCompositionPassthroughV1Marker = "normalizer/nfc@1",
    CompatibilityCompositionPassthroughV1Marker = "normalizer/nfkc@1",
    Uts46CompositionPassthroughV1Marker = "normalizer/uts46@1"
)]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CompositionPassthroughV1<'data> {
    /// The set of characters that are starters that normalize to themselves
    /// if the next character doesn't combine backwards and that themselves
    /// never combine backwards.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub potential_passthrough_and_not_backward_combining: UnicodeSet<'data>,
}

/// Non-recursive canonical decompositions that differ from
/// `DecompositionDataV1`.
#[icu_provider::data_struct(NonRecursiveDecompositionSupplementV1Marker = "normalizer/decomp@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake), databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct NonRecursiveDecompositionSupplementV1<'data> {
    /// Trie for the supplementary non-recursive decompositions
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    /// Decompositions with at least one character outside
    /// the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars24: ZeroVec<'data, U24>,
}
