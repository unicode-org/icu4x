// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{ComplexHandler, RuleBreakIterator, RuleBreakType};
use crate::complex::*;
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::iterator_helpers::derive_usize_iterator_with_type;
use crate::provider::*;
use crate::rule_segmenter::*;
#[cfg(feature = "compiled_data")]
use crate::word::WordBreakInvariantOptions;
use crate::word::WordBreakOptions;
use crate::word::WordType;
use core::marker::PhantomData;
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
pub struct WordBreakIterator<'data, 's, Y: RuleBreakType>(
    RuleBreakIterator<'data, 's, Y, ComplexWord<Y>>,
);

derive_usize_iterator_with_type!(WordBreakIterator, 'data);

impl<'data, 's, Y: RuleBreakType> WordBreakIterator<'data, 's, Y> {
    /// Returns the word type of the segment preceding the current boundary.
    #[inline]
    pub fn word_type(&self) -> WordType {
        match self.0.last_accepting_status {
            0 => WordType::None,
            1 => WordType::Number,
            _ => WordType::Letter,
        }
    }

    /// Returns an iterator over pairs of boundary position and word type.
    pub fn iter_with_word_type(
        self,
    ) -> impl Iterator<Item = (usize, WordType)> + use<'data, 's, Y> {
        WordBreakIteratorWithWordType(self)
    }

    /// Returns `true` when the segment preceding the current boundary is word-like,
    /// such as letters, numbers, or CJKV ideographs.
    #[inline]
    pub fn is_word_like(&self) -> bool {
        self.word_type().is_word_like()
    }
}

struct WordBreakIteratorWithWordType<'data, 's, Y: RuleBreakType>(WordBreakIterator<'data, 's, Y>);

impl<Y: RuleBreakType> Iterator for WordBreakIteratorWithWordType<'_, '_, Y> {
    type Item = (usize, WordType);
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0.next()?;
        Some((ret, self.0.word_type()))
    }
}

/// Supports loading word break data, and creating word break iterators for different string
/// encodings.
///
/// Most segmentation methods live on [`WordSegmenterBorrowed`], which can be obtained via
/// [`WordSegmenter::new_auto()`] (etc) or [`WordSegmenter::as_borrowed()`].
///
/// Word segmenter is currently compatible with [Unicode Standard Annex #29][UAX29] (Version 17.0.0).
///
/// [UAX29]: https://www.unicode.org/reports/tr29/tr29-47.html
///
/// # Content Locale
///
/// You can optionally provide a _content locale_ to the [`WordSegmenter`] constructor. If you
/// have information on the language of the text being segmented, providing this hint can
/// produce higher-quality results.
///
/// If you have a content locale, use [`WordBreakOptions`] and a constructor beginning with `try_new`.
/// If you do not have a content locale use [`WordBreakInvariantOptions`] and a constructor
/// beginning with `new`.
///
/// # Examples
///
/// Segment a string:
///
/// ```rust
/// use icu::segmenter::WordSegmenter;
///
/// let segmenter = WordSegmenter::new_auto(Default::default());
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 5, 6, 11]);
/// ```
///
/// Segment a Latin1 byte string with a content locale:
///
/// ```rust
/// use icu::locale::langid;
/// use icu::segmenter::options::WordBreakOptions;
/// use icu::segmenter::WordSegmenter;
///
/// let mut options = WordBreakOptions::default();
/// let langid = &langid!("en");
/// options.content_locale = Some(langid);
/// let segmenter = WordSegmenter::try_new_auto(options).unwrap();
///
/// let breakpoints: Vec<usize> = segmenter
///     .as_borrowed()
///     .segment_latin1(b"Hello World")
///     .collect();
/// assert_eq!(&breakpoints, &[0, 5, 6, 11]);
/// ```
///
/// Successive boundaries can be used to retrieve the segments.
/// In particular, the first boundary is always 0, and the last one is the
/// length of the segmented text in code units.
///
/// ```rust
/// # use icu::segmenter::{WordSegmenter, options::WordBreakInvariantOptions};
/// # let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
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
/// classify the preceding segment; [`WordBreakIterator::iter_with_word_type()`]
/// associates each boundary with its status.
/// ```rust
/// # use itertools::Itertools;
/// # use icu::segmenter::WordSegmenter;
/// # use icu::segmenter::options::{WordType, WordBreakInvariantOptions};
/// # let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
/// # let text = "Mark’d ye his words?";
/// let words: Vec<&str> = segmenter
///     .segment_str(text)
///     .iter_with_word_type()
///     .tuple_windows()
///     .filter(|(_, (_, segment_type))| segment_type.is_word_like())
///     .map(|((i, _), (j, _))| &text[i..j])
///     .collect();
/// assert_eq!(&words, &["Mark’d", "ye", "his", "words"]);
/// ```
#[derive(Debug)]
pub struct WordSegmenter {
    payload: DataPayload<SegmenterBreakWordV2>,
    complex: ComplexPayloads,
}

