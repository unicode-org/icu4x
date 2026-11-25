// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The reordering algorithms in this file are adapted from ICU4C and,
// therefore, are subject to the ICU license as described in LICENSE.

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

use icu_collections::char16trie::Char16TrieCursor;
use icu_collections::codepointtrie::SmallCodePointTrie;
use icu_collections::codepointtrie::TypedCodePointTrie;
use icu_provider::prelude::*;
use zerovec::ZeroVec;
use zerovec::ule::AsULE;
use zerovec::{ZeroSlice, zeroslice};

use crate::elements::CollationElement;
use crate::elements::CollationElement32;
use crate::elements::Tag;

#[cfg(feature = "serde")]
use crate::elements::{
    COMMON_SEC_AND_TER_CE32, LONG_PRIMARY_FOR_REORDERABLE_CE32_LOW_BYTE, MAX_INDEX,
    SPECIAL_CE32_LOW_BYTE,
};

use crate::elements::EMPTY_U16;
use crate::elements::FFFD_CE;
use crate::elements::FFFD_CE_VALUE;
use crate::elements::FFFD_CE32;
use crate::elements::FFFD_CE32_VALUE;
use crate::elements::FFFD_PRIMARY;
use crate::elements::NO_CE_PRIMARY;
use crate::preferences::CollationCaseFirst;

use crate::options::MaxVariable;

// For `upgrade_v1_to_v2`, which is used by datagen and the serde
// constructors. (`datagen` implies `serde`.)
#[cfg(feature = "serde")]
use crate::elements::{HANGUL_S_BASE, HANGUL_S_COUNT};
#[cfg(feature = "serde")]
use alloc::vec::Vec;
#[cfg(feature = "serde")]
use icu_collections::codepointtrie::{CodePointTrieBuilder, TrieType};
#[cfg(feature = "serde")]
use icu_normalizer::properties::CanonicalCombiningClassMapBorrowed;

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
    use icu_collator_data::*;
    pub mod icu {
        pub use crate as collator;
        pub use icu_collections as collections;
    }
    make_provider!(Baked);
    impl_collation_root_v2!(Baked);
    impl_collation_tailoring_v2!(Baked);
    impl_collation_diacritics_v1!(Baked);
    impl_collation_jamo_v1!(Baked);
    impl_collation_metadata_v1!(Baked);
    impl_collation_special_primaries_v1!(Baked);
    impl_collation_reordering_v1!(Baked);
};

const SCRIPT_FALLBACK: icu_provider::fallback::LocaleFallbackConfig = {
    let mut c = icu_provider::fallback::LocaleFallbackConfig::default();
    c.priority = icu_provider::fallback::LocaleFallbackPriority::Script;
    c
};

icu_provider::data_marker!(
    /// Data marker for singleton root collation data.
    CollationRootV1,
    "collation/root/v1",
    CollationData<'static>,
    is_singleton = true,
);
icu_provider::data_marker!(
    /// Data marker for collation tailorings.
    CollationTailoringV1,
    "collation/tailoring/v1",
    CollationData<'static>,
    fallback_config = SCRIPT_FALLBACK,
    #[cfg(feature = "datagen")]
    attributes_domain = "collator",
);
icu_provider::data_marker!(
    /// Data marker for singleton root collation data, v2.
    ///
    /// Compared to v1, the struct is the same, but v2 has the additional
    /// invariant that characters that have a non-zero canonical combining
    /// class don't pass the quick primary check.
    CollationRootV2,
    "collation/root/v2",
    CollationData<'static>,
    is_singleton = true,
);
icu_provider::data_marker!(
    /// Data marker for collation tailorings, v2.
    ///
    /// Compared to v1, the struct is the same, but v2 has the additional
    /// invariant that characters that have a non-zero canonical combining
    /// class don't pass the quick primary check.
    CollationTailoringV2,
    "collation/tailoring/v2",
    CollationData<'static>,
    fallback_config = SCRIPT_FALLBACK,
    #[cfg(feature = "datagen")]
    attributes_domain = "collator",
);
icu_provider::data_marker!(
    /// Data marker for collation diacritics data.
    CollationDiacriticsV1,
    "collation/diacritics/v1",
    CollationDiacritics<'static>,
    fallback_config = SCRIPT_FALLBACK,
    #[cfg(feature = "datagen")]
    attributes_domain = "collator",
    #[cfg(feature = "datagen")]
    expose_baked_consts = true,
);
icu_provider::data_marker!(
    /// Data marker for collation jamo data.
    CollationJamoV1,
    "collation/jamo/v1",
    CollationJamo<'static>,
    is_singleton = true,
);
icu_provider::data_marker!(
    /// Data marker for collation reordering data.
    CollationReorderingV1,
    "collation/reordering/v1",
    CollationReordering<'static>,
    fallback_config = SCRIPT_FALLBACK,
    #[cfg(feature = "datagen")]
    attributes_domain = "collator",
);
icu_provider::data_marker!(
    /// Data marker for collation metadata.
    CollationMetadataV1,
    "collation/metadata/v1",
    CollationMetadata,
    fallback_config = SCRIPT_FALLBACK,
    #[cfg(feature = "datagen")]
    attributes_domain = "collator",
    #[cfg(feature = "datagen")]
    expose_baked_consts = true,
);
icu_provider::data_marker!(
    /// Data marker for collcation special primaries data.
    CollationSpecialPrimariesV1,
    "collation/special/primaries/v1",
    CollationSpecialPrimaries<'static>,
    is_singleton = true,
);

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    CollationRootV2::INFO,
    CollationTailoringV2::INFO,
    CollationDiacriticsV1::INFO,
    CollationJamoV1::INFO,
    CollationMetadataV1::INFO,
    CollationReorderingV1::INFO,
    CollationSpecialPrimariesV1::INFO,
];

