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

use icu_collections::char16trie::Char16Trie;
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
    use icu_normalizer_data::*;
    pub mod icu {
        pub use crate as normalizer;
        pub use icu_collections as collections;
    }
    make_provider!(Baked);
    impl_normalizer_nfc_v1!(Baked);
    impl_normalizer_nfd_data_v1!(Baked);
    impl_normalizer_nfd_supplement_v1!(Baked);
    impl_normalizer_nfd_tables_v1!(Baked);
    impl_normalizer_nfkd_data_v1!(Baked);
    impl_normalizer_nfkd_tables_v1!(Baked);
    impl_normalizer_uts46_data_v1!(Baked);
};

icu_provider::data_marker!(
    /// Marker for data for canonical decomposition.
    NormalizerNfdDataV1,
    "normalizer/nfd/data/v1",
    DecompositionData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for additional data for canonical decomposition.
    NormalizerNfdTablesV1,
    "normalizer/nfd/tables/v1",
    DecompositionTables<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for data for compatibility decomposition.
    NormalizerNfkdDataV1,
    "normalizer/nfkd/data/v1",
    DecompositionData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for additional data for compatibility decomposition.
    NormalizerNfkdTablesV1,
    "normalizer/nfkd/tables/v1",
    DecompositionTables<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for data for UTS-46 decomposition.
    NormalizerUts46DataV1,
    "normalizer/uts46/data/v1",
    DecompositionData<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for data for composition.
    NormalizerNfcV1,
    "normalizer/nfc/v1",
    CanonicalCompositions<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// Marker for additional data for non-recusrsive composition.
    NormalizerNfdSupplementV1,
    "normalizer/nfd/supplement/v1",
    NonRecursiveDecompositionSupplement<'static>,
    is_singleton = true
);

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    NormalizerNfcV1::INFO,
    NormalizerNfdDataV1::INFO,
    NormalizerNfdTablesV1::INFO,
    NormalizerNfkdDataV1::INFO,
    NormalizerNfkdTablesV1::INFO,
    NormalizerNfdSupplementV1::INFO,
    NormalizerUts46DataV1::INFO,
];

/// Decomposition data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionData<'data> {
    /// Trie for decomposition.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    /// The passthrough bounds of NFD/NFC are lowered to this
    /// maximum instead. (16-bit, because cannot be higher
    /// than 0x0300, which is the bound for NFC.)
    pub passthrough_cap: u16,
}

icu_provider::data_struct!(
    DecompositionData<'_>,
    #[cfg(feature = "datagen")]
);

