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

mod v1;
pub use v1::*;
#[cfg(feature = "unstable")]
#[allow(missing_docs)]
mod v2;
#[cfg(feature = "unstable")]
pub use v2::*;
mod complex;
pub use complex::*;

#[cfg(feature = "datagen")]
use icu_provider::prelude::*;

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
#[allow(unused_imports, missing_docs)]
const _: () = {
    use icu_segmenter_data::*;
    pub mod icu {
        pub use crate as segmenter;
        pub use icu_collections as collections;
    }
    make_provider!(Baked);
    impl_segmenter_break_sentence_v1!(Baked);
    impl_segmenter_dictionary_auto_v1!(Baked);
    impl_segmenter_break_grapheme_cluster_v1!(Baked);
    impl_segmenter_dictionary_extended_v1!(Baked);
    impl_segmenter_break_line_v1!(Baked);
    #[cfg(feature = "unstable")]
    impl_segmenter_break_line_v3!(Baked);
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

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    SegmenterBreakGraphemeClusterV1::INFO,
    SegmenterBreakLineV1::INFO,
    #[cfg(feature = "unstable")]
    SegmenterBreakLineV3::INFO,
    SegmenterBreakSentenceOverrideV1::INFO,
    SegmenterBreakSentenceV1::INFO,
    SegmenterBreakWordOverrideV1::INFO,
    SegmenterBreakWordV1::INFO,
    SegmenterDictionaryAutoV1::INFO,
    SegmenterDictionaryExtendedV1::INFO,
    SegmenterLstmAutoV1::INFO,
    #[cfg(feature = "unstable")]
    SegmenterUnihanRadicalV1::INFO,
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

/// A complex script that requires special handling in the segmenter.
#[allow(missing_docs)] // trivial
#[zerovec::make_ule(ComplexScriptULE)]
#[derive(PartialEq, Debug, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
#[repr(u8)]
pub enum ComplexScript {
    None = 0,
    Myanmar = 1,
    ChineseOrJapanese = 2,
    Khmer = 3,
    Lao = 4,
    Thai = 5,
}
