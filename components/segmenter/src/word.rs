// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::iterator_helpers::derive_usize_iterator_with_type;
use crate::provider::*;
use crate::rule_segmenter::*;
use crate::SegmenterError;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

/// Implements the [`Iterator`] trait over the word boundaries of the given string.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the segmenter object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// The [`Iterator::Item`] is an [`usize`] representing index of a code unit
/// _after_ the boundary (for a boundary at the end of text, this index is the length
/// of the [`str`] or array of code units).
///
/// For examples of use, see [`WordSegmenter`].
#[derive(Debug)]
pub struct WordBreakIterator<'l, 's, Y: RuleBreakType<'l, 's> + ?Sized>(
    RuleBreakIterator<'l, 's, Y>,
);

derive_usize_iterator_with_type!(WordBreakIterator);

/// The word type tag that is returned by [`WordBreakIterator::word_type()`].
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum WordType {
    /// No category tag.
    None = 0,
    /// Number category tag.
    Number = 1,
    /// Letter category tag, including CJK.
    Letter = 2,
}

impl<'l, 's, Y: RuleBreakType<'l, 's> + ?Sized> WordBreakIterator<'l, 's, Y> {
    /// Returns the word type of the segment preceding the current boundary.
    #[inline]
    pub fn word_type(&self) -> WordType {
        match self.0.rule_status() {
            RuleStatusType::None => WordType::None,
            RuleStatusType::Number => WordType::Number,
            RuleStatusType::Letter => WordType::Letter,
        }
    }
    /// Returns `true` when the segment preceding the current boundary is word-like,
    /// such as letter, number, or CJK.
    #[inline]
    pub fn is_word_like(&self) -> bool {
        self.0.is_word_like()
    }
}

/// Word break iterator for an `str` (a UTF-8 string).
///
/// For examples of use, see [`WordSegmenter`].
pub type WordBreakIteratorUtf8<'l, 's> = WordBreakIterator<'l, 's, WordBreakTypeUtf8>;

/// Word break iterator for a potentially invalid UTF-8 string.
///
/// For examples of use, see [`WordSegmenter`].
pub type WordBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    WordBreakIterator<'l, 's, WordBreakTypePotentiallyIllFormedUtf8>;

/// Word break iterator for a Latin-1 (8-bit) string.
///
/// For examples of use, see [`WordSegmenter`].
pub type WordBreakIteratorLatin1<'l, 's> = WordBreakIterator<'l, 's, RuleBreakTypeLatin1>;

/// Word break iterator for a UTF-16 string.
///
/// For examples of use, see [`WordSegmenter`].
pub type WordBreakIteratorUtf16<'l, 's> = WordBreakIterator<'l, 's, WordBreakTypeUtf16>;

/// Supports loading word break data, and creating word break iterators for different string
/// encodings.
///
/// # Examples
///
/// Segment a string:
///
/// ```rust
/// use icu_segmenter::WordSegmenter;
/// let segmenter = WordSegmenter::new_auto();
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
/// let segmenter = WordSegmenter::new_auto();
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 5, 6, 11]);
/// ```
///
/// Successive boundaries can be used to retrieve the segments.
/// In particular, the first boundary is always 0, and the last one is the
/// length of the segmented text in code units.
///
/// ```rust
/// # use icu_segmenter::WordSegmenter;
/// # let segmenter = WordSegmenter::new_auto();
/// use itertools::Itertools;
/// let text = "Mark’d ye his words?";
/// let segments: Vec<&str> = segmenter
///     .segment_str(text)
///     .tuple_windows()
///     .map(|(i, j)| &text[i..j])
///     .collect();
/// assert_eq!(
///     &segments,
///     &["Mark’d", " ", "ye", " ", "his", " ", "words", "?"]
/// );
/// ```
///
/// Not all segments delimited by word boundaries are words; some are interword
/// segments such as spaces and punctuation.
/// The [`WordBreakIterator::word_type()`] of a boundary can be used to
/// classify the preceding segment.
/// ```rust
/// # use itertools::Itertools;
/// # use icu_segmenter::{WordType, WordSegmenter};
/// # let segmenter = WordSegmenter::new_auto();
/// # let text = "Mark’d ye his words?";
/// let words: Vec<&str> = {
///     let mut it = segmenter.segment_str(text);
///     std::iter::from_fn(move || it.next().map(|i| (i, it.word_type())))
///         .tuple_windows()
///         .filter(|(_, (_, status))| *status == WordType::Letter)
///         .map(|((i, _), (j, _))| &text[i..j])
///         .collect()
/// };
/// assert_eq!(&words, &["Mark’d", "ye", "his", "words"]);
/// ```
#[derive(Debug)]
pub struct WordSegmenter {
    payload: DataPayload<WordBreakDataV1Marker>,
    complex: ComplexPayloads,
}

