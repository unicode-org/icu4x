// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Various collation-related algorithms and constants in this file are
// adapted from ICU4C and, therefore, are subject to the ICU license as
// described in LICENSE.

//! This module holds the 64-bit `CollationElement` struct used for
//! the actual comparison, the 32-bit `CollationElement32` struct
//! that's used for storage. (Strictly speaking, the storage is
//! `RawBytesULE<4>`.) And the `CollationElements` iterator adapter
//! that turns an iterator over `char` into an iterator over
//! `CollationElement`. (To match the structure of ICU4C, this isn't
//! a real Rust `Iterator`. Instead of signaling end by returning
//! `None`, it signals end by returning `NO_CE`.)
//!
//! This module also declares various constants that are also used
//! by the `comparison` module.

use core::char::REPLACEMENT_CHARACTER;
use icu_char16trie::char16trie::TrieResult;
use icu_codepointtrie::CodePointTrie;
use icu_normalizer::provider::CanonicalDecompositionDataV1;
use icu_properties::CanonicalCombiningClass;
use smallvec::SmallVec;
use zerovec::ule::AsULE;
use zerovec::ule::RawBytesULE;
use zerovec::ZeroSlice;

use crate::provider::CollationDataV1;

// These constants originate from page 143 of Unicode 14.0
const HANGUL_S_BASE: u32 = 0xAC00;
const HANGUL_L_BASE: u32 = 0x1100;
const HANGUL_V_BASE: u32 = 0x1161;
const HANGUL_T_BASE: u32 = 0x11A7;
const HANGUL_T_COUNT: u32 = 28;
const HANGUL_N_COUNT: u32 = 588;
const HANGUL_S_COUNT: u32 = 11172;

pub(crate) const JAMO_COUNT: usize = 256; // 0x1200 - 0x1100

const COMBINING_DIACRITICS_BASE: usize = 0x0300;
const COMBINING_DIACRITICS_LIMIT: usize = 0x0370;
pub(crate) const COMBINING_DIACRITICS_COUNT: usize =
    COMBINING_DIACRITICS_LIMIT - COMBINING_DIACRITICS_BASE;

pub(crate) const CASE_MASK: u16 = 0xC000;
pub(crate) const TERTIARY_MASK: u16 = 0x3F3F; // ONLY_TERTIARY_MASK in ICU4C
pub(crate) const QUATERNARY_MASK: u16 = 0xC0;

// A CE32 is special if its low byte is this or greater.
// Impossible case bits 11 mark special CE32s.
// This value itself is used to indicate a fallback to the base collator.
const SPECIAL_CE32_LOW_BYTE: u8 = 0xC0;
const FALLBACK_CE32: CollationElement32 = CollationElement32(SPECIAL_CE32_LOW_BYTE as u32);
const LONG_PRIMARY_CE32_LOW_BYTE: u8 = 0xC1; // SPECIAL_CE32_LOW_BYTE | LONG_PRIMARY_TAG
const COMMON_SECONDARY_CE: u64 = 0x05000000;
const COMMON_TERTIARY_CE: u64 = 0x0500;
const COMMON_SEC_AND_TER_CE: u64 = COMMON_SECONDARY_CE | COMMON_TERTIARY_CE;

const UNASSIGNED_IMPLICIT_BYTE: u8 = 0xFE;

/// Set if there is no match for the single (no-suffix) character itself.
/// This is only possible if there is a prefix.
/// In this case, discontiguous contraction matching cannot add combining marks
/// starting from an empty suffix.
/// The default CE32 is used anyway if there is no suffix match.
// const CONTRACT_SINGLE_CP_NO_MATCH: u32 = 0x100;

/// Set if the first character of every contraction suffix has lccc!=0.
const CONTRACT_NEXT_CCC: u32 = 0x200;
/// Set if any contraction suffix ends with lccc!=0.
const CONTRACT_TRAILING_CCC: u32 = 0x400;
/// Set if at least one contraction suffix contains a starter
const CONTRACT_HAS_STARTER: u32 = 0x800;

// const NO_CE32: CollationElement32 = CollationElement32::const_default();
// constants named NO_CE* : End of input. Only used in runtime code, not stored in data.
pub(crate) const NO_CE: CollationElement = CollationElement::const_default();
pub(crate) const NO_CE_PRIMARY: u32 = 1; // not a left-adjusted weight
                                         // const NO_CE_NON_PRIMARY: NonPrimary = NonPrimary::const_default();
pub(crate) const NO_CE_SECONDARY: u16 = 0x0100;
pub(crate) const NO_CE_TERTIARY: u16 = 0x0100;
const NO_CE_VALUE: u64 =
    ((NO_CE_PRIMARY as u64) << 32) | ((NO_CE_SECONDARY as u64) << 16) | (NO_CE_TERTIARY as u64); // 0x101000100

// See ICU4C collation.h and https://www.unicode.org/reports/tr10/#Trailing_Weights
const FFFD_PRIMARY: u32 = 0xFFFD0000; // U+FFFD
pub(crate) const FFFD_CE_VALUE: u64 = ((FFFD_PRIMARY as u64) << 32) | COMMON_SEC_AND_TER_CE;
pub(crate) const FFFD_CE: CollationElement = CollationElement(FFFD_CE_VALUE);
pub(crate) const FFFD_CE32_VALUE: u32 = 0xFFFD0505;
pub(crate) const FFFD_CE32: CollationElement32 = CollationElement32(FFFD_CE32_VALUE);

pub(crate) const EMPTY_U16: &ZeroSlice<u16> =
    ZeroSlice::<u16>::from_ule_slice_const(&<u16 as AsULE>::ULE::from_array([]));
const EMPTY_U32: &ZeroSlice<u32> =
    ZeroSlice::<u32>::from_ule_slice_const(&<u32 as AsULE>::ULE::from_array([]));
const SINGLE_U16: &ZeroSlice<u16> =
    ZeroSlice::<u16>::from_ule_slice_const(&<u16 as AsULE>::ULE::from_array([0xFFFD]));
const SINGLE_U32: &ZeroSlice<u32> =
    ZeroSlice::<u32>::from_ule_slice_const(&<u32 as AsULE>::ULE::from_array([0xFFFD]));

/// If `opt` is `Some`, unwrap it. If `None`, panic if debug assertions
/// are enabled and return `default` if debug assertions are not enabled.
///
/// Use this only if the only reason why `opt` could be `None` is bogus
/// data from the provider.
#[inline(always)]
pub(crate) fn unwrap_or_gigo<T>(opt: Option<T>, default: T) -> T {
    if let Some(val) = opt {
        val
    } else {
        // GIGO case
        debug_assert!(false);
        default
    }
}

/// Convert a `u32` _obtained from data provider data_ to `char`.
#[inline(always)]
fn char_from_u32(u: u32) -> char {
    unwrap_or_gigo(core::char::from_u32(u), REPLACEMENT_CHARACTER)
}

/// Convert a `u16` _obtained from data provider data_ to `char`.
#[inline(always)]
fn char_from_u16(u: u16) -> char {
    char_from_u32(u32::from(u))
}

#[inline(always)]
fn split_first_u16(s: Option<&ZeroSlice<u16>>) -> (char, &ZeroSlice<u16>) {
    if let Some(slice) = s {
        if let Some(first) = slice.first() {
            // `unwrap()` must succeed, because `first()` returned `Some`.
            return (
                char_from_u16(first),
                slice.get_subslice(1..slice.len()).unwrap(),
            );
        }
    }
    // GIGO case
    debug_assert!(false);
    (REPLACEMENT_CHARACTER, EMPTY_U16)
}

#[inline(always)]
fn split_first_u32(s: Option<&ZeroSlice<u32>>) -> (char, &ZeroSlice<u32>) {
    if let Some(slice) = s {
        if let Some(first) = slice.first() {
            // `unwrap()` must succeed, because `first()` returned `Some`.
            return (
                char_from_u32(first),
                slice.get_subslice(1..slice.len()).unwrap(),
            );
        }
    }
    // GIGO case
    debug_assert!(false);
    (REPLACEMENT_CHARACTER, EMPTY_U32)
}

#[inline(always)]
fn in_inclusive_range(c: char, start: char, end: char) -> bool {
    u32::from(c).wrapping_sub(u32::from(start)) <= (u32::from(end) - u32::from(start))
}

