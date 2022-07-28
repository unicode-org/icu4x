// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        // TODO(#1668): enable clippy::exhaustive_structs,
        // TODO(#1668): enable clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

//! Normalizing text into Unicode Normalization Forms.
//!
//! This module is published as its own crate ([`icu_normalizer`](https://docs.rs/icu_normalizer/latest/icu_normalizer/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! # Implementation notes
//!
//! The normalizer operates on a lazy iterator over Unicode scalar values (Rust `char`) internally
//! and iterating over guaranteed-valid UTF-8, potentially-invalid UTF-8, and potentially-invalid
//! UTF-16 is a step that doesn’t leak into the normalizer internals. UTF errors are treated as
//! U+FFFD.
//!
//! The normalizer data layout is not based on the ICU4C design at all. Instead, the normalization
//! data layout is a clean-slate design optimized for the concept of fusing the NFD decomposition
//! into the collator. That is, the decomposing normalizer is a by-product of the collator-motivated
//! data layout.
//!
//! Notably, the decomposition data structure is optimized for a starter decomposing to itself,
//! which is the most common case, and for a starter decomposing to a starter and a non-starter
//! on the Basic Multilingual Plane. Notably, in this case, the collator makes use of the
//! knowledge that the second character of such a decomposition is a non-starter. Therefore,
//! decomposition into two starters is handled by generic fallback path that looks the
//! decomposion from an array by offset and length instead of baking a BMP starter pair directly
//! into a trie value.
//!
//! The decompositions into non-starters are hard-coded. At present in Unicode, these appear
//! to be special cases falling into three categories:
//!
//! 1. Deprecated combining marks.
//! 2. Particular Tibetan vowel sings.
//! 3. NFKD only: half-width kana voicing marks.
//!
//! Hopefully Unicode never adds more decompositions into non-starters (other than a character
//! decomposing to itself), but if it does, a code update is needed instead of a mere data update.
//!
//! The composing normalizer builds on the decomposing normalizer by performing the canonical
//! composition post-processing per spec. As an optimization, though, the composing normalizer
//! attempts to pass through already-normalized text consisting of starters that never combine
//! backwards and that map to themselves if followed by a character whose decomposition starts
//! with a starter that never combines backwards.
//!
//! As a difference with ICU4C, the composing normalizer has only the simplest possible
//! passthrough (only one inversion list lookup per character in the best case) and the full
//! decompose-then-canonically-compose behavior, whereas ICU4C has other paths between these
//! extremes. The ICU4X collator doesn't make use of the FCD concept at all in order to avoid
//! doing the work of checking whether the FCD condition holds.

extern crate alloc;

pub mod error;
pub mod provider;
pub mod u24;

use crate::error::NormalizerError;
use crate::provider::CanonicalDecompositionDataV1Marker;
use crate::provider::CompatibilityDecompositionSupplementV1Marker;
use crate::provider::DecompositionDataV1;
#[cfg(any(test, feature = "experimental"))]
use crate::provider::Uts46DecompositionSupplementV1Marker;
use alloc::string::String;
use alloc::vec::Vec;
use core::char::REPLACEMENT_CHARACTER;
use icu_char16trie::char16trie::Char16Trie;
use icu_char16trie::char16trie::Char16TrieIterator;
use icu_char16trie::char16trie::TrieResult;
use icu_codepointtrie::CodePointTrie;
use icu_properties::maps::{CodePointMapData, CodePointMapDataBorrowed};
use icu_properties::CanonicalCombiningClass;
use icu_provider::prelude::*;
use provider::CanonicalCompositionPassthroughV1Marker;
use provider::CanonicalCompositionsV1Marker;
use provider::CanonicalDecompositionTablesV1Marker;
use provider::CompatibilityCompositionPassthroughV1Marker;
use provider::CompatibilityDecompositionTablesV1Marker;
use provider::CompositionPassthroughV1;
use provider::DecompositionSupplementV1;
use provider::DecompositionTablesV1;
use provider::NonRecursiveDecompositionSupplementV1Marker;
#[cfg(any(test, feature = "experimental"))]
use provider::Uts46CompositionPassthroughV1Marker;
use smallvec::SmallVec;
use u24::EMPTY_U24;
use u24::U24;
use utf16_iter::Utf16CharsEx;
use utf8_iter::Utf8CharsEx;
use zerofrom::ZeroFrom;
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

/// Wrapper around trie to get the needed set semantics.
struct PassthroughSet<'data> {
    trie: &'data CodePointTrie<'data, u8>,
}

impl<'data> PassthroughSet<'data> {
    /// Constructor
    pub fn new(trie: &'data CodePointTrie<'data, u8>) -> Self {
        Self { trie }
    }
    /// Lookup by scalar value
    pub fn contains(&self, c: char) -> bool {
        self.contains_u32(u32::from(c))
    }
    /// Lookup by code point
    pub fn contains_u32(&self, u: u32) -> bool {
        let head = u >> 3;
        let tail = (u & 0b111) as u8;
        let trie_val = self.trie.get(head);
        // Bit 1 means not passthrough
        (trie_val & (1 << tail)) == 0
    }
}

enum SupplementPayloadHolder {
    Compatibility(DataPayload<CompatibilityDecompositionSupplementV1Marker>),
    #[cfg(any(test, feature = "experimental"))]
    Uts46(DataPayload<Uts46DecompositionSupplementV1Marker>),
}

impl SupplementPayloadHolder {
    fn get(&self) -> &DecompositionSupplementV1 {
        match self {
            SupplementPayloadHolder::Compatibility(d) => d.get(),
            #[cfg(any(test, feature = "experimental"))]
            SupplementPayloadHolder::Uts46(d) => d.get(),
        }
    }
}

enum PassthroughPayloadHolder {
    Canonical(DataPayload<CanonicalCompositionPassthroughV1Marker>),
    Compatibility(DataPayload<CompatibilityCompositionPassthroughV1Marker>),
    #[cfg(any(test, feature = "experimental"))]
    Uts46(DataPayload<Uts46CompositionPassthroughV1Marker>),
}

impl PassthroughPayloadHolder {
    fn get(&self) -> &CompositionPassthroughV1 {
        match self {
            PassthroughPayloadHolder::Canonical(d) => d.get(),
            PassthroughPayloadHolder::Compatibility(d) => d.get(),
            #[cfg(any(test, feature = "experimental"))]
            PassthroughPayloadHolder::Uts46(d) => d.get(),
        }
    }
}

// Magic marker trie value for characters whose decomposition
// starts with a non-starter. The actual decompostion is
// hard-coded.
const DECOMPOSITION_STARTS_WITH_NON_STARTER: u32 = 2 << 16;

/// The tail (everything after the first character) of the NFKD form U+FDFA
/// as 16-bit units.
static FDFA_NFKD: [u16; 17] = [
    0x644, 0x649, 0x20, 0x627, 0x644, 0x644, 0x647, 0x20, 0x639, 0x644, 0x64A, 0x647, 0x20, 0x648,
    0x633, 0x644, 0x645,
];

// These constants originate from page 143 of Unicode 14.0
/// Syllable base
const HANGUL_S_BASE: u32 = 0xAC00;
/// Lead jamo base
const HANGUL_L_BASE: u32 = 0x1100;
/// Vowel jamo base
const HANGUL_V_BASE: u32 = 0x1161;
/// Trail jamo base (deliberately off by one to account for the absence of a trail)
const HANGUL_T_BASE: u32 = 0x11A7;
/// Lead jamo count
const HANGUL_L_COUNT: u32 = 19;
/// Vowel jamo count
const HANGUL_V_COUNT: u32 = 21;
/// Trail jamo count (deliberately off by one to account for the absence of a trail)
const HANGUL_T_COUNT: u32 = 28;
/// Vowel jamo count times trail jamo count
const HANGUL_N_COUNT: u32 = 588;
/// Syllable count
const HANGUL_S_COUNT: u32 = 11172;

/// One past the conjoining jamo block
const HANGUL_JAMO_LIMIT: u32 = 0x1200;

