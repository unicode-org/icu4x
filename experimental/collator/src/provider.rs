// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The reordering algorithms in this file are adapted from ICU4C and,
// therefore, are subject to the ICU license as described in LICENSE.

use icu_char16trie::char16trie::Char16TrieIterator;
use icu_codepointtrie::CodePointTrie;
use icu_provider::{yoke, zerofrom};
use zerovec::ule::AsULE;
use zerovec::ule::RawBytesULE;
use zerovec::ZeroVec;

use crate::elements::CollationElement;
use crate::elements::CollationElement32;
use crate::elements::Tag;
use crate::elements::NO_CE_PRIMARY;

use super::CaseFirst;
use super::MaxVariable;

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
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
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
    pub(crate) fn get_ce32s(&'data self, index: usize, len: usize) -> &'data [RawBytesULE<4>] {
        &self.ce32s.as_ule_slice()[index..index + len]
    }
    pub(crate) fn get_ces(&'data self, index: usize, len: usize) -> &'data [RawBytesULE<8>] {
        &self.ces.as_ule_slice()[index..index + len]
    }
    fn get_default_and_trie_impl(
        &'data self,
        index: usize,
    ) -> (CollationElement32, &'data [RawBytesULE<2>]) {
        let tail = &self.contexts.as_ule_slice()[index..];
        let (default, trie) = tail.split_at(2);
        let first = u16::from_unaligned(default[0]);
        let second = u16::from_unaligned(default[1]);
        (
            CollationElement32::new((u32::from(first) << 16) | u32::from(second)),
            trie,
        )
    }
    pub(crate) fn get_default_and_trie(
        &'data self,
        index: usize,
    ) -> (CollationElement32, Char16TrieIterator<'data>) {
        let (ce32, trie) = self.get_default_and_trie_impl(index);
        (ce32, Char16TrieIterator::new(trie))
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
        debug_assert!(ce32.tag() == Tag::OffsetTag);
        let data_ce = u64::from_unaligned(self.ces.as_ule_slice()[ce32.index()]);
        CollationElement::new_from_primary(data_ce_to_primary(data_ce, c))
    }
}

#[icu_provider::data_struct(CollationDiacriticsV1Marker = "collator/dia@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationDiacriticsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ce32s: ZeroVec<'data, u32>,
}

#[icu_provider::data_struct(CollationJamoV1Marker = "collator/jamo@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationJamoV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ce32s: ZeroVec<'data, u32>,
}

#[icu_provider::data_struct(CollationReorderingV1Marker = "collator/reord@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
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
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationMetadataV1 {
    /// 8 highest bits for numeric primary.
    /// The low 2 bits are the default strength - 1,
    /// i.e. 0 for primary, 1 for secondary.
    /// The rest for various flags, some of the space
    /// being unused.
    pub bits: u32,
}

impl CollationMetadataV1 {
    const MAX_VARIABLE_MASK: u32 = 0b11;
    const TAILORED_MASK: u32 = 1 << 3;
    const TAILORED_DIACRITICS_MASK: u32 = 1 << 4;
    const TAILORED_JAMO_MASK: u32 = 1 << 5;
    const REORDERING_MASK: u32 = 1 << 6;
    const LITHUANIAN_DOT_ABOVE_MASK: u32 = 1 << 7;
    const BACWARD_SECOND_LEVEL_MASK: u32 = 1 << 8;
    const ALTERNATE_SHIFTED_MASK: u32 = 1 << 9;
    const CASE_FIRST_MASK: u32 = 1 << 10;
    const UPPER_FIRST_MASK: u32 = 1 << 11;

    #[inline(always)]
    pub fn max_variable(&self) -> MaxVariable {
        unsafe { core::mem::transmute((self.bits & CollationMetadataV1::MAX_VARIABLE_MASK) as u8) }
    }

    #[inline(always)]
    pub fn numeric_primary(&self) -> u32 {
        self.bits & 0xFF000000
    }

    #[inline(always)]
    pub fn tailored(&self) -> bool {
        self.bits & CollationMetadataV1::TAILORED_MASK != 0
    }

    /// Vietnamese and Ewe
    #[inline(always)]
    pub fn tailored_diacritics(&self) -> bool {
        self.bits & CollationMetadataV1::TAILORED_DIACRITICS_MASK != 0
    }

    /// Korean search
    #[inline(always)]
    pub fn tailored_jamo(&self) -> bool {
        self.bits & CollationMetadataV1::TAILORED_JAMO_MASK != 0
    }

    /// Lithuanian
    #[inline(always)]
    pub fn lithuanian_dot_above(&self) -> bool {
        self.bits & CollationMetadataV1::LITHUANIAN_DOT_ABOVE_MASK != 0
    }

    /// Canadian French
    #[inline(always)]
    pub fn backward_second_level(&self) -> bool {
        self.bits & CollationMetadataV1::BACWARD_SECOND_LEVEL_MASK != 0
    }

    #[inline(always)]
    pub fn reordering(&self) -> bool {
        self.bits & CollationMetadataV1::REORDERING_MASK != 0
    }

    /// Thai
    #[inline(always)]
    pub fn alternate_shifted(&self) -> bool {
        self.bits & CollationMetadataV1::ALTERNATE_SHIFTED_MASK != 0
    }

    #[inline(always)]
    pub fn case_first(&self) -> CaseFirst {
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

    // XXX alternate_non_ignorable Doesn't exist in current CLDR
}

#[icu_provider::data_struct(CollationSpecialPrimariesV1Marker = "collator/prim@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CollationSpecialPrimariesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub last_primaries: ZeroVec<'data, u16>,
    pub numeric_primary: u8,
}

impl<'data> CollationSpecialPrimariesV1<'data> {
    pub(crate) fn last_primary_for_group(&self, max_variable: MaxVariable) -> u32 {
        // Minus one to generate the right lower 16 bits from the high 16 bits.
        // See parse.cpp in genrb and getLastPrimaryForGroup in ICU4C.
        (u32::from(self.last_primaries.get(max_variable as usize).unwrap()) << 16) - 1
    }
}