/// Special-CE32 tags, from bits 3..0 of a special 32-bit CE.
/// Bits 31..8 are available for tag-specific data.
/// Bits  5..4: Reserved. May be used in the future to indicate lccc!=0 and tccc!=0.
#[derive(Eq, PartialEq, Debug)]
#[allow(dead_code)]
#[repr(u8)]
pub(crate) enum Tag {
    /// Fall back to the base collator.
    /// This is the tag value in SPECIAL_CE32_LOW_BYTE and FALLBACK_CE32.
    /// Bits 31..8: Unused, 0.
    Fallback = 0,
    /// Long-primary CE with COMMON_SEC_AND_TER_CE.
    /// Bits 31..8: Three-byte primary.
    LongPrimary = 1,
    /// Long-secondary CE with zero primary.
    /// Bits 31..16: Secondary weight.
    /// Bits 15.. 8: Tertiary weight.
    LongSecondary = 2,
    /// Unused.
    /// May be used in the future for single-byte secondary CEs (SHORT_SECONDARY_TAG),
    /// storing the secondary in bits 31..24, the ccc in bits 23..16,
    /// and the tertiary in bits 15..8.
    Reserved3 = 3,
    /// Latin mini expansions of two simple CEs [pp, 05, tt] [00, ss, 05].
    /// Bits 31..24: Single-byte primary weight pp of the first CE.
    /// Bits 23..16: Tertiary weight tt of the first CE.
    /// Bits 15.. 8: Secondary weight ss of the second CE.
    /// Unused by ICU4X, may get repurposed for jamo expansions is Korean search.
    LatinExpansion = 4,
    /// Points to one or more simple/long-primary/long-secondary 32-bit CE32s.
    /// Bits 31..13: Index into uint32_t table.
    /// Bits 12.. 8: Length=1..31.
    Expansion32 = 5,
    /// Points to one or more 64-bit CEs.
    /// Bits 31..13: Index into CE table.
    /// Bits 12.. 8: Length=1..31.
    Expansion = 6,
    /// Builder data, used only in the CollationDataBuilder, not in runtime data.
    ///
    /// If bit 8 is 0: Builder context, points to a list of context-sensitive mappings.
    /// Bits 31..13: Index to the builder's list of ConditionalCE32 for this character.
    /// Bits 12.. 9: Unused, 0.
    ///
    /// If bit 8 is 1 (IS_BUILDER_JAMO_CE32): Builder-only jamoCE32 value.
    /// The builder fetches the Jamo CE32 from the trie.
    /// Bits 31..13: Jamo code point.
    /// Bits 12.. 9: Unused, 0.
    BuilderData = 7,
    /// Points to prefix trie.
    /// Bits 31..13: Index into prefix/contraction data.
    /// Bits 12.. 8: Unused, 0.
    Prefix = 8,
    /// Points to contraction data.
    /// Bits 31..13: Index into prefix/contraction data.
    /// Bits 12..11: Unused, 0.
    /// Bit      10: CONTRACT_TRAILING_CCC flag.
    /// Bit       9: CONTRACT_NEXT_CCC flag.
    /// Bit       8: CONTRACT_SINGLE_CP_NO_MATCH flag.
    Contraction = 9,
    /// Decimal digit.
    /// Bits 31..13: Index into uint32_t table for non-numeric-collation CE32.
    /// Bit      12: Unused, 0.
    /// Bits 11.. 8: Digit value 0..9.
    Digit = 10,
    /// Tag for U+0000, for moving the NUL-termination handling
    /// from the regular fastpath into specials-handling code.
    /// Bits 31..8: Unused, 0.
    /// Not used by ICU4X.
    U0000 = 11,
    /// Tag for a Hangul syllable.
    /// Bits 31..9: Unused, 0.
    /// Bit      8: HANGUL_NO_SPECIAL_JAMO flag.
    /// Not used by ICU4X, may get reused for compressing Hanja expansions.
    Hangul = 12,
    /// Tag for a lead surrogate code unit.
    /// Optional optimization for UTF-16 string processing.
    /// Bits 31..10: Unused, 0.
    ///       9.. 8: =0: All associated supplementary code points are unassigned-implict.
    ///              =1: All associated supplementary code points fall back to the base data.
    ///              else: (Normally 2) Look up the data for the supplementary code point.
    /// Not used by ICU4X.
    LeadSurrogate = 13,
    /// Tag for CEs with primary weights in code point order.
    /// Bits 31..13: Index into CE table, for one data "CE".
    /// Bits 12.. 8: Unused, 0.
    ///
    /// This data "CE" has the following bit fields:
    /// Bits 63..32: Three-byte primary pppppp00.
    ///      31.. 8: Start/base code point of the in-order range.
    ///           7: Flag isCompressible primary.
    ///       6.. 0: Per-code point primary-weight increment.
    Offset = 14,
    /// Implicit CE tag. Compute an unassigned-implicit CE.
    /// All bits are set (UNASSIGNED_CE32=0xffffffff).
    Implicit = 15,
}

/// A compressed form of a collation element as stored in the collation
/// data.
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct CollationElement32(u32);

impl CollationElement32 {
    #[inline(always)]
    pub fn new(bits: u32) -> Self {
        CollationElement32(bits)
    }

    #[inline(always)]
    pub fn new_from_ule(ule: RawBytesULE<4>) -> Self {
        CollationElement32(u32::from_unaligned(ule))
    }

    #[inline(always)]
    fn low_byte(&self) -> u8 {
        self.0 as u8
    }

    #[inline(always)]
    fn tag_checked(&self) -> Option<Tag> {
        let t = self.low_byte();
        if t < SPECIAL_CE32_LOW_BYTE {
            None
        } else {
            Some(self.tag())
        }
    }

    /// Returns the tag if this element is special.
    /// Non-specialness should first be checked by seeing if either
    /// `to_ce_simple_or_long_primary()` or `to_ce_self_contained()`
    /// returns non-`None`.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if called on a non-special element.
    #[inline(always)]
    pub(crate) fn tag(&self) -> Tag {
        debug_assert!(self.low_byte() >= SPECIAL_CE32_LOW_BYTE);
        // By construction, the byte being transmuted to the enum is within
        // the value space of the enum, so the transmute cannot be UB.
        unsafe { core::mem::transmute(self.low_byte() & 0xF) }
    }

    /// Expands to 64 bits if the expansion is to a single 64-bit collation
    /// element and is not a long-secondary expansion.
    #[inline(always)]
    pub fn to_ce_simple_or_long_primary(self) -> Option<CollationElement> {
        let t = self.low_byte();
        if t < SPECIAL_CE32_LOW_BYTE {
            // Not special
            let as64 = u64::from(self.0);
            Some(CollationElement::new(
                ((as64 & 0xFFFF0000) << 32) | ((as64 & 0xFF00) << 16) | (u64::from(t) << 8),
            ))
        } else if t == LONG_PRIMARY_CE32_LOW_BYTE {
            let as64 = u64::from(self.0);
            Some(CollationElement::new(
                ((as64 - u64::from(t)) << 32) | COMMON_SEC_AND_TER_CE,
            ))
        } else {
            // Could still be long secondary (or not self-contained at all).
            // See `to_ce_self_contained()`.
            None
        }
    }

    /// Expands to 64 bits if the expansion is to a single 64-bit collation
    /// element.
    #[inline(always)]
    pub fn to_ce_self_contained(self) -> Option<CollationElement> {
        if let Some(ce) = self.to_ce_simple_or_long_primary() {
            return Some(ce);
        }
        if self.tag() == Tag::LongSecondary {
            Some(CollationElement::new(u64::from(self.0 & 0xffffff00)))
        } else {
            None
        }
    }

    /// Expands to 64 bits if the expansion is to a single 64-bit collation
    /// element or otherwise returns the collation element for U+FFFD.
    #[inline(always)]
    pub fn to_ce_self_contained_or_gigo(self) -> CollationElement {
        unwrap_or_gigo(self.to_ce_self_contained(), FFFD_CE)
    }

    /// Gets the length from this element.
    ///
    /// # Panics
    ///
    /// In debug builds if this element doesn't have a length.
    #[inline(always)]
    pub fn len(&self) -> usize {
        debug_assert!(self.tag() == Tag::Expansion32 || self.tag() == Tag::Expansion);
        ((self.0 >> 8) & 31) as usize
    }

    /// Gets the index from this element.
    ///
    /// # Panics
    ///
    /// In debug builds if this element doesn't have an index.
    #[inline(always)]
    pub fn index(&self) -> usize {
        debug_assert!(
            self.tag() == Tag::Expansion32
                || self.tag() == Tag::Expansion
                || self.tag() == Tag::Contraction
                || self.tag() == Tag::Digit
                || self.tag() == Tag::Prefix
                || self.tag() == Tag::Offset
        );
        (self.0 >> 13) as usize
    }

    #[inline(always)]
    pub fn digit(&self) -> u8 {
        debug_assert!(self.tag() == Tag::Digit);
        ((self.0 >> 8) & 0xF) as u8
    }

    #[inline(always)]
    pub fn every_suffix_starts_with_combining(&self) -> bool {
        debug_assert!(self.tag() == Tag::Contraction);
        (self.0 & CONTRACT_NEXT_CCC) != 0
    }
    #[inline(always)]
    pub fn at_least_one_suffix_contains_starter(&self) -> bool {
        debug_assert!(self.tag() == Tag::Contraction);
        (self.0 & CONTRACT_HAS_STARTER) != 0
    }
    #[inline(always)]
    pub fn at_least_one_suffix_ends_with_non_starter(&self) -> bool {
        debug_assert!(self.tag() == Tag::Contraction);
        (self.0 & CONTRACT_TRAILING_CCC) != 0
    }
}

impl Default for CollationElement32 {
    fn default() -> Self {
        CollationElement32(1) // NO_CE32
    }
}

/// A collation element is a 64-bit value.
///
/// Bits 63..32 are the primary weight.
/// Bits 31..16 are the secondary weight.
/// Bits 15..14 are the case bits.
/// Bits 13..8 and 5..0 are the (bitwise discontiguous) tertiary weight.
/// Bits 7..6 the quaternary weight.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct CollationElement(u64);

impl CollationElement {
    #[inline(always)]
    pub fn new(bits: u64) -> Self {
        CollationElement(bits)
    }