/// If `opt` is `Some`, unwrap it. If `None`, panic if debug assertions
/// are enabled and return `default` if debug assertions are not enabled.
///
/// Use this only if the only reason why `opt` could be `None` is bogus
/// data from the provider.
#[inline(always)]
fn unwrap_or_gigo<T>(opt: Option<T>, default: T) -> T {
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

/// Convert a `U24` _obtained from data provider data_ to `char`.
#[inline(always)]
fn char_from_u24(u: U24) -> char {
    char_from_u32(u.into())
}

const EMPTY_U16: &ZeroSlice<u16> =
    ZeroSlice::<u16>::from_ule_slice(&<u16 as AsULE>::ULE::from_array([]));

#[inline(always)]
fn split_first_u16(s: Option<&ZeroSlice<u16>>) -> (char, &ZeroSlice<u16>) {
    if let Some(slice) = s {
        if let Some(first) = slice.first() {
            return (
                char_from_u16(first),
                // `unwrap()` must succeed, because `first()` returned `Some`.
                #[allow(clippy::unwrap_used)]
                slice.get_subslice(1..slice.len()).unwrap(),
            );
        }
    }
    // GIGO case
    debug_assert!(false);
    (REPLACEMENT_CHARACTER, EMPTY_U16)
}

#[inline(always)]
fn split_first_u24(s: Option<&ZeroSlice<U24>>) -> (char, &ZeroSlice<U24>) {
    if let Some(slice) = s {
        if let Some(first) = slice.first() {
            return (
                char_from_u24(first),
                // `unwrap()` must succeed, because `first()` returned `Some`.
                #[allow(clippy::unwrap_used)]
                slice.get_subslice(1..slice.len()).unwrap(),
            );
        }
    }
    // GIGO case
    debug_assert!(false);
    (REPLACEMENT_CHARACTER, EMPTY_U24)
}

#[inline(always)]
fn in_inclusive_range(c: char, start: char, end: char) -> bool {
    u32::from(c).wrapping_sub(u32::from(start)) <= (u32::from(end) - u32::from(start))
}

/// Performs canonical composition (including Hangul) on a pair of
/// characters or returns `None` if these characters don't compose.
/// Composition exclusions are taken into account.
#[inline]
fn compose(iter: Char16TrieIterator, starter: char, second: char) -> Option<char> {
    let v = u32::from(second).wrapping_sub(HANGUL_V_BASE);
    if v >= HANGUL_JAMO_LIMIT - HANGUL_V_BASE {
        return compose_non_hangul(iter, starter, second);
    }
    if v < HANGUL_V_COUNT {
        let l = u32::from(starter).wrapping_sub(HANGUL_L_BASE);
        if l < HANGUL_L_COUNT {
            let lv = l * HANGUL_N_COUNT + v * HANGUL_T_COUNT;
            // Safe, because the inputs are known to be in range.
            return Some(unsafe { char::from_u32_unchecked(HANGUL_S_BASE + lv) });
        }
        return None;
    }
    if in_inclusive_range(second, '\u{11A8}', '\u{11C2}') {
        let lv = u32::from(starter).wrapping_sub(HANGUL_S_BASE);
        if lv < HANGUL_S_COUNT && lv % HANGUL_T_COUNT == 0 {
            let lvt = lv + (u32::from(second) - HANGUL_T_BASE);
            // Safe, because the inputs are known to be in range.
            return Some(unsafe { char::from_u32_unchecked(HANGUL_S_BASE + lvt) });
        }
    }
    None
}

/// Performs (non-Hangul) canonical composition on a pair of characters
/// or returns `None` if these characters don't compose. Composition
/// exclusions are taken into account.
fn compose_non_hangul(mut iter: Char16TrieIterator, starter: char, second: char) -> Option<char> {
    // To make the trie smaller, the pairs are stored second character first.
    // Given how this method is used in ways where it's known that `second`
    // is or isn't a starter. We could potentially split the trie into two
    // tries depending on whether `second` is a starter.
    match iter.next(second) {
        TrieResult::NoMatch => None,
        TrieResult::NoValue => match iter.next(starter) {
            TrieResult::NoMatch => None,
            TrieResult::FinalValue(i) => {
                if let Some(c) = char::from_u32(i as u32) {
                    Some(c)
                } else {
                    // GIGO case
                    debug_assert!(false);
                    None
                }
            }
            TrieResult::NoValue | TrieResult::Intermediate(_) => {
                // GIGO case
                debug_assert!(false);
                None
            }
        },
        TrieResult::FinalValue(_) | TrieResult::Intermediate(_) => {
            // GIGO case
            debug_assert!(false);
            None
        }
    }
}

/// Struct for holding together a character and the value
/// looked up for it from the NFD trie in a more explicit
/// way than an anonymous pair.
#[derive(Debug, PartialEq, Eq)]
struct CharacterAndTrieValue {
    character: char,
    trie_val: u32,
}

/// Pack a `char` and a `CanonicalCombiningClass` in
/// 32 bits (the former in the lower 24 bits and the
/// latter in the high 8 bits). The latter is
/// initialized to 0 upon creation and can be set
/// by calling `set_ccc_from_trie`. This type is
/// intentionally non-`Copy` to get compiler help
/// in making sure that the class is set on the
/// instance on which it is intended to be set
/// and not on a temporary copy.
///
/// Note: As a micro-optimization, this struct is
/// distinct from the struct of the same name in
/// the collator. They have the opposite default
/// bit pattern for the high 8 bits.
#[derive(Debug)]
struct CharacterAndClass(u32);

impl CharacterAndClass {
    pub fn new(c: char) -> Self {
        // The combining class bits default to zero.
        CharacterAndClass(u32::from(c))
    }
    pub fn character(&self) -> char {
        // Safe, because the low 24 bits came from a `char`.
        unsafe { char::from_u32_unchecked(self.0 & 0xFFFFFF) }
    }
    pub fn ccc(&self) -> CanonicalCombiningClass {
        CanonicalCombiningClass((self.0 >> 24) as u8)
    }
    pub fn character_and_ccc(&self) -> (char, CanonicalCombiningClass) {
        (self.character(), self.ccc())
    }
    pub fn set_ccc_from_trie(
        &mut self,
        ccc_trie: CodePointMapDataBorrowed<CanonicalCombiningClass>,
    ) {
        debug_assert_eq!(self.0 >> 24, 0, "This method has already been called!");
        self.0 |= (ccc_trie.get_u32(self.0).0 as u32) << 24;
    }
}

// This function exists as a borrow check helper.
#[inline(always)]
fn assign_ccc_and_sort_combining(
    slice: &mut [CharacterAndClass],
    combining_start: usize,
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
) {
    slice.iter_mut().for_each(|cc| cc.set_ccc_from_trie(ccc));
    // Slicing succeeds by construction; we've always ensured that `combining_start`
    // is in permissible range.
    #[allow(clippy::indexing_slicing)]
    slice[combining_start..].sort_by_key(|cc| cc.ccc());
}

// This function exists as a borrow check helper.
#[inline(always)]
fn sort_slice_by_ccc(
    slice: &mut [CharacterAndClass],
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
) {
    // We don't look up the canonical combining class for starters
    // of for single combining characters between starters. When
    // there's more than one combining character between starters,
    // we look up the canonical combining class for each character
    // exactly once.
    if slice.len() < 2 {
        return;
    }
    slice.iter_mut().for_each(|cc| cc.set_ccc_from_trie(ccc));
    slice.sort_by_key(|cc| cc.ccc());
}

/// An iterator adaptor that turns an `Iterator` over `char` into
/// a lazily-decomposed `char` sequence.
pub struct Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    delegate: I,
    buffer: SmallVec<[CharacterAndClass; 17]>, // Enough to hold NFKD for U+FDFA
    /// The index of the next item to be read from `buffer`.
    /// The purpose if this index is to avoid having to move
    /// the rest upon every read.
    buffer_pos: usize,
    // At the start of `next()` if not `None`, this is a pending unnormalized
    // starter. When `Decomposition` appears alone, this is never a non-starter.
    // However, when `Decomposition` appears inside a `Composition`, this
    // may become a non-starter before `decomposing_next()` is called.
    pending: Option<CharacterAndTrieValue>, // None at end of stream
    trie: &'data CodePointTrie<'data, u32>,
    supplementary_trie: Option<&'data CodePointTrie<'data, u32>>,
    scalars16: &'data ZeroSlice<u16>,
    scalars24: &'data ZeroSlice<U24>,
    supplementary_scalars16: &'data ZeroSlice<u16>,
    supplementary_scalars24: &'data ZeroSlice<U24>,
    ccc: CodePointMapDataBorrowed<'data, CanonicalCombiningClass>,
    half_width_voicing_marks_become_non_starters: bool,
    iota_subscript_becomes_starter: bool,
    has_starter_exceptions: bool,
    /// This set is only used when this `Decomposition` is used inside `Composition`.
    /// Therefore, when this `is_some()`, other things should take `Composition`-specific
    /// actions and when this `is_none()`, `Decomposition`-only optimizations may be
    /// taken.
    ///
    /// Whether `Decomposition`-only mode vs. inside-`Composition` mode should be
    /// a compile-time specialization instead of an `Option` discriminant is a topic
    /// to be revisited.
    ///
    /// As for the semantics of the set itself:
    /// In order to have to check each character only from one set in the
    /// best case, this set combines both the characteristic that a starter
    /// maps to itself if not followed by something that combines backwards
    /// and the characteristic that the character is a starter that never
    /// combines backwards. Therefore, if both the current character and the
    /// next character are in this set, it is OK for `Composition` to pass
    /// the current character through.
    ///
    /// Membership in this set is an optimization, so it's always valid to
    /// omit some characters from this set. As a consequence, multiple
    /// normalization forms whose sets are similar may use the intersection
    /// of their exact sets in order to need to store only the intersection.
    potential_passthrough_and_not_backward_combining: Option<PassthroughSet<'data>>,
    /// The character in `pending` is a in the
    /// `potential_passthrough_and_not_backward_combining` set.
    /// This flag is meaningful only when `pending.is_some()` and
    /// `potential_passthrough_and_not_backward_combining.is_some()`.
    /// That is, this flag is only used in the inside-`Composition` mode.
    pending_is_potential_passthrough_and_not_backward_combining: bool,
}

