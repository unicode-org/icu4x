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
use icu_collections::char16trie::TrieResult;
use icu_collections::codepointtrie::CodePointTrie;
use icu_normalizer::provider::DecompositionData;
use icu_normalizer::provider::DecompositionTables;
use icu_properties::props::CanonicalCombiningClass;
use smallvec::SmallVec;
use zerovec::ule::AsULE;
use zerovec::ule::RawBytesULE;
use zerovec::{zeroslice, ZeroSlice};

use crate::provider::CollationData;

// Start `SmallVec` size constants.
//
// These are the on-stack buffer sizes. If the buffers need
// to grow larger, they are spilled to the heap.
//
// TODO(#2005): Figure out good sizes for these.

/// The number of full 64-bit collation units that get buffered
/// in the primary comparison loop so that they can be examined
/// by the subsequent comparison stregths.
///
/// Note 1: If a primary difference is found, the comparison
/// returns early, so these buffers end up holding all the
/// collation elements only if there is no primary difference.
///
/// Note 2: Unfortunately for now, a sentinel value signaling
/// the end of input gets written into the buffer in addition
/// to the real collation elements.
///
/// This should probably either be halved to 4 on the logic
/// that especially in the presence of the identical prefix
/// optimization, most comparisons return after a couple of
/// primary comparisons or increased to 32 on the logic that
/// such a buffer could better hold a file or human name that
/// differs on secordary or higher level.
pub(crate) const CE_BUFFER_SIZE: usize = 8;

/// The number of extra full 64-bit collation units that have
/// already been computed as part of an operation that yields
/// multiple collation units at a time.
const PENDING_CE_BUFFER_SIZE: usize = 6;

/// Either the identical prefix or the lookahead plus the next
/// upcoming character.
///
/// The longest contraction suffix in CLDR 40 is 7 characters long.
const UPCOMING_CHARACTER_BUFFER_SIZE: usize = 10;

/// The contiguous sequence of combining characters.
const COMBINING_CHARACTER_BUFFER_SIZE: usize = 7;

/// The sequence of digits in the numeric mode.
const DIGIT_BUFFER_SIZE: usize = 8;

/// The number of combining characters that a contraction has
/// matched.
const PENDING_REMOVALS_SIZE: usize = 1;

// End `SmallVec` constants

/// Marker that the decomposition does not round trip via NFC.
///
/// See components/normalizer/trie-value-format.md
pub(crate) const NON_ROUND_TRIP_MARKER: u32 = 1 << 30;

/// Marker that the first character of the decomposition
/// can combine backwards.
///
/// See components/normalizer/trie-value-format.md
pub(crate) const BACKWARD_COMBINING_MARKER: u32 = 1 << 31;

/// Mask for the bits have to be zero for this to be a BMP
/// singleton decomposition, or value baked into the surrogate
/// range.
///
/// See components/normalizer/trie-value-format.md
pub(crate) const HIGH_ZEROS_MASK: u32 = 0x3FFF0000;

/// Mask for the bits have to be zero for this to be a complex
/// decomposition.
///
/// See components/normalizer/trie-value-format.md
pub(crate) const LOW_ZEROS_MASK: u32 = 0xFFE0;

/// Marker value for U+FDFA in NFKD. (Unified with
/// `HANGUL_SYLLABLE_MARKER`, but they differ by
/// `NON_ROUND_TRIP_MARKER`.)
///
/// See components/normalizer/trie-value-format.md
const FDFA_MARKER: u16 = 1;

/// Marker value for Hangul syllables. (Unified with `FDFA_MARKER``,
/// but they differ by `NON_ROUND_TRIP_MARKER`.)
///
/// See components/normalizer/trie-value-format.md
pub(crate) const HANGUL_SYLLABLE_MARKER: u32 = 1;

/// Checks if a trie value carries a (non-zero) canonical
/// combining class.
///
/// See components/normalizer/trie-value-format.md
fn trie_value_has_ccc(trie_value: u32) -> bool {
    (trie_value & 0x3FFFFE00) == 0xD800
}

/// Checks if the trie signifies a special non-starter decomposition.
///
/// See components/normalizer/trie-value-format.md
fn trie_value_indicates_special_non_starter_decomposition(trie_value: u32) -> bool {
    (trie_value & 0x3FFFFF00) == 0xD900
}

/// Checks if a trie value signifies a character whose decomposition
/// starts with a non-starter.
///
/// See components/normalizer/trie-value-format.md
fn decomposition_starts_with_non_starter(trie_value: u32) -> bool {
    trie_value_has_ccc(trie_value)
}

/// Extracts a canonical combining class (possibly zero) from a trie value.
///
/// See components/normalizer/trie-value-format.md
fn ccc_from_trie_value(trie_value: u32) -> CanonicalCombiningClass {
    if trie_value_has_ccc(trie_value) {
        CanonicalCombiningClass::from_icu4c_value(trie_value as u8)
    } else {
        CanonicalCombiningClass::NotReordered
    }
}

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
const OPTIMIZED_DIACRITICS_LIMIT: usize = 0x034F;
pub(crate) const OPTIMIZED_DIACRITICS_MAX_COUNT: usize =
    OPTIMIZED_DIACRITICS_LIMIT - COMBINING_DIACRITICS_BASE;

pub(crate) const CASE_MASK: u16 = 0xC000;
pub(crate) const TERTIARY_MASK: u16 = 0x3F3F; // ONLY_TERTIARY_MASK in ICU4C
pub(crate) const QUATERNARY_MASK: u16 = 0xC0;

// A CE32 is special if its low byte is this or greater.
// Impossible case bits 11 mark special CE32s.
// This value itself is used to indicate a fallback to the root collation.
const SPECIAL_CE32_LOW_BYTE: u8 = 0xC0;
pub(crate) const FALLBACK_CE32: CollationElement32 =
    CollationElement32(SPECIAL_CE32_LOW_BYTE as u32);
const LONG_PRIMARY_CE32_LOW_BYTE: u8 = 0xC1; // SPECIAL_CE32_LOW_BYTE | LONG_PRIMARY_TAG
const COMMON_SECONDARY_CE: u64 = 0x05000000;
const COMMON_TERTIARY_CE: u64 = 0x0500;
const COMMON_SEC_AND_TER_CE: u64 = COMMON_SECONDARY_CE | COMMON_TERTIARY_CE;

const UNASSIGNED_IMPLICIT_BYTE: u8 = 0xFE;

// /// Set if there is no match for the single (no-suffix) character itself.
// /// This is only possible if there is a prefix.
// /// In this case, discontiguous contraction matching cannot add combining marks
// /// starting from an empty suffix.
// /// The default CE32 is used anyway if there is no suffix match.
// const CONTRACT_SINGLE_CP_NO_MATCH: u32 = 0x100;

/// Set if the first character of every contraction suffix has lccc!=0.
const CONTRACT_NEXT_CCC: u32 = 0x200;
/// Set if any contraction suffix ends with lccc!=0.
const CONTRACT_TRAILING_CCC: u32 = 0x400;
/// Set if at least one contraction suffix contains a starter
const CONTRACT_HAS_STARTER: u32 = 0x800;

// const NO_CE32: CollationElement32 = CollationElement32::default();
// constants named NO_CE* : End of input. Only used in runtime code, not stored in data.
pub(crate) const NO_CE: CollationElement = CollationElement::default();
pub(crate) const NO_CE_PRIMARY: u32 = 1; // not a left-adjusted weight
                                         // const NO_CE_NON_PRIMARY: NonPrimary = NonPrimary::default();