/// `true` iff `ce32`, when interpreted as `CollationElement32`,
/// is self-contained.
///
/// For use by datagen only.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases.
/// </div>
#[cfg(feature = "datagen")]
#[doc(hidden)]
pub fn is_self_contained(ce32: u32) -> bool {
    CollationElement32::new(ce32)
        .to_ce_self_contained()
        .is_some()
}

const SINGLE_U32: &ZeroSlice<u32> =
    zeroslice!(u32; <u32 as AsULE>::ULE::from_unsigned; [FFFD_CE32_VALUE]);
const SINGLE_U64: &ZeroSlice<u64> =
    zeroslice!(u64; <u64 as AsULE>::ULE::from_unsigned; [FFFD_CE_VALUE]);

fn data_ce_to_primary(data_ce: u64, c: char) -> u32 {
    // Collation::getThreeBytePrimaryForOffsetData
    let p = (data_ce >> 32) as u32; // three-byte primary pppppp00
    let lower32 = data_ce as u32 as i32; // base code point b & step s: bbbbbbss (bit 7: isCompressible)
    let mut offset = ((u32::from(c) as i32) - (lower32 >> 8)) * (lower32 & 0x7F); // delta * increment
    let is_compressible = (lower32 & 0x80) != 0;
    // Collation::incThreeBytePrimaryByOffset
    offset += (((p >> 8) & 0xFF) as i32) - 2;
    let mut primary = (((offset % 254) + 2) as u32) << 8;
    offset /= 254;
    // Same with the second byte,
    // but reserve the PRIMARY_COMPRESSION_LOW_BYTE and high byte if necessary.
    if is_compressible {
        offset += (((p >> 16) & 0xFF) as i32) - 4;
        primary |= (((offset % 251) + 4) as u32) << 16;
        offset /= 251;
    } else {
        offset += (((p >> 16) & 0xFF) as i32) - 2;
        primary |= (((offset % 254) + 2) as u32) << 16;
        offset /= 254;
    }
    primary | ((p & 0xFF000000) + ((offset as u32) << 24))
}

/// The main collation data either for the root or for a tailoring
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationData<'data> {
    /// Mapping from `char` to `CollationElement32` (represented
    /// as its `u32` bits).
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: SmallCodePointTrie<'data, u32>,
    /// `CollationElement`s used in expansions and offset CE32s
    /// (represented as their `u64` bits)
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ces: ZeroVec<'data, u64>,
    /// `CollationElement32`s used in expansions and as defaults
    /// for digits when the numeric mode is not in use
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ce32s: ZeroVec<'data, u32>,
    /// Defaults and tries for prefix and contraction matching
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub contexts: ZeroVec<'data, u16>,
}

icu_provider::data_struct!(
    CollationData<'_>,
    #[cfg(feature = "datagen")]
);