impl<'data, I> Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    /// Constructs a decomposing iterator adapter from a delegate
    /// iterator and references to the necessary data, without
    /// supplementary data.
    ///
    /// Use `DecomposingNormalizer::normalize_iter()` instead unless
    /// there's a good reason to use this constructor directly.
    ///
    /// Public but hidden in order to be able to use this from the
    /// collator.
    #[doc(hidden)]
    pub fn new(
        delegate: I,
        decompositions: &'data DecompositionDataV1,
        tables: &'data DecompositionTablesV1,
        ccc: CodePointMapDataBorrowed<'data, CanonicalCombiningClass>,
    ) -> Self {
        Self::new_with_supplements(delegate, decompositions, None, tables, None, ccc, None)
    }

    /// Constructs a decomposing iterator adapter from a delegate
    /// iterator and references to the necessary data, including
    /// supplementary data.
    ///
    /// Use `DecomposingNormalizer::normalize_iter()` instead unless
    /// there's a good reason to use this constructor directly.
    fn new_with_supplements(
        delegate: I,
        decompositions: &'data DecompositionDataV1,
        supplementary_decompositions: Option<&'data DecompositionSupplementV1>,
        tables: &'data DecompositionTablesV1,
        supplementary_tables: Option<&'data DecompositionTablesV1>,
        ccc: CodePointMapDataBorrowed<'data, CanonicalCombiningClass>,
        potential_passthrough_and_not_backward_combining: Option<PassthroughSet<'data>>,
    ) -> Self {
        let (half_width_voicing_marks_become_non_starters, iota_subscript_becomes_starter) =
            if let Some(supplementary) = supplementary_decompositions {
                (
                    supplementary.half_width_voicing_marks_become_non_starters(),
                    supplementary.iota_subscript_becomes_starter(),
                )
            } else {
                (false, false)
            };
        let mut ret = Decomposition::<I> {
            delegate,
            buffer: SmallVec::new(), // Normalized
            buffer_pos: 0,
            // Initialize with a placeholder starter in case
            // the real stream starts with a non-starter.
            pending: Some(CharacterAndTrieValue {
                character: '\u{FFFF}',
                trie_val: 0,
            }),
            trie: &decompositions.trie,
            supplementary_trie: supplementary_decompositions.map(|s| &s.trie),
            scalars16: &tables.scalars16,
            scalars24: &tables.scalars24,
            supplementary_scalars16: if let Some(supplementary) = supplementary_tables {
                &supplementary.scalars16
            } else {
                EMPTY_U16
            },
            supplementary_scalars24: if let Some(supplementary) = supplementary_tables {
                &supplementary.scalars24
            } else {
                EMPTY_U24
            },
            ccc,
            half_width_voicing_marks_become_non_starters,
            iota_subscript_becomes_starter,
            has_starter_exceptions: half_width_voicing_marks_become_non_starters
                || iota_subscript_becomes_starter,
            potential_passthrough_and_not_backward_combining,
            pending_is_potential_passthrough_and_not_backward_combining: true, // U+FFFF
        };
        let _ = ret.next(); // Remove the U+FFFF placeholder
        ret
    }

    fn push_decomposition16(
        &mut self,
        low: u16,
        offset: usize,
        slice16: &ZeroSlice<u16>,
    ) -> (char, usize) {
        let len = usize::from(low >> 13) + 2;
        let (starter, tail) = split_first_u16(slice16.get_subslice(offset..offset + len));
        if low & 0x1000 != 0 {
            // All the rest are combining
            for u in tail.iter() {
                self.buffer.push(CharacterAndClass::new(char_from_u16(u)));
            }
            (starter, 0)
        } else {
            let mut i = 0;
            let mut combining_start = 0;
            for u in tail.iter() {
                let ch = char_from_u16(u);
                self.buffer.push(CharacterAndClass::new(ch));
                i += 1;
                // Half-width kana and iota subscript don't occur in the tails
                // of these multicharacter decompositions.
                if self.trie.get(u32::from(ch)) != DECOMPOSITION_STARTS_WITH_NON_STARTER {
                    combining_start = i;
                }
            }
            (starter, combining_start)
        }
    }

    fn push_decomposition32(
        &mut self,
        low: u16,
        offset: usize,
        slice32: &ZeroSlice<U24>,
    ) -> (char, usize) {
        let len = usize::from(low >> 13) + 1;
        let (starter, tail) = split_first_u24(slice32.get_subslice(offset..offset + len));
        if low & 0x1000 != 0 {
            // All the rest are combining
            for u in tail.iter() {
                self.buffer.push(CharacterAndClass::new(char_from_u24(u)));
            }
            (starter, 0)
        } else {
            let mut i = 0;
            let mut combining_start = 0;
            for u in tail.iter() {
                let ch = char_from_u24(u);
                self.buffer.push(CharacterAndClass::new(ch));
                i += 1;
                // Half-width kana and iota subscript don't occur in the tails
                // of these multicharacter decompositions.
                if self.trie.get(u32::from(ch)) != DECOMPOSITION_STARTS_WITH_NON_STARTER {
                    combining_start = i;
                }
            }
            (starter, combining_start)
        }
    }

    fn delegate_next_no_pending(&mut self) -> Option<CharacterAndTrieValue> {
        debug_assert!(self.pending.is_none());
        let c = self.delegate.next()?;

        if self.has_starter_exceptions {
            if u32::from(c) & !1 == 0xFF9E && self.half_width_voicing_marks_become_non_starters {
                return Some(CharacterAndTrieValue {
                    character: c,
                    trie_val: DECOMPOSITION_STARTS_WITH_NON_STARTER,
                });
            }
            if c == '\u{0345}' && self.iota_subscript_becomes_starter {
                return Some(CharacterAndTrieValue {
                    character: c,
                    trie_val: u32::from('ι') << 16,
                });
            }
        }

        Some(CharacterAndTrieValue {
            character: c,
            trie_val: self.trie.get(u32::from(c)),
        })
    }

    fn delegate_next(&mut self) -> Option<CharacterAndTrieValue> {
        if let Some(pending) = self.pending.take() {
            debug_assert!(self
                .potential_passthrough_and_not_backward_combining
                .is_some());
            Some(pending)
        } else {
            self.delegate_next_no_pending()
        }
    }

    fn decomposing_next(&mut self, c_and_trie_val: CharacterAndTrieValue) -> char {
        let (starter, combining_start) = {
            let c = c_and_trie_val.character;
            let hangul_offset = u32::from(c).wrapping_sub(HANGUL_S_BASE); // SIndex in the spec
            if hangul_offset >= HANGUL_S_COUNT {
                let mut decomposition;
                // The loop is only broken out of as goto forward
                #[allow(clippy::never_loop)]
                loop {
                    if let Some(supplementary) = self.supplementary_trie {
                        decomposition = supplementary.get(u32::from(c));
                        if decomposition != 0 {
                            break;
                        }
                    }
                    decomposition = c_and_trie_val.trie_val;
                    break;
                }
                if decomposition == 0 {
                    // The character is its own decomposition
                    (c, 0)
                } else {
                    let high = (decomposition >> 16) as u16;
                    let low = decomposition as u16;
                    if high != 0 && low != 0 {
                        // Decomposition into two BMP characters: starter and non-starter
                        let starter = char_from_u16(high);
                        let combining = char_from_u16(low);
                        self.buffer.push(CharacterAndClass::new(combining));
                        (starter, 0)
                    } else if high != 0 {
                        if high != 1 {
                            debug_assert_ne!(
                                high, 2,
                                "Should not reach this point with non-starter marker"
                            );
                            // Decomposition into one BMP character
                            let starter = char_from_u16(high);
                            (starter, 0)
                        } else {
                            // Special case for the NFKD form of U+FDFA.
                            for u in FDFA_NFKD {
                                // Safe, because `FDFA_NFKD` is known not to contain
                                // surrogates.
                                self.buffer.push(CharacterAndClass::new(unsafe {
                                    core::char::from_u32_unchecked(u32::from(u))
                                }));
                            }
                            ('\u{0635}', 17)
                        }
                    } else {
                        // Complex decomposition
                        // Format for 16-bit value:
                        // 15..13: length minus two for 16-bit case and length minus one for
                        //         the 32-bit case. Length 8 needs to fit in three bits in
                        //         the 16-bit case, and this way the value is future-proofed
                        //         up to 9 in the 16-bit case. Zero is unused and length one
                        //         in the 16-bit case goes directly into the trie.
                        //     12: 1 if all trailing characters are guaranteed non-starters,
                        //         0 if no guarantees about non-starterness.
                        //         Note: The bit choice is this way around to allow for
                        //         dynamically falling back to not having this but instead
                        //         having one more bit for length by merely choosing
                        //         different masks.
                        //  11..0: Start offset in storage. The offset is to the logical
                        //         sequence of scalars16, scalars32, supplementary_scalars16,
                        //         supplementary_scalars32.
                        let offset = usize::from(low & 0xFFF);
                        if offset < self.scalars16.len() {
                            self.push_decomposition16(low, offset, self.scalars16)
                        } else if offset < self.scalars16.len() + self.scalars24.len() {
                            self.push_decomposition32(
                                low,
                                offset - self.scalars16.len(),
                                self.scalars24,
                            )
                        } else if offset
                            < self.scalars16.len()
                                + self.scalars24.len()
                                + self.supplementary_scalars16.len()
                        {
                            self.push_decomposition16(
                                low,
                                offset - (self.scalars16.len() + self.scalars24.len()),
                                self.supplementary_scalars16,
                            )
                        } else {
                            self.push_decomposition32(
                                low,
                                offset
                                    - (self.scalars16.len()
                                        + self.scalars24.len()
                                        + self.supplementary_scalars16.len()),
                                self.supplementary_scalars24,
                            )
                        }
                    }
                }
            } else {
                // Hangul syllable
                // The math here comes from page 144 of Unicode 14.0
                let l = hangul_offset / HANGUL_N_COUNT;
                let v = (hangul_offset % HANGUL_N_COUNT) / HANGUL_T_COUNT;
                let t = hangul_offset % HANGUL_T_COUNT;

                // The unsafe blocks here are OK, because the values stay
                // within the Hangul jamo block and, therefore, the scalar
                // value range by construction.
                self.buffer.push(CharacterAndClass::new(unsafe {
                    core::char::from_u32_unchecked(HANGUL_V_BASE + v)
                }));
                let first = unsafe { core::char::from_u32_unchecked(HANGUL_L_BASE + l) };
                if t != 0 {
                    self.buffer.push(CharacterAndClass::new(unsafe {
                        core::char::from_u32_unchecked(HANGUL_T_BASE + t)
                    }));
                    (first, 2)
                } else {
                    (first, 1)
                }
            }
        };
        debug_assert!(
            self.potential_passthrough_and_not_backward_combining
                .is_some()
                || self.pending.is_none()
        );
        // Not a `for` loop to avoid holding a mutable reference to `self` across
        // the loop body.
        while let Some(ch_and_trie_val) = self.delegate_next() {
            if ch_and_trie_val.trie_val == DECOMPOSITION_STARTS_WITH_NON_STARTER {
                let ch = ch_and_trie_val.character;
                let mapped = if !(in_inclusive_range(ch, '\u{0340}', '\u{0F81}')
                    || (u32::from(ch) & !1 == 0xFF9E))
                {
                    ch
                } else {
                    // The Tibetan special cases are starters that decompose into non-starters.
                    // The same applies to the half-width kana voicing marks in NFKD.
                    //
                    // Logically the canonical combining class of each special case is known
                    // at compile time, but all characters in the buffer are treated the same
                    // when looking up the canonical combining class to avoid a per-character
                    // branch that would only benefit these rare special cases.
                    match ch {
                        '\u{0340}' => {
                            // COMBINING GRAVE TONE MARK
                            '\u{0300}'
                        }
                        '\u{0341}' => {
                            // COMBINING ACUTE TONE MARK
                            '\u{0301}'
                        }
                        '\u{0343}' => {
                            // COMBINING GREEK KORONIS
                            '\u{0313}'
                        }
                        '\u{0344}' => {
                            // COMBINING GREEK DIALYTIKA TONOS
                            self.buffer.push(CharacterAndClass::new('\u{0308}'));
                            '\u{0301}'
                        }
                        '\u{0F73}' => {
                            // TIBETAN VOWEL SIGN II
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            '\u{0F72}'
                        }
                        '\u{0F75}' => {
                            // TIBETAN VOWEL SIGN UU
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            '\u{0F74}'
                        }
                        '\u{0F81}' => {
                            // TIBETAN VOWEL SIGN REVERSED II
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            '\u{0F80}'
                        }
                        '\u{FF9E}' => {
                            // HALFWIDTH KATAKANA VOICED SOUND MARK
                            // Compatibility decomposition only; can't come here
                            // in NFD.
                            '\u{3099}'
                        }
                        '\u{FF9F}' => {
                            // HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
                            // Compatibility decomposition only; can't come here
                            // in NFD.
                            '\u{309A}'
                        }
                        _ => ch,
                    }
                };
                self.buffer.push(CharacterAndClass::new(mapped));
            } else {
                if let Some(set) = self
                    .potential_passthrough_and_not_backward_combining
                    .as_ref()
                {
                    self.pending_is_potential_passthrough_and_not_backward_combining =
                        set.contains(ch_and_trie_val.character)
                }
                self.pending = Some(ch_and_trie_val);
                break;
            }
        }
        if self
            .potential_passthrough_and_not_backward_combining
            .is_some()
        {
            // As part of `Composition`, we have to compute the canonical combining
            // class for every character in `buffer`.
            assign_ccc_and_sort_combining(&mut self.buffer[..], combining_start, self.ccc);
        } else {
            // Slicing succeeds by construction; we've always ensured that `combining_start`
            // is in permissible range.
            #[allow(clippy::indexing_slicing)]
            sort_slice_by_ccc(&mut self.buffer[combining_start..], self.ccc);
        }
        starter
    }
}