impl WordSegmenter {
    /// Constructs a [`WordSegmenter`] with an invariant locale and the best available compiled data for
    /// complex scripts (Chinese, Japanese, Khmer, Lao, Myanmar, and Thai).
    ///
    /// The current behavior, which is subject to change, is to use the LSTM model when available
    /// and the dictionary model for Chinese and Japanese.
    ///
    /// ✨ *Enabled with the `compiled_data` and `auto` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Behavior with complex scripts:
    ///
    /// ```
    /// use icu::segmenter::WordSegmenter;
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter = WordSegmenter::new_auto();
    ///
    /// let th_bps = segmenter.segment_str(th_str).collect::<Vec<_>>();
    /// let ja_bps = segmenter.segment_str(ja_str).collect::<Vec<_>>();
    ///
    /// assert_eq!(th_bps, [0, 9, 18, 39]);
    /// assert_eq!(ja_bps, [0, 15, 21]);
    /// ```
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "auto")]
    pub fn new_auto() -> Self {
        Self {
            payload: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_WORD_V1,
            ),
            complex: ComplexPayloads::new_auto(),
        }
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            try_new_auto,
            try_new_auto_with_any_provider,
            try_new_auto_with_buffer_provider,
            try_new_auto_unstable,
            Self
        ]
    );

    #[cfg(feature = "auto")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_auto)]
    pub fn try_new_auto_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<DictionaryForWordOnlyAutoV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Ok(Self {
            payload: provider.load(Default::default())?.take_payload()?,
            complex: ComplexPayloads::try_new_auto(provider)?,
        })
    }

    /// Constructs a [`WordSegmenter`] with an invariant locale and compiled LSTM data for
    /// complex scripts (Burmese, Khmer, Lao, and Thai).
    ///
    /// The LSTM, or Long Term Short Memory, is a machine learning model. It is smaller than
    /// the full dictionary but more expensive during segmentation (inference).
    ///
    /// Warning: there is not currently an LSTM model for Chinese or Japanese, so the [`WordSegmenter`]
    /// created by this function will have unexpected behavior in spans of those scripts.
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Behavior with complex scripts:
    ///
    /// ```
    /// use icu::segmenter::WordSegmenter;
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter = WordSegmenter::new_lstm();
    ///
    /// let th_bps = segmenter.segment_str(th_str).collect::<Vec<_>>();
    /// let ja_bps = segmenter.segment_str(ja_str).collect::<Vec<_>>();
    ///
    /// assert_eq!(th_bps, [0, 9, 18, 39]);
    ///
    /// // Note: We aren't able to find a suitable breakpoint in Chinese/Japanese.
    /// assert_eq!(ja_bps, [0, 21]);
    /// ```
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "lstm")]
    pub fn new_lstm() -> Self {
        Self {
            payload: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_WORD_V1,
            ),
            complex: ComplexPayloads::new_lstm(),
        }
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            new_lstm,
            try_new_lstm_with_any_provider,
            try_new_lstm_with_buffer_provider,
            try_new_lstm_unstable,
            Self
        ]
    );

    #[cfg(feature = "lstm")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_lstm)]
    pub fn try_new_lstm_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Ok(Self {
            payload: provider.load(Default::default())?.take_payload()?,
            complex: ComplexPayloads::try_new_lstm(provider)?,
        })
    }

    /// Construct a [`WordSegmenter`] with an invariant locale and compiled dictionary data for
    /// complex scripts (Chinese, Japanese, Khmer, Lao, Myanmar, and Thai).
    ///
    /// The dictionary model uses a list of words to determine appropriate breakpoints. It is
    /// faster than the LSTM model but requires more data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    ///
    /// Behavior with complex scripts:
    ///
    /// ```
    /// use icu::segmenter::WordSegmenter;
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter = WordSegmenter::new_dictionary();
    ///
    /// let th_bps = segmenter.segment_str(th_str).collect::<Vec<_>>();
    /// let ja_bps = segmenter.segment_str(ja_str).collect::<Vec<_>>();
    ///
    /// assert_eq!(th_bps, [0, 9, 18, 39]);
    /// assert_eq!(ja_bps, [0, 15, 21]);
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn new_dictionary() -> Self {
        Self {
            payload: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_WORD_V1,
            ),
            complex: ComplexPayloads::new_dict(),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            new_dictionary,
            try_new_dictionary_with_any_provider,
            try_new_dictionary_with_buffer_provider,
            try_new_dictionary_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_dictionary)]
    pub fn try_new_dictionary_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<WordBreakDataV1Marker>
            + DataProvider<DictionaryForWordOnlyAutoV1Marker>
            + DataProvider<DictionaryForWordLineExtendedV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Ok(Self {
            payload: provider.load(Default::default())?.take_payload()?,
            complex: ComplexPayloads::try_new_dict(provider)?,
        })
    }

    /// Creates a word break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> WordBreakIteratorUtf8<'l, 's> {
        WordBreakIterator(RuleBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: Some(&self.complex),
            boundary_property: 0,
        })
    }

    /// Creates a word break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf8<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> WordBreakIteratorPotentiallyIllFormedUtf8<'l, 's> {
        WordBreakIterator(RuleBreakIterator {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: Some(&self.complex),
            boundary_property: 0,
        })
    }

    /// Creates a word break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'l, 's>(&'l self, input: &'s [u8]) -> WordBreakIteratorLatin1<'l, 's> {
        WordBreakIterator(RuleBreakIterator {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: Some(&self.complex),
            boundary_property: 0,
        })
    }

    /// Creates a word break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> WordBreakIteratorUtf16<'l, 's> {
        WordBreakIterator(RuleBreakIterator {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: Some(&self.complex),
            boundary_property: 0,
        })
    }
}