    #[inline(always)]
    pub fn new_from_primary(primary: u32) -> Self {
        CollationElement((u64::from(primary) << 32) | COMMON_SEC_AND_TER_CE)
    }

    #[inline(always)]
    pub fn new_implicit_from_char(c: char) -> Self {
        // Collation::unassignedPrimaryFromCodePoint
        // Create a gap before U+0000. Use c-1 for [first unassigned].
        let mut c_with_offset = u32::from(c) + 1;
        // Fourth byte: 18 values, every 14th byte value (gap of 13).
        let mut primary: u32 = 2 + (c_with_offset % 18) * 14;
        c_with_offset /= 18;
        // Third byte: 254 values
        primary |= (2 + (c_with_offset % 254)) << 8;
        c_with_offset /= 254;
        // Second byte: 251 values 04..FE excluding the primary compression bytes.
        primary |= (4 + (c_with_offset % 251)) << 16;
        // One lead byte covers all code points (c < 0x1182B4 = 1*251*254*18).
        primary |= u32::from(UNASSIGNED_IMPLICIT_BYTE) << 24;
        CollationElement::new_from_primary(primary)
    }

    #[inline(always)]
    pub fn clone_with_non_primary_zeroed(&self) -> Self {
        CollationElement(self.0 & 0xFFFFFFFF00000000)
    }

    /// Get the primary weight
    #[inline(always)]
    pub fn primary(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Get the non-primary weights
    #[inline(always)]
    pub fn non_primary(&self) -> NonPrimary {
        NonPrimary::new(self.0 as u32)
    }

    /// Get the secondary weight
    #[inline(always)]
    pub fn secondary(&self) -> u16 {
        self.non_primary().secondary()
    }
    #[inline(always)]
    pub fn quaternary(&self) -> u32 {
        self.non_primary().quaternary()
    }
    #[inline(always)]
    pub fn tertiary_ignorable(&self) -> bool {
        self.non_primary().tertiary_ignorable()
    }
    #[inline(always)]
    pub fn either_half_zero(&self) -> bool {
        self.primary() == 0 || (self.0 as u32) == 0
    }

    #[inline(always)]
    pub const fn const_default() -> CollationElement {
        CollationElement(NO_CE_VALUE) // NO_CE
    }
}

impl Default for CollationElement {
    #[inline(always)]
    fn default() -> Self {
        CollationElement(NO_CE_VALUE) // NO_CE
    }
}

impl Default for &CollationElement {
    #[inline(always)]
    fn default() -> Self {
        &CollationElement(NO_CE_VALUE) // NO_CE
    }
}

/// The purpose of grouping the non-primary bits
/// into a struct is to allow for a future optimization
/// that specializes code over whether storage for primary
/// weights is needed or not. (I.e. whether to specialize
/// on `CollationElement` or `NonPrimary`.)
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct NonPrimary(u32);

impl NonPrimary {
    /// Constructor
    pub fn new(bits: u32) -> Self {
        NonPrimary(bits)
    }
    /// Get the bits
    pub fn bits(&self) -> u32 {
        self.0
    }
    /// Get the secondary weight
    #[inline(always)]
    pub fn secondary(&self) -> u16 {
        (self.0 >> 16) as u16
    }
    /// Get the case bits as the high two bits of a u16
    #[inline(always)]
    pub fn case(&self) -> u16 {
        (self.0 as u16) & CASE_MASK
    }
    /// Get the tertiary weight as u16 with the high
    /// two bits of each half zeroed.
    #[inline(always)]
    pub fn tertiary(&self) -> u16 {
        (self.0 as u16) & TERTIARY_MASK
    }
    #[inline(always)]
    pub fn tertiary_ignorable(&self) -> bool {
        (self.0 as u16) <= NO_CE_TERTIARY
    }
    /// Get the quaternary weight in the original
    /// storage bit positions with the other bits
    /// set to one.
    #[inline(always)]
    pub fn quaternary(&self) -> u32 {
        self.0 | !(QUATERNARY_MASK as u32)
    }
    /// Get any combination of tertiary, case, and quaternary
    /// by mask.
    #[inline(always)]
    pub fn tertiary_case_quarternary(&self, mask: u16) -> u16 {
        debug_assert!((mask & CASE_MASK) == CASE_MASK || (mask & CASE_MASK) == 0);
        debug_assert!((mask & TERTIARY_MASK) == TERTIARY_MASK || (mask & TERTIARY_MASK) == 0);
        debug_assert!((mask & QUATERNARY_MASK) == QUATERNARY_MASK || (mask & QUATERNARY_MASK) == 0);
        (self.0 as u16) & mask
    }

    #[inline(always)]
    pub fn case_quaternary(&self) -> u16 {
        (self.0 as u16) & (CASE_MASK | QUATERNARY_MASK)
    }
}

impl Default for NonPrimary {
    #[inline(always)]
    fn default() -> Self {
        NonPrimary(0x01000100) // Low 32 bits of NO_CE
    }
}

/// Pack a `char` and a `CanonicalCombiningClass` in
/// 32 bits. The latter is initialized to 0xFF upon
/// creation and can be set by calling `set_ccc`.
/// This type is intentionally non-`Copy` to get
/// compiler help in making sure that the class is
/// set on the instance on which it is intended to
/// be set and not on a temporary copy.
///
/// XXX check that 0xFF is actually reserved by the spec.
#[derive(Debug)]
struct CharacterAndClass(u32);

impl CharacterAndClass {
    pub fn new(c: char) -> Self {
        // Setting the placeholder combining class to 0xFF to
        // make it greater than zero when there is only one
        // combining character and we don't do the trie lookup.
        CharacterAndClass(u32::from(c) | (0xFF << 24))
    }
    pub fn character(&self) -> char {
        // Safe, because the low 24 bits came from a `char`
        // originally.
        unsafe { char::from_u32_unchecked(self.0 & 0xFFFFFF) }
    }
    pub fn ccc(&self) -> CanonicalCombiningClass {
        CanonicalCombiningClass((self.0 >> 24) as u8)
    }
    // XXX need better naming here.
    pub fn set_ccc(&mut self, ccc: &CodePointTrie<CanonicalCombiningClass>) {
        debug_assert_eq!(self.0 >> 24, 0xFF, "This method has already been called!");
        let scalar = self.0 & 0xFFFFFF;
        self.0 = ((ccc.get(scalar).0 as u32) << 24) | scalar;
    }
}

// This trivial function exists as a borrow check helper.
#[inline(always)]
fn sort_slice_by_ccc<'data>(
    slice: &mut [char],
    ccc: &CodePointTrie<'data, CanonicalCombiningClass>,
) {
    slice.sort_by_key(|cc| ccc.get(u32::from(*cc)));
}

/// Iterator that transforms an iterator over `char` into an iterator
/// over `CollationElement` with a tailoring.
/// Not a real Rust iterator: Instead of `None` uses `NO_CE` to indicate
/// end of iteration to optimize comparison.
pub(crate) struct CollationElements<'data, I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    /// Already computed but not yet returned `CollationElement`s.
    pending: SmallVec<[CollationElement; 6]>, // TODO Figure out good length
    /// The index of the next item to be returned from `pending`. The purpose
    /// of this index is to avoid moving the rest of the items.
    pending_pos: usize,
    /// The characters most previously seen (or never-matching placeholders)
    /// CLDR, as of 40, has two kinds of prefixes:
    /// Prefixes that contain a single starter
    /// Prefixes that contain a starter followed by either U+3099 or U+309A
    /// Last-pushed is at index 0 and previously-pushed at index 1
    prefix: [char; 2],
    /// `upcoming` holds the characters that have already been read from
    /// `iter` but haven't yet been mapped to `CollationElement`s.
    ///
    /// Typically, `upcoming` holds one character and corresponds semantically
    /// to `pending_unnormalized_starter` in `icu_normalizer::Decomposition`.
    /// This is why there isn't a move avoidance optimization similar to
    /// `pending_pos` above for this buffer. A complex decomposition, a
    /// Hangul syllable followed by a non-starter, or lookahead can cause
    /// `pending` to hold more than one `char`.
    ///
    /// Invariant: `upcoming` is allowed to become empty only after `iter`
    /// has been exhausted.
    ///
    /// Invariant: (Checked by `debug_assert!`) At the start of `next()` call,
    /// if `upcoming` isn't empty (with `iter` having been exhausted), the
    /// first `char` in `upcoming` must have its decompostion start with a
    /// starter.
    upcoming: SmallVec<[char; 10]>, // TODO Figure out good length; longest contraction suffix in CLDR 40 is 7 characters long
    /// The root collation data.
    root: &'data CollationDataV1<'data>,
    /// Tailoring if applicable.
    tailoring: &'data CollationDataV1<'data>,
    /// The `CollationElement32` mapping for the Hangul Jamo block.
    ///
    /// Note: in ICU4C the jamo table contains only modern jamo. Here, the jamo table contains the whole Unicode block.
    jamo: &'data [<u32 as AsULE>::ULE; JAMO_COUNT],
    /// The `CollationElement32` mapping for the Combining Diacritical Marks block.
    diacritics: &'data [<u32 as AsULE>::ULE; COMBINING_DIACRITICS_COUNT],
    /// NFD data.
    decompositions: &'data CanonicalDecompositionDataV1<'data>,
    /// Canonical Combining Class data.
    ccc: &'data CodePointTrie<'data, CanonicalCombiningClass>,
    /// If numeric mode is enabled, the 8 high bits of the numeric primary.
    /// `None` if disabled.
    numeric_primary: Option<u8>,
    /// Whether the Lithuanian combining dot above handling is enabled.
    lithuanian_dot_above: bool,
    #[cfg(debug_assertions)]
    /// Whether `iter` has been exhausted
    iter_exhausted: bool,
}