impl<'data, I> Iterator for Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some(ret) = self.buffer.get(self.buffer_pos).map(|c| c.character()) {
            self.buffer_pos += 1;
            if self.buffer_pos == self.buffer.len() {
                self.buffer.clear();
                self.buffer_pos = 0;
            }
            return Some(ret);
        }
        debug_assert_eq!(self.buffer_pos, 0);
        let c_and_trie_val = self.pending.take()?;
        Some(self.decomposing_next(c_and_trie_val))
    }
}

/// An iterator adaptor that turns an `Iterator` over `char` into
/// a lazily-decomposed and then canonically composed `char` sequence.
pub struct Composition<'data, I>
where
    I: Iterator<Item = char>,
{
    /// The decomposing part of the normalizer than operates before
    /// the canonical composition is performed on its output.
    decomposition: Decomposition<'data, I>,
    /// Non-Hangul canonical composition data.
    /// TODO: Experiment if it makes sense to split this into two tries:
    /// One where the second character is a starter and another where
    /// the second character is a non-starter. Upon access, we know the
    /// starterness of the second character anyway.
    canonical_compositions: Char16Trie<'data>,
    /// To make `next()` yield in cases where there's a non-composing
    /// starter in the decomposition buffer, we put it here to let it
    /// wait for the next `next()` call (or a jump forward within the
    /// `next()` call).
    unprocessed_starter: Option<char>,
}

