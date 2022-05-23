// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

//! `icu_normalizer` is one of the ICU4X components.
//!
//! This API provides necessary functionality for normalizing text into Unicode
//! Normalization Forms.
//!
//! # Implementation notes
//!
//! The normalizer data layout is not based on the ICU4C design at all. Instead, the normalization
//! data layout is a clean-slate design optimized for the concept of fusing the NFD decomposition
//! into the collator. That is, the normalizer proper is a by-product of the collator-motivated
//! data layout and the design isn’t informed by NFC needs. Instead, NFC is treated as a separate
//! project that is assumed to build on top of the NFD data, but beyond vaguely assuming that
//! that’s possible (from the spec definitions, it has to be at least in non-performance-optimized
//! sense), the needs of NFC haven’t informed the NFD data layout at all.
//!
//! Notably, the data structure is optimized for a starter decomposing to itself, which is the
//! most common case, and for a starter decomposing to a starter and a non-starter. Notably, in
//! this case, the collator in particular makes use of the knowledge that the second character
//! of such a decomposition is a non-starter. Therefore, decomposition into two starters is
//! handled by generic fallback path that looks the decomposion from an array by offset and length
//! instead of baking a BMP character pair directly into a trie value.
//!
//! The normalizer operates on a lazy iterator over Unicode scalar values (Rust `char`) internally
//! and iterating over guaranteed-valid UTF-8, potentially-invalid UTF-8, and potentially-invalid
//! UTF-16 is a step that doesn’t leak into the normalizer internals. UTF errors are treated as
//! U+FFFD.
//!
//! Since the normalizer produces output with `char` values read from the data, UB ensues if
//! invalid data with values outside the scalar value range is used. TODO: data validation.
//!
//! The decompositions of non-starters are hard-coded. At present in Unicode, these appear
//! to be special cases falling into two categories:
//!
//! 1. Deprecated Greek combining marks.
//! 2. Particular Tibetan vowel sings.
//!
//! Hopefully Unicode never adds more decomposing non-starters, but if it does, a code update
//! is needed instead of a mere data update.

extern crate alloc;

pub mod error;
pub mod provider;

use crate::error::NormalizerError;
use crate::provider::CanonicalDecompositionDataV1;
use crate::provider::CanonicalDecompositionDataV1Marker;
use alloc::string::String;
use alloc::vec::Vec;
use core::char::{decode_utf16, DecodeUtf16Error, REPLACEMENT_CHARACTER};
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::CanonicalCombiningClassV1Marker;
use icu_properties::CanonicalCombiningClass;
use icu_provider::DataPayload;
use icu_provider::DataRequest;
use icu_provider::ResourceProvider;
use smallvec::SmallVec;
use utf8_iter::Utf8CharsEx;
use zerovec::ule::AsULE;

// These constants originate from page 143 of Unicode 14.0
/// Syllable base
const HANGUL_S_BASE: u32 = 0xAC00;
/// Lead jamo base
const HANGUL_L_BASE: u32 = 0x1100;
/// Vowel jamo base
const HANGUL_V_BASE: u32 = 0x1161;
/// Trail jamo base
const HANGUL_T_BASE: u32 = 0x11A7;
/// Trail jamo count
const HANGUL_T_COUNT: u32 = 28;
/// Vowel jamo count times trail jamo count
const HANGUL_N_COUNT: u32 = 588;
/// Syllable count
const HANGUL_S_COUNT: u32 = 11172;

#[inline(always)]
fn char_from_u32(u: u32) -> char {
    if let Some(c) = core::char::from_u32(u) {
        c
    } else {
        debug_assert!(false);
        REPLACEMENT_CHARACTER
    }
}

#[inline(always)]
fn char_from_u16(u: u16) -> char {
    char_from_u32(u32::from(u))
}

#[inline(always)]
fn in_inclusive_range(c: char, start: char, end: char) -> bool {
    u32::from(c).wrapping_sub(u32::from(start)) <= (u32::from(end) - u32::from(start))
}

// Hoisted to function, because the compiler doesn't like having
// two identical closures.
#[inline(always)]
fn utf16_error_to_replacement(r: Result<char, DecodeUtf16Error>) -> char {
    r.unwrap_or(REPLACEMENT_CHARACTER)
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
    pub fn set_ccc_from_trie(&mut self, ccc_trie: &CodePointTrie<CanonicalCombiningClass>) {
        debug_assert_eq!(self.0 >> 24, 0, "This method has already been called!");
        self.0 |= (ccc_trie.get(self.0).0 as u32) << 24;
    }
}