/// Segments a string into words (borrowed version).
///
/// See [`WordSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct WordSegmenterBorrowed<'data> {
    data: &'data SegmenterStateMachine<'data>,
    complex: ComplexPayloadsBorrowed<'data>,
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
    /// use icu::segmenter::{options::WordBreakInvariantOptions, WordSegmenter};
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter =
    ///     WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    ///
    /// let th_bps = segmenter.segment_str(th_str).collect::<Vec<_>>();
    /// let ja_bps = segmenter.segment_str(ja_str).collect::<Vec<_>>();
    ///
    /// assert_eq!(th_bps, [0, 9, 18, 39]);
    /// assert_eq!(ja_bps, [0, 15, 21]);
    /// ```
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "auto")]
    pub fn new_auto(_options: WordBreakInvariantOptions) -> WordSegmenterBorrowed<'static> {
        let mut complex = ComplexPayloadsBorrowed::new();
        complex.with_southeast_asian_lstms();
        complex.with_japanese_dictionary();
        WordSegmenterBorrowed {
            data: Baked::SINGLETON_SEGMENTER_BREAK_WORD_V2,
            complex,
        }
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_buffer_data_constructors!(
        (options: WordBreakOptions) -> error: DataError,
        functions: [
            try_new_auto,
            try_new_auto_with_buffer_provider,
            try_new_auto_unstable,
            Self
        ]
    );

    #[cfg(feature = "auto")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_auto)]
    pub fn try_new_auto_unstable<D>(
        provider: &D,
        _options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV2>
            + DataProvider<SegmenterDictionaryAutoV1>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut complex = ComplexPayloads::try_new(provider)?;
        complex.with_southeast_asian_lstms(provider)?;
        complex.with_japanese_dictionary(provider)?;
        Ok(Self {
            payload: provider.load(Default::default())?.payload,
            complex,
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
    /// use icu::segmenter::{options::WordBreakInvariantOptions, WordSegmenter};
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter =
    ///     WordSegmenter::new_lstm(WordBreakInvariantOptions::default());
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
    pub fn new_lstm(options: WordBreakInvariantOptions) -> WordSegmenterBorrowed<'static> {
        let mut s = Self::new_for_non_complex_scripts(options);
        s.load_lstm();
        s
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_buffer_data_constructors!(
        (options: WordBreakOptions) -> error: DataError,
        functions: [
            try_new_lstm,
            try_new_lstm_with_buffer_provider,
            try_new_lstm_unstable,
            Self
        ]
    );

    #[cfg(feature = "lstm")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_lstm)]
    pub fn try_new_lstm_unstable<D>(
        provider: &D,
        options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV2>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut s = Self::try_new_for_non_complex_scripts_unstable(provider, options)?;
        s.load_lstm_unstable(provider)?;
        Ok(s)
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
    /// use icu::segmenter::{options::WordBreakInvariantOptions, WordSegmenter};
    ///
    /// let th_str = "ทุกสองสัปดาห์";
    /// let ja_str = "こんにちは世界";
    ///
    /// let segmenter =
    ///     WordSegmenter::new_dictionary(WordBreakInvariantOptions::default());
    ///
    /// let th_bps = segmenter.segment_str(th_str).collect::<Vec<_>>();
    /// let ja_bps = segmenter.segment_str(ja_str).collect::<Vec<_>>();
    ///
    /// assert_eq!(th_bps, [0, 9, 18, 39]);
    /// assert_eq!(ja_bps, [0, 15, 21]);
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn new_dictionary(options: WordBreakInvariantOptions) -> WordSegmenterBorrowed<'static> {
        let mut s = Self::new_for_non_complex_scripts(options);
        s.load_dictionary();
        s
    }

    icu_provider::gen_buffer_data_constructors!(
        (options: WordBreakOptions) -> error: DataError,
        functions: [
            try_new_dictionary,
            try_new_dictionary_with_buffer_provider,
            try_new_dictionary_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_dictionary)]
    pub fn try_new_dictionary_unstable<D>(
        provider: &D,
        options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV2>
            + DataProvider<SegmenterDictionaryAutoV1>
            + DataProvider<SegmenterDictionaryExtendedV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut s = Self::try_new_for_non_complex_scripts_unstable(provider, options)?;
        s.load_dictionary_unstable(provider)?;
        Ok(s)
    }

    /// Construct a [`WordSegmenter`] with an invariant locale and no support for
    /// scripts requiring complex context dependent word breaks (Chinese, Japanese, Khmer, Lao, Myanmar, and Thai).
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_for_non_complex_scripts(
        _options: WordBreakInvariantOptions,
    ) -> WordSegmenterBorrowed<'static> {
        WordSegmenterBorrowed {
            data: Baked::SINGLETON_SEGMENTER_BREAK_WORD_V2,
            complex: ComplexPayloadsBorrowed::new(),
        }
    }

    icu_provider::gen_buffer_data_constructors!(
        (options: WordBreakOptions) -> error: DataError,
        functions: [
            try_new_for_non_complex_scripts,
            try_new_for_non_complex_scripts_with_buffer_provider,
            try_new_for_non_complex_scripts_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_for_non_complex_scripts)]
    pub fn try_new_for_non_complex_scripts_unstable<D>(
        provider: &D,
        _options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV2>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self {
            payload: provider.load(Default::default())?.payload,
            complex: ComplexPayloads::try_new(provider)?,
        })
    }

    /// Loads LSTM data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `lstm` Cargo feature.*
    #[cfg(feature = "lstm")]
    pub fn load_lstm_unstable<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterLstmAutoV1> + ?Sized,
    {
        self.complex.with_southeast_asian_lstms(provider)?;
        Ok(())
    }

    /// A version of [`Self::load_lstm_unstable`] that uses custom data
    /// provided by a [`BufferProvider`].
    ///
    /// ✨ *Enabled with the `serde` Cargo feature.*
    #[cfg(feature = "serde")]
    #[cfg(feature = "lstm")]
    pub fn load_lstm_with_buffer_provider(
        &mut self,
        provider: &(impl BufferProvider + ?Sized),
    ) -> Result<(), DataError> {
        self.load_lstm_unstable(&provider.as_deserializing())
    }

    /// Loads dictionary data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    pub fn load_dictionary_unstable<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterDictionaryAutoV1>
            + DataProvider<SegmenterDictionaryExtendedV1>
            + ?Sized,
    {
        self.complex.with_southeast_asian_dictionaries(provider)?;
        self.complex.with_japanese_dictionary(provider)?;
        Ok(())
    }

    /// A version of [`Self::load_dictionary_unstable`] that uses custom data
    /// provided by a [`BufferProvider`].
    ///
    /// ✨ *Enabled with the `serde` Cargo feature.*
    #[cfg(feature = "serde")]
    pub fn load_dictionary_with_buffer_provider(
        &mut self,
        provider: &(impl BufferProvider + ?Sized),
    ) -> Result<(), DataError> {
        self.load_dictionary_unstable(&provider.as_deserializing())
    }

    /// Constructs a borrowed version of this type for more efficient querying.
    ///
    /// Most useful methods for segmentation are on this type.
    pub fn as_borrowed(&self) -> WordSegmenterBorrowed<'_> {
        WordSegmenterBorrowed {
            data: self.payload.get(),
            complex: self.complex.as_borrowed(),
        }
    }
}

