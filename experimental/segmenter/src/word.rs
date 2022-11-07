// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::grapheme::GraphemeClusterSegmenter;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::provider::*;
use crate::rule_segmenter::*;
use crate::SegmenterError;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;
use icu_locid::{locale, Locale};
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

/// Word break iterator for an `str` (a UTF-8 string).
pub type WordBreakIteratorUtf8<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeUtf8>;

/// Word break iterator for a potentially invalid UTF-8 string
pub type WordBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    RuleBreakIterator<'l, 's, WordBreakTypePotentiallyIllFormedUtf8>;

/// Word break iterator for a Latin-1 (8-bit) string.
pub type WordBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeLatin1>;

/// Word break iterator for a UTF-16 string.
pub type WordBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, WordBreakTypeUtf16>;

/// Supports loading word break data, and creating word break iterators for different string
/// encodings.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
///
/// # Examples
///
/// Segment a string:
///
/// ```rust
/// use icu_segmenter::WordSegmenter;
/// let segmenter =
///     WordSegmenter::try_new_unstable(&icu_testdata::unstable())
///         .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 5, 6, 11]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu_segmenter::WordSegmenter;
/// let segmenter =
///     WordSegmenter::try_new_unstable(&icu_testdata::unstable())
///         .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 5, 6, 11]);
/// ```
pub struct WordSegmenter {
    payload: DataPayload<WordBreakDataV1Marker>,
    dictionary: Dictionary,
    lstm: LstmPayloads,
    grapheme: Option<GraphemeClusterSegmenter>,
}

impl WordSegmenter {
    /// Construct a [`WordSegmenter`].
    #[cfg(feature = "lstm")]
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let grapheme = GraphemeClusterSegmenter::try_new_unstable(provider).ok();

        let cj = Self::load_dictionary(provider, locale!("ja")).ok();

        let lstm = if cfg!(feature = "lstm") {
            let burmese = Self::load_lstm(provider, locale!("my")).ok();
            let khmer = Self::load_lstm(provider, locale!("lo")).ok();
            let lao = Self::load_lstm(provider, locale!("lo")).ok();
            let thai = Self::load_lstm(provider, locale!("th")).ok();
            LstmPayloads {
                burmese,
                khmer,
                lao,
                thai,
            }
        } else {
            LstmPayloads::default()
        };

        Ok(Self {
            payload,
            dictionary: Dictionary {
                burmese: None,
                khmer: None,
                lao: None,
                thai: None,
                cj,
            },
            lstm,
            grapheme,
        })
    }

    /// Construct a [`WordSegmenter`].
    #[cfg(not(feature = "lstm"))]
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let grapheme = GraphemeClusterSegmenter::try_new_unstable(provider).ok();

        let dictionary = if cfg!(feature = "lstm") {
            let cj = Self::load_dictionary(provider, locale!("ja")).ok();
            Dictionary {
                burmese: None,
                khmer: None,
                lao: None,
                thai: None,
                cj,
            }
        } else {
            let cj = Self::load_dictionary(provider, locale!("ja")).ok();
            let burmese = Self::load_dictionary(provider, locale!("my")).ok();
            let khmer = Self::load_dictionary(provider, locale!("km")).ok();
            let lao = Self::load_dictionary(provider, locale!("lo")).ok();
            let thai = Self::load_dictionary(provider, locale!("th")).ok();
            Dictionary {
                burmese,
                khmer,
                lao,
                thai,
                cj,
            }
        };

        Ok(Self {
            payload,
            dictionary,
            lstm: LstmPayloads::default(),
            grapheme,
        })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: SegmenterError);

    fn load_dictionary<D: DataProvider<UCharDictionaryBreakDataV1Marker> + ?Sized>(
        provider: &D,
        locale: Locale,
    ) -> Result<DataPayload<UCharDictionaryBreakDataV1Marker>, DataError> {
        provider
            .load(DataRequest {
                locale: &DataLocale::from(locale),
                metadata: Default::default(),
            })?
            .take_payload()
    }

    #[cfg(feature = "lstm")]
    fn load_lstm<D: DataProvider<LstmDataV1Marker> + ?Sized>(
        provider: &D,
        locale: Locale,
    ) -> Result<DataPayload<LstmDataV1Marker>, DataError> {
        provider
            .load(DataRequest {
                locale: &DataLocale::from(locale),
                metadata: Default::default(),
            })?
            .take_payload()
    }

    /// Create a word break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> WordBreakIteratorUtf8<'l, 's> {
        WordBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.as_ref(),
        }
    }

    /// Create a word break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    pub fn segment_utf8<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> WordBreakIteratorPotentiallyIllFormedUtf8<'l, 's> {
        WordBreakIteratorPotentiallyIllFormedUtf8 {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.as_ref(),
        }
    }

    /// Create a word break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(&'l self, input: &'s [u8]) -> WordBreakIteratorLatin1<'l, 's> {
        WordBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.as_ref(),
        }
    }

    /// Create a word break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> WordBreakIteratorUtf16<'l, 's> {
        WordBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.as_ref(),
        }
    }
}

