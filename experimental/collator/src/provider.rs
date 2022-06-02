// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The reordering algorithms in this file are adapted from ICU4C and,
// therefore, are subject to the ICU license as described in LICENSE.

use icu_char16trie::char16trie::Char16TrieIterator;
use icu_codepointtrie::CodePointTrie;
use icu_provider::{yoke, zerofrom};
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;
use zerovec::ZeroVec;

use crate::elements::CollationElement;
use crate::elements::CollationElement32;
use crate::elements::Tag;
use crate::elements::EMPTY_U16;
use crate::elements::FFFD_CE;
use crate::elements::FFFD_CE32;
use crate::elements::FFFD_CE32_VALUE;
use crate::elements::FFFD_CE_VALUE;
use crate::elements::NO_CE_PRIMARY;

use super::CaseFirst;
use super::MaxVariable;

const SINGLE_U32: &ZeroSlice<u32> =
    ZeroSlice::<u32>::from_ule_slice(&<u32 as AsULE>::ULE::from_array([FFFD_CE32_VALUE]));
const SINGLE_U64: &ZeroSlice<u64> =
    ZeroSlice::<u64>::from_ule_slice(&<u64 as AsULE>::ULE::from_array([FFFD_CE_VALUE]));

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

#[icu_provider::data_struct(CollationDataV1Marker = "collator/data@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationDataV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, u32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ces: ZeroVec<'data, u64>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ce32s: ZeroVec<'data, u32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub contexts: ZeroVec<'data, u16>,
}

impl<'data> CollationDataV1<'data> {
    pub(crate) fn ce32_for_char(&self, c: char) -> CollationElement32 {
        CollationElement32::new(self.trie.get(c as u32))
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
        if len > 0 {
            if let Some(slice) = self.ce32s.get_subslice(index..index + len) {
                return slice;
            }
        }
        // GIGO case
        debug_assert!(false);
        SINGLE_U32
    }
    pub(crate) fn get_ces(&'data self, index: usize, len: usize) -> &'data ZeroSlice<u64> {
        if len > 0 {
            if let Some(slice) = self.ces.get_subslice(index..index + len) {
                return slice;
            }
        }
        // GIGO case
        debug_assert!(false);
        SINGLE_U64
    }
    fn get_default_and_trie_impl(
        &'data self,
        index: usize,
    ) -> (CollationElement32, &'data ZeroSlice<u16>) {
        if let Some(slice) = self.contexts.get_subslice(index..self.contexts.len()) {
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
    ) -> (CollationElement32, Char16TrieIterator<'data>) {
        let (ce32, trie) = self.get_default_and_trie_impl(index);
        (ce32, Char16TrieIterator::new(trie.as_ule_slice()))
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
}

#[icu_provider::data_struct(CollationDiacriticsV1Marker = "collator/dia@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationDiacriticsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub secondaries: ZeroVec<'data, u16>,
}

#[icu_provider::data_struct(CollationJamoV1Marker = "collator/jamo@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationJamoV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ce32s: ZeroVec<'data, u32>,
}

#[icu_provider::data_struct(CollationReorderingV1Marker = "collator/reord@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationReorderingV1<'data> {
    pub min_high_no_reorder: u32,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub reorder_table: ZeroVec<'data, u8>, // len always 256
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub reorder_ranges: ZeroVec<'data, u32>,
}

impl<'data> CollationReorderingV1<'data> {
    pub fn reorder(&self, primary: u32) -> u32 {
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

#[icu_provider::data_struct(CollationMetadataV1Marker = "collator/meta@1")]
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationMetadataV1 {
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

impl CollationMetadataV1 {
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
    pub(crate) fn max_variable(&self) -> MaxVariable {
        // Safe, because the possible numeric values for `MaxVariable` are from 0 to 3, inclusive,
        // and we take the two low bits.
        unsafe { core::mem::transmute((self.bits & CollationMetadataV1::MAX_VARIABLE_MASK) as u8) }
    }

    #[inline(always)]
    pub(crate) fn tailored(&self) -> bool {
        self.bits & CollationMetadataV1::TAILORED_MASK != 0
    }

    /// Vietnamese and Ewe
    #[inline(always)]
    pub(crate) fn tailored_diacritics(&self) -> bool {
        self.bits & CollationMetadataV1::TAILORED_DIACRITICS_MASK != 0
    }

    /// Lithuanian
    #[inline(always)]
    pub(crate) fn lithuanian_dot_above(&self) -> bool {
        self.bits & CollationMetadataV1::LITHUANIAN_DOT_ABOVE_MASK != 0
    }

    /// Canadian French
    #[inline(always)]
    pub(crate) fn backward_second_level(&self) -> bool {
        self.bits & CollationMetadataV1::BACWARD_SECOND_LEVEL_MASK != 0
    }

    #[inline(always)]
    pub(crate) fn reordering(&self) -> bool {
        self.bits & CollationMetadataV1::REORDERING_MASK != 0
    }

    /// Thai
    #[inline(always)]
    pub(crate) fn alternate_shifted(&self) -> bool {
        self.bits & CollationMetadataV1::ALTERNATE_SHIFTED_MASK != 0
    }

    #[inline(always)]
    pub(crate) fn case_first(&self) -> CaseFirst {
        if self.bits & CollationMetadataV1::CASE_FIRST_MASK != 0 {
            if self.bits & CollationMetadataV1::UPPER_FIRST_MASK != 0 {
                CaseFirst::UpperFirst
            } else {
                CaseFirst::LowerFirst
            }
        } else {
            CaseFirst::Off
        }
    }
}

#[icu_provider::data_struct(CollationSpecialPrimariesV1Marker = "collator/prim@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, crabbake::Bakeable), crabbake(path = icu_collator::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationSpecialPrimariesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub last_primaries: ZeroVec<'data, u16>,
    pub numeric_primary: u8,
}

impl<'data> CollationSpecialPrimariesV1<'data> {
    pub(crate) fn last_primary_for_group(&self, max_variable: MaxVariable) -> u32 {
        // `unwrap` is OK, because `Collator::try_new` validates the length.
        //
        // Minus one to generate the right lower 16 bits from the high 16 bits.
        // See parse.cpp in genrb and getLastPrimaryForGroup in ICU4C.
        (u32::from(self.last_primaries.get(max_variable as usize).unwrap()) << 16) - 1
    }
}