// This function exists as a borrow check helper.
#[inline(always)]
fn sort_slice_by_ccc<'data>(
    slice: &mut [CharacterAndClass],
    ccc: &CodePointTrie<'data, CanonicalCombiningClass>,
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
/// a lazily-decomposed (currently only NFD though NFKD should also
/// be possible) `char` sequence.
pub struct Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    delegate: I,
    buffer: SmallVec<[CharacterAndClass; 10]>, // TODO Figure out good length
    /// The index of the next item to be read from `buffer`.
    /// The purpose if this index is to avoid having to move
    /// the rest upon every read.
    buffer_pos: usize,
    pending_unnormalized_starter: Option<char>, // None at end of stream
    decompositions: &'data CanonicalDecompositionDataV1<'data>,
    ccc: &'data CodePointTrie<'data, CanonicalCombiningClass>,
}

impl<'data, I> Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    /// Constructs a decomposing iterator adapter from a delegate
    /// iterator and references to the necessary data.
    ///
    /// Use `DecomposingNormalizer::normalize_iter()` instead unless
    /// there's a good reason to use this constructor directly.
    pub fn new(
        delegate: I,
        decompositions: &'data CanonicalDecompositionDataV1,
        ccc: &'data CodePointTrie<'data, CanonicalCombiningClass>,
    ) -> Self {
        let mut ret = Decomposition::<I> {
            delegate,
            buffer: SmallVec::new(), // Normalized
            buffer_pos: 0,
            // Initialize with a placeholder starter in case
            // the real stream starts with a non-starter.
            pending_unnormalized_starter: Some('\u{FFFF}'),
            decompositions,
            ccc,
        };
        let _ = ret.next(); // Remove the U+FFFF placeholder
        ret
    }
}

impl<'data, I> Iterator for Decomposition<'data, I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.buffer_pos < self.buffer.len() {
            let ret = self.buffer[self.buffer_pos].character();
            self.buffer_pos += 1;
            if self.buffer_pos == self.buffer.len() {
                self.buffer.clear();
                self.buffer_pos = 0;
            }
            return Some(ret);
        }
        debug_assert_eq!(self.buffer_pos, 0);
        let c = self.pending_unnormalized_starter.take()?;
        let (starter, combining_start) = {
            let hangul_offset = u32::from(c).wrapping_sub(HANGUL_S_BASE); // SIndex in the spec
            if hangul_offset >= HANGUL_S_COUNT {
                let decomposition = self.decompositions.trie.get(u32::from(c));
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
                        // Decomposition into one BMP character
                        let starter = char_from_u16(high);
                        (starter, 0)
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
                            let (&first, tail) = &self.decompositions.scalars16.as_ule_slice()
                                [offset..offset + len]
                                .split_first()
                                .unwrap();
                            // Starter
                            let starter = char_from_u16(u16::from_unaligned(first));
                            if low & 0x800 == 0 {
                                // All the rest are combining
                                for &ule in tail.iter() {
                                    self.buffer.push(CharacterAndClass::new(char_from_u16(
                                        u16::from_unaligned(ule),
                                    )));
                                }
                                (starter, 0)
                            } else {
                                let mut i = 0;
                                let mut combining_start = 0;
                                for &ule in tail.iter() {
                                    let ch = char_from_u16(u16::from_unaligned(ule));
                                    self.buffer.push(CharacterAndClass::new(ch));
                                    i += 1;
                                    if !self
                                        .decompositions
                                        .decomposition_starts_with_non_starter
                                        .contains(ch)
                                    {
                                        combining_start = i;
                                    }
                                }
                                (starter, combining_start)
                            }
                        } else {
                            let (&first, tail) = &self.decompositions.scalars32.as_ule_slice()
                                [offset..offset + len]
                                .split_first()
                                .unwrap();
                            let starter = char_from_u32(u32::from_unaligned(first));
                            if low & 0x800 == 0 {
                                // All the rest are combining
                                for &ule in tail.iter() {
                                    self.buffer.push(CharacterAndClass::new(char_from_u32(
                                        u32::from_unaligned(ule),
                                    )));
                                }
                                (starter, 0)
                            } else {
                                let mut i = 0;
                                let mut combining_start = 0;
                                for &ule in tail.iter() {
                                    let ch = char_from_u32(u32::from_unaligned(ule));
                                    self.buffer.push(CharacterAndClass::new(ch));
                                    i += 1;
                                    if !self
                                        .decompositions
                                        .decomposition_starts_with_non_starter
                                        .contains(ch)
                                    {
                                        combining_start = i;
                                    }
                                }
                                (starter, combining_start)
                            }
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
        debug_assert_eq!(self.pending_unnormalized_starter, None);
        for ch in self.delegate.by_ref() {
            if self
                .decompositions
                .decomposition_starts_with_non_starter
                .contains(ch)
            {
                if !in_inclusive_range(ch, '\u{0340}', '\u{0F81}') {
                    self.buffer.push(CharacterAndClass::new(ch));
                } else {
                    // The Tibetan special cases are starters that decompose into non-starters.
                    //
                    // Logically the canonical combining class of each special case is known
                    // at compile time, but all characters in the buffer are treated the same
                    // when looking up the canonical combining class to avoid a per-character
                    // branch that would only benefit these rare special cases.
                    match ch {
                        '\u{0340}' => {
                            // COMBINING GRAVE TONE MARK
                            self.buffer.push(CharacterAndClass::new('\u{0300}'));
                        }
                        '\u{0341}' => {
                            // COMBINING ACUTE TONE MARK
                            self.buffer.push(CharacterAndClass::new('\u{0301}'));
                        }
                        '\u{0343}' => {
                            // COMBINING GREEK KORONIS
                            self.buffer.push(CharacterAndClass::new('\u{0313}'));
                        }
                        '\u{0344}' => {
                            // COMBINING GREEK DIALYTIKA TONOS
                            self.buffer.push(CharacterAndClass::new('\u{0308}'));
                            self.buffer.push(CharacterAndClass::new('\u{0301}'));
                        }
                        '\u{0F73}' => {
                            // TIBETAN VOWEL SIGN II
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            self.buffer.push(CharacterAndClass::new('\u{0F72}'));
                        }
                        '\u{0F75}' => {
                            // TIBETAN VOWEL SIGN UU
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            self.buffer.push(CharacterAndClass::new('\u{0F74}'));
                        }
                        '\u{0F81}' => {
                            // TIBETAN VOWEL SIGN REVERSED II
                            self.buffer.push(CharacterAndClass::new('\u{0F71}'));
                            self.buffer.push(CharacterAndClass::new('\u{0F80}'));
                        }
                        _ => {
                            self.buffer.push(CharacterAndClass::new(ch));
                        }
                    };
                }
            } else {
                self.pending_unnormalized_starter = Some(ch);
                break;
            }
        }

        sort_slice_by_ccc(&mut self.buffer[combining_start..], self.ccc);

        Some(starter)
    }
}