impl<'data, I> CollationElements<'data, I>
where
    I: Iterator<Item = char>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        delegate: I,
        root: &'data CollationDataV1,
        tailoring: &'data CollationDataV1,
        jamo: &'data [<u32 as AsULE>::ULE; JAMO_COUNT],
        diacritics: &'data [<u32 as AsULE>::ULE; COMBINING_DIACRITICS_COUNT],
        decompositions: &'data CanonicalDecompositionDataV1,
        ccc: &'data CodePointTrie<'data, CanonicalCombiningClass>,
        numeric_primary: Option<u8>,
        lithuanian_dot_above: bool,
    ) -> Self {
        let mut u = SmallVec::new();
        u.push('\u{FFFF}'); // Make sure the process always begins with a starter
        let mut ret = CollationElements::<I> {
            iter: delegate,
            pending: SmallVec::new(),
            pending_pos: 0,
            prefix: ['\u{FFFF}'; 2],
            upcoming: u,
            root,
            tailoring,
            jamo,
            diacritics,
            decompositions,
            ccc,
            numeric_primary,
            lithuanian_dot_above,
            #[cfg(debug_assertions)]
            iter_exhausted: false,
        };
        let _ = ret.next(); // Remove the placeholder starter
        ret
    }

    fn next_internal(&mut self) -> Option<char> {
        if self.upcoming.is_empty() {
            return None;
        }
        let ret = self.upcoming.remove(0);
        if self.upcoming.is_empty() {
            if let Some(c) = self.iter.next() {
                self.upcoming.push(c);
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
            }
        }
        Some(ret)
    }

    fn maybe_gather_combining(&mut self) {
        if self.upcoming.len() != 1 {
            return;
        }
        if !self
            .decompositions
            .decomposition_starts_with_non_starter
            .contains(self.upcoming[0])
        {
            return;
        }
        // Not using `while let` to be able to set `iter_exhausted`
        loop {
            if let Some(ch) = self.iter.next() {
                if self
                    .decompositions
                    .decomposition_starts_with_non_starter
                    .contains(ch)
                {
                    if !in_inclusive_range(ch, '\u{0340}', '\u{0F81}') {
                        self.upcoming.push(ch);
                    } else {
                        // The Tibetan special cases are starters that decompose into non-starters.
                        match ch {
                            '\u{0340}' => {
                                // COMBINING GRAVE TONE MARK
                                self.upcoming.push('\u{0300}');
                            }
                            '\u{0341}' => {
                                // COMBINING ACUTE TONE MARK
                                self.upcoming.push('\u{0301}');
                            }
                            '\u{0343}' => {
                                // COMBINING GREEK KORONIS
                                self.upcoming.push('\u{0313}');
                            }
                            '\u{0344}' => {
                                // COMBINING GREEK DIALYTIKA TONOS
                                self.upcoming.push('\u{0308}');
                                self.upcoming.push('\u{0301}');
                            }
                            '\u{0F73}' => {
                                // TIBETAN VOWEL SIGN II
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F72}');
                            }
                            '\u{0F75}' => {
                                // TIBETAN VOWEL SIGN UU
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F74}');
                            }
                            '\u{0F81}' => {
                                // TIBETAN VOWEL SIGN REVERSED II
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F80}');
                            }
                            _ => {
                                self.upcoming.push(ch);
                            }
                        };
                    }
                } else {
                    // Got a new starter
                    self.upcoming.push(ch);
                    break;
                }
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
                break;
            }
        }
    }

    // Decomposes `c`, pushes it to `self.upcoming` (unless the character is
    // a Hangul syllable; Hangul isn't allowed to participate in contractions),
    // gathers the following combining characters from `self.iter` and the following starter.
    // Sorts the combining characters and leaves the starter at the end
    // unnormalized. The trailing unnormalized starter doesn't get appended if
    // `self.iter` is exhausted.
    fn push_decomposed_and_gather_combining(&mut self, c: char) {
        let mut search_start_combining = false;
        let old_len = self.upcoming.len();
        // Not inserting early returns below to keep the same structure
        // as in the ce32 mapping code.

        // Hangul syllable check omitted, because it's fine not to decompose
        // Hangul syllables in lookahead, because Hangul isn't allowed to
        // participate in contractions, and the trie default is that a character
        // is its own decomposition.
        let decomposition = self.decompositions.trie.get(u32::from(c));
        if decomposition == 0 {
            // The character is its own decomposition (or Hangul syllable)
            self.upcoming.push(c);
        } else {
            let high = (decomposition >> 16) as u16;
            let low = decomposition as u16;
            if high != 0 && low != 0 {
                // Decomposition into two BMP characters: starter and non-starter
                self.upcoming.push(char_from_u16(high));
                self.upcoming.push(char_from_u16(low));
            } else if high != 0 {
                // Decomposition into one BMP character
                self.upcoming.push(char_from_u16(high));
            } else {
                // Complex decomposition
                // Format for 16-bit value:
                // Three highest bits: length (always makes the whole thing non-zero, since
                // zero is not a length in use; one bit is "wasted" in order to ensure the
                // 16 bits always end up being non-zero as a whole)
                // Fourth-highest bit: 0 if 16-bit units, 1 if 32-bit units
                // Fifth-highest bit:  0 if all trailing characters are non-starter, 1 if
                //                     at least one trailing character is a starter.
                //                     As of Unicode 14, there a two BMP characters that
                //                     decompose to three characters starter, starter,
                //                     non-starter, and plane 1 has characters that
                //                     decompose to two starters. However, for forward
                //                     compatibility, the semantics here are more generic.
                // Lower bits: Start index in storage
                let offset = usize::from(low & 0x7FF);
                let len = usize::from(low >> 13);
                if low & 0x1000 == 0 {
                    for u in unwrap_or_gigo(
                        self.decompositions
                            .scalars16
                            .get_subslice(offset..offset + len),
                        SINGLE_U16, // single instead of empty for consistency with the other code path
                    )
                    .iter()
                    {
                        self.upcoming.push(char_from_u16(u));
                    }
                } else {
                    for u in unwrap_or_gigo(
                        self.decompositions
                            .scalars32
                            .get_subslice(offset..offset + len),
                        SINGLE_U32, // single instead of empty for consistency with the other code path
                    )
                    .iter()
                    {
                        self.upcoming.push(char_from_u32(u));
                    }
                }
                if low & 0x800 != 0 {
                    search_start_combining = true;
                }
            }
        }
        let start_combining = if search_start_combining {
            // The decomposition contains starters. As of Unicode 14,
            // There are two possible patterns:
            // BMP: starter, starter, non-starter
            // Plane 1: starter, starter.
            // However, for forward compatility, support any combination
            // and search for the last starter.
            let mut i = self.upcoming.len() - 1;
            while self
                .decompositions
                .decomposition_starts_with_non_starter
                .contains(self.upcoming[i])
            {
                i -= 1;
            }
            i + 1
        } else {
            old_len + 1
        };
        let mut end_combining = start_combining;
        // Not using `while let` to be able to set `iter_exhausted`
        loop {
            if let Some(ch) = self.iter.next() {
                if self
                    .decompositions
                    .decomposition_starts_with_non_starter
                    .contains(ch)
                {
                    if !in_inclusive_range(ch, '\u{0340}', '\u{0F81}') {
                        self.upcoming.push(ch);
                    } else {
                        // The Tibetan special cases are starters that decompose into non-starters.
                        match ch {
                            '\u{0340}' => {
                                // COMBINING GRAVE TONE MARK
                                self.upcoming.push('\u{0300}');
                            }
                            '\u{0341}' => {
                                // COMBINING ACUTE TONE MARK
                                self.upcoming.push('\u{0301}');
                            }
                            '\u{0343}' => {
                                // COMBINING GREEK KORONIS
                                self.upcoming.push('\u{0313}');
                            }
                            '\u{0344}' => {
                                // COMBINING GREEK DIALYTIKA TONOS
                                self.upcoming.push('\u{0308}');
                                self.upcoming.push('\u{0301}');
                                end_combining += 1;
                            }
                            '\u{0F73}' => {
                                // XXX check if we can actually come here.
                                // TIBETAN VOWEL SIGN II
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F72}');
                                end_combining += 1;
                            }
                            '\u{0F75}' => {
                                // XXX check if we can actually come here.
                                // TIBETAN VOWEL SIGN UU
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F74}');
                                end_combining += 1;
                            }
                            '\u{0F81}' => {
                                // XXX check if we can actually come here.
                                // TIBETAN VOWEL SIGN REVERSED II
                                self.upcoming.push('\u{0F71}');
                                self.upcoming.push('\u{0F80}');
                                end_combining += 1;
                            }
                            _ => {
                                self.upcoming.push(ch);
                            }
                        };
                    }
                    end_combining += 1;
                } else {
                    // Got a new starter
                    self.upcoming.push(ch);
                    break;
                }
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
                break;
            }
        }
        // Perhaps there is a better borrow checker idiom than a function
        // call for indicating that `upcoming` and `ccc` are disjoint and don't
        // overlap. However, this works.
        sort_slice_by_ccc(&mut self.upcoming[start_combining..end_combining], self.ccc);
    }

    // Assumption: `pos` starts from zero and increases one by one.
    fn look_ahead(&mut self, pos: usize) -> Option<char> {
        if pos + 1 == self.upcoming.len() {
            let c = self.upcoming.remove(pos);
            self.push_decomposed_and_gather_combining(c);
            Some(self.upcoming[pos])
        } else if pos == self.upcoming.len() {
            if let Some(c) = self.iter.next() {
                self.push_decomposed_and_gather_combining(c);
                Some(self.upcoming[pos])
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
                None
            }
        } else {
            Some(self.upcoming[pos])
        }
    }

    fn is_next_decomposition_starts_with_starter(&self) -> bool {
        if self.upcoming.is_empty() {
            return true;
        }
        !self
            .decompositions
            .decomposition_starts_with_non_starter
            .contains(self.upcoming[0])
    }

    fn prepend_and_sort_non_starter_prefix_of_suffix(&mut self, c: char) {
        // Add one for the insertion afterwards.
        let end = 1 + {
            let mut iter = self.upcoming.iter().enumerate();
            loop {
                if let Some((i, &ch)) = iter.next() {
                    if !self
                        .decompositions
                        .decomposition_starts_with_non_starter
                        .contains(ch)
                    {
                        break i;
                    }
                } else {
                    #[cfg(debug_assertions)]
                    {
                        self.iter_exhausted = true;
                    }
                    break self.upcoming.len();
                }
            }
        };
        self.upcoming.insert(0, c);
        let start = if self
            .decompositions
            .decomposition_starts_with_non_starter
            .contains(c)
        {
            0
        } else {
            1
        };
        sort_slice_by_ccc(&mut self.upcoming[start..end], self.ccc);
    }

    fn prefix_push(&mut self, c: char) {
        self.prefix[1] = self.prefix[0];
        self.prefix[0] = c;
    }

    /// Micro optimization for doing a simpler write when
    /// we know the most recent character was a non-starter
    /// that is not a kana voicing mark.
    fn mark_prefix_unmatchable(&mut self) {
        self.prefix[0] = '\u{FFFF}';
    }

    pub fn next(&mut self) -> CollationElement {
        debug_assert!(self.is_next_decomposition_starts_with_starter());
        if self.pending_pos < self.pending.len() {
            let ret = self.pending[self.pending_pos];
            self.pending_pos += 1;
            if self.pending_pos == self.pending.len() {
                self.pending.clear();
                self.pending_pos = 0;
            }
            return ret;
        }
        debug_assert_eq!(self.pending_pos, 0);
        if let Some(mut c) = self.next_internal() {
            let mut next_is_known_to_decompose_to_non_starter = false; // micro optimization to avoid checking `UnicodeSet` twice
            let mut ce32;
            let mut data: &CollationDataV1 = self.tailoring;
            let mut combining_characters: SmallVec<[CharacterAndClass; 7]> = SmallVec::new(); // XXX figure out proper size

            // Betting that fusing the NFD algorithm into this one at the
            // expense of the repetitiveness below, the common cases become
            // fast in a way that offsets the lack of the canonical closure.
            // The wall of code before the "Slow path" is an attempt to
            // optimize based on that bet.
            // TODO: ASCII fast path here if ASCII not tailored with
            // starters.
            let hangul_offset = u32::from(c).wrapping_sub(HANGUL_S_BASE); // SIndex in the spec
            if hangul_offset >= HANGUL_S_COUNT {
                let decomposition = self.decompositions.trie.get(u32::from(c));
                if decomposition == 0 {
                    // The character is its own decomposition
                    let jamo_index = (c as usize).wrapping_sub(HANGUL_L_BASE as usize);
                    if jamo_index >= self.jamo.len() {
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                    } else {
                        // The purpose of reading the CE32 from the jamo table instead
                        // of the trie even in this case is to make it unnecessary
                        // for all search collation tries to carry a copy of the Hangul
                        // part of the search root. Instead, all non-Korean tailorings
                        // can use a shared copy of the non-Korean search jamo table.
                        //
                        // XXX This isn't actually true with the current jamo search
                        // expansions!

                        // TODO: Instead of having different jamo CE32 table for "search"
                        // collations, we could instead decompose the archaic jamo to
                        // the modern approximation sequences here and then map those
                        // by looking up the modern jamo from the normal root.

                        // We need to set data to root, because archaic jamo refer to
                        // the root.
                        data = self.root;
                        ce32 = CollationElement32::new_from_ule(self.jamo[jamo_index]);
                    }
                    if self.is_next_decomposition_starts_with_starter() {
                        if let Some(ce) = ce32.to_ce_simple_or_long_primary() {
                            self.prefix_push(c);
                            return ce;
                        } else if ce32.tag() == Tag::Contraction
                            && ce32.every_suffix_starts_with_combining()
                        {
                            // Avoid falling onto the slow path e.g. that letters that
                            // may contract with a diacritic when we know that it won't
                            // contract with the next character.
                            let default = data.get_default(ce32.index());
                            if let Some(ce) = default.to_ce_simple_or_long_primary() {
                                self.prefix_push(c);
                                return ce;
                            }
                        }
                    } else {
                        next_is_known_to_decompose_to_non_starter = true;
                    }
                } else {
                    let high = (decomposition >> 16) as u16;
                    let low = decomposition as u16;
                    if high != 0 && low != 0 {
                        // Decomposition into two BMP characters: starter and non-starter
                        c = char_from_u16(high);
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                        let combining = char_from_u16(low);
                        if self.is_next_decomposition_starts_with_starter() {
                            let diacritic_index =
                                (low as usize).wrapping_sub(COMBINING_DIACRITICS_BASE);
                            if diacritic_index < self.diacritics.len() {
                                debug_assert!(low != 0x0344, "Should never have COMBINING GREEK DIALYTIKA TONOS here, since it should have decomposed further.");
                                if let Some(ce) = ce32.to_ce_simple_or_long_primary() {
                                    // Inner unwrap: already checked len()
                                    // Outer unwrap: expectation of data integrity.
                                    let ce_for_combining = CollationElement32::new_from_ule(
                                        self.diacritics[diacritic_index],
                                    )
                                    .to_ce_self_contained_or_gigo();
                                    self.pending.push(ce_for_combining);
                                    self.mark_prefix_unmatchable();
                                    return ce;
                                }
                                if ce32.tag() == Tag::Contraction
                                    && ce32.every_suffix_starts_with_combining()
                                {
                                    let (default, mut trie) =
                                        data.get_default_and_trie(ce32.index());
                                    match trie.next(combining) {
                                        TrieResult::NoMatch | TrieResult::NoValue => {
                                            if let Some(ce) = default.to_ce_simple_or_long_primary()
                                            {
                                                // Inner unwrap: already checked len()
                                                // Outer unwrap: expectation of data integrity.
                                                let ce_for_combining =
                                                    CollationElement32::new_from_ule(
                                                        self.diacritics[diacritic_index],
                                                    )
                                                    .to_ce_self_contained_or_gigo();
                                                self.pending.push(ce_for_combining);
                                                self.mark_prefix_unmatchable();
                                                return ce;
                                            }
                                        }
                                        TrieResult::Intermediate(trie_ce32)
                                        | TrieResult::FinalValue(trie_ce32) => {
                                            // Assuming that we don't have longer matches with
                                            // a starter at this point. XXX Is this true?
                                            if let Some(ce) =
                                                CollationElement32::new(trie_ce32 as u32)
                                                    .to_ce_simple_or_long_primary()
                                            {
                                                self.mark_prefix_unmatchable();
                                                return ce;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            next_is_known_to_decompose_to_non_starter = true;
                        }
                        combining_characters.push(CharacterAndClass::new(combining));
                    } else if high != 0 {
                        // Decomposition into one BMP character
                        c = char_from_u16(high);
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                        if self.is_next_decomposition_starts_with_starter() {
                            if let Some(ce) = ce32.to_ce_simple_or_long_primary() {
                                self.prefix_push(c);
                                return ce;
                            }
                        } else {
                            next_is_known_to_decompose_to_non_starter = true;
                        }
                    } else {
                        // Complex decomposition
                        // Format for 16-bit value:
                        // Three highest bits: length (always makes the whole thing non-zero, since
                        // zero is not a length in use; one bit is "wasted" in order to ensure the
                        // 16 bits always end up being non-zero as a whole)
                        // Fourth-highest bit: 0 if 16-bit units, 1 if 32-bit units
                        // Fifth-highest bit:  0 if all trailing characters are non-starter, 1 if
                        //                     at least one trailing character is a starter.
                        //                     As of Unicode 14, there a two BMP characters that
                        //                     decompose to three characters starter, starter,
                        //                     non-starter, and plane 1 has characters that
                        //                     decompose to two starters. However, for forward
                        //                     compatibility, the semantics here are more generic.
                        // Lower bits: Start index in storage
                        let offset = usize::from(low & 0x7FF);
                        let len = usize::from(low >> 13);
                        if low & 0x1000 == 0 {
                            let (starter, tail) = split_first_u16(
                                self.decompositions
                                    .scalars16
                                    .get_subslice(offset..offset + len),
                            );
                            c = starter;
                            if low & 0x800 == 0 {
                                for u in tail.iter() {
                                    combining_characters
                                        .push(CharacterAndClass::new(char_from_u16(u)));
                                }
                            } else {
                                next_is_known_to_decompose_to_non_starter = false;
                                let mut it = tail.iter();
                                while let Some(u) = it.next() {
                                    let ch = char_from_u16(u);
                                    if self
                                        .decompositions
                                        .decomposition_starts_with_non_starter
                                        .contains(ch)
                                    {
                                        // As of Unicode 14, this branch is never taken.
                                        // It exist for forward compatibility.
                                        combining_characters.push(CharacterAndClass::new(ch));
                                        continue;
                                    }

                                    // At this point, we might have a single newly-read
                                    // combining character in self.upcoming. In that case, we
                                    // need to buffer up the upcoming combining characters, too,
                                    // in order to make `prepend_and_sort_non_starter_prefix_of_suffix`
                                    // sort the right characters.
                                    self.maybe_gather_combining();

                                    while let Some(u) = it.next_back() {
                                        self.prepend_and_sort_non_starter_prefix_of_suffix(
                                            char_from_u16(u),
                                        );
                                    }
                                    self.prepend_and_sort_non_starter_prefix_of_suffix(ch);
                                    break;
                                }
                            }
                        } else {
                            let (starter, tail) = split_first_u32(
                                self.decompositions
                                    .scalars32
                                    .get_subslice(offset..offset + len),
                            );
                            c = starter;
                            if low & 0x800 == 0 {
                                for u in tail.iter() {
                                    combining_characters
                                        .push(CharacterAndClass::new(char_from_u32(u)));
                                }
                            } else {
                                next_is_known_to_decompose_to_non_starter = false;
                                let mut it = tail.iter();
                                while let Some(u) = it.next() {
                                    let ch = char_from_u32(u);
                                    if self
                                        .decompositions
                                        .decomposition_starts_with_non_starter
                                        .contains(ch)
                                    {
                                        // As of Unicode 14, this branch is never taken.
                                        // It exist for forward compatibility.
                                        combining_characters.push(CharacterAndClass::new(ch));
                                        continue;
                                    }
                                    // At this point, we might have a single newly-read
                                    // combining character in self.upcoming. In that case, we
                                    // need to buffer up the upcoming combining characters, too,
                                    // in order to make `prepend_and_sort_non_starter_prefix_of_suffix`
                                    // sort the right characters.
                                    self.maybe_gather_combining();

                                    while let Some(u) = it.next_back() {
                                        self.prepend_and_sort_non_starter_prefix_of_suffix(
                                            char_from_u32(u),
                                        );
                                    }
                                    self.prepend_and_sort_non_starter_prefix_of_suffix(ch);
                                    break;
                                }
                            }
                        }
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                    }
                }
            } else {
                // Hangul syllable
                // The math here comes from page 144 of Unicode 14.0
                let l = hangul_offset / HANGUL_N_COUNT;
                let v = (hangul_offset % HANGUL_N_COUNT) / HANGUL_T_COUNT;
                let t = hangul_offset % HANGUL_T_COUNT;

                // No prefix matches on Hangul
                self.mark_prefix_unmatchable();
                if self.is_next_decomposition_starts_with_starter() {
                    // XXX Figure out if non-self-contained jamo CE32s exist in
                    // CLDR for modern jamo.
                    self.pending.push(
                        CollationElement32::new_from_ule(
                            self.jamo[(HANGUL_V_BASE - HANGUL_L_BASE + v) as usize],
                        )
                        .to_ce_self_contained_or_gigo(),
                    );
                    if t != 0 {
                        self.pending.push(
                            CollationElement32::new_from_ule(
                                self.jamo[(HANGUL_T_BASE - HANGUL_L_BASE + t) as usize],
                            )
                            .to_ce_self_contained_or_gigo(),
                        );
                    }
                    return CollationElement32::new_from_ule(self.jamo[l as usize])
                        .to_ce_self_contained_or_gigo();
                }

                // Uphold the invariant that the upcoming character is a starter (or end of stream)
                // at the start of the next `next()` call. We uphold this invariant by leaving the
                // last jamo unmapped to `CollationElement` in `pending` and instead prepend it to
                // `upcoming`.
                //
                // The `unsafe` blocks are OK, because the value is by construction in the Hangul
                // jamo block, which is in the scalar value range.
                if t != 0 {
                    self.pending.push(
                        CollationElement32::new_from_ule(
                            self.jamo[(HANGUL_V_BASE - HANGUL_L_BASE + v) as usize],
                        )
                        .to_ce_self_contained_or_gigo(),
                    );
                    self.upcoming.insert(0, unsafe {
                        core::char::from_u32_unchecked(HANGUL_T_BASE + t)
                    });
                } else {
                    self.upcoming.insert(0, unsafe {
                        core::char::from_u32_unchecked(HANGUL_V_BASE + v)
                    });
                }

                return CollationElement32::new_from_ule(self.jamo[l as usize])
                    .to_ce_self_contained_or_gigo();
            }
            let mut may_have_contracted_starter = false;
            // Slow path
            self.collect_combining(
                &mut next_is_known_to_decompose_to_non_starter,
                &mut combining_characters,
            );
            // Now:
            // c is the starter character
            // ce32 is the CollationElement32 for the starter
            // combining_characters contains all the combining characters before
            // the next starter sorted by combining class.
            let mut looked_ahead = 0;
            let mut drain_from_suffix = 0;
            'outer: loop {
                'ce32loop: loop {
                    if let Some(ce) = ce32.to_ce_self_contained() {
                        self.pending.push(ce);
                        break 'ce32loop;
                    } else {
                        match ce32.tag() {
                            Tag::Expansion32 => {
                                let ce32s = data.get_ce32s(ce32.index(), ce32.len());
                                for u in ce32s.iter() {
                                    self.pending.push(
                                        CollationElement32::new(u).to_ce_self_contained_or_gigo(),
                                    );
                                }
                                break 'ce32loop;
                            }
                            Tag::Expansion => {
                                let ces = data.get_ces(ce32.index(), ce32.len());
                                for u in ces.iter() {
                                    self.pending.push(CollationElement::new(u));
                                }
                                break 'ce32loop;
                            }
                            Tag::Prefix => {
                                let (default, mut trie) = data.get_default_and_trie(ce32.index());
                                ce32 = default;
                                for &ch in self.prefix.iter() {
                                    match trie.next(ch) {
                                        TrieResult::NoValue => {}
                                        TrieResult::NoMatch => {
                                            continue 'ce32loop;
                                        }
                                        TrieResult::Intermediate(ce32_i) => {
                                            ce32 = CollationElement32::new(ce32_i as u32);
                                        }
                                        TrieResult::FinalValue(ce32_i) => {
                                            ce32 = CollationElement32::new(ce32_i as u32);
                                            continue 'ce32loop;
                                        }
                                    }
                                }
                                continue 'ce32loop;
                            }
                            Tag::Contraction => {
                                let every_suffix_starts_with_combining =
                                    ce32.every_suffix_starts_with_combining();
                                let at_least_one_suffix_contains_starter =
                                    ce32.at_least_one_suffix_contains_starter();
                                let at_least_one_suffix_ends_with_non_starter =
                                    ce32.at_least_one_suffix_ends_with_non_starter();
                                let (default, mut trie) = data.get_default_and_trie(ce32.index());
                                ce32 = default;
                                if every_suffix_starts_with_combining
                                    && combining_characters.is_empty()
                                {
                                    continue 'ce32loop;
                                }
                                let mut longest_matching_state = trie.clone();
                                let mut longest_matching_index = 0;
                                let mut attempt = 0;
                                let mut i = 0;
                                let mut most_recent_skipped_ccc =
                                    CanonicalCombiningClass::NotReordered;
                                // XXX pending removals will in practice be small numbers.
                                // What if we made the item smaller than usize?
                                let mut pending_removals: SmallVec<[usize; 1]> = SmallVec::new();
                                while i < combining_characters.len() {
                                    let combining_and_class = &combining_characters[i];
                                    let ccc = combining_and_class.ccc();
                                    match (
                                        most_recent_skipped_ccc < ccc,
                                        trie.next(combining_and_class.character()),
                                    ) {
                                        (true, TrieResult::Intermediate(ce32_i)) => {
                                            let _ = combining_characters.remove(i);
                                            while let Some(idx) = pending_removals.pop() {
                                                combining_characters.remove(idx);
                                                i -= 1; // Adjust for the shortening
                                            }
                                            attempt = 0;
                                            longest_matching_index = i;
                                            longest_matching_state = trie.clone();
                                            ce32 = CollationElement32::new(ce32_i as u32);
                                        }
                                        (true, TrieResult::FinalValue(ce32_i)) => {
                                            let _ = combining_characters.remove(i);
                                            while let Some(idx) = pending_removals.pop() {
                                                combining_characters.remove(idx);
                                            }
                                            ce32 = CollationElement32::new(ce32_i as u32);
                                            continue 'ce32loop;
                                        }
                                        (_, TrieResult::NoValue) => {
                                            pending_removals.push(i);
                                            i += 1;
                                        }
                                        _ => {
                                            pending_removals.clear();
                                            most_recent_skipped_ccc = ccc;
                                            attempt += 1;
                                            i = longest_matching_index + attempt;
                                            trie = longest_matching_state.clone();
                                        }
                                    }
                                }
                                if !(at_least_one_suffix_contains_starter
                                    && combining_characters.is_empty())
                                {
                                    continue 'ce32loop;
                                }
                                // Let's just set this flag here instead of trying to make
                                // it more granular and, therefore, more error-prone.
                                // After all, this flag is just about optimizing away one
                                // `UnicodeSet` check in the common case.
                                may_have_contracted_starter = true;
                                debug_assert!(pending_removals.is_empty());
                                loop {
                                    let ahead = self.look_ahead(looked_ahead);
                                    looked_ahead += 1;
                                    if let Some(ch) = ahead {
                                        match trie.next(ch) {
                                            TrieResult::NoValue => {}
                                            TrieResult::NoMatch => {
                                                if !at_least_one_suffix_ends_with_non_starter {
                                                    continue 'ce32loop;
                                                }
                                                if !self
                                                    .decompositions
                                                    .decomposition_starts_with_non_starter
                                                    .contains(ch)
                                                {
                                                    continue 'ce32loop;
                                                }
                                                // The last-checked character is non-starter
                                                // and at least one contraction suffix ends
                                                // with a non-starter. Try a discontiguous
                                                // match.
                                                trie = longest_matching_state.clone();
                                                // For clarity, mint a new set of variables that
                                                // behave consistently with the
                                                // `combining_characters` case
                                                let mut longest_matching_index = 0;
                                                let mut attempt = 0;
                                                let mut i = 0;
                                                most_recent_skipped_ccc =
                                                    self.ccc.get(u32::from(ch));
                                                loop {
                                                    let ahead = self.look_ahead(looked_ahead + i);
                                                    if let Some(ch) = ahead {
                                                        let ccc = self.ccc.get(u32::from(ch));
                                                        if ccc
                                                            == CanonicalCombiningClass::NotReordered
                                                        {
                                                            // If we came here, we had an intervening non-matching
                                                            // non-starter, after which we cannot contract another
                                                            // starter anymore.
                                                            continue 'ce32loop;
                                                        }
                                                        match (
                                                            most_recent_skipped_ccc < ccc,
                                                            trie.next(ch),
                                                        ) {
                                                            (
                                                                true,
                                                                TrieResult::Intermediate(ce32_i),
                                                            ) => {
                                                                let _ = self
                                                                    .upcoming
                                                                    .remove(looked_ahead + i);
                                                                while let Some(idx) =
                                                                    pending_removals.pop()
                                                                {
                                                                    self.upcoming
                                                                        .remove(looked_ahead + idx);
                                                                    i -= 1; // Adjust for the shortening
                                                                }
                                                                attempt = 0;
                                                                longest_matching_index = i;
                                                                longest_matching_state =
                                                                    trie.clone();
                                                                ce32 = CollationElement32::new(
                                                                    ce32_i as u32,
                                                                );
                                                            }
                                                            (
                                                                true,
                                                                TrieResult::FinalValue(ce32_i),
                                                            ) => {
                                                                let _ = self
                                                                    .upcoming
                                                                    .remove(looked_ahead + i);
                                                                while let Some(idx) =
                                                                    pending_removals.pop()
                                                                {
                                                                    self.upcoming
                                                                        .remove(looked_ahead + idx);
                                                                }
                                                                ce32 = CollationElement32::new(
                                                                    ce32_i as u32,
                                                                );
                                                                continue 'ce32loop;
                                                            }
                                                            (_, TrieResult::NoValue) => {
                                                                pending_removals.push(i);
                                                                i += 1;
                                                            }
                                                            _ => {
                                                                pending_removals.clear();
                                                                most_recent_skipped_ccc = ccc;
                                                                attempt += 1;
                                                                i = longest_matching_index
                                                                    + attempt;
                                                                trie =
                                                                    longest_matching_state.clone();
                                                            }
                                                        }
                                                    } else {
                                                        continue 'ce32loop;
                                                    }
                                                }
                                            }
                                            TrieResult::Intermediate(ce32_i) => {
                                                longest_matching_state = trie.clone();
                                                drain_from_suffix = looked_ahead;
                                                ce32 = CollationElement32::new(ce32_i as u32);
                                            }
                                            TrieResult::FinalValue(ce32_i) => {
                                                drain_from_suffix = looked_ahead;
                                                ce32 = CollationElement32::new(ce32_i as u32);
                                                continue 'ce32loop;
                                            }
                                        }
                                    } else {
                                        continue 'ce32loop;
                                    }
                                }
                                // Unreachable
                            }
                            Tag::Digit => {
                                if let Some(high_bits) = self.numeric_primary {
                                    let mut digits: SmallVec<[u8; 8]> = SmallVec::new(); // XXX figure out proper size
                                    digits.push(ce32.digit());
                                    let numeric_primary = u32::from(high_bits) << 24;
                                    if combining_characters.is_empty() {
                                        // Numeric collation doesn't work with combining
                                        // characters applied to the digits.
                                        // XXX Does any tailoring actually tailor digits?
                                        may_have_contracted_starter = true;
                                        while let Some(upcoming) = self.look_ahead(looked_ahead) {
                                            looked_ahead += 1;
                                            ce32 = self.tailoring.ce32_for_char(upcoming);
                                            if ce32 == FALLBACK_CE32 {
                                                ce32 = self.root.ce32_for_char(upcoming);
                                            }
                                            if ce32.tag_checked() != Some(Tag::Digit) {
                                                break;
                                            }
                                            drain_from_suffix = looked_ahead;
                                            digits.push(ce32.digit());
                                        }
                                    }
                                    // Skip leading zeros
                                    let mut zeros = 0;
                                    while let Some(&digit) = digits.get(zeros) {
                                        if digit != 0 {
                                            break;
                                        }
                                        zeros += 1;
                                    }
                                    if zeros == digits.len() {
                                        // All zeros, keep a zero
                                        zeros = digits.len() - 1;
                                    }
                                    let mut remaining = &digits[zeros..];
                                    while !remaining.is_empty() {
                                        // Numeric CEs are generated for segments of
                                        // up to 254 digits.
                                        let (head, tail) = if remaining.len() > 254 {
                                            remaining.split_at(254)
                                        } else {
                                            (remaining, &b""[..])
                                        };
                                        remaining = tail;
                                        // From ICU4C CollationIterator::appendNumericSegmentCEs
                                        if head.len() <= 7 {
                                            let mut digit_iter = head.iter();
                                            // Unwrap succeeds, because we always have at least one
                                            // digit to even start numeric processing.
                                            let mut value = u32::from(*digit_iter.next().unwrap());
                                            for &digit in digit_iter {
                                                value *= 10;
                                                value += u32::from(digit);
                                            }
                                            // Primary weight second byte values:
                                            //     74 byte values   2.. 75 for small numbers in two-byte primary weights.
                                            //     40 byte values  76..115 for medium numbers in three-byte primary weights.
                                            //     16 byte values 116..131 for large numbers in four-byte primary weights.
                                            //    124 byte values 132..255 for very large numbers with 4..127 digit pairs.
                                            let mut first_byte = 2u32;
                                            let mut num_bytes = 74u32;
                                            if value < num_bytes {
                                                self.pending.push(
                                                    CollationElement::new_from_primary(
                                                        numeric_primary
                                                            | ((first_byte + value) << 16),
                                                    ),
                                                );
                                                continue;
                                            }
                                            value -= num_bytes;
                                            first_byte += num_bytes;
                                            num_bytes = 40;
                                            if value < num_bytes * 254 {
                                                // Three-byte primary for 74..10233=74+40*254-1, good for year numbers and more.
                                                self.pending.push(
                                                    CollationElement::new_from_primary(
                                                        numeric_primary
                                                            | ((first_byte + value / 254) << 16)
                                                            | ((2 + value % 254) << 8),
                                                    ),
                                                );
                                                continue;
                                            }
                                            value -= num_bytes * 254;
                                            first_byte += num_bytes;
                                            num_bytes = 16;
                                            if value < num_bytes * 254 * 254 {
                                                // Four-byte primary for 10234..1042489=10234+16*254*254-1.
                                                let mut primary =
                                                    numeric_primary | (2 + value % 254);
                                                value /= 254;
                                                primary |= (2 + value % 254) << 8;
                                                value /= 254;
                                                primary |= (first_byte + value % 254) << 16;
                                                self.pending.push(
                                                    CollationElement::new_from_primary(primary),
                                                );
                                                continue;
                                            }
                                            // original value > 1042489
                                        }
                                        debug_assert!(head.len() >= 7);
                                        // The second primary byte value 132..255 indicates the number of digit pairs (4..127),
                                        // then we generate primary bytes with those pairs.
                                        // Omit trailing 00 pairs.
                                        // Decrement the value for the last pair.

                                        // Set the exponent. 4 pairs->132, 5 pairs->133, ..., 127 pairs->255.
                                        let mut len = head.len();
                                        let num_pairs = (len as u32 + 1) / 2; // as u32 OK, because capped to 254
                                        let mut primary =
                                            numeric_primary | ((132 - 4 + num_pairs) << 16);
                                        // Find the length without trailing 00 pairs.
                                        // XXX what guarantees [len - 2] not being index out of bounds?
                                        while head[len - 1] == 0 && head[len - 2] == 0 {
                                            len -= 2;
                                        }
                                        // Read the first pair
                                        let mut digit_iter = head[..len].iter();
                                        let mut pair = if len & 1 == 1 {
                                            // Only "half a pair" if we have an odd number of digits.
                                            u32::from(*digit_iter.next().unwrap())
                                        } else {
                                            u32::from(*digit_iter.next().unwrap()) * 10
                                                + u32::from(*digit_iter.next().unwrap())
                                        };
                                        pair = 11 + 2 * pair;
                                        let mut shift = 8u32;
                                        while let (Some(&left), Some(&right)) =
                                            (digit_iter.next(), digit_iter.next())
                                        {
                                            if shift == 0 {
                                                primary |= pair;
                                                self.pending.push(
                                                    CollationElement::new_from_primary(primary),
                                                );
                                                primary = numeric_primary;
                                                shift = 16;
                                            } else {
                                                primary |= pair << shift;
                                                shift -= 8;
                                            }
                                            pair =
                                                11 + 2 * (u32::from(left) * 10 + u32::from(right));
                                        }
                                        primary |= (pair - 1) << shift;
                                        self.pending
                                            .push(CollationElement::new_from_primary(primary));
                                    }
                                    break 'ce32loop;
                                }
                                ce32 = data.get_ce32(ce32.index());
                                continue 'ce32loop;
                            }
                            // XXX how common are the following two cases? Should these
                            // be baked into the fast path, since they yield a single CE?
                            Tag::Offset => {
                                self.pending.push(data.ce_from_offset_ce32(c, ce32));
                                break 'ce32loop;
                            }
                            Tag::Implicit => {
                                self.pending
                                    .push(CollationElement::new_implicit_from_char(c));
                                break 'ce32loop;
                            }
                            Tag::Fallback
                            | Tag::Reserved3
                            | Tag::LongPrimary
                            | Tag::LongSecondary
                            | Tag::BuilderData
                            | Tag::LeadSurrogate
                            | Tag::LatinExpansion
                            | Tag::U0000
                            | Tag::Hangul => {
                                debug_assert!(false);
                                // GIGO case
                                self.pending.push(FFFD_CE);
                                break 'ce32loop;
                            }
                        }
                    }
                }
                self.prefix_push(c);
                'combining_outer: loop {
                    debug_assert!(drain_from_suffix == 0 || combining_characters.is_empty());
                    let mut i = 0;
                    'combining: while i < combining_characters.len() {
                        c = combining_characters[i].character();
                        let diacritic_index = (c as usize).wrapping_sub(COMBINING_DIACRITICS_BASE);
                        if let Some(&diacritic) = self.diacritics.get(diacritic_index) {
                            // TODO: unlikely annotation for the first two conditions here:
                            if c == '\u{0307}'
                                && self.lithuanian_dot_above
                                && i + 1 < combining_characters.len()
                            {
                                let next_c = combining_characters[i + 1].character();
                                if next_c == '\u{0300}'
                                    || next_c == '\u{0301}'
                                    || next_c == '\u{0303}'
                                {
                                    // Lithuanian contracts COMBINING DOT ABOVE with three other diacritics of the
                                    // same combining class such that the COMBINING DOT ABOVE is ignored for
                                    // collation. Since the combining class is the same, it's valid to simply
                                    // look at the next character in `combining_characters`.
                                    i += 1;
                                    continue 'combining;
                                }
                            }
                            self.pending.push(
                                CollationElement32::new_from_ule(diacritic)
                                    .to_ce_self_contained_or_gigo(),
                            );
                            self.mark_prefix_unmatchable();
                            i += 1;
                            continue 'combining;
                        }
                        // `c` is not from the Combining Diacritical Marks block.
                        // Not bothering to micro optimize away the move of the remaining
                        // part of `combining_characters`.
                        let _ = combining_characters.drain(..=i);
                        data = self.tailoring;
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                        continue 'outer;
                    }
                    // XXX the borrow checker didn't like the iterator formulation
                    // for the loop below, because the `Drain` would have kept `self`
                    // mutable borrowed when trying to call `prefix_push`. To change
                    // this, `prefix` and `prefix_push` would need to be refactored
                    // into a struct.
                    i = 0;
                    while i < drain_from_suffix {
                        let ch = self.upcoming[i];
                        self.prefix_push(ch);
                        i += 1;
                    }
                    // XXX The above makes prefix out of sync when starter-contracting
                    // contractions use `pending_removals` instead of `drain_from_suffix`.
                    // Do there exist prefixes that overlap contraction suffixes?
                    // At least as of CLDR 40, the two possible non-starters in prefixes,
                    // kana voicing marks, shouldn't be participating in Brahmic contractions.
                    let _ = self.upcoming.drain(..drain_from_suffix);
                    if self.upcoming.is_empty() {
                        // Make the assertion conditional to make CI happy.
                        #[cfg(debug_assertions)]
                        debug_assert!(self.iter_exhausted || may_have_contracted_starter);
                        if let Some(c) = self.iter.next() {
                            self.upcoming.push(c);
                        } else {
                            #[cfg(debug_assertions)]
                            {
                                self.iter_exhausted = true;
                            }
                        }
                    }
                    if may_have_contracted_starter {
                        may_have_contracted_starter = false;
                        if !self.is_next_decomposition_starts_with_starter() {
                            // We need to loop back and process another round of
                            // non-starters in order to maintain the invariant of
                            // `upcoming` on the next call to `next()`.
                            drain_from_suffix = 0;
                            next_is_known_to_decompose_to_non_starter = true;
                            self.collect_combining(
                                &mut next_is_known_to_decompose_to_non_starter,
                                &mut combining_characters,
                            );
                            continue 'combining_outer;
                        }
                    }
                    let ret = self.pending[0];
                    debug_assert_eq!(self.pending_pos, 0);
                    if self.pending.len() == 1 {
                        self.pending.clear();
                    } else {
                        self.pending_pos = 1;
                    }
                    return ret;
                }
            }
        } else {
            NO_CE
        }
    }

    #[inline(always)]
    fn collect_combining(
        &mut self,
        next_is_known_to_decompose_to_non_starter: &mut bool,
        combining_characters: &mut SmallVec<[CharacterAndClass; 7]>,
    ) {
        while *next_is_known_to_decompose_to_non_starter
            || !self.is_next_decomposition_starts_with_starter()
        {
            *next_is_known_to_decompose_to_non_starter = false;
            // `unwrap` is OK, because `!self.is_next_decomposition_starts_with_starter()`
            // means the `unwrap()` must succeed.
            let combining = self.next_internal().unwrap();
            if !in_inclusive_range(combining, '\u{0340}', '\u{0F81}') {
                combining_characters.push(CharacterAndClass::new(combining));
            } else {
                // The Tibetan special cases are starters that decompose into non-starters.
                //
                // Logically the canonical combining class of each special case is known
                // at compile time, but all characters in the buffer are treated the same
                // when looking up the canonical combining class to avoid a per-character
                // branch that would only benefit these rare special cases.
                match combining {
                    '\u{0340}' => {
                        // COMBINING GRAVE TONE MARK
                        combining_characters.push(CharacterAndClass::new('\u{0300}'));
                    }
                    '\u{0341}' => {
                        // COMBINING ACUTE TONE MARK
                        combining_characters.push(CharacterAndClass::new('\u{0301}'));
                    }
                    '\u{0343}' => {
                        // COMBINING GREEK KORONIS
                        combining_characters.push(CharacterAndClass::new('\u{0313}'));
                    }
                    '\u{0344}' => {
                        // COMBINING GREEK DIALYTIKA TONOS
                        combining_characters.push(CharacterAndClass::new('\u{0308}'));
                        combining_characters.push(CharacterAndClass::new('\u{0301}'));
                    }
                    '\u{0F73}' => {
                        // TIBETAN VOWEL SIGN II
                        combining_characters.push(CharacterAndClass::new('\u{0F71}'));
                        combining_characters.push(CharacterAndClass::new('\u{0F72}'));
                    }
                    '\u{0F75}' => {
                        // TIBETAN VOWEL SIGN UU
                        combining_characters.push(CharacterAndClass::new('\u{0F71}'));
                        combining_characters.push(CharacterAndClass::new('\u{0F74}'));
                    }
                    '\u{0F81}' => {
                        // TIBETAN VOWEL SIGN REVERSED II
                        combining_characters.push(CharacterAndClass::new('\u{0F71}'));
                        combining_characters.push(CharacterAndClass::new('\u{0F80}'));
                    }
                    _ => {
                        combining_characters.push(CharacterAndClass::new(combining));
                    }
                };
            }
        }
        if combining_characters.len() > 1 {
            // This optimizes away the class lookup when len() == 1.
            // Unclear if this micro optimization is worthwhile.
            // In any case, we store the CanonicalCombiningClass in order to
            // avoid having to look it up again when deciding whether to proceed
            // with a discontiguous match. As a side effect, it also means that
            // duplicate lookups aren't needed if the sort below happens to compare
            // an item more than once.
            combining_characters
                .iter_mut()
                .for_each(|cc| cc.set_ccc(self.ccc));
            combining_characters.sort_by_key(|cc| cc.ccc());
        }
    }
}