/// Constants describing the trie value format of [`DecompositionData`].
///
/// See `trie-value-format.md` in the `icu_normalizer` crate for the full description.
impl DecompositionData<'_> {
    /// Marker for UTS 46 ignorables. Not supported in NFD or NFKD data.
    pub const IGNORABLE_MARKER: u32 = 0xFFFFFFFF;

    /// Marker that the decomposition does not round trip via NFC.
    pub const NON_ROUND_TRIP_MARKER: u32 = 1 << 30;

    /// Marker that the first character of the decomposition can combine backwards.
    pub const BACKWARD_COMBINING_MARKER: u32 = 1 << 31;

    /// Mask for the bits that have to be zero for the trie value to be a BMP singleton
    /// decomposition, or a value baked into the surrogate range.
    pub const HIGH_ZEROS_MASK: u32 = 0x3FFF0000;

    /// Mask for the bits that have to be zero for the trie value to be a complex
    /// decomposition.
    pub const LOW_ZEROS_MASK: u32 = 0xFFE0;

    /// Marker (in the second-least-significant byte) for non-starters that decompose to
    /// themselves. The least-significant byte is the canonical combining class.
    pub const NON_STARTER_MARKER: u32 = 0xD800;

    /// Marker (in the second-least-significant byte) for non-starters whose decomposition
    /// starts with a non-starter. The least-significant byte is the canonical combining
    /// class of the undecomposed character. The decompositions are hard-coded, see
    /// [`Self::SPECIAL_NON_STARTER_DECOMPOSITIONS`].
    pub const SPECIAL_NON_STARTER_MARKER: u32 = 0xD900;

    /// COMBINING GREEK YPOGEGRAMMENI, a non-starter that UTS 46 maps to a starter.
    //
    /// Like [`Self::HALF_WIDTH_VOICED_SOUND_MARK`] and
    /// [`Self::HALF_WIDTH_SEMI_VOICED_SOUND_MARK`], this character must not occur in the
    /// tail of a complex decomposition in UTS 46 data, because whether a tail character is
    /// a non-starter is determined from its trie value, which differs between flavors.
    pub const IOTA_SUBSCRIPT: char = '\u{0345}';

    /// HALFWIDTH KATAKANA VOICED SOUND MARK, a starter that decomposes to a non-starter in
    /// NFKD and UTS 46. Must not occur in the tail of a complex decomposition.
    pub const HALF_WIDTH_VOICED_SOUND_MARK: char = '\u{FF9E}';

    /// HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK, a starter that decomposes to a non-starter
    /// in NFKD and UTS 46. Must not occur in the tail of a complex decomposition.
    pub const HALF_WIDTH_SEMI_VOICED_SOUND_MARK: char = '\u{FF9F}';

    /// The characters whose trie values use [`Self::SPECIAL_NON_STARTER_MARKER`], with
    /// their hard-coded decompositions.
    pub const SPECIAL_NON_STARTER_DECOMPOSITIONS: &'static [(char, &'static [char])] = &[
        // COMBINING GRAVE TONE MARK
        ('\u{0340}', &['\u{0300}']),
        // COMBINING ACUTE TONE MARK
        ('\u{0341}', &['\u{0301}']),
        // COMBINING GREEK KORONIS
        ('\u{0343}', &['\u{0313}']),
        // COMBINING GREEK DIALYTIKA TONOS
        ('\u{0344}', &['\u{0308}', '\u{0301}']),
        // TIBETAN VOWEL SIGN II
        ('\u{0F73}', &['\u{0F71}', '\u{0F72}']),
        // TIBETAN VOWEL SIGN UU
        ('\u{0F75}', &['\u{0F71}', '\u{0F74}']),
        // TIBETAN VOWEL SIGN REVERSED II
        ('\u{0F81}', &['\u{0F71}', '\u{0F80}']),
        // NFKD and UTS 46 only
        (Self::HALF_WIDTH_VOICED_SOUND_MARK, &['\u{3099}']),
        // NFKD and UTS 46 only
        (Self::HALF_WIDTH_SEMI_VOICED_SOUND_MARK, &['\u{309A}']),
    ];

    /// Trie value of Hangul syllables, which are decomposed algorithmically.
    pub const HANGUL_SYLLABLE_MARKER: u32 = 1;

    /// Low half of the trie value for U+FDFA in NFKD, whose decomposition is too long to
    /// be stored in the tables. Deliberately unified with
    /// [`Self::HANGUL_SYLLABLE_MARKER`], but differs from it by
    /// [`Self::NON_ROUND_TRIP_MARKER`].
    pub const FDFA_MARKER: u16 = 1;

    /// The tail (everything after the first character) of the NFKD form of U+FDFA as
    /// 16-bit units.
    pub const FDFA_NFKD: [u16; 17] = [
        0x644, 0x649, 0x20, 0x627, 0x644, 0x644, 0x647, 0x20, 0x639, 0x644, 0x64A, 0x647, 0x20,
        0x648, 0x633, 0x644, 0x645,
    ];

    /// ANGSTROM SIGN, which never uses the inline two-character form, and whose
    /// non-recursive decomposition is hard-coded.
    pub const ANGSTROM_SIGN: char = '\u{212B}';

    /// Shift of the trailing non-starter in the inline two-character form.
    pub const PAIR_TRAIL_SHIFT: u32 = 15;

    /// Mask of a character in the inline two-character form.
    pub const PAIR_CHAR_MASK: u32 = 0x7FFF;

    /// Mask of the length bits of a complex decomposition.
    pub const COMPLEX_LENGTH_MASK: u32 = 0b1111;

    /// Marker that all trailing characters of a complex decomposition are non-starters.
    pub const COMPLEX_ONLY_NON_STARTERS_IN_TRAIL_MARKER: u32 = 0b10000;

    /// Shift of the (one-based) offset of a complex decomposition.
    pub const COMPLEX_OFFSET_SHIFT: u32 = 16;
}

/// The expansion tables for cases where the decomposition isn't
/// contained in the trie value
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecompositionTables<'data> {
    /// Decompositions that are fully within the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars16: ZeroVec<'data, u16>,
    /// Decompositions with at least one character outside
    /// the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars24: ZeroVec<'data, char>,
}

icu_provider::data_struct!(
    DecompositionTables<'_>,
    #[cfg(feature = "datagen")]
);

impl DecompositionTables<'_> {
    /// The maximum combined length of the NFD and supplementary (NFKD) tables, as
    /// limited by the offset bits of complex decompositions in [`DecompositionData`].
    pub const MAX_COMBINED_LEN: usize = 0xFFF;
}

/// Non-Hangul canonical compositions
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CanonicalCompositions<'data> {
    /// Trie keys are two-`char` strings with the second
    /// character coming first. The value, if any, is the
    /// (non-Hangul) canonical composition.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub canonical_compositions: Char16Trie<'data>,
}

icu_provider::data_struct!(
    CanonicalCompositions<'_>,
    #[cfg(feature = "datagen")]
);

/// Non-recursive canonical decompositions that differ from
/// `DecompositionData`.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_normalizer::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct NonRecursiveDecompositionSupplement<'data> {
    /// Trie for the supplementary non-recursive decompositions
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    /// Decompositions with at least one character outside
    /// the BMP
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub scalars24: ZeroVec<'data, char>,
}

icu_provider::data_struct!(
    NonRecursiveDecompositionSupplement<'_>,
    #[cfg(feature = "datagen")]
);

impl NonRecursiveDecompositionSupplement<'_> {
    /// Characters with oxia, which are always included in the supplement in order to
    /// catch singleton decompositions to the corresponding characters with tonos.
    pub const OXIA_RANGE: core::ops::RangeInclusive<char> = '\u{1F71}'..='\u{1FFB}';
}