#[derive(Debug)]
struct ComplexWord<Y>(PhantomData<Y>);

impl<Y: RuleBreakType> ComplexHandler<Y> for ComplexWord<Y> {
    const BREAK_AT_BOUNDARIES: bool = true;
    const BREAK_STATUS: u8 = WordType::Letter as u8;

    type Data<'s> = Y::ComplexData<'s>;

    #[inline(always)]
    fn is_complex(data: &Self::Data<'_>, iter: &Y::IterAttr<'_>) -> bool {
        Y::is_complex(data, iter)
    }

    #[inline(always)]
    fn handle<'s>(
        data: &Self::Data<'_>,
        complex: &<Y as RuleBreakType>::IterAttr<'s>,
        past_complex: &<Y as RuleBreakType>::IterAttr<'s>,
    ) -> impl Iterator<Item = usize> + use<'s, Y> {
        Y::handle_complex(data, complex, past_complex)
    }
}

impl<'data> WordSegmenterBorrowed<'data> {
    /// Creates a word break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> WordBreakIterator<'data, 's, Utf8> {
        WordBreakIterator(RuleBreakIterator::new(
            input.char_indices(),
            self.data,
            None,
            Some(self.complex),
        ))
    }

    /// Creates a word break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> WordBreakIterator<'data, 's, PotentiallyIllFormedUtf8> {
        WordBreakIterator(RuleBreakIterator::new(
            Utf8CharIndices::new(input),
            self.data,
            None,
            Some(self.complex),
        ))
    }

    /// Creates a word break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> WordBreakIterator<'data, 's, Latin1> {
        WordBreakIterator(RuleBreakIterator::new(
            Latin1Indices::new(input),
            self.data,
            None,
            None,
        ))
    }

    /// Creates a word break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> WordBreakIterator<'data, 's, Utf16> {
        WordBreakIterator(RuleBreakIterator::new(
            Utf16Indices::new(input),
            self.data,
            None,
            Some(self.complex),
        ))
    }
}

impl WordSegmenterBorrowed<'static> {
    /// Loads LSTM data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub fn load_lstm(&mut self) {
        self.complex.with_southeast_asian_lstms();
    }

    /// Loads dictionary data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    #[cfg(feature = "compiled_data")]
    pub fn load_dictionary(&mut self) {
        self.complex.with_southeast_asian_dictionaries();
        self.complex.with_japanese_dictionary();
    }

    /// Cheaply converts a [`WordSegmenterBorrowed<'static>`] into a [`WordSegmenter`].
    ///
    /// Note: Due to branching and indirection, using [`WordSegmenter`] might inhibit some
    /// compile-time optimizations that are possible with [`WordSegmenterBorrowed`].
    pub fn static_to_owned(self) -> WordSegmenter {
        WordSegmenter {
            payload: DataPayload::from_static_ref(self.data),
            complex: self.complex.static_to_owned(),
        }
    }
}

#[test]
fn empty_string() {
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}
