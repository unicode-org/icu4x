// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::indices::*;
use crate::provider::*;
use crate::scaffold::*;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use icu_locale_core::LanguageIdentifier;
use icu_provider::prelude::*;

/// Options to tailor word breaking behavior.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct WordBreakOptions<'a> {
    /// Content locale for word segmenter
    ///
    /// If you know the language of the text being segmented, provide it here in order to produce
    /// higher quality breakpoints.
    pub content_locale: Option<&'a LanguageIdentifier>,
    /// Options independent of the locale
    pub invariant_options: WordBreakInvariantOptions,
}

impl WordBreakOptions<'_> {
    /// `const` version of [`Default::default`]
    pub const fn default() -> Self {
        Self {
            content_locale: None,
            invariant_options: WordBreakInvariantOptions::default(),
        }
    }
}

/// Locale-independent options to tailor word breaking behavior
///
/// Currently empty but may grow in the future
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct WordBreakInvariantOptions {}

impl WordBreakInvariantOptions {
    /// `const` version of [`Default::default`]
    pub const fn default() -> Self {
        Self {}
    }
}

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
pub struct WordBreakIterator<'data, 's, Y: RuleBreakType>(WordBreakIteratorInner<'data, 's, Y>);

#[derive(Debug)]
enum WordBreakIteratorInner<'data, 's, Y: RuleBreakType> {
    V1(crate::rule_segmenter_v1::RuleBreakIterator<'data, 's, Y>),
    #[cfg(feature = "unstable")]
    V2(crate::rule_segmenter_v2::RuleBreakIterator<'data, 's, Y, ComplexWord<Y>>),
}

impl<Y: RuleBreakType> Iterator for WordBreakIterator<'_, '_, Y> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            WordBreakIteratorInner::V1(ref mut iter) => iter.next(),
            #[cfg(feature = "unstable")]
            WordBreakIteratorInner::V2(ref mut iter) => iter.next(),
        }
    }
}

/// The word type tag that is returned by [`WordBreakIterator::word_type()`].
///
/// [`WordBreakIterator::word_type()`]: WordBreakIterator::word_type
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

impl WordType {
    /// Whether the segment is word-like; word-like segments include numbers, as
    /// well as segments made up of letters (including CJKV ideographs).
    pub fn is_word_like(&self) -> bool {
        self != &WordType::None
    }
}

impl<'data, 's, Y: RuleBreakType> WordBreakIterator<'data, 's, Y> {
    /// Returns the word type of the segment preceding the current boundary.
    #[inline]
    pub fn word_type(&self) -> WordType {
        let last_accepting_status = match self.0 {
            WordBreakIteratorInner::V1(ref iter) => iter.rule_status(),
            #[cfg(feature = "unstable")]
            WordBreakIteratorInner::V2(ref iter) => iter.last_accepting_status(),
        };
        match last_accepting_status {
            0 => WordType::None,
            1 => WordType::Number,
            _ => WordType::Letter,
        }
    }

    /// Returns an iterator over pairs of boundary position and word type.
    pub fn iter_with_word_type(self) -> WordBreakIteratorWithWordType<'data, 's, Y> {
        WordBreakIteratorWithWordType(self)
    }

    /// Returns `true` when the segment preceding the current boundary is word-like,
    /// such as letters, numbers, or CJKV ideographs.
    #[inline]
    pub fn is_word_like(&self) -> bool {
        self.word_type().is_word_like()
    }
}

/// Word break iterator that also returns the word type
// We can use impl Trait here once `use<..>` syntax is available, see https://github.com/rust-lang/rust/issues/61756
#[derive(Debug)]
pub struct WordBreakIteratorWithWordType<'data, 's, Y: RuleBreakType>(
    WordBreakIterator<'data, 's, Y>,
);

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
pub struct WordSegmenter(WordSegmenterInner);

#[derive(Debug)]
enum WordSegmenterInner {
    V1 {
        payload: DataPayload<SegmenterBreakWordV1>,
        complex: ComplexPayloads,
        payload_locale_override: Option<DataPayload<SegmenterBreakWordOverrideV1>>,
    },
    #[cfg(feature = "unstable")]
    V2 {
        payload: DataPayload<SegmenterBreakWordV2>,
        complex: ComplexPayloads,
    },
}