impl<'data> CollationData<'data> {
    pub(crate) fn ce32_for_char(&self, c: char) -> CollationElement32 {
        CollationElement32::new(self.trie.get(c))
    }
    pub(crate) fn get_ce32(&'data self, index: usize) -> CollationElement32 {
        CollationElement32::new(if let Some(u) = self.ce32s.get(index) {
            u
        } else {
            // GIGO case
            debug_assert!(false);
            FFFD_CE32_VALUE
        })
    }
    pub(crate) fn get_ce32s(&'data self, index: usize, len: usize) -> &'data ZeroSlice<u32> {
        if len > 0
            && let Some(slice) = self.ce32s.get_subslice(index..index + len)
        {
            return slice;
        }
        // GIGO case
        debug_assert!(false);
        SINGLE_U32
    }
    pub(crate) fn get_ces(&'data self, index: usize, len: usize) -> &'data ZeroSlice<u64> {
        if len > 0
            && let Some(slice) = self.ces.get_subslice(index..index + len)
        {
            return slice;
        }
        // GIGO case
        debug_assert!(false);
        SINGLE_U64
    }
    pub(crate) fn get_primary_from_ces(&'data self, index: usize) -> u32 {
        if let Some(ce) = self.ces.get(index) {
            (ce >> 32) as u32
        } else {
            // GIGO case
            debug_assert!(false);
            FFFD_PRIMARY
        }
    }
    fn get_default_and_trie_impl(
        &'data self,
        index: usize,
    ) -> (CollationElement32, &'data ZeroSlice<u16>) {
        if let Some(slice) = self.contexts.get_subslice(index..self.contexts.len()) {
            #[expect(clippy::unwrap_used)]
            if slice.len() >= 2 {
                // `unwrap` must succeed due to the length check above.
                let first = slice.get(0).unwrap();
                let second = slice.get(1).unwrap();
                let trie = slice.get_subslice(2..slice.len()).unwrap();
                return (
                    CollationElement32::new((u32::from(first) << 16) | u32::from(second)),
                    trie,
                );
            }
        }
        // GIGO case
        debug_assert!(false);
        (FFFD_CE32, EMPTY_U16)
    }
    pub(crate) fn get_default_and_trie(
        &'data self,
        index: usize,
    ) -> (CollationElement32, Char16TrieCursor<'data>) {
        let (ce32, trie) = self.get_default_and_trie_impl(index);
        (ce32, Char16TrieCursor::new(trie))
    }
    pub(crate) fn get_default(&'data self, index: usize) -> CollationElement32 {
        let (ce32, _) = self.get_default_and_trie_impl(index);
        ce32
    }
    pub(crate) fn ce_from_offset_ce32(
        &self,
        c: char,
        ce32: CollationElement32,
    ) -> CollationElement {
        debug_assert!(ce32.tag() == Tag::Offset);
        if let Some(data_ce) = self.ces.get(ce32.index()) {
            CollationElement::new_from_primary(data_ce_to_primary(data_ce, c))
        } else {
            // GIGO case
            debug_assert!(false);
            FFFD_CE
        }
    }

    /// Upgrade v1 data to v2 data. The struct looks the same, but
    /// v2 has the additional invariant that characters that have
    /// a non-zero canonical combining class (according to the
    /// normalization data passed as an argument), are not allowed
    /// to return non-zero from the `to_primary_in_quick_check`
    /// method of their `CollationElement32` (with this struct
    /// as the argument to that method).
    #[cfg(feature = "serde")]
    pub fn upgrade_v1_to_v2(
        &self,
        ccc: &CanonicalCombiningClassMapBorrowed<'_>,
    ) -> Result<Self, DataError> {
        // Note: `self.trie.as_untyped_ref().null_value()` as `default_value` would
        // happen to work at the time of writing, but the builder mechanics don't
        // guarantee it. Therefore, read the trie value for U+10FFFF, a noncharacter
        // that is assumed never to be mapped explicitly by the data, so that it has
        // the value of unmapped code points both for root and tailorings.
        let default_value = self.trie.get('\u{10FFFF}');
        let mut augmented_ce32s = self.ce32s.to_vec();
        let mut builder = CodePointTrieBuilder::new(
            default_value,
            self.trie.as_untyped_ref().error_value(),
            TrieType::Small,
        );
        // We don't want non-default values for surrogates.
        for c in '\u{0}'..='\u{10FFFF}' {
            let cp = u32::from(c);
            // We don't want non-default values for Hangul syllables.
            if cp.wrapping_sub(HANGUL_S_BASE) < HANGUL_S_COUNT {
                continue;
            }
            let ce32 = self.trie.get(c);
            if ce32 == default_value {
                continue;
            }
            builder.set_value(
                cp,
                rewrite_reorderable(cp, ce32, &mut augmented_ce32s, ccc)?,
            );
        }
        Ok(Self {
            trie: SmallCodePointTrie::try_from(builder.build())
                .map_err(|_| DataError::custom("Built trie is not a small trie"))?,
            ces: self.ces.clone(),
            ce32s: ZeroVec::alloc_from_slice(&augmented_ce32s),
            contexts: self.contexts.clone(),
        })
    }
}

#[cfg(feature = "serde")]
fn rewrite_reorderable(
    c: u32,
    ce32: u32,
    ce32s: &mut Vec<u32>,
    ccc: &CanonicalCombiningClassMapBorrowed<'_>,
) -> Result<u32, DataError> {
    let ce32 = CollationElement32::new(ce32);
    let tag = ce32.tag_checked();
    if let Some(ref t) = tag {
        match t {
            Tag::LongPrimaryForReorderable | Tag::ExpansionForReorderable => {
                return Err(DataError::custom(
                    "Trying to upgrade already-upgraded collation data",
                ));
            }
            Tag::Reserved3 => {
                return Err(DataError::custom("Collation data contains Reserved3"));
            }
            Tag::BuilderData => {
                return Err(DataError::custom("Collation data contains BuilderData"));
            }
            Tag::LatinExpansion => {
                return Err(DataError::custom("Collation data contains LatinExpansion"));
            }
            Tag::Hangul => {
                return Err(DataError::custom("Collation data contains Hangul"));
            }
            _ => {}
        }
    }
    if ccc.get32_u8(c) == 0 {
        // Not reorderable.
        return Ok(ce32.bits());
    }
    if let Some(ref t) = tag {
        match t {
            Tag::LongPrimary => {
                // Replace the tag.
                return Ok((ce32.bits() & 0xFFFF_FFF0) | (Tag::LongPrimaryForReorderable as u32));
            }
            Tag::Expansion => {
                // Replace the tag.
                return Ok((ce32.bits() & 0xFFFF_FFF0) | (Tag::ExpansionForReorderable as u32));
            }
            Tag::Digit => {
                return Err(DataError::custom(
                    "Digit ce32 not allowed for reorderable characters.",
                ));
            }
            _ => {
                // The primary quick-check does not examine this ce32 type.
                return Ok(ce32.bits());
            }
        }
    }
    // Simple
    if (ce32.bits() & 0xFFFF_0000) == 0 {
        // No primary. We don't need to hide this, because the primary quick-check
        // compares primaries.
        return Ok(ce32.bits());
    }
    if ce32.bits() as u16 == COMMON_SEC_AND_TER_CE32 {
        // `LongPrimaryForReorderable` implies common
        // secondary and tertiary weights, so we can rewrite this.
        return Ok(
            (ce32.bits() & 0xFFFF_0000) | (LONG_PRIMARY_FOR_REORDERABLE_CE32_LOW_BYTE as u32)
        );
    }
    // There is no tag to swap. The low byte needs to be preserved, so we
    // can't add a tag. We need to hide this ce32 behind indirection.
    // As of Unicode 17, there should be 34 of these across the collations,
    // so the data size increase is acceptable.
    // Notably, combining Latin and Cyrillic letters above are here.
    // Characters that have an `Expansion32`-typed ce32 to begin with
    // tend to be compatibility characters and symbols, so there isn't
    // performance pressure to handle `Expansion32` in the quick primary
    // check.
    let len = 1; // We store one item.
    let index = ce32s.len();
    if index > MAX_INDEX {
        return Err(DataError::custom(
            "Index too large to fit in an Expansion32",
        ));
    }
    ce32s.push(ce32.bits());
    // See `makeCE32FromTagIndexAndLength` in ICU4C.
    Ok(((index as u32) << 13)
        | (len << 8)
        | u32::from(SPECIAL_CE32_LOW_BYTE)
        | (Tag::Expansion32 as u32))
}

/// Secondary weights for the start of the Combining Diacritics block.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationDiacritics<'data> {
    /// Secondary weights for characters starting from U+0300 up
    /// to but not including U+034F. May be shorter than that;
    /// zero-length when a tailoring opts out of using this
    /// feature altogether.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub secondaries: ZeroVec<'data, u16>,
}

icu_provider::data_struct!(
    CollationDiacritics<'_>,
    #[cfg(feature = "datagen")]
);

/// `CollationElement32`s for the Hangul Jamo Unicode Block
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
pub struct CollationJamo<'data> {
    /// `CollationElement32`s (as `u32`s) for the Hangul Jamo Unicode Block.
    /// The length must be equal to the size of the block (256).
    pub ce32s: ZeroVec<'data, u32>,
}