pub(crate) const NO_CE_SECONDARY: u16 = 0x0100;
pub(crate) const NO_CE_TERTIARY: u16 = 0x0100;
pub(crate) const NO_CE_QUATERNARY: u16 = 0x0100;
const NO_CE_VALUE: u64 =
    ((NO_CE_PRIMARY as u64) << 32) | ((NO_CE_SECONDARY as u64) << 16) | (NO_CE_TERTIARY as u64); // 0x101000100

// See ICU4C collation.h and https://www.unicode.org/reports/tr10/#Trailing_Weights
const FFFD_PRIMARY: u32 = 0xFFFD0000; // U+FFFD
pub(crate) const FFFD_CE_VALUE: u64 = ((FFFD_PRIMARY as u64) << 32) | COMMON_SEC_AND_TER_CE;
pub(crate) const FFFD_CE: CollationElement = CollationElement(FFFD_CE_VALUE);
pub(crate) const FFFD_CE32_VALUE: u32 = 0xFFFD0505;
pub(crate) const FFFD_CE32: CollationElement32 = CollationElement32(FFFD_CE32_VALUE);

pub(crate) const EMPTY_U16: &ZeroSlice<u16> = zeroslice![];
const SINGLE_REPLACEMENT_CHARACTER_U16: &ZeroSlice<u16> =
    zeroslice!(u16; <u16 as AsULE>::ULE::from_unsigned; [REPLACEMENT_CHARACTER as u16]);

pub(crate) const EMPTY_CHAR: &ZeroSlice<char> = zeroslice![];
const SINGLE_REPLACEMENT_CHARACTER_CHAR: &ZeroSlice<char> =
    zeroslice!(char; <char as AsULE>::ULE::from_aligned; [REPLACEMENT_CHARACTER]);

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
pub(crate) fn char_from_u32(u: u32) -> char {
    unwrap_or_gigo(core::char::from_u32(u), REPLACEMENT_CHARACTER)
}