/// Segments a string into words (borrowed version).
///
/// See [`WordSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct WordSegmenterBorrowed<'data>(WordSegmenterBorrowedInner<'data>);

#[derive(Clone, Debug, Copy)]
enum WordSegmenterBorrowedInner<'data> {
    V1 {
        data: &'data RuleBreakData<'data>,
        complex: ComplexPayloadsBorrowed<'data>,
        locale_override: Option<&'data RuleBreakDataOverride<'data>>,
    },
    #[cfg(feature = "unstable")]
    V2 {
        data: &'data SegmenterStateMachine<'data>,
        complex: ComplexPayloadsBorrowed<'data>,
    },
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
        let mut s = Self::new_for_non_complex_scripts(_options);
        s.load_auto();
        s
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
        options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV1>
            + DataProvider<SegmenterBreakWordOverrideV1>
            + DataProvider<SegmenterDictionaryAutoV1>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut s = Self::try_new_for_non_complex_scripts_unstable(provider, options)?;
        s.load_auto_unstable(provider)?;
        Ok(s)
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
        D: DataProvider<SegmenterBreakWordV1>
            + DataProvider<SegmenterBreakWordOverrideV1>
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
        D: DataProvider<SegmenterBreakWordV1>
            + DataProvider<SegmenterBreakWordOverrideV1>
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
        WordSegmenterBorrowed(WordSegmenterBorrowedInner::V1 {
            data: Baked::SINGLETON_SEGMENTER_BREAK_WORD_V1,
            complex: ComplexPayloadsBorrowed::new(),
            locale_override: None,
        })
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
        options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV1>
            + DataProvider<SegmenterBreakWordOverrideV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self(WordSegmenterInner::V1 {
            payload: provider.load(Default::default())?.payload,
            complex: ComplexPayloads::try_new(provider)?,
            payload_locale_override: if let Some(locale) = options.content_locale {
                let locale = DataLocale::from(locale);
                let req = DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&locale),
                    metadata: {
                        let mut metadata = DataRequestMetadata::default();
                        metadata.silent = true;
                        metadata
                    },
                };
                provider
                    .load(req)
                    .allow_identifier_not_found()?
                    .map(|r| r.payload)
            } else {
                None
            },
        }))
    }

    /// Construct a [`WordSegmenter`] with an invariant locale and no support for
    /// scripts requiring complex context dependent word breaks (Chinese, Japanese, Khmer, Lao, Myanmar, and Thai).
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "unstable")]
    pub const fn new_neo_for_non_complex_scripts(
        _options: WordBreakInvariantOptions,
    ) -> WordSegmenterBorrowed<'static> {
        WordSegmenterBorrowed(WordSegmenterBorrowedInner::V2 {
            data: Baked::SINGLETON_SEGMENTER_BREAK_WORD_V2,
            complex: ComplexPayloadsBorrowed::new(),
        })
    }

    #[cfg(feature = "unstable")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_neo_for_non_complex_scripts)]
    pub fn try_new_neo_for_non_complex_scripts_unstable<D>(
        provider: &D,
        _options: WordBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakWordV2>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self(WordSegmenterInner::V2 {
            payload: provider.load(Default::default())?.payload,
            complex: ComplexPayloads::try_new(provider)?,
        }))
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
        let complex = match self.0 {
            WordSegmenterInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_lstms(provider)?;
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
        let complex = match self.0 {
            WordSegmenterInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_dictionaries(provider)?;
        complex.with_japanese_dictionary(provider)?;
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

    /// Loads the best available complex script data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    #[cfg(feature = "auto")]
    pub fn load_auto_unstable<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterDictionaryAutoV1> + DataProvider<SegmenterLstmAutoV1> + ?Sized,
    {
        let complex = match self.0 {
            WordSegmenterInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_lstms(provider)?;
        complex.with_japanese_dictionary(provider)?;
        Ok(())
    }

    /// A version of [`Self::load_auto_unstable`] that uses custom data
    /// provided by a [`BufferProvider`].
    ///
    /// ✨ *Enabled with the `serde` Cargo feature.*
    #[cfg(feature = "auto")]
    #[cfg(feature = "serde")]
    pub fn load_auto_with_buffer_provider(
        &mut self,
        provider: &(impl BufferProvider + ?Sized),
    ) -> Result<(), DataError> {
        self.load_auto_unstable(&provider.as_deserializing())
    }

    /// Constructs a borrowed version of this type for more efficient querying.
    ///
    /// Most useful methods for segmentation are on this type.
    pub fn as_borrowed(&self) -> WordSegmenterBorrowed<'_> {
        WordSegmenterBorrowed(match &self.0 {
            WordSegmenterInner::V1 {
                payload,
                complex,
                payload_locale_override,
            } => WordSegmenterBorrowedInner::V1 {
                data: payload.get(),
                complex: complex.as_borrowed(),
                locale_override: payload_locale_override.as_ref().map(|p| p.get()),
            },
            #[cfg(feature = "unstable")]
            WordSegmenterInner::V2 { payload, complex } => WordSegmenterBorrowedInner::V2 {
                data: payload.get(),
                complex: complex.as_borrowed(),
            },
        })
    }
}

impl<'data> WordSegmenterBorrowed<'data> {
    /// Creates a word break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> WordBreakIterator<'data, 's, Utf8> {
        WordBreakIterator(match self.0 {
            WordSegmenterBorrowedInner::V1 {
                data,
                complex,
                locale_override,
            } => WordBreakIteratorInner::V1(crate::rule_segmenter_v1::RuleBreakIterator {
                iter: input.char_indices(),
                len: input.len(),
                current_pos_data: None,
                result_cache: Vec::new(),
                data,
                complex: Some(complex),
                boundary_property: 0,
                locale_override,
                handle_complex: handle_complex_utf8,
            }),
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 { data, complex } => {
                WordBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                    input.char_indices(),
                    data,
                    None,
                    Some(complex),
                ))
            }
        })
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
        WordBreakIterator(match self.0 {
            WordSegmenterBorrowedInner::V1 {
                data,
                complex,
                locale_override,
            } => WordBreakIteratorInner::V1(crate::rule_segmenter_v1::RuleBreakIterator {
                iter: Utf8CharIndices::new(input),
                len: input.len(),
                current_pos_data: None,
                result_cache: Vec::new(),
                data,
                complex: Some(complex),
                boundary_property: 0,
                locale_override,
                handle_complex: handle_complex_utf8,
            }),
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 { data, complex } => {
                WordBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                    Utf8CharIndices::new(input),
                    data,
                    None,
                    Some(complex),
                ))
            }
        })
    }

    /// Creates a word break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> WordBreakIterator<'data, 's, Latin1> {
        WordBreakIterator(match self.0 {
            WordSegmenterBorrowedInner::V1 {
                data,
                complex,
                locale_override,
            } => WordBreakIteratorInner::V1(crate::rule_segmenter_v1::RuleBreakIterator {
                iter: Latin1Indices::new(input),
                len: input.len(),
                current_pos_data: None,
                result_cache: Vec::new(),
                data,
                complex: Some(complex),
                boundary_property: 0,
                locale_override,
                handle_complex: crate::rule_segmenter_v1::empty_handle_complex,
            }),
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 { data, .. } => {
                WordBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                    Latin1Indices::new(input),
                    data,
                    None,
                    None,
                ))
            }
        })
    }

    /// Creates a word break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> WordBreakIterator<'data, 's, Utf16> {
        WordBreakIterator(match self.0 {
            WordSegmenterBorrowedInner::V1 {
                data,
                complex,
                locale_override,
            } => WordBreakIteratorInner::V1(crate::rule_segmenter_v1::RuleBreakIterator {
                iter: Utf16Indices::new(input),
                len: input.len(),
                current_pos_data: None,
                result_cache: Vec::new(),
                data,
                complex: Some(complex),
                boundary_property: 0,
                locale_override,
                handle_complex: handle_complex_utf16,
            }),
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 { data, complex } => {
                WordBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                    Utf16Indices::new(input),
                    data,
                    None,
                    Some(complex),
                ))
            }
        })
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
        let complex = match self.0 {
            WordSegmenterBorrowedInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_lstms();
    }

    /// Loads dictionary data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    #[cfg(feature = "compiled_data")]
    pub fn load_dictionary(&mut self) {
        let complex = match self.0 {
            WordSegmenterBorrowedInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_dictionaries();
        complex.with_japanese_dictionary();
    }

    /// Loads the best available complex script data for a [`WordSegmenter`] constructed with
    /// [`WordSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    #[cfg(feature = "auto")]
    #[cfg(feature = "compiled_data")]
    pub fn load_auto(&mut self) {
        let complex = match self.0 {
            WordSegmenterBorrowedInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 {
                ref mut complex, ..
            } => complex,
        };
        complex.with_southeast_asian_lstms();
        complex.with_japanese_dictionary();
    }

    /// Cheaply converts a [`WordSegmenterBorrowed<'static>`] into a [`WordSegmenter`].
    ///
    /// Note: Due to branching and indirection, using [`WordSegmenter`] might inhibit some
    /// compile-time optimizations that are possible with [`WordSegmenterBorrowed`].
    pub fn static_to_owned(self) -> WordSegmenter {
        WordSegmenter(match self.0 {
            WordSegmenterBorrowedInner::V1 {
                data,
                complex,
                locale_override,
            } => WordSegmenterInner::V1 {
                payload: DataPayload::from_static_ref(data),
                complex: complex.static_to_owned(),
                payload_locale_override: locale_override.map(DataPayload::from_static_ref),
            },
            #[cfg(feature = "unstable")]
            WordSegmenterBorrowedInner::V2 { data, complex } => WordSegmenterInner::V2 {
                payload: DataPayload::from_static_ref(data),
                complex: complex.static_to_owned(),
            },
        })
    }
}