impl<'data> CollationJamo<'data> {
    pub(crate) fn as_array(
        &'data self,
    ) -> &'data [<u32 as AsULE>::ULE; crate::elements::JAMO_COUNT] {
        #[allow(clippy::unwrap_used)] // by invariant
        self.ce32s.as_ule_slice().try_into().unwrap()
    }
}

// TODO: redesign Korean search collation handling

#[cfg(feature = "compiled_data")]
const _: () = assert!(
    Baked::SINGLETON_COLLATION_JAMO_V1.ce32s.as_slice().len() == crate::elements::JAMO_COUNT
);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CollationJamo<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Raw<'data> {
            #[cfg_attr(feature = "serde", serde(borrow))]
            ce32s: ZeroVec<'data, u32>,
        }

        let Raw { ce32s } = Raw::deserialize(deserializer)?;

        if ce32s.len() != crate::elements::JAMO_COUNT {
            return Err(serde::de::Error::custom("invalid"));
        }

        Ok(Self { ce32s })
    }
}

icu_provider::data_struct!(
    CollationJamo<'_>,
    #[cfg(feature = "datagen")]
);

/// Script reordering data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationReordering<'data> {
    /// Limit of last reordered range. 0 if no reordering or no split bytes.
    ///
    /// Comment from ICU4C's `collationsettings.h`
    pub min_high_no_reorder: u32,
    /// 256-byte table for reordering permutation of primary lead
    /// bytes; NULL if no reordering. A 0 entry at a non-zero index means
    /// that the primary lead byte is "split" (there are different offsets
    /// for primaries that share that lead byte) and the reordering offset
    /// must be determined via the reorderRanges.
    ///
    /// Comment from ICU4C's `collationsettings.h`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub reorder_table: ZeroVec<'data, u8>, // len always 256
    /// Primary-weight ranges for script reordering, to be used by
    /// reorder(p) for split-reordered primary lead bytes.
    ///
    /// Each entry is a (limit, offset) pair. The upper 16 bits of the
    /// entry are the upper 16 bits of the exclusive primary limit of
    /// a range. Primaries between the previous limit and this one have
    /// their lead bytes modified by the signed offset (-0xff..+0xff)
    /// stored in the lower 16 bits.
    ///
    /// `CollationData::makeReorderRanges()` writes a full list where the
    /// first range (at least for terminators and separators) has a 0
    /// offset. The last range has a non-zero offset. minHighNoReorder
    /// is set to the limit of that last range.
    ///
    /// In the settings object, the initial ranges before the first
    /// split lead byte are omitted for efficiency; they are handled
    /// by reorder(p) via the reorderTable. If there are no
    /// split-reordered lead bytes, then no ranges are needed.
    ///
    /// Comment from ICU4C's `collationsettings.h`; names refer to
    /// ICU4C.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub reorder_ranges: ZeroVec<'data, u32>,
}