impl<'data, I> Composition<'data, I>
where
    I: Iterator<Item = char>,
{
    fn new(
        decomposition: Decomposition<'data, I>,
        canonical_compositions: Char16Trie<'data>,
    ) -> Self {
        Self {
            decomposition,
            canonical_compositions,
            unprocessed_starter: None,
        }
    }

    /// Performs canonical composition (including Hangul) on a pair of
    /// characters or returns `None` if these characters don't compose.
    /// Composition exclusions are taken into account.
    #[inline(always)]
    pub fn compose(&self, starter: char, second: char) -> Option<char> {
        compose(self.canonical_compositions.iter(), starter, second)
    }

    /// Performs (non-Hangul) canonical composition on a pair of characters
    /// or returns `None` if these characters don't compose. Composition
    /// exclusions are taken into account.
    #[inline(always)]
    fn compose_non_hangul(&self, starter: char, second: char) -> Option<char> {
        compose_non_hangul(self.canonical_compositions.iter(), starter, second)
    }
}

impl<'data, I> Iterator for Composition<'data, I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        let mut undecomposed_starter = CharacterAndTrieValue {
            character: '\u{0}',
            trie_val: 0,
        }; // The compiler can't figure out that this gets overwritten before use.
        if self.unprocessed_starter.is_none() {
            // The loop is only broken out of as goto forward
            #[allow(clippy::never_loop)]
            loop {
                if let Some((character, ccc)) = self
                    .decomposition
                    .buffer
                    .get(self.decomposition.buffer_pos)
                    .map(|c| c.character_and_ccc())
                {
                    self.decomposition.buffer_pos += 1;
                    if self.decomposition.buffer_pos == self.decomposition.buffer.len() {
                        self.decomposition.buffer.clear();
                        self.decomposition.buffer_pos = 0;
                    }
                    if ccc == CanonicalCombiningClass::NotReordered {
                        // Previous decomposition contains a starter. This must
                        // now become the `unprocessed_starter` for it to have
                        // a chance to compose with the upcoming characters.
                        //
                        // E.g. parenthesized Hangul in NFKC comes through here,
                        // but suitable composition exclusion could exercise this
                        // in NFC.
                        self.unprocessed_starter = Some(character);
                        break; // We already have a starter, so skip taking one from `pending`.
                    }
                    return Some(character);
                }
                debug_assert_eq!(self.decomposition.buffer_pos, 0);
                undecomposed_starter = self.decomposition.pending.take()?;
                if self
                    .decomposition
                    .pending_is_potential_passthrough_and_not_backward_combining
                {
                    // Attribute belongs on inner expression, but
                    // https://github.com/rust-lang/rust/issues/15701
                    #[allow(clippy::unwrap_used)]
                    if let Some(upcoming) = self.decomposition.delegate_next_no_pending() {
                        // Note: If `upcoming` is not in the set due to its decomposition
                        // starting with a non-starter, we end up checking set membership
                        // again. Bookkeeping to avoid the second check would be possible
                        // but error-prone.
                        //
                        // `unwrap` is OK, because we are in `Composition` and
                        // `potential_passthrough_and_not_backward_combining.is_some()` whenever
                        // `Decomposition` is inside `Composition`.
                        self.decomposition
                            .pending_is_potential_passthrough_and_not_backward_combining = self
                            .decomposition
                            .potential_passthrough_and_not_backward_combining
                            .as_ref()
                            .unwrap()
                            .contains(upcoming.character);
                        self.decomposition.pending = Some(upcoming);
                        if self
                            .decomposition
                            .pending_is_potential_passthrough_and_not_backward_combining
                        {
                            // Fast-track succeeded!
                            return Some(undecomposed_starter.character);
                        }
                    } else {
                        // End of stream
                        return Some(undecomposed_starter.character);
                    }
                }
                break; // Not actually looping
            }
        }
        let mut starter = '\u{0}'; // The compiler can't figure out this gets overwritten before use.

        // The point of having this boolean is to have only one call site to
        // `self.decomposition.decomposing_next`, which is hopefully beneficial for
        // code size under inlining.
        let mut attempt_composition = false;
        loop {
            if let Some(unprocessed) = self.unprocessed_starter.take() {
                debug_assert_eq!(
                    undecomposed_starter,
                    CharacterAndTrieValue {
                        character: '\u{0}',
                        trie_val: 0
                    }
                );
                debug_assert_eq!(starter, '\u{0}');
                starter = unprocessed;
            } else {
                debug_assert_eq!(self.decomposition.buffer_pos, 0);
                let next_starter = self.decomposition.decomposing_next(undecomposed_starter);
                if !attempt_composition {
                    starter = next_starter;
                } else if let Some(composed) = self.compose(starter, next_starter) {
                    starter = composed;
                } else {
                    // This is our yield point. We'll pick this up above in the
                    // next call to `next()`.
                    self.unprocessed_starter = Some(next_starter);
                    return Some(starter);
                }
            }
            // We first loop by index to avoid moving the contents of `buffer`, but
            // if there's a discontiguous match, we'll start modifying `buffer` instead.
            loop {
                let (character, ccc) = if let Some((character, ccc)) = self
                    .decomposition
                    .buffer
                    .get(self.decomposition.buffer_pos)
                    .map(|c| c.character_and_ccc())
                {
                    (character, ccc)
                } else {
                    self.decomposition.buffer.clear();
                    self.decomposition.buffer_pos = 0;
                    break;
                };
                if let Some(composed) = self.compose(starter, character) {
                    starter = composed;
                    self.decomposition.buffer_pos += 1;
                    continue;
                }
                let mut most_recent_skipped_ccc = ccc;
                {
                    let _ = self
                        .decomposition
                        .buffer
                        .drain(0..self.decomposition.buffer_pos);
                }
                self.decomposition.buffer_pos = 0;
                if most_recent_skipped_ccc == CanonicalCombiningClass::NotReordered {
                    // We failed to compose a starter. Discontiguous match not allowed.
                    // We leave the starter in `buffer` for `next()` to find.
                    return Some(starter);
                }
                let mut i = 1; // We have skipped one non-starter.
                while let Some((character, ccc)) = self
                    .decomposition
                    .buffer
                    .get(i)
                    .map(|c| c.character_and_ccc())
                {
                    if ccc == CanonicalCombiningClass::NotReordered {
                        // Discontiguous match not allowed.
                        return Some(starter);
                    }
                    debug_assert!(ccc >= most_recent_skipped_ccc);
                    if ccc != most_recent_skipped_ccc {
                        // Using the non-Hangul version as a micro-optimization, since
                        // we already rejected the case where `second` is a starter
                        // above, and conjoining jamo are starters.
                        if let Some(composed) = self.compose_non_hangul(starter, character) {
                            self.decomposition.buffer.remove(i);
                            starter = composed;
                            continue;
                        }
                    }
                    most_recent_skipped_ccc = ccc;
                    i += 1;
                }
                break;
            }

            debug_assert_eq!(self.decomposition.buffer_pos, 0);

            if !self.decomposition.buffer.is_empty() {
                return Some(starter);
            }
            // Now we need to check if composition with an upcoming starter is possible.
            #[allow(clippy::unwrap_used)]
            if self.decomposition.pending.is_some() {
                // We know that `pending_starter` decomposes to start with a starter.
                // Otherwise, it would have been moved to `self.decomposition.buffer`
                // by `self.decomposing_next()`. We do this set lookup here in order
                // to get an opportunity to go back to the fast track.
                // Note that this check has to happen _after_ checking that `pending`
                // holds a character, because this flag isn't defined to be meaningful
                // when `pending` isn't holding a character.
                if self
                    .decomposition
                    .pending_is_potential_passthrough_and_not_backward_combining
                {
                    // Won't combine backwards anyway.
                    return Some(starter);
                }
                // Consume what we peeked. `unwrap` OK, because we checked `is_some()`
                // above.
                undecomposed_starter = self.decomposition.pending.take().unwrap();
                // The following line is OK, because we're about to loop back
                // to `self.decomposition.decomposing_next(c);`, which will
                // restore the between-`next()`-calls invariant of `pending`
                // before this function returns.
                attempt_composition = true;
                continue;
            }
            // End of input
            return Some(starter);
        }
    }
}