fn handle_complex_utf8<T>(
    iter: &mut crate::rule_segmenter_v1::RuleBreakIterator<'_, '_, T>,
    left_codepoint: T::CharType,
) -> Option<usize>
where
    T: RuleBreakType<CharType = char>,
{
    // word segmenter doesn't define break rules for some scripts such as Thai.
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
    #[expect(clippy::unwrap_used)] // iter.complex present for word segmenter
    let breaks = iter.complex.unwrap().segment_str(&s);
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
        i += iter.get_current_codepoint().map_or(0, T::char_len);
        iter.advance_iter();
        if iter.is_eof() {
            iter.result_cache.clear();
            return Some(iter.len);
        }
    }
}

fn handle_complex_utf16<T>(
    iter: &mut crate::rule_segmenter_v1::RuleBreakIterator<'_, '_, T>,
    left_codepoint: T::CharType,
) -> Option<usize>
where
    T: RuleBreakType<CharType = u32>,
{
    // word segmenter doesn't define break rules for some scripts such as Thai.
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
    #[expect(clippy::unwrap_used)] // iter.complex present for word segmenter
    let breaks = iter.complex.unwrap().segment_utf16(&s);
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

#[derive(Debug)]
#[cfg(feature = "unstable")]
struct ComplexWord<Y>(core::marker::PhantomData<Y>);

#[cfg(feature = "unstable")]
impl<Y: RuleBreakType> crate::rule_segmenter_v2::ComplexHandler<Y> for ComplexWord<Y> {
    const BREAK_AT_BOUNDARIES: bool = true;
    type Cache = [usize; 16];
    const BREAK_STATUS: u8 = WordType::Letter as u8;

    type ComplexPayloads<'s> = Y::ComplexPayloads<'s>;
    type ComplexPayload<'s> = Y::ComplexPayload<'s>;

    fn select<'data>(
        complex_payloads: &Y::ComplexPayloads<'data>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'data>> {
        Y::select_complex(complex_payloads, complex_script)
    }

    fn handle<'data, 's>(
        complex_payloads: &Self::ComplexPayload<'data>,
        iter: &Y::IterAttr<'s>,
        past_complex: &Y::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Y> {
        Y::handle_complex(complex_payloads, iter, past_complex)
    }
}

#[test]
fn empty_string() {
    let segmenter =
        WordSegmenter::new_for_non_complex_scripts(WordBreakInvariantOptions::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}

#[test]
fn empty_string_neo() {
    let segmenter =
        WordSegmenter::new_neo_for_non_complex_scripts(WordBreakInvariantOptions::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}