icu_provider::data_struct!(
    CollationReordering<'_>,
    #[cfg(feature = "datagen")]
);

impl CollationReordering<'_> {
    pub(crate) fn reorder(&self, primary: u32) -> u32 {
        if let Some(b) = self.reorder_table.get((primary >> 24) as usize) {
            if b != 0 || primary <= NO_CE_PRIMARY {
                (u32::from(b) << 24) | (primary & 0x00FFFFFF)
            } else {
                self.reorder_ex(primary)
            }
        } else {
            // GIGO case
            debug_assert!(false);
            primary
        }
    }

    fn reorder_ex(&self, primary: u32) -> u32 {
        if primary >= self.min_high_no_reorder {
            return primary;
        }
        let q = primary | 0xFFFF;
        for &range in self.reorder_ranges.as_ule_slice().iter() {
            let r = u32::from_unaligned(range);
            if q < r {
                return primary.wrapping_add(r << 24);
            }
        }
        // GIGO case
        debug_assert!(false);
        primary
    }
}

/// Each non-alias collation that the data provider knows
/// about explicitly has an data entry at least for this
/// struct.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationMetadata {
    /// See the mask constants in the `impl` block for the
    /// bit layout. The other bits are ignored: They could
    /// be from the future if their semantics such that
    /// old code may ignore them.
    ///
    /// Note: At present, it's bogus for the bit for "upper
    /// first" to be set if "case first" isn't also set.
    /// However, the methods handle this case gracefully,
    /// so there is no need for invariant validation.
    pub bits: u32,
}

