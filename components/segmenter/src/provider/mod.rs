// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

#[cfg(feature = "unstable")]
#[allow(missing_docs)]
mod neo;
#[cfg(feature = "unstable")]
pub use neo::*;
mod lstm;
pub use lstm::*;
#[cfg(feature = "unstable")]
pub mod radical;

use icu_collections::codepointtrie::CodePointTrie;
use icu_provider::prelude::*;
use zerovec::ZeroVec;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_segmenter_data::*;
    pub mod icu {
        pub use crate as segmenter;
        pub use icu_collections as collections;
        pub use icu_locale as locale;
    }
    make_provider!(Baked);
    impl_segmenter_break_sentence_v1!(Baked);
    impl_segmenter_dictionary_auto_v1!(Baked);
    impl_segmenter_break_grapheme_cluster_v1!(Baked);
    impl_segmenter_dictionary_extended_v1!(Baked);
    impl_segmenter_break_line_v1!(Baked);
    #[cfg(feature = "lstm")]
    impl_segmenter_lstm_auto_v1!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_unihan_radical_v1!(Baked);
    impl_segmenter_break_word_v1!(Baked);
    impl_segmenter_break_word_override_v1!(Baked);
    impl_segmenter_break_sentence_override_v1!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_line_v2!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_word_v2!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_sentence_v2!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_grapheme_cluster_v2!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_line_override_v2!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_sentence_override_v2!(Baked);
};

icu_provider::data_marker!(
    /// `SegmenterLstmWordLineAutoV1`
    SegmenterLstmAutoV1,
    "segmenter/lstm/auto/v1",
    LstmData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);
icu_provider::data_marker!(
    /// `SegmenterDictionaryWordAutoV1`
    SegmenterDictionaryAutoV1,
    "segmenter/dictionary/auto/v1",
    UCharDictionaryBreakData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);
icu_provider::data_marker!(
    /// `SegmenterDictionaryExtendedV1`
    SegmenterDictionaryExtendedV1,
    "segmenter/dictionary/extended/v1",
    UCharDictionaryBreakData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);
icu_provider::data_marker!(
    /// `SegmenterBreakSentenceOverrideV1`
    SegmenterBreakSentenceOverrideV1,
    "segmenter/break/sentence/override/v1",
    RuleBreakDataOverride<'static>,
);
icu_provider::data_marker!(
    /// `SegmenterBreakWordOverrideV1`
    SegmenterBreakWordOverrideV1,
    "segmenter/break/word/override/v1",
    RuleBreakDataOverride<'static>,
);
icu_provider::data_marker!(
    /// `SegmenterBreakLineV1`
    SegmenterBreakLineV1,
    "segmenter/break/line/v1",
    RuleBreakData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `SegmenterBreakWordV1`
    SegmenterBreakWordV1,
    "segmenter/break/word/v1",
    RuleBreakData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `SegmenterBreakGraphemeClusterV1`
    SegmenterBreakGraphemeClusterV1,
    "segmenter/break/grapheme/cluster/v1",
    RuleBreakData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `SegmenterBreakSentenceV1`
    SegmenterBreakSentenceV1,
    "segmenter/break/sentence/v1",
    RuleBreakData<'static>,
    is_singleton = true
);

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    SegmenterBreakGraphemeClusterV1::INFO,
    SegmenterBreakLineV1::INFO,
    SegmenterBreakSentenceOverrideV1::INFO,
    SegmenterBreakSentenceV1::INFO,
    SegmenterBreakWordOverrideV1::INFO,
    SegmenterBreakWordV1::INFO,
    SegmenterDictionaryAutoV1::INFO,
    SegmenterDictionaryExtendedV1::INFO,
    SegmenterLstmAutoV1::INFO,
    #[cfg(feature = "unstable")]
    radical::SegmenterUnihanRadicalV1::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakLineV2::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakWordV2::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakSentenceV2::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakGraphemeClusterV2::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakLineOverrideV2::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakSentenceOverrideV2::INFO,
];