pub struct WordBreakTypeUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().len_utf8()
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        handle_complex_language_utf8(iter, left_codepoint)
    }
}
pub struct WordBreakTypePotentiallyIllFormedUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypePotentiallyIllFormedUtf8 {
    type IterAttr = Utf8CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().len_utf8()
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        handle_complex_language_utf8(iter, left_codepoint)
    }
}

/// handle_complex_language impl for UTF8 iterators
fn handle_complex_language_utf8<'l, 's, T>(
    iter: &mut RuleBreakIterator<'l, 's, T>,
    left_codepoint: T::CharType,
) -> Option<usize>
where
    T: RuleBreakType<'l, 's, CharType = char>,
{
    // word segmenter doesn't define break rules for some languages such as Thai.
    let start_iter = iter.iter.clone();
    let start_point = iter.current_pos_data;
    let mut s = String::new();
    s.push(left_codepoint);
    loop {
        s.push(iter.get_current_codepoint());
        iter.advance_iter();
        if iter.is_eof() {
            break;
        }
        if iter.get_current_break_property() != iter.data.complex_property {
            break;
        }
    }

    // Restore iterator to move to head of complex string
    iter.iter = start_iter;
    iter.current_pos_data = start_point;
    let breaks = complex_language_segment_str(iter.dictionary, iter.lstm, iter.grapheme, &s);
    iter.result_cache = breaks;
    let first_pos = *iter.result_cache.first()?;
    let mut i = iter.get_current_codepoint().len_utf8();
    loop {
        if i == first_pos {
            // Re-calculate breaking offset
            iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
            return Some(iter.get_current_position());
        }
        iter.advance_iter();
        if iter.is_eof() {
            iter.result_cache.clear();
            return Some(iter.len);
        }
        i += T::get_current_position_character_len(iter);
    }
}

pub struct WordBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.get_current_codepoint();
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = iter.iter.clone();
        let start_point = iter.current_pos_data;
        let mut s = vec![left_codepoint as u16];
        loop {
            s.push(iter.get_current_codepoint() as u16);
            iter.advance_iter();
            if iter.is_eof() {
                break;
            }
            if iter.get_current_break_property() != iter.data.complex_property {
                break;
            }
        }

        // Restore iterator to move to head of complex string
        iter.iter = start_iter;
        iter.current_pos_data = start_point;
        let breaks = complex_language_segment_utf16(iter.dictionary, iter.lstm, iter.grapheme, &s);
        let mut i = 1;
        iter.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        let first_pos = *iter.result_cache.first()?;
        loop {
            if i == first_pos {
                // Re-calculate breaking offset
                iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
                return Some(iter.get_current_position());
            }
            iter.advance_iter();
            if iter.is_eof() {
                iter.result_cache.clear();
                return Some(iter.len);
            }
            i += 1;
        }
    }
}