icu_provider::data_struct!(
    CollationMetadata,
    #[cfg(feature = "datagen")]
);

impl CollationMetadata {
    const MAX_VARIABLE_MASK: u32 = 0b11;
    const TAILORED_MASK: u32 = 1 << 3;
    const TAILORED_DIACRITICS_MASK: u32 = 1 << 4;
    const REORDERING_MASK: u32 = 1 << 5;
    const LITHUANIAN_DOT_ABOVE_MASK: u32 = 1 << 6;
    const BACWARD_SECOND_LEVEL_MASK: u32 = 1 << 7;
    const ALTERNATE_SHIFTED_MASK: u32 = 1 << 8;
    const CASE_FIRST_MASK: u32 = 1 << 9;
    const UPPER_FIRST_MASK: u32 = 1 << 10;

    #[inline(always)]
    pub(crate) const fn max_variable(self) -> MaxVariable {
        // Safety: the possible numeric values for `MaxVariable` are from 0 to 3, inclusive,
        // and it is repr(u8). MAX_VARIABLE_MASK here ensures our values have most 2 bits, which produces
        // the same range.
        unsafe {
            core::mem::transmute::<u8, MaxVariable>(
                (self.bits & CollationMetadata::MAX_VARIABLE_MASK) as u8,
            )
        }
    }

    #[inline(always)]
    pub(crate) const fn tailored(self) -> bool {
        self.bits & CollationMetadata::TAILORED_MASK != 0
    }

    /// Vietnamese and Ewe
    #[inline(always)]
    pub(crate) const fn tailored_diacritics(self) -> bool {
        self.bits & CollationMetadata::TAILORED_DIACRITICS_MASK != 0
    }

    /// Lithuanian
    #[inline(always)]
    pub(crate) const fn lithuanian_dot_above(self) -> bool {
        self.bits & CollationMetadata::LITHUANIAN_DOT_ABOVE_MASK != 0
    }

    /// Canadian French
    #[inline(always)]
    pub(crate) const fn backward_second_level(self) -> bool {
        self.bits & CollationMetadata::BACWARD_SECOND_LEVEL_MASK != 0
    }

    #[inline(always)]
    pub(crate) const fn reordering(self) -> bool {
        self.bits & CollationMetadata::REORDERING_MASK != 0
    }

    /// Thai
    #[inline(always)]
    pub(crate) const fn alternate_shifted(self) -> bool {
        self.bits & CollationMetadata::ALTERNATE_SHIFTED_MASK != 0
    }

    #[inline(always)]
    pub(crate) const fn case_first(self) -> CollationCaseFirst {
        if self.bits & CollationMetadata::CASE_FIRST_MASK != 0 {
            if self.bits & CollationMetadata::UPPER_FIRST_MASK != 0 {
                CollationCaseFirst::Upper
            } else {
                CollationCaseFirst::Lower
            }
        } else {
            CollationCaseFirst::False
        }
    }
}

/// Root-associated additional data that doesn't change in tailorings
///
/// These are the fields that logically belong to the root data but
/// don't belong to the tailoring data and that are on this separate
/// struct, since we have the same struct for a tailoring and the
/// bulk of the root.
///
/// As a practical matter, this struct happens to only carry
/// information about what concrete numeric values for primary
/// weights are special in particular ways. In principle, when the
/// root data is built, the root builder is allowed to assign the
/// numeric values as it sees fit, which is why these aren't
/// hard-coded.
///
/// Note: In 2.0.0 and prior, this struct was loaded only if
/// it was known at collator construction time (based on options)
/// that the data here was going to be needed. With the introduction
/// of collation keys and the decision not to introduce a collator
/// key generator object separate from the collator, this struct
/// is now always loaded.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_collator::provider))]
pub struct CollationSpecialPrimaries<'data> {
    /// The primaries corresponding to `MaxVariable`
    /// character classes packed so that each fits in
    /// 16 bits. Length must match the number of enum
    /// variants in `MaxVariable`, currently 4.
    pub last_primaries: ZeroVec<'data, u16>,
    /// The high 8 bits of the numeric primary
    pub numeric_primary: u8,
    /// 256 bits (packed in 16 u16s) to classify every possible
    /// byte into compressible or non-compressible.
    ///
    /// In the serde encoding, this is appended to `last_primaries`,
    /// or might be missing.
    pub compressible_bytes: ZeroVec<'data, u16>,
}