/// Convert a `u16` _obtained from data provider data_ to `char`.
#[inline(always)]
fn char_from_u16(u: u16) -> char {
    char_from_u32(u32::from(u))
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
#[repr(u8)] // This repr is necessary for transmute safety
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
    ///       9.. 8: =0: All associated supplementary code points are unassigned-implicit.
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
///
/// A `CollationElement32` can be "normal" or "special".
/// Bits 7 and 6 are case bits for the "normal" case and setting
/// both is an impossible case bit combination. Hence, "special"
/// `CollationElement32`s are marked by setting both case bits
/// to 1. This is equivalent with the low byte being less than
/// `SPECIAL_CE32_LOW_BYTE` (0xC0, i.e. 0b11000000) in the "normal"
/// case and equal to or greater in the "special" case.
///
/// For the normal case:
/// Bits: 31..16: Primary weight
/// Bits: 15..8: Secondary weight
/// Bits:  7..6: Case bits (cannot both be 1 simultaneously)
/// Bits:  5..0: The high part of the discontiguous tertiary weight
/// (The quaternary weight and the low part of the discontiguous
/// tertiary weight are zero.)
///
/// For the special case:
/// Bits 31..8: tag-specific; see the documentation for `Tag`.
/// Bits  7..6: The specialness marker; both bits set to 1
/// Bits  5..4: Reserved. May be used in the future to indicate lccc!=0 and tccc!=0.
/// Bits  3..0: the tag (bit-compatible with `Tag`)
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
    fn low_byte(self) -> u8 {
        self.0 as u8
    }

    #[inline(always)]
    pub(crate) fn tag_checked(self) -> Option<Tag> {
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
    pub(crate) fn tag(self) -> Tag {
        debug_assert!(self.low_byte() >= SPECIAL_CE32_LOW_BYTE);
        // Safety: Tag has values 0 to 15, which are filtered for with the 0xF mask.
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
    pub fn len(self) -> usize {
        debug_assert!(self.tag() == Tag::Expansion32 || self.tag() == Tag::Expansion);
        ((self.0 >> 8) & 31) as usize
    }

    /// Gets the index from this element.
    ///
    /// # Panics
    ///
    /// In debug builds if this element doesn't have an index.
    #[inline(always)]
    pub fn index(self) -> usize {
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
    pub fn digit(self) -> u8 {
        debug_assert!(self.tag() == Tag::Digit);
        ((self.0 >> 8) & 0xF) as u8
    }

    #[inline(always)]
    pub fn every_suffix_starts_with_combining(self) -> bool {
        debug_assert!(self.tag() == Tag::Contraction);
        (self.0 & CONTRACT_NEXT_CCC) != 0
    }
    #[inline(always)]
    pub fn at_least_one_suffix_contains_starter(self) -> bool {
        debug_assert!(self.tag() == Tag::Contraction);
        (self.0 & CONTRACT_HAS_STARTER) != 0
    }
    #[inline(always)]
    pub fn at_least_one_suffix_ends_with_non_starter(self) -> bool {
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
    pub fn new_from_secondary(secondary: u16) -> Self {
        CollationElement((u64::from(secondary) << 16) | COMMON_TERTIARY_CE)
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
    pub fn clone_with_non_primary_zeroed(self) -> Self {
        CollationElement(self.0 & 0xFFFFFFFF00000000)
    }

    /// Get the primary weight
    #[inline(always)]
    pub fn primary(self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Get the non-primary weights
    #[inline(always)]
    pub fn non_primary(self) -> NonPrimary {
        NonPrimary::new(self.0 as u32)
    }

    /// Get the secondary weight
    #[inline(always)]
    pub fn secondary(self) -> u16 {
        self.non_primary().secondary()
    }
    #[inline(always)]
    pub fn quaternary(self) -> u32 {
        self.non_primary().quaternary()
    }
    #[inline(always)]
    pub fn tertiary_ignorable(self) -> bool {
        self.non_primary().tertiary_ignorable()
    }
    #[inline(always)]
    pub fn either_half_zero(self) -> bool {
        self.primary() == 0 || (self.0 as u32) == 0
    }

    #[inline(always)]
    pub const fn default() -> CollationElement {
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
    pub fn bits(self) -> u32 {
        self.0
    }
    /// Get the secondary weight
    #[inline(always)]
    pub fn secondary(self) -> u16 {
        (self.0 >> 16) as u16
    }
    /// Get the case bits as the high two bits of a u16
    #[inline(always)]
    pub fn case(self) -> u16 {
        (self.0 as u16) & CASE_MASK
    }
    /// Get the tertiary weight as u16 with the high
    /// two bits of each half zeroed.
    #[inline(always)]
    pub fn tertiary(self) -> u16 {
        (self.0 as u16) & TERTIARY_MASK
    }
    #[inline(always)]
    pub fn tertiary_ignorable(self) -> bool {
        (self.0 as u16) <= NO_CE_TERTIARY
    }
    /// Get the quaternary weight in the original
    /// storage bit positions with the other bits
    /// set to one.
    #[inline(always)]
    pub fn quaternary(self) -> u32 {
        self.0 | !(QUATERNARY_MASK as u32)
    }
    /// Get any combination of tertiary, case, and quaternary
    /// by mask.
    #[inline(always)]
    pub fn tertiary_case_quarternary(self, mask: u16) -> u16 {
        debug_assert!((mask & CASE_MASK) == CASE_MASK || (mask & CASE_MASK) == 0);
        debug_assert!((mask & TERTIARY_MASK) == TERTIARY_MASK || (mask & TERTIARY_MASK) == 0);
        debug_assert!((mask & QUATERNARY_MASK) == QUATERNARY_MASK || (mask & QUATERNARY_MASK) == 0);
        (self.0 as u16) & mask
    }

    #[inline(always)]
    pub fn case_quaternary(self) -> u16 {
        (self.0 as u16) & (CASE_MASK | QUATERNARY_MASK)
    }

    #[inline(always)]
    pub fn ignorable(self) -> bool {
        self.0 == 0
    }
}

impl Default for NonPrimary {
    #[inline(always)]
    fn default() -> Self {
        NonPrimary(0x01000100) // Low 32 bits of NO_CE
    }
}

/// This struct makes the handling of the `upcoming` buffer
/// easily so that trie lookups are done at most once. However,
/// when `upcoming[0]` is an undecomposed starter, we don't
/// need the ccc yet, and when lookahead has already done the
/// trie lookups, we don't need `trie_value`, as it is implied
/// by ccc.
//
// TODO(#2386): This struct carries redundant information, and
// `upcoming` should be split into a buffer of `CharacterAndClass`
//  and an `Option<CharacterAndTrieValue>`, but that refactoring
// isn't 100% necessary, so focusing on data format stability
// for 1.0.
//
// (Deliberately non-`Copy`, because `CharacterAndClass` is
// non-`Copy`.)
#[derive(Debug, Clone)]
pub(crate) struct CharacterAndClassAndTrieValue {
    c_and_c: CharacterAndClass,
    pub trie_val: u32,
}

impl CharacterAndClassAndTrieValue {
    pub fn new_with_non_decomposing_starter(c: char) -> Self {
        CharacterAndClassAndTrieValue {
            c_and_c: CharacterAndClass::new(c, CanonicalCombiningClass::NotReordered),
            trie_val: 0,
        }
    }
    pub fn new_with_non_zero_ccc(c: char, ccc: CanonicalCombiningClass) -> Self {
        CharacterAndClassAndTrieValue {
            c_and_c: CharacterAndClass::new(c, ccc),
            trie_val: 0xD800 | u32::from(ccc.to_icu4c_value()),
        }
    }
    pub fn new_with_non_special_decomposition_trie_val(c: char, trie_val: u32) -> Self {
        debug_assert!(!trie_value_indicates_special_non_starter_decomposition(
            trie_val
        ));
        CharacterAndClassAndTrieValue {
            c_and_c: CharacterAndClass::new_with_trie_value(c, trie_val),
            trie_val,
        }
    }
    pub fn new_with_trie_val(c: char, trie_val: u32) -> Self {
        if !trie_value_indicates_special_non_starter_decomposition(trie_val) {
            CharacterAndClassAndTrieValue {
                c_and_c: CharacterAndClass::new_with_trie_value(c, trie_val),
                trie_val,
            }
        } else {
            CharacterAndClassAndTrieValue {
                c_and_c: CharacterAndClass::new(c, CanonicalCombiningClass::from_icu4c_value(0xFF)),
                trie_val,
            }
        }
    }

    pub fn decomposition_starts_with_non_starter(&self) -> bool {
        decomposition_starts_with_non_starter(self.trie_val)
    }

    pub fn character(&self) -> char {
        self.c_and_c.character()
    }

    fn ccc(&self) -> CanonicalCombiningClass {
        let ret = self.c_and_c.ccc();
        debug_assert_ne!(ret, CanonicalCombiningClass::from_icu4c_value(0xFF));
        ret
    }
}

/// Pack a `char` and a `CanonicalCombiningClass` in
/// 32 bits (the former in the lower 24 bits and the
/// latter in the high 8 bits). The latter can be
/// initialized to 0xFF upon creation, in which case
/// it can be actually set later by calling
/// `set_ccc_from_trie_if_not_already_set`. This is
/// a micro optimization to avoid the Canonical
/// Combining Class trie lookup when there is only
/// one combining character in a sequence. This type
/// is intentionally non-`Copy` to get compiler help
/// in making sure that the class is set on the
/// instance on which it is intended to be set
/// and not on a temporary copy.
///
/// Note that 0xFF is won't be assigned to an actual
/// canonical combining class per definition D104
/// in The Unicode Standard.
//
// NOTE: The Pernosco debugger has special knowledge
// of this struct. Please do not change the bit layout
// or the crate-module-qualified name of this struct
// without coordination.
#[derive(Debug, Clone)]
// Safety invariant: The low 24 bits are a valid char
struct CharacterAndClass(u32);

impl CharacterAndClass {
    pub fn new(c: char, ccc: CanonicalCombiningClass) -> Self {
        // Safety invariant upheld here: the first half is a valid char
        // and the second half does not affect the low 24 bits
        CharacterAndClass(u32::from(c) | (u32::from(ccc.to_icu4c_value()) << 24))
    }
    pub fn new_with_placeholder(c: char) -> Self {
        // Safety invariant upheld here: the first half is a valid char
        // and the second half does not affect the low 24 bits
        CharacterAndClass(u32::from(c) | ((0xFF) << 24))
    }
    pub fn new_with_trie_value(c: char, trie_value: u32) -> Self {
        Self::new(c, ccc_from_trie_value(trie_value))
    }
    pub fn character(&self) -> char {
        // Safety: from the safety invariant, this extracts the low 24 bits
        unsafe { char::from_u32_unchecked(self.0 & 0xFF_FFFF) }
    }
    pub fn ccc(&self) -> CanonicalCombiningClass {
        // Safety invariant upheld here: The argument is outside of the low 24 bits,
        // and \0 is a valid character
        CanonicalCombiningClass::from_icu4c_value((self.0 >> 24) as u8)
    }
    pub fn character_and_ccc(&self) -> (char, CanonicalCombiningClass) {
        (self.character(), self.ccc())
    }
    pub fn set_ccc_from_trie_if_not_already_set(&mut self, trie: &CodePointTrie<u32>) {
        if self.0 >> 24 != 0xFF {
            return;
        }
        let scalar = self.0 & 0xFF_FFFF;
        // Safety invariant upheld here: The first half doesn't affect the lower 24 bits,
        // and the second half was taken from the old `self` which had these invariants upheld already.
        self.0 =
            ((ccc_from_trie_value(trie.get32_u32(scalar)).to_icu4c_value() as u32) << 24) | scalar;
    }
}

/// Iterator that transforms an iterator over `char` into an iterator
/// over `CollationElement` with a tailoring.
/// Not a real Rust iterator: Instead of `None` uses `NO_CE` to indicate
/// end of iteration to optimize comparison.
///
/// It is _extremely_ important for performance that `SmallVec`s not be
/// moved. To facilitate move-avoidance, this struct has the following
/// life cycle where `new` returns the struct in a state that is not
/// yet valid for a `next` call until `init` is called:
///
/// 1. `new`.
/// 2. Some number of calls to `iter_next_before_init` and
///    `prepend_upcoming_before_init`.
/// 3. `init`.
/// 4. Some number of calls to `next`.
pub(crate) struct CollationElements<'data, I>
where
    I: Iterator<Item = char>,
{
    iter: I,
    /// Already computed but not yet returned `CollationElement`s.
    pending: SmallVec<[CollationElement; PENDING_CE_BUFFER_SIZE]>, // TODO(#2005): Figure out good length
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
    /// to `pending_unnormalized_starter` in `icu::normalizer::Decomposition`.
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
    /// first `char` in `upcoming` must have its decomposition start with a
    /// starter.
    ///
    /// TODO: Reverse the order, since now `insert(0, x)` and `remove(0)`
    /// are used more often than `push()` and `pop()`.
    upcoming: SmallVec<[CharacterAndClassAndTrieValue; UPCOMING_CHARACTER_BUFFER_SIZE]>,
    /// The root collation data.
    root: &'data CollationData<'data>,
    /// Tailoring if applicable.
    tailoring: &'data CollationData<'data>,
    /// The `CollationElement32` mapping for the Hangul Jamo block.
    ///
    /// Note: in ICU4C the jamo table contains only modern jamo. Here, the jamo table contains the whole Unicode block.
    jamo: &'data [<u32 as AsULE>::ULE; JAMO_COUNT],
    /// The `CollationElement32` mapping for the Combining Diacritical Marks block.
    diacritics: &'data ZeroSlice<u16>,
    /// NFD main trie.
    ///
    /// See components/normalizer/trie-value-format.md
    trie: &'data CodePointTrie<'data, u32>,
    /// NFD complex decompositions on the BMP
    scalars16: &'data ZeroSlice<u16>,
    /// NFD complex decompositions on supplementary planes
    scalars32: &'data ZeroSlice<char>,
    /// If numeric mode is enabled, the 8 high bits of the numeric primary.
    /// `None` if disabled.
    numeric_primary: Option<u8>,
    /// Whether the Lithuanian combining dot above handling is enabled.
    lithuanian_dot_above: bool,
    /// Whether `upcoming` (except the last item) has been normalized already
    upcoming_normalized: bool,
    #[cfg(debug_assertions)]
    /// Whether `iter` has been exhausted
    iter_exhausted: bool,
    #[cfg(debug_assertions)]
    /// Whether `init` has been called
    initialized: bool,
}

impl<'data, I> CollationElements<'data, I>
where
    I: Iterator<Item = char>,
{
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        delegate: I,
        root: &'data CollationData,
        tailoring: &'data CollationData,
        jamo: &'data [<u32 as AsULE>::ULE; JAMO_COUNT],
        diacritics: &'data ZeroSlice<u16>,
        decompositions: &'data DecompositionData,
        tables: &'data DecompositionTables,
        numeric_primary: Option<u8>,
        lithuanian_dot_above: bool,
    ) -> Self {
        CollationElements::<I> {
            iter: delegate,
            pending: SmallVec::new(),
            pending_pos: 0,
            prefix: ['\u{FFFF}'; 2],
            upcoming: SmallVec::new(),
            root,
            tailoring,
            jamo,
            diacritics,
            trie: &decompositions.trie,
            scalars16: &tables.scalars16,
            scalars32: &tables.scalars24,
            numeric_primary,
            lithuanian_dot_above,
            upcoming_normalized: false,
            #[cfg(debug_assertions)]
            iter_exhausted: false,
            #[cfg(debug_assertions)]
            initialized: false,
        }
    }

    pub fn iter_next_before_init(&mut self) -> Option<CharacterAndClassAndTrieValue> {
        #[cfg(debug_assertions)]
        debug_assert!(!self.initialized);
        self.iter_next()
    }

    pub fn prepend_upcoming_before_init(&mut self, c: CharacterAndClassAndTrieValue) {
        #[cfg(debug_assertions)]
        debug_assert!(!self.initialized);
        self.upcoming.insert(0, c);
    }

    pub fn init(&mut self) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(!self.initialized);
            self.initialized = true;
        }

        // Ensure the last item is a starter (unless)
        // iter exhausted.
        if let Some(last) = self.upcoming.last() {
            if last.decomposition_starts_with_non_starter() {
                // Not using `while let` to be able to set `iter_exhausted`
                loop {
                    if let Some(ch) = self.iter_next() {
                        let starter = !ch.decomposition_starts_with_non_starter();
                        self.upcoming.push(ch);
                        if starter {
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
        }

        let mut starts_with_starter = false;
        if let Some(first) = self.upcoming.first() {
            if !first.decomposition_starts_with_non_starter() {
                starts_with_starter = true;
            }
        }
        if !starts_with_starter {
            self.upcoming.insert(
                0,
                CharacterAndClassAndTrieValue::new_with_non_decomposing_starter('\u{FFFF}'),
            ); // Make sure the process always begins with a starter
            let _ = self.next(); // Remove the placeholder starter
        }
    }

    fn iter_next(&mut self) -> Option<CharacterAndClassAndTrieValue> {
        let c = self.iter.next()?;
        let trie_val = self.trie.get(c);
        Some(CharacterAndClassAndTrieValue::new_with_trie_val(
            c, trie_val,
        ))
    }

    fn next_internal(&mut self) -> Option<CharacterAndClassAndTrieValue> {
        if self.upcoming.is_empty() {
            return None;
        }
        let ret = self.upcoming.remove(0);
        if self.upcoming.is_empty() {
            if let Some(c) = self.iter_next() {
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
        // index has to be in range due to the check above.
        // rewriting with `get()` would result in two checks.
        #[expect(clippy::indexing_slicing)]
        if !self.upcoming[0].decomposition_starts_with_non_starter() {
            return;
        }
        // We now have a single character that decomposes to start with
        // a non-starter. Decompose it and assign the real canonical combining class.
        let first = self.upcoming.remove(0);
        self.push_decomposed_combining(first);
        // Not using `while let` to be able to set `iter_exhausted`
        loop {
            if let Some(ch) = self.iter_next() {
                if ch.decomposition_starts_with_non_starter() {
                    self.push_decomposed_combining(ch);
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

    /// Ensures that `upcoming` is normalized to NFD, except:
    /// 1. When the last item is a starter, it isn't necessarily normalized.
    /// 2. Hangul syllable are unnormalized.
    fn ensure_upcoming_normalized(&mut self) {
        if self.upcoming_normalized {
            return;
        }
        self.upcoming_normalized = true;
        let without_trailing_starter = if let Some((last, head)) = self.upcoming.split_last() {
            if !last.decomposition_starts_with_non_starter() {
                if head.is_empty() {
                    // There is a single starter, which isn't required
                    // to be normalized.
                    return;
                } else {
                    head
                }
            } else {
                &self.upcoming[..]
            }
        } else {
            // Make the assertion conditional to make CI happy.
            #[cfg(debug_assertions)]
            debug_assert!(self.iter_exhausted);
            return;
        };

        // It would be better to attempt to normalize in place, but let's do at
        // least this.
        if without_trailing_starter.iter().all(|c| {
            (c.trie_val
                & !(BACKWARD_COMBINING_MARKER | NON_ROUND_TRIP_MARKER | HANGUL_SYLLABLE_MARKER))
                == 0
        }) {
            return;
        }

        let mut unnormalized = core::mem::take(&mut self.upcoming);
        let last_index = unnormalized.len() - 1;
        // Indexing is for debug assert only.
        #[expect(clippy::indexing_slicing)]
        {
            debug_assert!(!unnormalized[0].decomposition_starts_with_non_starter());
        }
        let mut start_combining = 0;
        for (i, c) in unnormalized.drain(..).enumerate() {
            if c.decomposition_starts_with_non_starter() {
                self.push_decomposed_combining(c);
            } else if i == last_index {
                // Indices are in range by construction, so indexing is OK.
                #[expect(clippy::indexing_slicing)]
                self.upcoming[start_combining..].sort_by_key(|c| c.ccc());
                self.upcoming.push(c);
                return;
            } else {
                // Indices are in range by construction, so indexing is OK.
                #[expect(clippy::indexing_slicing)]
                self.upcoming[start_combining..].sort_by_key(|c| c.ccc());
                start_combining = self.push_decomposed_starter(c);
            }
        }
        // Make the assertion conditional to make CI happy.
        #[cfg(debug_assertions)]
        debug_assert!(self.iter_exhausted);
        // Indices are in range by construction, so indexing is OK.
        #[expect(clippy::indexing_slicing)]
        self.upcoming[start_combining..].sort_by_key(|c| c.ccc());
    }

    fn push_decomposed_combining(&mut self, c: CharacterAndClassAndTrieValue) {
        if !trie_value_indicates_special_non_starter_decomposition(c.trie_val) {
            debug_assert!(trie_value_has_ccc(c.trie_val));
            self.upcoming.push(c);
            return;
        }

        // The Tibetan special cases are starters that decompose into non-starters.
        match c.character() {
            '\u{0340}' => {
                // COMBINING GRAVE TONE MARK
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0300}',
                        CanonicalCombiningClass::Above,
                    ));
            }
            '\u{0341}' => {
                // COMBINING ACUTE TONE MARK
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0301}',
                        CanonicalCombiningClass::Above,
                    ));
            }
            '\u{0343}' => {
                // COMBINING GREEK KORONIS
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0313}',
                        CanonicalCombiningClass::Above,
                    ));
            }
            '\u{0344}' => {
                // COMBINING GREEK DIALYTIKA TONOS
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0308}',
                        CanonicalCombiningClass::Above,
                    ));
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0301}',
                        CanonicalCombiningClass::Above,
                    ));
            }
            '\u{0F73}' => {
                // TIBETAN VOWEL SIGN II
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F71}',
                        CanonicalCombiningClass::CCC129,
                    ));
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F72}',
                        CanonicalCombiningClass::CCC130,
                    ));
            }
            '\u{0F75}' => {
                // TIBETAN VOWEL SIGN UU
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F71}',
                        CanonicalCombiningClass::CCC129,
                    ));
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F74}',
                        CanonicalCombiningClass::CCC132,
                    ));
            }
            '\u{0F81}' => {
                // TIBETAN VOWEL SIGN REVERSED II
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F71}',
                        CanonicalCombiningClass::CCC129,
                    ));
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_zero_ccc(
                        '\u{0F80}',
                        CanonicalCombiningClass::CCC130,
                    ));
            }
            _ => {
                // GIGO case
                debug_assert!(false);
            }
        }
    }

    fn push_decomposed_starter(&mut self, c: CharacterAndClassAndTrieValue) -> usize {
        let mut search_start_combining = false;
        let old_len = self.upcoming.len();
        // Not inserting early returns below to keep the same structure
        // as in the ce32 mapping code.

        // Hangul syllable check omitted, because it's fine not to decompose
        // Hangul syllables in lookahead, because Hangul isn't allowed to
        // participate in contractions, and the trie default is that a character
        // is its own decomposition.

        // See components/normalizer/trie-value-format.md
        let decomposition = c.trie_val;
        if (decomposition & !(BACKWARD_COMBINING_MARKER | NON_ROUND_TRIP_MARKER))
            <= HANGUL_SYLLABLE_MARKER
        {
            // The character is its own decomposition (or Hangul syllable)
            // Set the Canonical Combining Class to zero
            self.upcoming.push(
                CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(c.character()),
            );
        } else {
            let high_zeros = (decomposition & HIGH_ZEROS_MASK) == 0;
            let low_zeros = (decomposition & LOW_ZEROS_MASK) == 0;
            if !high_zeros && !low_zeros {
                // Decomposition into two BMP characters: starter and non-starter
                let starter = char_from_u32(decomposition & 0x7FFF);
                let low_c = char_from_u32((decomposition >> 15) & 0x7FFF);
                self.upcoming
                    .push(CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(starter));
                let trie_value = self.trie.get(low_c);
                self.upcoming.push(
                    CharacterAndClassAndTrieValue::new_with_non_special_decomposition_trie_val(
                        low_c, trie_value,
                    ),
                );
            } else if high_zeros {
                let singleton = decomposition as u16;
                debug_assert_ne!(
                    singleton, FDFA_MARKER,
                    "How come U+FDFA NFKD marker seen in NFD?"
                );
                if (singleton & 0xFF00) == 0xD800 {
                    // We're at the end of the stream, so we aren't dealing with the
                    // next undecomposed starter but are dealing with an
                    // already-decomposed non-starter. Just put it back.
                    self.upcoming.push(c);
                    // Make the assertion conditional to make CI happy.
                    #[cfg(debug_assertions)]
                    debug_assert!(self.iter_exhausted);
                } else {
                    // Decomposition into one BMP character
                    self.upcoming.push(
                        CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(
                            char_from_u16(singleton),
                        ),
                    );
                }
            } else {
                debug_assert!(low_zeros);
                // Only 12 of 14 bits used as of Unicode 16.
                let offset = (((decomposition & !(0b11 << 30)) >> 16) as usize) - 1;
                // Only 3 of 4 bits used as of Unicode 16.
                let len_bits = decomposition & 0b1111;
                let only_non_starters_in_trail = (decomposition & 0b10000) != 0;
                if offset < self.scalars16.len() {
                    let len = (len_bits + 2) as usize;
                    for u in unwrap_or_gigo(
                        self.scalars16.get_subslice(offset..offset + len),
                        SINGLE_REPLACEMENT_CHARACTER_U16, // single instead of empty for consistency with the other code path
                    )
                    .iter()
                    {
                        let ch = char_from_u16(u);
                        let trie_value = self.trie.get(ch);
                        self.upcoming
                            .push(CharacterAndClassAndTrieValue::new_with_non_special_decomposition_trie_val(ch, trie_value));
                    }
                } else {
                    let len = (len_bits + 1) as usize;
                    let offset32 = offset - self.scalars16.len();
                    for ch in unwrap_or_gigo(
                        self.scalars32.get_subslice(offset32..offset32 + len),
                        SINGLE_REPLACEMENT_CHARACTER_CHAR, // single instead of empty for consistency with the other code path
                    )
                    .iter()
                    {
                        let trie_value = self.trie.get(ch);
                        self.upcoming
                            .push(CharacterAndClassAndTrieValue::new_with_non_special_decomposition_trie_val(ch, trie_value));
                    }
                }
                search_start_combining = !only_non_starters_in_trail;
            }
        }
        if search_start_combining {
            // The decomposition contains starters. As of Unicode 14,
            // There are two possible patterns:
            // BMP: starter, starter, non-starter
            // Plane 1: starter, starter.
            // However, for forward compatibility, support any combination
            // and search for the last starter.
            let mut i = self.upcoming.len() - 1;
            loop {
                if let Some(ch) = self.upcoming.get(i) {
                    if ch.decomposition_starts_with_non_starter() {
                        i -= 1;
                        continue;
                    }
                    break;
                }
                // GIGO case
                debug_assert!(false);
                // This will wrap to zero below
                i = usize::MAX;
                break;
            }
            i + 1
        } else {
            old_len + 1
        }
    }

    // Decomposes `c`, pushes it to `self.upcoming` (unless the character is
    // a Hangul syllable; Hangul isn't allowed to participate in contractions),
    // gathers the following combining characters from `self.iter` and the following starter.
    // Sorts the combining characters and leaves the starter at the end
    // unnormalized. The trailing unnormalized starter doesn't get appended if
    // `self.iter` is exhausted.
    fn push_decomposed_and_gather_combining(&mut self, c: CharacterAndClassAndTrieValue) {
        let start_combining = self.push_decomposed_starter(c);
        // Not using `while let` to be able to set `iter_exhausted`
        loop {
            if let Some(ch) = self.iter_next() {
                if ch.decomposition_starts_with_non_starter() {
                    self.push_decomposed_combining(ch);
                } else {
                    // Got a new starter
                    // Indices are in range by construction, so indexing is OK.
                    #[expect(clippy::indexing_slicing)]
                    self.upcoming[start_combining..].sort_by_key(|c| c.ccc());
                    self.upcoming.push(ch);
                    return;
                }
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
                // Indices are in range by construction, so indexing is OK.
                #[expect(clippy::indexing_slicing)]
                self.upcoming[start_combining..].sort_by_key(|c| c.ccc());
                return;
            }
        }
    }

    // Assumption: `pos` starts from zero and increases one by one.
    // Indexing is OK, because we check against `len()` and the `pos`
    // increases one by one by construction.
    #[expect(clippy::indexing_slicing)]
    fn look_ahead(&mut self, pos: usize) -> Option<CharacterAndClassAndTrieValue> {
        debug_assert!(self.upcoming_normalized);
        if pos + 1 == self.upcoming.len() {
            let c = self.upcoming.remove(pos);
            self.push_decomposed_and_gather_combining(c);
            Some(self.upcoming[pos].clone())
        } else if pos == self.upcoming.len() {
            if let Some(c) = self.iter_next() {
                debug_assert!(
                    false,
                    "The `upcoming` queue should be empty when iteration `pos` at the end"
                );
                self.push_decomposed_and_gather_combining(c);
                Some(self.upcoming[pos].clone())
            } else {
                #[cfg(debug_assertions)]
                {
                    self.iter_exhausted = true;
                }
                None
            }
        } else {
            Some(self.upcoming[pos].clone())
        }
    }

    fn is_next_decomposition_starts_with_starter(&self) -> bool {
        if let Some(c_c_tv) = self.upcoming.first() {
            !c_c_tv.decomposition_starts_with_non_starter()
        } else {
            true
        }
    }

    fn prepend_and_sort_non_starter_prefix_of_suffix(&mut self, c: CharacterAndClassAndTrieValue) {
        // Add one for the insertion afterwards.
        let end = 1 + {
            let mut iter = self.upcoming.iter().enumerate();
            loop {
                if let Some((i, ch)) = iter.next() {
                    if !ch.decomposition_starts_with_non_starter() {
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
        let start = c.decomposition_starts_with_non_starter() as usize;
        self.upcoming.insert(0, c);
        // Indices in range by construction
        #[expect(clippy::indexing_slicing)]
        {
            let slice: &mut [CharacterAndClassAndTrieValue] = &mut self.upcoming[start..end];
            slice.sort_by_key(|cc| cc.ccc());
        };
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
        #[cfg(debug_assertions)]
        debug_assert!(self.initialized);
        debug_assert!(self.is_next_decomposition_starts_with_starter());
        if let Some(&ret) = self.pending.get(self.pending_pos) {
            self.pending_pos += 1;
            if self.pending_pos == self.pending.len() {
                self.pending.clear();
                self.pending_pos = 0;
            }
            return ret;
        }
        debug_assert_eq!(self.pending_pos, 0);
        if let Some(c_c_tv) = self.next_internal() {
            let mut c = c_c_tv.character();
            let mut ce32;
            let mut data: &CollationData = self.tailoring;
            let mut combining_characters: SmallVec<
                [CharacterAndClass; COMBINING_CHARACTER_BUFFER_SIZE],
            > = SmallVec::new(); // TODO(#2005): Figure out good length

            // Betting that fusing the NFD algorithm into this one at the
            // expense of the repetitiveness below, the common cases become
            // fast in a way that offsets the lack of the canonical closure.
            // The wall of code before the "Slow path" is an attempt to
            // optimize based on that bet.
            let hangul_offset = u32::from(c).wrapping_sub(HANGUL_S_BASE); // SIndex in the spec
            if hangul_offset >= HANGUL_S_COUNT {
                // See components/normalizer/trie-value-format.md
                let decomposition = c_c_tv.trie_val;
                if (decomposition & !(BACKWARD_COMBINING_MARKER | NON_ROUND_TRIP_MARKER)) == 0 {
                    // The character is its own decomposition
                    let jamo_index = (c as usize).wrapping_sub(HANGUL_L_BASE as usize);
                    // Attribute belongs on an inner expression, but
                    // https://github.com/rust-lang/rust/issues/15701
                    #[expect(clippy::indexing_slicing)]
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
                        // TODO(#1941): This isn't actually true with the current jamo
                        // search expansions!

                        // TODO(#1941): Instead of having different jamo CE32 table for
                        // "search" collations, we could instead decompose the archaic
                        // jamo to the modern approximation sequences here and then map
                        // those by looking up the modern jamo from the normal root.

                        // We need to set data to root, because archaic jamo refer to
                        // the root.
                        data = self.root;
                        // Index in range by construction above. Not using `get` with
                        // `if let` in order to put the likely branch first.
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
                        // TODO(2003): Figure out if it would be an optimization to
                        // handle `Implicit` and `Offset` tags here.
                    }
                } else {
                    let high_zeros = (decomposition & HIGH_ZEROS_MASK) == 0;
                    let low_zeros = (decomposition & LOW_ZEROS_MASK) == 0;
                    if !high_zeros && !low_zeros {
                        // Decomposition into two BMP characters: starter and non-starter
                        c = char_from_u32(decomposition & 0x7FFF);
                        ce32 = data.ce32_for_char(c);
                        if ce32 == FALLBACK_CE32 {
                            data = self.root;
                            ce32 = data.ce32_for_char(c);
                        }
                        let combining = char_from_u32((decomposition >> 15) & 0x7FFF);
                        if self.is_next_decomposition_starts_with_starter() {
                            let diacritic_index =
                                (combining as usize).wrapping_sub(COMBINING_DIACRITICS_BASE);
                            if let Some(secondary) = self.diacritics.get(diacritic_index) {
                                debug_assert_ne!(combining, '\u{0344}', "Should never have COMBINING GREEK DIALYTIKA TONOS here, since it should have decomposed further.");
                                if let Some(ce) = ce32.to_ce_simple_or_long_primary() {
                                    let ce_for_combining =
                                        CollationElement::new_from_secondary(secondary);
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
                                                let ce_for_combining =
                                                    CollationElement::new_from_secondary(secondary);
                                                self.pending.push(ce_for_combining);
                                                self.mark_prefix_unmatchable();
                                                return ce;
                                            }
                                        }
                                        TrieResult::Intermediate(trie_ce32) => {
                                            if !ce32.at_least_one_suffix_contains_starter() {
                                                if let Some(ce) =
                                                    CollationElement32::new(trie_ce32 as u32)
                                                        .to_ce_simple_or_long_primary()
                                                {
                                                    self.mark_prefix_unmatchable();
                                                    return ce;
                                                }
                                            }
                                        }
                                        TrieResult::FinalValue(trie_ce32) => {
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
                        }
                        combining_characters
                            .push(CharacterAndClass::new_with_placeholder(combining));
                    } else if high_zeros {
                        let singleton = decomposition as u16;
                        debug_assert_ne!(
                            singleton, FDFA_MARKER,
                            "How come U+FDFA NFKD marker seen in NFD?"
                        );
                        // Decomposition into one BMP character
                        c = char_from_u16(singleton);
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
                        }
                    } else {
                        debug_assert!(low_zeros);
                        // Only 12 of 14 bits used as of Unicode 16.
                        let offset = (((decomposition & !(0b11 << 30)) >> 16) as usize) - 1;
                        // Only 3 of 4 bits used as of Unicode 16.
                        let len_bits = decomposition & 0b1111;
                        let only_non_starters_in_trail = (decomposition & 0b10000) != 0;
                        if offset < self.scalars16.len() {
                            let len = (len_bits + 2) as usize;
                            let (starter, tail) = self
                                .scalars16
                                .get_subslice(offset..offset + len)
                                .and_then(ZeroSlice::split_first)
                                .map_or_else(
                                    || {
                                        // GIGO case
                                        debug_assert!(false);
                                        (REPLACEMENT_CHARACTER, EMPTY_U16)
                                    },
                                    |(first, tail)| (char_from_u16(first), tail),
                                );
                            c = starter;
                            if only_non_starters_in_trail {
                                for u in tail.iter() {
                                    let char_from_u = char_from_u16(u);
                                    let trie_value = self.trie.get(char_from_u);
                                    let ccc = ccc_from_trie_value(trie_value);
                                    combining_characters
                                        .push(CharacterAndClass::new(char_from_u, ccc));
                                }
                            } else {
                                let mut it = tail.iter();
                                while let Some(u) = it.next() {
                                    let ch = char_from_u16(u);
                                    let ccc = ccc_from_trie_value(self.trie.get(ch));
                                    if ccc != CanonicalCombiningClass::NotReordered {
                                        // As of Unicode 14, this branch is never taken.
                                        // It exist for forward compatibility.
                                        combining_characters.push(CharacterAndClass::new(ch, ccc));
                                        continue;
                                    }

                                    // At this point, we might have a single newly-read
                                    // combining character in self.upcoming. In that case, we
                                    // need to buffer up the upcoming combining characters, too,
                                    // in order to make `prepend_and_sort_non_starter_prefix_of_suffix`
                                    // sort the right characters.
                                    self.maybe_gather_combining();

                                    while let Some(u) = it.next_back() {
                                        let tail_char = char_from_u16(u);
                                        let trie_value = self.trie.get(tail_char);
                                        self.prepend_and_sort_non_starter_prefix_of_suffix(CharacterAndClassAndTrieValue::new_with_non_special_decomposition_trie_val(tail_char, trie_value));
                                    }
                                    self.prepend_and_sort_non_starter_prefix_of_suffix(CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(ch));
                                    break;
                                }
                            }
                        } else {
                            let len = (len_bits + 1) as usize;
                            let offset32 = offset - self.scalars16.len();
                            let (starter, tail) = self
                                .scalars32
                                .get_subslice(offset32..offset32 + len)
                                .and_then(|slice| slice.split_first())
                                .unwrap_or_else(|| {
                                    // GIGO case
                                    debug_assert!(false);
                                    (REPLACEMENT_CHARACTER, EMPTY_CHAR)
                                });

                            c = starter;
                            if only_non_starters_in_trail {
                                for ch in tail.iter() {
                                    let trie_value = self.trie.get(ch);
                                    let ccc = ccc_from_trie_value(trie_value);
                                    combining_characters.push(CharacterAndClass::new(ch, ccc));
                                }
                            } else {
                                let mut it = tail.iter();
                                while let Some(ch) = it.next() {
                                    let ccc = ccc_from_trie_value(self.trie.get(ch));
                                    if ccc != CanonicalCombiningClass::NotReordered {
                                        // As of Unicode 14, this branch is never taken.
                                        // It exist for forward compatibility.
                                        combining_characters.push(CharacterAndClass::new(ch, ccc));
                                        continue;
                                    }
                                    // At this point, we might have a single newly-read
                                    // combining character in self.upcoming. In that case, we
                                    // need to buffer up the upcoming combining characters, too,
                                    // in order to make `prepend_and_sort_non_starter_prefix_of_suffix`
                                    // sort the right characters.
                                    self.maybe_gather_combining();

                                    while let Some(tail_char) = it.next_back() {
                                        let trie_value = self.trie.get(tail_char);
                                        self.prepend_and_sort_non_starter_prefix_of_suffix(CharacterAndClassAndTrieValue::new_with_non_special_decomposition_trie_val(tail_char, trie_value));
                                    }
                                    self.prepend_and_sort_non_starter_prefix_of_suffix(CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(ch));
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
                // Indexing OK, because indices in range by construction
                #[expect(clippy::indexing_slicing)]
                if self.is_next_decomposition_starts_with_starter() {
                    // TODO(#1941): Assuming self-contained CE32s is OK for the root,
                    // but not currently OK for search collation, which at this time
                    // do not support tailored Hangul.
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
                // Indexing OK, because indices in range by construction
                #[expect(clippy::indexing_slicing)]
                if t != 0 {
                    self.pending.push(
                        CollationElement32::new_from_ule(
                            self.jamo[(HANGUL_V_BASE - HANGUL_L_BASE + v) as usize],
                        )
                        .to_ce_self_contained_or_gigo(),
                    );
                    self.upcoming.insert(
                        0,
                        // Safety: HANGUL_T_BASE is 0x11A7, t is < HANGUL_T_COUNT = 28, so this is definitely
                        // in range for a char (≤ 0xD800)
                        CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(unsafe {
                            core::char::from_u32_unchecked(HANGUL_T_BASE + t)
                        }),
                    );
                } else {
                    self.upcoming.insert(
                        0,
                        // Safety: HANGUL_V_BASE is 0x1161, v is < HANGUL_N_COUNT = 588, so this is definitely
                        // in range for a char (≤ 0xD800)
                        CharacterAndClassAndTrieValue::new_with_non_decomposing_starter(unsafe {
                            core::char::from_u32_unchecked(HANGUL_V_BASE + v)
                        }),
                    );
                }

                // Indexing OK, because indices in range by construction
                #[expect(clippy::indexing_slicing)]
                return CollationElement32::new_from_ule(self.jamo[l as usize])
                    .to_ce_self_contained_or_gigo();
            }
            let mut may_have_contracted_starter = false;
            // Slow path
            self.collect_combining(&mut combining_characters);
            // Now:
            // c is the starter character
            // ce32 is the CollationElement32 for the starter
            // combining_characters contains all the combining characters before
            // the next starter sorted by combining class.
            let mut looked_ahead = 0;
            let mut drain_from_upcoming = 0;
            'outer: loop {
                'ce32loop: loop {
                    // TODO(#2002): Ensure that the CE32 flavors in this loop are checked in the optimal
                    // order given their frequency in real workloads.
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
                                // TODO(#2001): Pending removals will in practice be small numbers.
                                // What if we made the item smaller than usize?
                                let mut pending_removals: SmallVec<[usize; PENDING_REMOVALS_SIZE]> =
                                    SmallVec::new();
                                while let Some((character, ccc)) =
                                    combining_characters.get(i).map(|c| c.character_and_ccc())
                                {
                                    match (most_recent_skipped_ccc < ccc, trie.next(character)) {
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
                                // `CodePointInversionList` check in the common case.
                                may_have_contracted_starter = true;
                                debug_assert!(pending_removals.is_empty());
                                self.ensure_upcoming_normalized();
                                loop {
                                    let ahead = self.look_ahead(looked_ahead);
                                    looked_ahead += 1;
                                    if let Some(ch) = ahead {
                                        match trie.next(ch.character()) {
                                            TrieResult::NoValue => {}
                                            TrieResult::NoMatch => {
                                                if !at_least_one_suffix_ends_with_non_starter {
                                                    continue 'ce32loop;
                                                }
                                                if !ch.decomposition_starts_with_non_starter() {
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
                                                most_recent_skipped_ccc = ch.ccc();
                                                self.ensure_upcoming_normalized();
                                                loop {
                                                    let ahead = self.look_ahead(looked_ahead + i);
                                                    if let Some(ch) = ahead {
                                                        let ccc = ch.ccc();
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
                                                            trie.next(ch.character()),
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
                                                drain_from_upcoming = looked_ahead;
                                                ce32 = CollationElement32::new(ce32_i as u32);
                                            }
                                            TrieResult::FinalValue(ce32_i) => {
                                                drain_from_upcoming = looked_ahead;
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
                                    let mut digits: SmallVec<[u8; DIGIT_BUFFER_SIZE]> =
                                        SmallVec::new(); // TODO(#2005): Figure out good length
                                    digits.push(ce32.digit());
                                    let numeric_primary = u32::from(high_bits) << 24;
                                    if combining_characters.is_empty() {
                                        // Numeric collation doesn't work with combining
                                        // characters applied to the digits.
                                        // It's unclear if reading from the tailoring first
                                        // is needed for practical purposes, since it doesn't
                                        // make much sense to tailor the numeric value of digits.
                                        // Performing the usual fallback pattern anyway just in
                                        // case.
                                        may_have_contracted_starter = true;
                                        self.ensure_upcoming_normalized();
                                        while let Some(upcoming) = self.look_ahead(looked_ahead) {
                                            looked_ahead += 1;
                                            ce32 =
                                                self.tailoring.ce32_for_char(upcoming.character());
                                            if ce32 == FALLBACK_CE32 {
                                                ce32 =
                                                    self.root.ce32_for_char(upcoming.character());
                                            }
                                            if ce32.tag_checked() != Some(Tag::Digit) {
                                                break;
                                            }
                                            drain_from_upcoming = looked_ahead;
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
                                    // Index in range by construction above
                                    #[expect(clippy::indexing_slicing)]
                                    let mut remaining = &digits[zeros..];
                                    while !remaining.is_empty() {
                                        // Numeric CEs are generated for segments of
                                        // up to 254 digits.
                                        let (head, tail) = remaining
                                            .split_at_checked(254)
                                            .unwrap_or((remaining, b""));
                                        remaining = tail;
                                        // From ICU4C CollationIterator::appendNumericSegmentCEs
                                        if head.len() <= 7 {
                                            let mut digit_iter = head.iter();
                                            // `unwrap` succeeds, because we always have at least one
                                            // digit to even start numeric processing.
                                            #[expect(clippy::unwrap_used)]
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
                                        let num_pairs = (len as u32).div_ceil(2); // as u32 OK, because capped to 254
                                        let mut primary =
                                            numeric_primary | ((132 - 4 + num_pairs) << 16);
                                        // Find the length without trailing 00 pairs.
                                        //
                                        // The indexing below is within bounds due to the following:
                                        //
                                        // * We skipped leading zeros.
                                        // * If `len == 2`: The loop condition is false, because
                                        //   `head[len - 2]` isn't a leading zero.
                                        // * If `len == 1`: The loop condition is false, because
                                        //   `head[len - 1]` isn't a leading zero, and `&&`
                                        //   short-circuits, so the `head[len - 2]` access doesn't
                                        //   occur.
                                        #[expect(clippy::indexing_slicing)]
                                        while head[len - 1] == 0 && head[len - 2] == 0 {
                                            len -= 2;
                                        }
                                        // Read the first pair
                                        // Index in bounds by construction above.
                                        #[expect(clippy::indexing_slicing)]
                                        let mut digit_iter = head[..len].iter();
                                        // `unwrap` succeeds by construction
                                        #[expect(clippy::unwrap_used)]
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
                    debug_assert!(drain_from_upcoming == 0 || combining_characters.is_empty());
                    let mut i = 0;
                    'combining: while let Some(ch) =
                        combining_characters.get(i).map(|c| c.character())
                    {
                        c = ch;
                        let diacritic_index = (c as usize).wrapping_sub(COMBINING_DIACRITICS_BASE);
                        if let Some(secondary) = self.diacritics.get(diacritic_index) {
                            // TODO(#2006): unlikely annotation
                            if c == '\u{0307}' && self.lithuanian_dot_above {
                                if let Some(next_c) =
                                    combining_characters.get(i + 1).map(|c| c.character())
                                {
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
                            }
                            self.pending
                                .push(CollationElement::new_from_secondary(secondary));
                            self.mark_prefix_unmatchable();
                            i += 1;
                            continue 'combining;
                        }
                        // `c` is not a table-optimized diacritic.
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
                    // Note: The borrow checker didn't like the iterator formulation
                    // for the loop below, because the `Drain` would have kept `self`
                    // mutable borrowed when trying to call `prefix_push`. To change
                    // this, `prefix` and `prefix_push` would need to be refactored
                    // into a struct.
                    i = 0;
                    while i < drain_from_upcoming {
                        // By construction, `drain_from_upcoming` doesn't exceed `upcoming.len()`
                        #[expect(clippy::indexing_slicing)]
                        let ch = self.upcoming[i].character();
                        self.prefix_push(ch);
                        i += 1;
                    }
                    // TODO(#2004): The above makes prefix out of sync when starter-contracting
                    // contractions use `pending_removals` instead of `drain_from_upcoming`.
                    // Do there exist prefixes that overlap contraction suffixes?
                    // At least as of CLDR 40, the two possible non-starters in prefixes,
                    // kana voicing marks, shouldn't be participating in Brahmic contractions.
                    let _ = self.upcoming.drain(..drain_from_upcoming);
                    if self.upcoming.is_empty() {
                        // Make the assertion conditional to make CI happy.
                        #[cfg(debug_assertions)]
                        debug_assert!(self.iter_exhausted || may_have_contracted_starter);
                        if let Some(c_c_tv) = self.iter_next() {
                            self.upcoming.push(c_c_tv);
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
                            drain_from_upcoming = 0;
                            self.collect_combining(&mut combining_characters);
                            continue 'combining_outer;
                        }
                    }
                    // By construction, we have at least on pending CE by now.
                    #[expect(clippy::indexing_slicing)]
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
        combining_characters: &mut SmallVec<[CharacterAndClass; COMBINING_CHARACTER_BUFFER_SIZE]>,
    ) {
        while !self.is_next_decomposition_starts_with_starter() {
            // `unwrap` is OK, because `!self.is_next_decomposition_starts_with_starter()`
            // means the `unwrap()` must succeed.
            #[expect(clippy::unwrap_used)]
            let combining = self.next_internal().unwrap().c_and_c;
            let combining_c = combining.character();
            if !in_inclusive_range(combining_c, '\u{0340}', '\u{0F81}') {
                combining_characters.push(combining);
            } else {
                // The Tibetan special cases are starters that decompose into non-starters.
                match combining_c {
                    '\u{0340}' => {
                        // COMBINING GRAVE TONE MARK
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0300}',
                            CanonicalCombiningClass::Above,
                        ));
                    }
                    '\u{0341}' => {
                        // COMBINING ACUTE TONE MARK
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0301}',
                            CanonicalCombiningClass::Above,
                        ));
                    }
                    '\u{0343}' => {
                        // COMBINING GREEK KORONIS
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0313}',
                            CanonicalCombiningClass::Above,
                        ));
                    }
                    '\u{0344}' => {
                        // COMBINING GREEK DIALYTIKA TONOS
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0308}',
                            CanonicalCombiningClass::Above,
                        ));
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0301}',
                            CanonicalCombiningClass::Above,
                        ));
                    }
                    '\u{0F73}' => {
                        // TIBETAN VOWEL SIGN II
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F71}',
                            CanonicalCombiningClass::CCC129,
                        ));
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F72}',
                            CanonicalCombiningClass::CCC130,
                        ));
                    }
                    '\u{0F75}' => {
                        // TIBETAN VOWEL SIGN UU
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F71}',
                            CanonicalCombiningClass::CCC129,
                        ));
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F74}',
                            CanonicalCombiningClass::CCC132,
                        ));
                    }
                    '\u{0F81}' => {
                        // TIBETAN VOWEL SIGN REVERSED II
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F71}',
                            CanonicalCombiningClass::CCC129,
                        ));
                        combining_characters.push(CharacterAndClass::new(
                            '\u{0F80}',
                            CanonicalCombiningClass::CCC130,
                        ));
                    }
                    _ => {
                        combining_characters.push(combining);
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
                .for_each(|cc| cc.set_ccc_from_trie_if_not_already_set(self.trie));
            combining_characters.sort_by_key(|cc| cc.ccc());
        }
    }
}