macro_rules! normalizer_methods {
    () => {
        /// Normalize a string slice into a `String`.
        pub fn normalize(&self, text: &str) -> String {
            self.normalize_iter(text.chars()).collect()
        }

        /// Check whether a string slice is normalized.
        pub fn is_normalized(&self, text: &str) -> bool {
            self.normalize_iter(text.chars()).eq(text.chars())
        }

        /// Normalize a string slice into a `Write` sink.
        pub fn normalize_to<W: core::fmt::Write + ?Sized>(
            &self,
            text: &str,
            sink: &mut W,
        ) -> core::fmt::Result {
            for c in self.normalize_iter(text.chars()) {
                sink.write_char(c)?;
            }
            Ok(())
        }

        /// Normalize a slice of potentially-invalid UTF-16 into a `Vec`.
        ///
        /// Unpaired surrogates are mapped to the REPLACEMENT CHARACTER
        /// before normalizing.
        pub fn normalize_utf16(&self, text: &[u16]) -> Vec<u16> {
            let mut ret = Vec::with_capacity(text.len());
            for c in self.normalize_iter(text.chars()) {
                // This is measurably faster than `encode_utf16`.
                if c <= '\u{FFFF}' {
                    ret.push(c as u16);
                } else {
                    let u = u32::from(c);
                    ret.push((0xD7C0 + (u >> 10)) as u16);
                    ret.push((0xDC00 + (u & 0x3FF)) as u16);
                }
            }
            ret
        }

        /// Normalize a slice of potentially-invalid UTF-16 into a `Write` sink.
        ///
        /// Unpaired surrogates are mapped to the REPLACEMENT CHARACTER
        /// before normalizing.
        pub fn normalize_utf16_to<W: write16::Write16 + ?Sized>(
            &self,
            text: &[u16],
            sink: &mut W,
        ) -> core::fmt::Result {
            for c in self.normalize_iter(text.chars()) {
                sink.write_char(c)?;
            }
            Ok(())
        }

        /// Checks whether a slice of potentially-invalid UTF-16 is normalized.
        ///
        /// Unpaired surrogates are treated as the REPLACEMENT CHARACTER.
        pub fn is_normalized_utf16(&self, text: &[u16]) -> bool {
            self.normalize_iter(text.chars()).eq(text.chars())
        }

        /// Normalize a slice of potentially-invalid UTF-8 into a `String`.
        ///
        /// Errors are mapped to the REPLACEMENT CHARACTER according
        /// to the WHATWG Encoding Standard.
        pub fn normalize_utf8(&self, text: &[u8]) -> String {
            self.normalize_iter(text.chars()).collect()
        }

        /// Normalize a slice of potentially-invalid UTF-8 into a `Write` sink.
        ///
        /// Errors are mapped to the REPLACEMENT CHARACTER according
        /// to the WHATWG Encoding Standard.
        pub fn normalize_utf8_to<W: core::fmt::Write + ?Sized>(
            &self,
            text: &[u8],
            sink: &mut W,
        ) -> core::fmt::Result {
            for c in self.normalize_iter(text.chars()) {
                sink.write_char(c)?;
            }
            Ok(())
        }

        /// Check if a slice of potentially-invalid UTF-8 is normalized.
        ///
        /// Errors are mapped to the REPLACEMENT CHARACTER according
        /// to the WHATWG Encoding Standard before checking.
        pub fn is_normalized_utf8(&self, text: &[u8]) -> bool {
            self.normalize_iter(text.chars()).eq(text.chars())
        }
    };
}

/// A normalizer for performing decomposing normalization.
pub struct DecomposingNormalizer {
    decompositions: DataPayload<CanonicalDecompositionDataV1Marker>,
    supplementary_decompositions: Option<SupplementPayloadHolder>,
    tables: DataPayload<CanonicalDecompositionTablesV1Marker>,
    supplementary_tables: Option<DataPayload<CompatibilityDecompositionTablesV1Marker>>,
    ccc: CodePointMapData<CanonicalCombiningClass>,
}

impl DecomposingNormalizer {
    /// NFD constructor.
    pub fn try_new_nfd<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;

        if tables.get().scalars16.len() + tables.get().scalars24.len() > 0xFFF {
            // The data is from a future where there exists a normalization flavor whose
            // complex decompositions take more than 0xFFF but fewer than 0x1FFF code points
            // of space. If a good use case from such a decomposition flavor arises, we can
            // dynamically change the bit masks so that the length mask becomes 0x1FFF instead
            // of 0xFFF and the all-non-starters mask becomes 0 instead of 0x1000. However,
            // since for now the masks are hard-coded, error out.
            return Err(NormalizerError::FutureExtension);
        }

        let ccc = icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions,
            supplementary_decompositions: None,
            tables,
            supplementary_tables: None,
            ccc,
        })
    }

    /// NFKD constructor.
    pub fn try_new_nfkd<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let supplementary_decompositions: DataPayload<
            CompatibilityDecompositionSupplementV1Marker,
        > = data_provider.load(Default::default())?.take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let supplementary_tables: DataPayload<CompatibilityDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;

        if tables.get().scalars16.len()
            + tables.get().scalars24.len()
            + supplementary_tables.get().scalars16.len()
            + supplementary_tables.get().scalars24.len()
            > 0xFFF
        {
            // The data is from a future where there exists a normalization flavor whose
            // complex decompositions take more than 0xFFF but fewer than 0x1FFF code points
            // of space. If a good use case from such a decomposition flavor arises, we can
            // dynamically change the bit masks so that the length mask becomes 0x1FFF instead
            // of 0xFFF and the all-non-starters mask becomes 0 instead of 0x1000. However,
            // since for now the masks are hard-coded, error out.
            return Err(NormalizerError::FutureExtension);
        }

        let ccc = icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions,
            supplementary_decompositions: Some(SupplementPayloadHolder::Compatibility(
                supplementary_decompositions,
            )),
            tables,
            supplementary_tables: Some(supplementary_tables),
            ccc,
        })
    }

    /// UTS 46 decomposed constructor (testing only)
    ///
    /// This is a special building block normalization for IDNA. It is the decomposed counterpart of
    /// ICU4C's UTS 46 normalization with two exceptions: characters that UTS 46 disallows and
    /// ICU4C maps to U+FFFD and characters that UTS 46 maps to the empty string normalize as in
    /// NFD in this normalization. In both cases, the previous UTS 46 processing before using
    /// normalization is expected to deal with these characters. Making the disallowed characters
    /// behave like this is beneficial to data size, and this normalizer implementation cannot
    /// deal with a character normalizing to the empty string, which doesn't happen in NFD or
    /// NFKD as of Unicode 14.
    ///
    /// Warning: In this normalization, U+0345 COMBINING GREEK YPOGEGRAMMENI exhibits a behavior
    /// that no character in Unicode exhibits in NFD, NFKD, NFC, or NFKC: Case folding turns
    /// U+0345 from a reordered character into a non-reordered character before reordering happens.
    /// Therefore, the output of this normalization may differ for different inputs that are
    /// canonically equivant with each other if they differ by how U+0345 is ordered relative
    /// to other reorderable characters.
    ///
    /// Deliberately private and not available outside the crate.
    #[cfg(any(test, feature = "experimental"))]
    fn try_new_uts46_decomposed_without_ignored_and_disallowed<D>(
        data_provider: &D,
    ) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<Uts46DecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            // UTS 46 tables merged into CompatibilityDecompositionTablesV1Marker
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let supplementary_decompositions: DataPayload<Uts46DecompositionSupplementV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let supplementary_tables: DataPayload<CompatibilityDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;

        if tables.get().scalars16.len()
            + tables.get().scalars24.len()
            + supplementary_tables.get().scalars16.len()
            + supplementary_tables.get().scalars24.len()
            > 0xFFF
        {
            // The data is from a future where there exists a normalization flavor whose
            // complex decompositions take more than 0xFFF but fewer than 0x1FFF code points
            // of space. If a good use case from such a decomposition flavor arises, we can
            // dynamically change the bit masks so that the length mask becomes 0x1FFF instead
            // of 0xFFF and the all-non-starters mask becomes 0 instead of 0x1000. However,
            // since for now the masks are hard-coded, error out.
            return Err(NormalizerError::FutureExtension);
        }

        let ccc = icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions,
            supplementary_decompositions: Some(SupplementPayloadHolder::Uts46(
                supplementary_decompositions,
            )),
            tables,
            supplementary_tables: Some(supplementary_tables),
            ccc,
        })
    }

    /// Wraps a delegate iterator into a decomposing iterator
    /// adapter by using the data already held by this normalizer.
    pub fn normalize_iter<I: Iterator<Item = char>>(&self, iter: I) -> Decomposition<I> {
        Decomposition::new_with_supplements(
            iter,
            self.decompositions.get(),
            self.supplementary_decompositions.as_ref().map(|s| s.get()),
            self.tables.get(),
            self.supplementary_tables.as_ref().map(|s| s.get()),
            self.ccc.as_borrowed(),
            None,
        )
    }

    normalizer_methods!();
}