/// Pre-processed Unicode data in the form of tables to be used for rule-based breaking.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RuleBreakData<'data> {
    /// Property table.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub property_table: CodePointTrie<'data, u8>,

    /// Break state table.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub break_state_table: ZeroVec<'data, BreakState>,

    /// State status table. Only used for word segmenter.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub rule_status_table: ZeroVec<'data, u8>,

    /// Number of properties; should be the square root of the length of [`Self::break_state_table`].
    pub property_count: u8,

    /// The index of the last simple state for [`Self::break_state_table`]. (A simple state has no
    /// `left` nor `right` in `SegmenterProperty`).
    pub last_codepoint_property: u8,

    /// The index of SOT (start of text) state for [`Self::break_state_table`].
    pub sot_property: u8,

    /// The index of EOT (end of text) state [`Self::break_state_table`].
    pub eot_property: u8,

    /// The index of "SA" state (or 127 if the complex language isn't handled) for
    /// [`Self::break_state_table`].
    pub complex_property: u8,
}

impl RuleBreakData<'_> {
    #[inline]
    pub(crate) fn get_break_state_from_table(&self, left: u8, right: u8) -> BreakState {
        let idx = (left as usize) * (self.property_count as usize) + (right as usize);
        // We use unwrap_or to fall back to the base case and prevent panics on bad data.
        self.break_state_table.get(idx).unwrap_or(BreakState::Keep)
    }
}

icu_provider::data_struct!(
    RuleBreakData<'_>,
    #[cfg(feature = "datagen")]
);

/// char16trie data for dictionary break
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct UCharDictionaryBreakData<'data> {
    /// Dictionary data of char16trie.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie_data: ZeroVec<'data, u16>,
}

icu_provider::data_struct!(
    UCharDictionaryBreakData<'_>,
    #[cfg(feature = "datagen")]
);

pub(crate) struct UCharDictionaryBreakDataV1;

impl DynamicDataMarker for UCharDictionaryBreakDataV1 {
    type DataStruct = UCharDictionaryBreakData<'static>;
}

/// codepoint trie data that the difference by specific locale
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RuleBreakDataOverride<'data> {
    /// The difference of property table for special locale.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub property_table_override: CodePointTrie<'data, u8>,
}

icu_provider::data_struct!(
    RuleBreakDataOverride<'_>,
    #[cfg(feature = "datagen")]
);

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
/// Break state
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub enum BreakState {
    /// Break
    Break,
    /// Keep rule
    Keep,
    /// Non-matching rule
    NoMatch,
    /// We have to look ahead one more character.
    Intermediate(u8),
    /// Index of a state.
    Index(u8),
}

#[cfg(feature = "datagen")]
impl serde::Serialize for BreakState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // would be nice to use the derive serde for JSON, but can't break serialization
        if serializer.is_human_readable() {
            i8::from_le_bytes([zerovec::ule::AsULE::to_unaligned(*self)]).serialize(serializer)
        } else {
            zerovec::ule::AsULE::to_unaligned(*self).serialize(serializer)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BreakState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Ok(zerovec::ule::AsULE::from_unaligned(
                i8::deserialize(deserializer)?.to_le_bytes()[0],
            ))
        } else {
            u8::deserialize(deserializer).map(zerovec::ule::AsULE::from_unaligned)
        }
    }
}

impl zerovec::ule::AsULE for BreakState {
    type ULE = u8;

    fn to_unaligned(self) -> Self::ULE {
        match self {
            BreakState::Break => 253,
            BreakState::Keep => 255,
            BreakState::NoMatch => 254,
            BreakState::Intermediate(i) => i + 120,
            BreakState::Index(i) => i,
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        match unaligned {
            253 => BreakState::Break,
            255 => BreakState::Keep,
            254 => BreakState::NoMatch,
            i if (120..253).contains(&i) => BreakState::Intermediate(i - 120),
            i => BreakState::Index(i),
        }
    }
}
