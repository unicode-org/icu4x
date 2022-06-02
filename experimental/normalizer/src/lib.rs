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
//! to be special cases falling into three categories:
//!
//! 1. Deprecated Greek combining marks.
//! 2. Particular Tibetan vowel sings.
//! 3. NFKD only: half-width kana voicing marks.
//!
//! Hopefully Unicode never adds more decomposing non-starters, but if it does, a code update
//! is needed instead of a mere data update.
//!
//! # Data size considerations
//!
//! The normalizer supports three flavors: NFD, NFKD, and the decomposed counterpart of
//! NFKC_CaseFold without ignoring default ignorables. Logically NFKD adds data on top of NFD
//! and case fold adds data on top of NFKD (some of the details may be a bit more complicated).
//!
//! Currently, the complex decomposition expansion data is consolidated such that there is one
//! data struct that contains the expansions needed by NFD and there's another data instance
//! that contains the expansions for both NFKD and fusion of case fold and NFKD.
//!
//! However, similar consolidation hasn't been applied to the trie or to the set of characters
//! whose decompositions starts with a non-starter. These are tradeoffs between data size and
//! run-time branchiness. It is unclear if the present situation is the right call.
//!
//! As of Unicode 14, the set of characters whose decompositions starts with a non-starter
//! is almost the same all three cases. NFKD adds two characters to the set compared to NFD:
//! the half-width kana voicing marks. The case fold variant then removes one character
//! compared to regular NFKD: U+0345 COMBINING GREEK YPOGEGRAMMENI.
//!
//! There are three alternatives to the current solution of having three pre-computed sets.
//!
//! First, we could add explicit checks for the chosen normalization form and these three
//! characters to the set lookup, which would add branches to each set lookup and would
//! hard-code the assumption that only these three characters may differ between the
//! normalization forms. Intuitively, it seems like a reasonable guess that these three
//! characters are all quite unusual and it's unlikely that Unicode would add more
//! characters that would make the sets differ in the future.
//!
//! Second, we could store two small sets: additions and removals relative to the base set.
//! However, using these would involve a heap allocation for the computed actual set.
//! In a multi-process architecture when using crabbake, having each process carry its
//! own heap allocation for the lifetime of the process would be worse than making the
//! static data larger.
//!
//! The third option would be to make the run-time lookup from three sets: the base,
//! additions, and removals. Since the additions and removals would be empty sets for NFD,
//! chances are that this would be less branchy in the NFD case than the first option,
//! especially if we made `UnicodeSet` be able to be shallow-copied in a way that borrows
//! its `ZeroVec` instead of making an owning copy of of the wrapped `ZeroVec`.
//!
//! As for the trie, the alternative would be to have two levels of tries: supplementary
//! and base where base would be the NFD trie and the default (zero) value from the
//! supplementary trie would mean falling back to the base trie.
//!
//! Compared to the current expansion supplement arrangement, this would differ in terms
//! of run-time cost. In the NFD case, the expansion supplement adds one branch for the
//! case where a character has a non-self supplementary-plane decomposition, which is
//! rare. In contrast, having the ability to make a lookup for supplementary trie would
//! mean having to branch on the presence of the supplementary trie for each starter.
//! For the normalization forms that would use the supplementary trie, each starter
//! would go through two full trie lookups instead of one.

extern crate alloc;

pub mod error;
pub mod provider;
pub mod u24;

use crate::error::NormalizerError;
use crate::provider::CanonicalDecompositionDataV1Marker;
use crate::provider::CompatibilityDecompositionDataV1Marker;
use crate::provider::DecompositionDataV1;
#[cfg(test)]
use crate::provider::Uts46DecompositionDataV1Marker;
use alloc::string::String;
use alloc::vec::Vec;
use core::char::{decode_utf16, DecodeUtf16Error, REPLACEMENT_CHARACTER};
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::CanonicalCombiningClassV1Marker;
use icu_properties::CanonicalCombiningClass;
use icu_provider::DataPayload;
use icu_provider::DataRequest;
use icu_provider::ResourceProvider;
use icu_uniset::UnicodeSet;
use provider::CanonicalDecompositionTablesV1Marker;
use provider::CompatibilityDecompositionTablesV1Marker;
use provider::DecompositionTablesV1;
use smallvec::SmallVec;
use u24::EMPTY_U24;
use u24::U24;
use utf8_iter::Utf8CharsEx;
use zerofrom::ZeroFrom;
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