/// A normalizer for performing composing normalization.
pub struct ComposingNormalizer {
    decomposing_normalizer: DecomposingNormalizer,
    canonical_compositions: DataPayload<CanonicalCompositionsV1Marker>,
    potential_passthrough_and_not_backward_combining: PassthroughPayloadHolder,
}

impl ComposingNormalizer {
    /// NFC constructor.
    pub fn try_new_nfc<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + DataProvider<CanonicalCompositionPassthroughV1Marker>
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decomposing_normalizer = DecomposingNormalizer::try_new_nfd(data_provider)?;

        let canonical_compositions: DataPayload<CanonicalCompositionsV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let potential_passthrough_and_not_backward_combining: DataPayload<
            CanonicalCompositionPassthroughV1Marker,
        > = data_provider.load(Default::default())?.take_payload()?;

        Ok(ComposingNormalizer {
            decomposing_normalizer,
            canonical_compositions,
            potential_passthrough_and_not_backward_combining: PassthroughPayloadHolder::Canonical(
                potential_passthrough_and_not_backward_combining,
            ),
        })
    }

    /// NFKC constructor.
    pub fn try_new_nfkc<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + DataProvider<CompatibilityCompositionPassthroughV1Marker>
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decomposing_normalizer = DecomposingNormalizer::try_new_nfkd(data_provider)?;

        let canonical_compositions: DataPayload<CanonicalCompositionsV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let potential_passthrough_and_not_backward_combining: DataPayload<
            CompatibilityCompositionPassthroughV1Marker,
        > = data_provider.load(Default::default())?.take_payload()?;

        Ok(ComposingNormalizer {
            decomposing_normalizer,
            canonical_compositions,
            potential_passthrough_and_not_backward_combining:
                PassthroughPayloadHolder::Compatibility(
                    potential_passthrough_and_not_backward_combining,
                ),
        })
    }

    /// UTS 46 constructor
    ///
    /// This is a special building block normalization for IDNA that implements parts of the Map
    /// step and the following Normalize step. The caller is responsible for performing the
    /// "disallowed", "ignored", and "deviation" parts of the Map step before passing data to
    /// this normalizer such that disallowed and ignored characters aren't passed to this
    /// normalizer.
    ///
    /// This is ICU4C's UTS 46 normalization with two exceptions: characters that UTS 46 disallows
    /// and ICU4C maps to U+FFFD and characters that UTS 46 maps to the empty string normalize as
    /// in NFC in this normalization. Making the disallowed characters behave like this is beneficial
    /// to data size, and this normalizer implementation cannot deal with a character normalizing
    /// to the empty string, which doesn't happen in NFC or NFKC as of Unicode 14.
    ///
    /// Warning: In this normalization, U+0345 COMBINING GREEK YPOGEGRAMMENI exhibits a behavior
    /// that no character in Unicode exhibits in NFD, NFKD, NFC, or NFKC: Case folding turns
    /// U+0345 from a reordered character into a non-reordered character before reordering happens.
    /// Therefore, the output of this normalization may differ for different inputs that are
    /// canonically equivant with each other if they differ by how U+0345 is ordered relative
    /// to other reorderable characters.
    ///
    /// NOTE: This method remains experimental until suitability of this feature as part of
    /// IDNA processing has been demonstrated.
    #[cfg(any(test, feature = "experimental"))]
    pub fn try_new_uts46_without_ignored_and_disallowed<D>(
        data_provider: &D,
    ) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<Uts46DecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            // UTS 46 tables merged into CompatibilityDecompositionTablesV1Marker
            + DataProvider<CanonicalCompositionsV1Marker>
            + DataProvider<Uts46CompositionPassthroughV1Marker>
            + DataProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decomposing_normalizer =
            DecomposingNormalizer::try_new_uts46_decomposed_without_ignored_and_disallowed(
                data_provider,
            )?;

        let canonical_compositions: DataPayload<CanonicalCompositionsV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let potential_passthrough_and_not_backward_combining: DataPayload<
            Uts46CompositionPassthroughV1Marker,
        > = data_provider.load(Default::default())?.take_payload()?;

        Ok(ComposingNormalizer {
            decomposing_normalizer,
            canonical_compositions,
            potential_passthrough_and_not_backward_combining: PassthroughPayloadHolder::Uts46(
                potential_passthrough_and_not_backward_combining,
            ),
        })
    }

    /// Wraps a delegate iterator into a composing iterator
    /// adapter by using the data already held by this normalizer.
    pub fn normalize_iter<I: Iterator<Item = char>>(&self, iter: I) -> Composition<I> {
        Composition::new(
            Decomposition::new_with_supplements(
                iter,
                self.decomposing_normalizer.decompositions.get(),
                self.decomposing_normalizer
                    .supplementary_decompositions
                    .as_ref()
                    .map(|s| s.get()),
                self.decomposing_normalizer.tables.get(),
                self.decomposing_normalizer
                    .supplementary_tables
                    .as_ref()
                    .map(|s| s.get()),
                self.decomposing_normalizer.ccc.as_borrowed(),
                Some(PassthroughSet::new(
                    &self
                        .potential_passthrough_and_not_backward_combining
                        .get()
                        .trie,
                )),
            ),
            ZeroFrom::zero_from(&self.canonical_compositions.get().canonical_compositions),
        )
    }

    normalizer_methods!();
}

/// A struct for providing the raw canonical composition operation.
///
/// Callers should generally use `ComposingNormalizer` instead of this API.
/// However, this API is provided for callers such as HarfBuzz that specifically
/// want access to the raw canonical composition operation e.g. for use in a
/// glyph-availability-guided custom normalizer.
pub struct CanonicalComposition {
    canonical_compositions: DataPayload<CanonicalCompositionsV1Marker>,
}

impl CanonicalComposition {
    /// Performs canonical composition (including Hangul) on a pair of
    /// characters or returns `None` if these characters don't compose.
    /// Composition exclusions are taken into account.
    ///
    /// ```
    /// let data_provider = icu_testdata::get_provider();
    /// let comp = icu_normalizer::CanonicalComposition::try_new(&data_provider).unwrap();
    ///
    /// assert_eq!(comp.compose('a', 'b'), None); // Just two non-composing starters
    /// assert_eq!(comp.compose('a', '\u{0308}'), Some('ä'));
    /// assert_eq!(comp.compose('ẹ', '\u{0302}'), Some('ệ'));
    /// assert_eq!(comp.compose('𝅗', '𝅥'), None); // Composition exclusion
    /// assert_eq!(comp.compose('ে', 'া'), Some('ো')); // Second is starter
    /// assert_eq!(comp.compose('ᄀ', 'ᅡ'), Some('가')); // Hangul LV
    /// assert_eq!(comp.compose('가', 'ᆨ'), Some('각')); // Hangul LVT
    /// ```
    #[inline(always)]
    pub fn compose(&self, starter: char, second: char) -> Option<char> {
        compose(
            self.canonical_compositions
                .get()
                .canonical_compositions
                .iter(),
            starter,
            second,
        )
    }

    /// Construct from data provider.
    pub fn try_new<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalCompositionsV1Marker> + ?Sized,
    {
        let canonical_compositions: DataPayload<CanonicalCompositionsV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        Ok(CanonicalComposition {
            canonical_compositions,
        })
    }
}

/// The outcome of non-recursive canonical decomposition of a character.
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, PartialEq, Eq)]
pub enum Decomposed {
    /// The character is its own canonical decomposition.
    Default,
    /// The character decomposes to a single different character.
    Singleton(char),
    /// The character decomposes to two characters.
    Expansion(char, char),
}

/// A struct for providing the raw (non-recursive) canonical decomposition operation.
///
/// Callers should generally use `DecomposingNormalizer` instead of this API.
/// However, this API is provided for callers such as HarfBuzz that specifically
/// want access to non-recursive canonical decomposition e.g. for use in a
/// glyph-availability-guided custom normalizer.
pub struct CanonicalDecomposition {
    decompositions: DataPayload<CanonicalDecompositionDataV1Marker>,
    tables: DataPayload<CanonicalDecompositionTablesV1Marker>,
    non_recursive: DataPayload<NonRecursiveDecompositionSupplementV1Marker>,
}