/// A normalizer for performing decomposing normalization (currently only NFD
/// but NFKD expected in the future).
pub struct DecomposingNormalizer {
    decompositions: DataPayload<CanonicalDecompositionDataV1Marker>,
    ccc: DataPayload<CanonicalCombiningClassV1Marker>,
}

impl DecomposingNormalizer {
    /// NFD constructor.
    pub fn try_new<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: ResourceProvider<CanonicalDecompositionDataV1Marker>
            + ResourceProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;

        let ccc: DataPayload<CanonicalCombiningClassV1Marker> =
            icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions,
            ccc,
        })
    }

    /// Wraps a delegate iterator into a decomposing iterator
    /// adapter by using the data already held by this normalizer.
    pub fn normalize_iter<I: Iterator<Item = char>>(&self, iter: I) -> Decomposition<I> {
        Decomposition::new(
            iter,
            self.decompositions.get(),
            &self.ccc.get().code_point_trie,
        )
    }

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
        let mut buf = [0u16; 2];
        let mut ret = Vec::new();
        for c in
            self.normalize_iter(decode_utf16(text.iter().copied()).map(utf16_error_to_replacement))
        {
            ret.extend_from_slice(c.encode_utf16(&mut buf));
        }
        ret
    }

    /// Normalize a slice of potentially-invalid UTF-16 into a `Write` sink.
    ///
    /// Unpaired surrogates are mapped to the REPLACEMENT CHARACTER
    /// before normalizing.
    pub fn normalize_utf16_to<W: core::fmt::Write + ?Sized>(
        &self,
        text: &[u16],
        sink: &mut W,
    ) -> core::fmt::Result {
        for c in
            self.normalize_iter(decode_utf16(text.iter().copied()).map(utf16_error_to_replacement))
        {
            sink.write_char(c)?;
        }
        Ok(())
    }

    /// Checks whether a slice of potentially-invalid UTF-16 is normalized.
    ///
    /// Unpaired surrogates are treated as the REPLACEMENT CHARACTER.
    pub fn is_normalized_utf16(&self, text: &[u16]) -> bool {
        self.normalize_iter(decode_utf16(text.iter().copied()).map(utf16_error_to_replacement))
            .eq(decode_utf16(text.iter().copied()).map(utf16_error_to_replacement))
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
}

#[cfg(test)]
mod tests {
    use crate::DecomposingNormalizer;

    #[test]
    fn test_basic() {
        let data_provider = icu_testdata::get_provider();

        let normalizer: DecomposingNormalizer =
            DecomposingNormalizer::try_new(&data_provider).unwrap();
        assert_eq!(normalizer.normalize("ä"), "a\u{0308}");
    }
}