#[cfg(feature = "serde")]
impl CollationSpecialPrimaries<'_> {
    /// The length of the compressible bytes array (256 bits packed in `u16`s).
    pub(crate) const COMPRESSIBLE_BYTES_LEN: usize = 256 / (u16::BITS as usize);
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CollationSpecialPrimaries<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Raw<'data> {
            #[cfg_attr(feature = "serde", serde(borrow))]
            last_primaries: ZeroVec<'data, u16>,
            numeric_primary: u8,
        }

        let Raw {
            last_primaries: concatenated,
            numeric_primary,
        } = Raw::deserialize(deserializer)?;

        use alloc::borrow::Cow;
        let (last_primaries, mut compressible_bytes) = match concatenated.into_cow() {
            Cow::Borrowed(s) => {
                let Some((l, c)) = s.split_at_checked(MaxVariable::VARIANT_COUNT) else {
                    return Err(serde::de::Error::custom("invalid"));
                };
                (
                    ZeroSlice::from_ule_slice(l).as_zerovec(),
                    ZeroSlice::from_ule_slice(c).as_zerovec(),
                )
            }
            Cow::Owned(v) => {
                let Some((l, c)) = v.split_at_checked(MaxVariable::VARIANT_COUNT) else {
                    return Err(serde::de::Error::custom("invalid"));
                };
                (
                    ZeroSlice::from_ule_slice(l).as_zerovec().into_owned(),
                    ZeroSlice::from_ule_slice(c).as_zerovec().into_owned(),
                )
            }
        };

        if compressible_bytes.len() != CollationSpecialPrimaries::COMPRESSIBLE_BYTES_LEN {
            compressible_bytes = zerovec::zerovec!(
                u16; <u16 as AsULE>::ULE::from_unsigned; [
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b1111_1111_1111_1110,
                0b1111_1111_1111_1111,
                0b0000_0000_0000_0001,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0000_0000_0000_0000,
                0b0100_0000_0000_0000,
            ]);
        }

        Ok(Self {
            last_primaries,
            numeric_primary,
            compressible_bytes,
        })
    }
}

#[cfg(feature = "datagen")]
impl serde::Serialize for CollationSpecialPrimaries<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct Raw {
            #[serde(rename = "last_primaries")]
            concatenated: ZeroVec<'static, u16>,
            numeric_primary: u8,
        }

        Raw {
            concatenated: self
                .last_primaries
                .iter()
                .chain(self.compressible_bytes.iter())
                .collect(),
            numeric_primary: self.numeric_primary,
        }
        .serialize(serializer)
    }
}

icu_provider::data_struct!(
    CollationSpecialPrimaries<'_>,
    #[cfg(feature = "datagen")]
);

impl CollationSpecialPrimaries<'_> {
    #[expect(clippy::unwrap_used)]
    pub(crate) fn last_primary_for_group(&self, max_variable: MaxVariable) -> u32 {
        // `unwrap` is OK, because `Collator::try_new` validates the length.
        //
        // Minus one to generate the right lower 16 bits from the high 16 bits.
        // See parse.cpp in genrb and getLastPrimaryForGroup in ICU4C.
        (u32::from(self.last_primaries.get(max_variable as usize).unwrap()) << 16) - 1
    }

    #[allow(dead_code)]
    pub(crate) fn is_compressible(&self, b: u8) -> bool {
        let field = self
            .compressible_bytes
            .get(usize::from(b >> 4))
            .unwrap_or_default();
        let mask = 1 << (b & 0b1111);
        (field & mask) != 0
    }
}

#[cfg(all(test, feature = "datagen", feature = "compiled_data"))]
#[test]
fn test_special_primaries_serde_roundtrip() {
    let payload = DataProvider::<CollationSpecialPrimariesV1>::load(&Baked, Default::default())
        .unwrap()
        .payload;
    let json = serde_json::to_string(payload.get()).unwrap();
    let deserialized: CollationSpecialPrimaries<'_> = serde_json::from_str(&json).unwrap();
    assert_eq!(payload.get(), &deserialized);
}