impl CanonicalDecomposition {
    /// Performs non-recursive canonical decomposition (including for Hangul).
    ///
    /// ```
    ///     use icu_normalizer::Decomposed;
    ///     let data_provider = icu_testdata::get_provider();
    ///     let decomp = icu_normalizer::CanonicalDecomposition::try_new(&data_provider).unwrap();
    ///
    ///     assert_eq!(decomp.decompose('e'), Decomposed::Default);
    ///     assert_eq!(
    ///         decomp.decompose('ệ'),
    ///         Decomposed::Expansion('ẹ', '\u{0302}')
    ///     );
    ///     assert_eq!(decomp.decompose('각'), Decomposed::Expansion('가', 'ᆨ'));
    ///     assert_eq!(decomp.decompose('\u{212B}'), Decomposed::Singleton('Å')); // ANGSTROM SIGN
    ///     assert_eq!(decomp.decompose('\u{2126}'), Decomposed::Singleton('Ω')); // OHM SIGN
    ///     assert_eq!(decomp.decompose('\u{1F71}'), Decomposed::Singleton('ά')); // oxia
    /// ```
    #[inline]
    pub fn decompose(&self, c: char) -> Decomposed {
        let lvt = u32::from(c).wrapping_sub(HANGUL_S_BASE);
        if lvt >= HANGUL_S_COUNT {
            return self.decompose_non_hangul(c);
        }
        let t = lvt % HANGUL_T_COUNT;
        if t == 0 {
            let l = lvt / HANGUL_N_COUNT;
            let v = (lvt % HANGUL_N_COUNT) / HANGUL_T_COUNT;
            // Safe because values known to be in range
            return Decomposed::Expansion(
                unsafe { char::from_u32_unchecked(HANGUL_L_BASE + l) },
                unsafe { char::from_u32_unchecked(HANGUL_V_BASE + v) },
            );
        }
        let lv = lvt - t;
        // Safe because values known to be in range
        Decomposed::Expansion(
            unsafe { char::from_u32_unchecked(HANGUL_S_BASE + lv) },
            unsafe { char::from_u32_unchecked(HANGUL_T_BASE + t) },
        )
    }

    /// Performs non-recursive canonical decomposition except Hangul syllables
    /// are reported as `Decomposed::Default`.
    #[inline(always)]
    fn decompose_non_hangul(&self, c: char) -> Decomposed {
        let decomposition = self.decompositions.get().trie.get(u32::from(c));
        if decomposition == 0 {
            return Decomposed::Default;
        }
        // The loop is only broken out of as goto forward
        #[allow(clippy::never_loop)]
        loop {
            let high = (decomposition >> 16) as u16;
            let low = decomposition as u16;
            if high != 0 && low != 0 {
                // Decomposition into two BMP characters: starter and non-starter
                if in_inclusive_range(c, '\u{1F71}', '\u{1FFB}') {
                    // Look in the other trie due to oxia singleton
                    // mappings to corresponding character with tonos.
                    break;
                } else if c == '\u{212B}' {
                    // ANGSTROM SIGN
                    return Decomposed::Singleton('\u{00C5}');
                }
                return Decomposed::Expansion(char_from_u16(high), char_from_u16(low));
            }
            if high != 0 {
                // Decomposition into one BMP character or non-starter
                debug_assert_ne!(high, 1, "How come we got the U+FDFA NFKD marker here?");
                if high == 2 {
                    // Non-starter
                    if !in_inclusive_range(c, '\u{0340}', '\u{0F81}') {
                        return Decomposed::Default;
                    }
                    return match c {
                        '\u{0340}' => {
                            // COMBINING GRAVE TONE MARK
                            Decomposed::Singleton('\u{0300}')
                        }
                        '\u{0341}' => {
                            // COMBINING ACUTE TONE MARK
                            Decomposed::Singleton('\u{0301}')
                        }
                        '\u{0343}' => {
                            // COMBINING GREEK KORONIS
                            Decomposed::Singleton('\u{0313}')
                        }
                        '\u{0344}' => {
                            // COMBINING GREEK DIALYTIKA TONOS
                            Decomposed::Expansion('\u{0308}', '\u{0301}')
                        }
                        '\u{0F73}' => {
                            // TIBETAN VOWEL SIGN II
                            Decomposed::Expansion('\u{0F71}', '\u{0F72}')
                        }
                        '\u{0F75}' => {
                            // TIBETAN VOWEL SIGN UU
                            Decomposed::Expansion('\u{0F71}', '\u{0F74}')
                        }
                        '\u{0F81}' => {
                            // TIBETAN VOWEL SIGN REVERSED II
                            Decomposed::Expansion('\u{0F71}', '\u{0F80}')
                        }
                        _ => Decomposed::Default,
                    };
                }
                return Decomposed::Singleton(char_from_u16(high));
            }
            // Complex decomposition
            // Format for 16-bit value:
            // 15..13: length minus two for 16-bit case and length minus one for
            //         the 32-bit case. Length 8 needs to fit in three bits in
            //         the 16-bit case, and this way the value is future-proofed
            //         up to 9 in the 16-bit case. Zero is unused and length one
            //         in the 16-bit case goes directly into the trie.
            //     12: 1 if all trailing characters are guaranteed non-starters,
            //         0 if no guarantees about non-starterness.
            //         Note: The bit choice is this way around to allow for
            //         dynamically falling back to not having this but instead
            //         having one more bit for length by merely choosing
            //         different masks.
            //  11..0: Start offset in storage. The offset is to the logical
            //         sequence of scalars16, scalars32, supplementary_scalars16,
            //         supplementary_scalars32.
            let offset = usize::from(low & 0xFFF);
            let tables = self.tables.get();
            if offset < tables.scalars16.len() {
                if usize::from(low >> 13) != 0 {
                    // i.e. logical len isn't 2
                    break;
                }
                if let Some(first) = tables.scalars16.get(offset) {
                    if let Some(second) = tables.scalars16.get(offset + 1) {
                        // Two BMP starters
                        return Decomposed::Expansion(char_from_u16(first), char_from_u16(second));
                    }
                }
                // GIGO case
                debug_assert!(false);
                return Decomposed::Default;
            }
            let len = usize::from(low >> 13) + 1;
            if len > 2 {
                break;
            }
            let offset24 = offset - tables.scalars16.len();
            if let Some(first) = tables.scalars24.get(offset24) {
                let first_c = char_from_u24(first);
                if len == 1 {
                    return Decomposed::Singleton(first_c);
                }
                if let Some(second) = tables.scalars24.get(offset24 + 1) {
                    let second_c = char_from_u24(second);
                    return Decomposed::Expansion(first_c, second_c);
                }
            }
            // GIGO case
            debug_assert!(false);
            return Decomposed::Default;
        }
        let non_recursive = self.non_recursive.get();
        let non_recursive_decomposition = non_recursive.trie.get(u32::from(c));
        if non_recursive_decomposition == 0 {
            // GIGO case
            debug_assert!(false);
            return Decomposed::Default;
        }
        let high = (non_recursive_decomposition >> 16) as u16;
        let low = non_recursive_decomposition as u16;
        if high != 0 && low != 0 {
            // Decomposition into two BMP characters
            return Decomposed::Expansion(char_from_u16(high), char_from_u16(low));
        }
        if high != 0 {
            // Decomposition into one BMP character
            return Decomposed::Singleton(char_from_u16(high));
        }
        // Decomposition into two non-BMP characters
        // Low is offset into a table plus one to keep it non-zero.
        let offset = usize::from(low - 1);
        if let Some(first) = non_recursive.scalars24.get(offset) {
            if let Some(second) = non_recursive.scalars24.get(offset + 1) {
                return Decomposed::Expansion(char_from_u24(first), char_from_u24(second));
            }
        }
        // GIGO case
        debug_assert!(false);
        Decomposed::Default
    }

    /// Construct from data provider.
    pub fn try_new<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<NonRecursiveDecompositionSupplementV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;

        if tables.get().scalars16.len() + tables.get().scalars24.len() > 0xFFF {
            // The data is from a future where there exists a normalization flavor whose
            // complex decompositions take more than 0xFFF but fewer than 0x1FFF code points
            // of space. If a good use case from such a decomposition flavor arises, we can
            // dynamically change the bit masks so that the length mask becomes 0x1FFF instead
            // of 0xFFF and the all-non-starters mask becomes 0 instead of 0x1000. However,
            // since for now the masks are hard-coded, error out.
            return Err(NormalizerError::FutureExtension);
        }

        let non_recursive: DataPayload<NonRecursiveDecompositionSupplementV1Marker> =
            data_provider.load(Default::default())?.take_payload()?;

        Ok(CanonicalDecomposition {
            decompositions,
            tables,
            non_recursive,
        })
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests;