#[derive(Debug)]
pub struct WordBreakTypeUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().map_or(0, |c| c.len_utf8())
    }

    fn handle_complex_language(
        iter: &mut RuleBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        handle_complex_language_utf8(iter, left_codepoint)
    }
}

#[derive(Debug)]
pub struct WordBreakTypePotentiallyIllFormedUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypePotentiallyIllFormedUtf8 {
    type IterAttr = Utf8CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().map_or(0, |c| c.len_utf8())
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
        debug_assert!(!iter.is_eof());
        s.push(iter.get_current_codepoint()?);
        iter.advance_iter();
        if let Some(current_break_property) = iter.get_current_break_property() {
            if current_break_property != iter.data.complex_property {
                break;
            }
        } else {
            // EOF
            break;
        }
    }

    // Restore iterator to move to head of complex string
    iter.iter = start_iter;
    iter.current_pos_data = start_point;
    #[allow(clippy::unwrap_used)] // iter.complex present for word segmenter
    let breaks = complex_language_segment_str(iter.complex.unwrap(), &s);
    iter.result_cache = breaks;
    let first_pos = *iter.result_cache.first()?;
    let mut i = left_codepoint.len_utf8();
    loop {
        if i == first_pos {
            // Re-calculate breaking offset
            iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
            return iter.get_current_position();
        }
        debug_assert!(
            i < first_pos,
            "we should always arrive at first_pos: near index {:?}",
            iter.get_current_position()
        );
        i += T::get_current_position_character_len(iter);
        iter.advance_iter();
        if iter.is_eof() {
            iter.result_cache.clear();
            return Some(iter.len);
        }
    }
}

#[derive(Debug)]
pub struct WordBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for WordBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        match iter.get_current_codepoint() {
            None => 0,
            Some(ch) if ch >= 0x10000 => 2,
            _ => 1,
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
            debug_assert!(!iter.is_eof());
            s.push(iter.get_current_codepoint()? as u16);
            iter.advance_iter();
            if let Some(current_break_property) = iter.get_current_break_property() {
                if current_break_property != iter.data.complex_property {
                    break;
                }
            } else {
                // EOF
                break;
            }
        }

        // Restore iterator to move to head of complex string
        iter.iter = start_iter;
        iter.current_pos_data = start_point;
        #[allow(clippy::unwrap_used)] // iter.complex present for word segmenter
        let breaks = complex_language_segment_utf16(iter.complex.unwrap(), &s);
        iter.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        let first_pos = *iter.result_cache.first()?;
        let mut i = 1;
        loop {
            if i == first_pos {
                // Re-calculate breaking offset
                iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
                return iter.get_current_position();
            }
            debug_assert!(
                i < first_pos,
                "we should always arrive at first_pos: near index {:?}",
                iter.get_current_position()
            );
            i += 1;
            iter.advance_iter();
            if iter.is_eof() {
                iter.result_cache.clear();
                return Some(iter.len);
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
#[test]
fn empty_string() {
    let segmenter = WordSegmenter::new_auto();
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}