enum PayloadHolder {
    Canonical(DataPayload<CanonicalDecompositionDataV1Marker>),
    Compatibility(DataPayload<CompatibilityDecompositionDataV1Marker>),
    #[cfg(test)]
    Uts46(DataPayload<Uts46DecompositionDataV1Marker>),
}

impl PayloadHolder {
    fn get(&self) -> &DecompositionDataV1 {
        match self {
            PayloadHolder::Canonical(d) => d.get(),
            PayloadHolder::Compatibility(d) => d.get(),
            #[cfg(test)]
            PayloadHolder::Uts46(d) => d.get(),
        }
    }
}

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
/// Trail jamo base
const HANGUL_T_BASE: u32 = 0x11A7;
/// Trail jamo count
const HANGUL_T_COUNT: u32 = 28;
/// Vowel jamo count times trail jamo count
const HANGUL_N_COUNT: u32 = 588;
/// Syllable count
const HANGUL_S_COUNT: u32 = 11172;

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
fn split_first_u24(s: Option<&ZeroSlice<U24>>) -> (char, &ZeroSlice<U24>) {
    if let Some(slice) = s {
        if let Some(first) = slice.first() {
            // `unwrap()` must succeed, because `first()` returned `Some`.
            return (
                char_from_u24(first),
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
    trie: &'data CodePointTrie<'data, u32>,
    decomposition_starts_with_non_starter: UnicodeSet<'data>,
    scalars16: &'data ZeroSlice<u16>,
    scalars24: &'data ZeroSlice<U24>,
    supplementary_scalars16: &'data ZeroSlice<u16>,
    supplementary_scalars24: &'data ZeroSlice<U24>,
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
        decompositions: &'data DecompositionDataV1,
        tables: &'data DecompositionTablesV1,
        supplementary_tables: Option<&'data DecompositionTablesV1>,
        ccc: &'data CodePointTrie<'data, CanonicalCombiningClass>,
    ) -> Self {
        let mut ret = Decomposition::<I> {
            delegate,
            buffer: SmallVec::new(), // Normalized
            buffer_pos: 0,
            // Initialize with a placeholder starter in case
            // the real stream starts with a non-starter.
            pending_unnormalized_starter: Some('\u{FFFF}'),
            trie: &decompositions.trie,
            decomposition_starts_with_non_starter: UnicodeSet::zero_from(
                &decompositions.decomposition_starts_with_non_starter,
            ),
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
                if !self.decomposition_starts_with_non_starter.contains(ch) {
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
                if !self.decomposition_starts_with_non_starter.contains(ch) {
                    combining_start = i;
                }
            }
            (starter, combining_start)
        }
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
                let decomposition = self.trie.get(u32::from(c));
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
        debug_assert_eq!(self.pending_unnormalized_starter, None);
        for ch in self.delegate.by_ref() {
            if self.decomposition_starts_with_non_starter.contains(ch) {
                if !(in_inclusive_range(ch, '\u{0340}', '\u{0F81}')
                    || (u32::from(ch) & !1 == 0xFF9E))
                {
                    self.buffer.push(CharacterAndClass::new(ch));
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
                        '\u{FF9E}' => {
                            // HALFWIDTH KATAKANA VOICED SOUND MARK
                            // Compatibility decomposition only; can't come here
                            // in NFD.
                            self.buffer.push(CharacterAndClass::new('\u{3099}'));
                        }
                        '\u{FF9F}' => {
                            // HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
                            // Compatibility decomposition only; can't come here
                            // in NFD.
                            self.buffer.push(CharacterAndClass::new('\u{309A}'));
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

/// A normalizer for performing decomposing normalization.
pub struct DecomposingNormalizer {
    decompositions: PayloadHolder,
    tables: DataPayload<CanonicalDecompositionTablesV1Marker>,
    supplementary_tables: Option<DataPayload<CompatibilityDecompositionTablesV1Marker>>,
    ccc: DataPayload<CanonicalCombiningClassV1Marker>,
}

impl DecomposingNormalizer {
    /// NFD constructor.
    pub fn try_new_nfd<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: ResourceProvider<CanonicalDecompositionDataV1Marker>
            + ResourceProvider<CanonicalDecompositionTablesV1Marker>
            + ResourceProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CanonicalDecompositionDataV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;

        if tables.get().scalars16.len() + tables.get().scalars24.len() > 0xFFF {
            // The data is from a future where there exists a normalization flavor whose
            // complex decompositions take more than 0xFFF but fewer than 0x1FFF code points
            // of space. If a good use case from such a decomposition flavor arises, we can
            // dynamically change the bit masks so that the length mask becomes 0x1FFF instead
            // of 0xFFF and the all-non-starters mask becomes 0 instead of 0x1000. However,
            // since for now the masks are hard-coded, error out.
            return Err(NormalizerError::FutureExtension);
        }

        let ccc: DataPayload<CanonicalCombiningClassV1Marker> =
            icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions: PayloadHolder::Canonical(decompositions),
            tables,
            supplementary_tables: None,
            ccc,
        })
    }

    /// NFKD constructor.
    pub fn try_new_nfkd<D>(data_provider: &D) -> Result<Self, NormalizerError>
    where
        D: ResourceProvider<CompatibilityDecompositionDataV1Marker>
            + ResourceProvider<CanonicalDecompositionTablesV1Marker>
            + ResourceProvider<CompatibilityDecompositionTablesV1Marker>
            + ResourceProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<CompatibilityDecompositionDataV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        let supplementary_tables: DataPayload<CompatibilityDecompositionTablesV1Marker> =
            data_provider
                .load_resource(&DataRequest::default())?
                .take_payload()?;

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

        let ccc: DataPayload<CanonicalCombiningClassV1Marker> =
            icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions: PayloadHolder::Compatibility(decompositions),
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
    /// This constructor exists only in the test mode in order to allow this data to be tested
    /// in isolation of the canonical composition step.
    #[cfg(test)]
    fn try_new_uts46_decomposed_without_ignored_and_disallowed<D>(
        data_provider: &D,
    ) -> Result<Self, NormalizerError>
    where
        D: ResourceProvider<Uts46DecompositionDataV1Marker>
            + ResourceProvider<CanonicalDecompositionTablesV1Marker>
            + ResourceProvider<CompatibilityDecompositionTablesV1Marker>
            // UTS 46 tables merged into CompatibilityDecompositionTablesV1Marker
            + ResourceProvider<icu_properties::provider::CanonicalCombiningClassV1Marker>
            + ?Sized,
    {
        let decompositions: DataPayload<Uts46DecompositionDataV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        let tables: DataPayload<CanonicalDecompositionTablesV1Marker> = data_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        let supplementary_tables: DataPayload<CompatibilityDecompositionTablesV1Marker> =
            data_provider
                .load_resource(&DataRequest::default())?
                .take_payload()?;

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

        let ccc: DataPayload<CanonicalCombiningClassV1Marker> =
            icu_properties::maps::get_canonical_combining_class(data_provider)?;

        Ok(DecomposingNormalizer {
            decompositions: PayloadHolder::Uts46(decompositions),
            tables,
            supplementary_tables: Some(supplementary_tables),
            ccc,
        })
    }

    /// Wraps a delegate iterator into a decomposing iterator
    /// adapter by using the data already held by this normalizer.
    pub fn normalize_iter<I: Iterator<Item = char>>(&self, iter: I) -> Decomposition<I> {
        Decomposition::new(
            iter,
            self.decompositions.get(),
            self.tables.get(),
            self.supplementary_tables.as_ref().map(|s| s.get()),
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

#[cfg(all(test, feature = "serde"))]
mod tests;
