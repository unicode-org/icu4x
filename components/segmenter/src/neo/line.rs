// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::RuleBreakIterator;
use crate::complex::{ComplexPayloads, ComplexPayloadsBorrowed};
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::iterator_helpers::derive_usize_iterator_with_type;
use crate::line::{LineBreakOptions, ResolvedLineBreakOptions};
#[cfg(feature = "compiled_data")]
use crate::provider::Baked;
#[cfg(feature = "lstm")]
use crate::provider::SegmenterLstmAutoV1;
use crate::provider::{
    RuleBreakDataOverride, SegmenterBreakGraphemeClusterV1, SegmenterBreakLineOverrideV2,
    SegmenterBreakLineV2, SegmenterDictionaryExtendedV1, SegmenterStateMachine,
};
use crate::scaffold::{Latin1, PotentiallyIllFormedUtf8, RuleBreakType, Utf16, Utf8};
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

/// Implements the [`Iterator`] trait over the line break opportunities of the given string.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the [`LineSegmenter`] object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// The [`Iterator::Item`] is an [`usize`] representing index of a code unit
/// _after_ the break (for a break at the end of text, this index is the length
/// of the [`str`] or array of code units).
///
/// For examples of use, see [`LineSegmenter`].
#[derive(Debug)]
pub struct LineBreakIterator<'data, 's, Y: RuleBreakType>(
    RuleBreakIterator<'data, 's, Y, Option<&'data RuleBreakDataOverride<'data>>>,
);

derive_usize_iterator_with_type!(LineBreakIterator, 'data);

/// Supports loading line break data, and creating line break iterators for different string
/// encodings.
///
/// Most segmentation methods live on [`LineSegmenterBorrowed`], which can be obtained via
/// [`LineSegmenter::new_auto()`] (etc) or [`LineSegmenter::as_borrowed()`].
///
/// The segmenter returns mandatory breaks (as defined by [definition LD7][LD7] of
/// Unicode Standard Annex #14, _Unicode Line Breaking Algorithm_) as well as
/// line break opportunities ([definition LD3][LD3]).
/// It does not distinguish them.  Callers requiring that distinction can check
/// the `Line_Break` property of the code point preceding the break against those
/// listed in rules [LB4][LB4] and [LB5][LB5], special-casing the end of text
/// according to [LB3][LB3].
///
/// For consistency with the grapheme, word, and sentence segmenters, there is
/// always a breakpoint returned at index 0, but this breakpoint is not a
/// meaningful line break opportunity.
///
/// Line segmenter is curretly compatible with [Unicode Standard Annex #14][UAX14] (Version 15.1.0).
///
/// [UAX14]: https://www.unicode.org/reports/tr14/tr14-51.html
///
/// [LD3]: https://www.unicode.org/reports/tr14/#LD3
/// [LD7]: https://www.unicode.org/reports/tr14/#LD7
/// [LB3]: https://www.unicode.org/reports/tr14/#LB3
/// [LB4]: https://www.unicode.org/reports/tr14/#LB4
/// [LB5]: https://www.unicode.org/reports/tr14/#LB5
///
/// ```rust
/// # use icu::segmenter::neo::LineSegmenter;
/// #
/// # let segmenter = LineSegmenter::new_auto(Default::default());
/// #
/// let text = "Summary\r\nThis annex…";
/// let breakpoints: Vec<usize> = segmenter.segment_str(text).collect();
/// // 9 and 22 are mandatory breaks, 14 is a line break opportunity.
/// assert_eq!(&breakpoints, &[0, 9, 14, 22]);
///
/// // There is a break opportunity between emoji, but not within the ZWJ sequence 🏳️‍🌈.
/// let flag_equation = "🏳️➕🌈🟰🏳️\u{200D}🌈";
/// let possible_first_lines: Vec<&str> =
///     segmenter.segment_str(flag_equation).skip(1).map(|i| &flag_equation[..i]).collect();
/// assert_eq!(
///     &possible_first_lines,
///     &[
///         "🏳️",
///         "🏳️➕",
///         "🏳️➕🌈",
///         "🏳️➕🌈🟰",
///         "🏳️➕🌈🟰🏳️‍🌈"
///     ]
/// );
/// ```
///
/// # Examples
///
/// Segment a string with default options:
///
/// ```rust
/// use icu::segmenter::neo::LineSegmenter;
///
/// let segmenter = LineSegmenter::new_auto(Default::default());
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 6, 11]);
/// ```
///
// / Segment a string with CSS option overrides:
// /
// / ```rust
// / use icu::segmenter::options::{
// /     LineBreakOptions, LineBreakStrictness, LineBreakWordOption,
// / };
// / use icu::segmenter::neo::LineSegmenter;
// /
// / let mut options = LineBreakOptions::default();
// / options.strictness = Some(LineBreakStrictness::Strict);
// / options.word_option = Some(LineBreakWordOption::BreakAll);
// / options.content_locale = None;
// / let segmenter = LineSegmenter::new_auto(options);
// /
// / let breakpoints: Vec<usize> =
// /     segmenter.segment_str("Hello World").collect();
// / assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
// / ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu::segmenter::neo::LineSegmenter;
///
/// let segmenter = LineSegmenter::new_auto(Default::default());
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 6, 11]);
/// ```
///
/// Separate mandatory breaks from the break opportunities:
///
/// ```rust
/// use icu::properties::{props::LineBreak, CodePointMapData};
/// use icu::segmenter::neo::LineSegmenter;
///
/// # let segmenter = LineSegmenter::new_auto(Default::default());
/// #
/// let text = "Summary\r\nThis annex…";
///
/// let mandatory_breaks: Vec<usize> = segmenter
///     .segment_str(text)
///     .filter(|&i| {
///         text[..i].chars().next_back().is_some_and(|c| {
///             matches!(
///                 CodePointMapData::<LineBreak>::new().get(c),
///                 LineBreak::MandatoryBreak
///                     | LineBreak::CarriageReturn
///                     | LineBreak::LineFeed
///                     | LineBreak::NextLine
///             ) || i == text.len()
///         })
///     })
///     .collect();
/// assert_eq!(&mandatory_breaks, &[9, 22]);
/// ```
#[derive(Debug)]
pub struct LineSegmenter {
    options: ResolvedLineBreakOptions,
    payload: DataPayload<SegmenterBreakLineV2>,
    tailoring: Option<DataPayload<SegmenterBreakLineOverrideV2>>,
    complex: ComplexPayloads,
}

/// Segments a string into lines (borrowed version).
///
/// See [`LineSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct LineSegmenterBorrowed<'data> {
    options: ResolvedLineBreakOptions,
    data: &'data SegmenterStateMachine<'data>,
    tailoring: Option<&'data RuleBreakDataOverride<'data>>,
    complex: ComplexPayloadsBorrowed<'data>,
}

impl LineSegmenter {
    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// the best available compiled data for complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The current behavior, which is subject to change, is to use the LSTM model when available.
    ///
    /// See also [`Self::new_auto`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `auto` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "auto")]
    #[cfg(feature = "compiled_data")]
    pub fn new_auto(options: LineBreakOptions) -> LineSegmenterBorrowed<'static> {
        Self::new_lstm(options)
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_buffer_data_constructors!(
        (options: LineBreakOptions) -> error: DataError,
        functions: [
            new_auto: skip,
            try_new_auto_with_buffer_provider,
            try_new_auto_unstable,
            Self,
        ]
    );

    #[cfg(feature = "auto")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_auto)]
    pub fn try_new_auto_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV2>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Self::try_new_lstm_unstable(provider, options)
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// compiled LSTM data for complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The LSTM, or Long Term Short Memory, is a machine learning model. It is smaller than
    /// the full dictionary but more expensive during segmentation (inference).
    ///
    /// See also [`Self::new_lstm`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub fn new_lstm(options: LineBreakOptions) -> LineSegmenterBorrowed<'static> {
        let mut s = Self::new_for_non_complex_scripts(options);
        s.load_lstm();
        s
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_buffer_data_constructors!(
        (options: LineBreakOptions) -> error: DataError,
        functions: [
            try_new_lstm: skip,
            try_new_lstm_with_buffer_provider,
            try_new_lstm_unstable,
            Self,
        ]
    );

    #[cfg(feature = "lstm")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_lstm)]
    pub fn try_new_lstm_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV2>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + DataProvider<SegmenterLstmAutoV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut s = Self::try_new_for_non_complex_scripts_unstable(provider, options)?;
        s.load_lstm_unstable(provider)?;
        Ok(s)
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// compiled dictionary data for complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The dictionary model uses a list of words to determine appropriate breakpoints. It is
    /// faster than the LSTM model but requires more data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new_dictionary(options: LineBreakOptions) -> LineSegmenterBorrowed<'static> {
        let mut s = Self::new_for_non_complex_scripts(options);
        s.load_dictionary();
        s
    }

    icu_provider::gen_buffer_data_constructors!(
        (options: LineBreakOptions) -> error: DataError,
        functions: [
            new_dictionary: skip,
            try_new_dictionary_with_buffer_provider,
            try_new_dictionary_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_dictionary)]
    pub fn try_new_dictionary_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV2>
            + DataProvider<SegmenterDictionaryExtendedV1>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        let mut s = Self::try_new_for_non_complex_scripts_unstable(provider, options)?;
        s.load_dictionary_unstable(provider)?;
        Ok(s)
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// no support for scripts requiring complex context dependent line breaks (Khmer, Lao, Myanmar, Thai).
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_for_non_complex_scripts(
        options: LineBreakOptions,
    ) -> LineSegmenterBorrowed<'static> {
        LineSegmenterBorrowed {
            options: options.resolve(),
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2,
            tailoring: None,
            complex: ComplexPayloadsBorrowed::new(),
        }
    }

    icu_provider::gen_buffer_data_constructors!(
        (options: LineBreakOptions) -> error: DataError,
        functions: [
            new_for_non_complex_scripts: skip,
            try_new_for_non_complex_scripts_with_buffer_provider,
            try_new_for_non_complex_scripts_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_for_non_complex_scripts)]
    pub fn try_new_for_non_complex_scripts_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV2>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + ?Sized,
    {
        let tailoring = if let Some(locale) = options.content_locale {
            provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_locale(&DataLocale::from(locale)),
                    metadata: {
                        let mut metadata = DataRequestMetadata::default();
                        metadata.silent = true;
                        metadata
                    },
                })
                .allow_identifier_not_found()?
                .map(|r| r.payload)
        } else {
            None
        };
        Ok(Self {
            options: options.resolve(),
            payload: provider.load(Default::default())?.payload,
            complex: ComplexPayloads::try_new(provider)?,
            tailoring,
        })
    }

    /// Loads LSTM data for a [`LineSegmenter`] constructed with
    /// [`LineSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `lstm` Cargo feature.*
    #[cfg(feature = "lstm")]
    pub fn load_lstm_unstable<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterLstmAutoV1> + ?Sized,
    {
        // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
        // characters [1]. Southeast Asian languages however require complex context analysis
        // [2].
        //
        // [1]: https://www.unicode.org/reports/tr14/#ID
        // [2]: https://www.unicode.org/reports/tr14/#SA
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

    /// Loads dictionary data for a [`LineSegmenter`] constructed with
    /// [`LineSegmenter::new_for_non_complex_scripts`].
    pub fn load_dictionary_unstable<D>(&mut self, provider: &D) -> Result<(), DataError>
    where
        D: DataProvider<SegmenterDictionaryExtendedV1> + ?Sized,
    {
        // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
        // characters [1]. Southeast Asian languages however require complex context analysis
        // [2].
        //
        // [1]: https://www.unicode.org/reports/tr14/#ID
        // [2]: https://www.unicode.org/reports/tr14/#SA
        self.complex.with_southeast_asian_dictionaries(provider)?;
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
    pub fn as_borrowed(&self) -> LineSegmenterBorrowed<'_> {
        LineSegmenterBorrowed {
            options: self.options,
            data: self.payload.get(),
            tailoring: self.tailoring.as_ref().map(|d| d.get()),
            complex: self.complex.as_borrowed(),
        }
    }
}

impl LineSegmenterBorrowed<'static> {
    /// Loads LSTM data for a [`LineSegmenter`] constructed with
    /// [`LineSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub fn load_lstm(&mut self) {
        // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
        // characters [1]. Southeast Asian languages however require complex context analysis
        // [2].
        //
        // [1]: https://www.unicode.org/reports/tr14/#ID
        // [2]: https://www.unicode.org/reports/tr14/#SA
        self.complex.with_southeast_asian_lstms();
    }

    /// Loads dictionary data for a [`LineSegmenter`] constructed with
    /// [`LineSegmenter::new_for_non_complex_scripts`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    #[cfg(feature = "compiled_data")]
    pub fn load_dictionary(&mut self) {
        // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
        // characters [1]. Southeast Asian languages however require complex context analysis
        // [2].
        //
        // [1]: https://www.unicode.org/reports/tr14/#ID
        // [2]: https://www.unicode.org/reports/tr14/#SA
        self.complex.with_southeast_asian_dictionaries();
    }

    /// Cheaply converts a [`LineSegmenterBorrowed<'static>`] into a [`LineSegmenter`].
    ///
    /// Note: Due to branching and indirection, using [`LineSegmenter`] might inhibit some
    /// compile-time optimizations that are possible with [`LineSegmenterBorrowed`].
    pub fn static_to_owned(self) -> LineSegmenter {
        LineSegmenter {
            payload: DataPayload::from_static_ref(self.data),
            tailoring: self.tailoring.map(DataPayload::from_static_ref),
            complex: self.complex.static_to_owned(),
            options: self.options,
        }
    }
}

impl<'data> LineSegmenterBorrowed<'data> {
    #[doc(hidden)]
    pub fn with_options(self, _options: LineBreakOptions) -> Self {
        Self { ..self }
    }

    /// Creates a line break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> LineBreakIterator<'data, 's, Utf8> {
        LineBreakIterator(RuleBreakIterator::new_with_complex(
            input.char_indices(),
            self.data,
            self.tailoring,
            self.complex,
            false,
            false as u8,
        ))
    }
    /// Creates a line break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> LineBreakIterator<'data, 's, PotentiallyIllFormedUtf8> {
        LineBreakIterator(RuleBreakIterator::new_with_complex(
            Utf8CharIndices::new(input),
            self.data,
            self.tailoring,
            self.complex,
            false,
            false as u8,
        ))
    }
    /// Creates a line break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> LineBreakIterator<'data, 's, Latin1> {
        LineBreakIterator(RuleBreakIterator::new_non_complex(
            Latin1Indices::new(input),
            self.data,
            self.tailoring,
        ))
    }

    /// Creates a line break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> LineBreakIterator<'data, 's, Utf16> {
        LineBreakIterator(RuleBreakIterator::new_with_complex(
            Utf16Indices::new(input),
            self.data,
            self.tailoring,
            self.complex,
            false,
            false as u8,
        ))
    }
}

impl<Y: RuleBreakType> LineBreakIterator<'_, '_, Y> {
    /// Returns whether the last break was mandatory
    pub fn is_mandatory(&self) -> bool {
        self.0.last_accepting_status == (true as u8)
    }
}

#[test]
fn test_mandatory() {
    let mut actual_breaks = LineSegmenter::new_for_non_complex_scripts(Default::default())
        .segment_str("this has a mandatory\nline break");

    assert_eq!(actual_breaks.next(), Some(0));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(5));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(9));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(11));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(21));
    assert_eq!(actual_breaks.is_mandatory(), true);
    assert_eq!(actual_breaks.next(), Some(26));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(31));
    assert_eq!(actual_breaks.is_mandatory(), true);
    assert_eq!(actual_breaks.next(), None);
}

#[test]
fn linebreak() {
    let segmenter = LineSegmenter::new_dictionary(Default::default());

    let mut iter = segmenter.segment_str("hello world");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(11), iter.next());
    assert_eq!(None, iter.next());

    iter = segmenter.segment_str("$10 $10");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(Some(7), iter.next());
    assert_eq!(None, iter.next());

    // LB10

    // LB14
    iter = segmenter.segment_str("[  abc def");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(7), iter.next());
    assert_eq!(Some(10), iter.next());
    assert_eq!(None, iter.next());

    let input: [u8; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
    let mut iter_u8 = segmenter.segment_latin1(&input);
    assert_eq!(Some(0), iter_u8.next());
    assert_eq!(Some(7), iter_u8.next());
    assert_eq!(Some(10), iter_u8.next());
    assert_eq!(None, iter_u8.next());

    let input: [u16; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(7), iter_u16.next());
    assert_eq!(Some(10), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    // LB15 used to prevent the break at 6, but has been removed in Unicode 15.1.
    iter = segmenter.segment_str("abc\u{0022}  (def");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(10), iter.next());
    assert_eq!(None, iter.next());

    let input: [u8; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
    let mut iter_u8 = segmenter.segment_latin1(&input);
    assert_eq!(Some(0), iter_u8.next());
    assert_eq!(Some(6), iter_u8.next());
    assert_eq!(Some(10), iter_u8.next());
    assert_eq!(None, iter_u8.next());

    let input: [u16; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(6), iter_u16.next());
    assert_eq!(Some(10), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    // Instead, in Unicode 15.1, LB15a and LB15b prevent these breaks.
    iter = segmenter.segment_str("« miaou »");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(11), iter.next());
    assert_eq!(None, iter.next());

    let input: Vec<u8> = "« miaou »"
        .chars()
        .map(|c| u8::try_from(u32::from(c)).unwrap())
        .collect();
    let mut iter_u8 = segmenter.segment_latin1(&input);
    assert_eq!(Some(0), iter_u8.next());
    assert_eq!(Some(9), iter_u8.next());
    assert_eq!(None, iter_u8.next());

    let input: Vec<u16> = "« miaou »".encode_utf16().collect();
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(9), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    // But not these:
    iter = segmenter.segment_str("Die Katze hat »miau« gesagt.");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(Some(10), iter.next());
    assert_eq!(Some(14), iter.next());
    assert_eq!(Some(23), iter.next());
    assert_eq!(Some(30), iter.next());
    assert_eq!(None, iter.next());

    let input: Vec<u8> = "Die Katze hat »miau« gesagt."
        .chars()
        .map(|c| u8::try_from(u32::from(c)).unwrap())
        .collect();
    let mut iter_u8 = segmenter.segment_latin1(&input);
    assert_eq!(Some(0), iter_u8.next());
    assert_eq!(Some(4), iter_u8.next());
    assert_eq!(Some(10), iter_u8.next());
    assert_eq!(Some(14), iter_u8.next());
    assert_eq!(Some(21), iter_u8.next());
    assert_eq!(Some(28), iter_u8.next());
    assert_eq!(None, iter_u8.next());

    let input: Vec<u16> = "Die Katze hat »miau« gesagt.".encode_utf16().collect();
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(4), iter_u16.next());
    assert_eq!(Some(10), iter_u16.next());
    assert_eq!(Some(14), iter_u16.next());
    assert_eq!(Some(21), iter_u16.next());
    assert_eq!(Some(28), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    // LB16
    iter = segmenter.segment_str("\u{0029}\u{203C}");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(4), iter.next());
    assert_eq!(None, iter.next());
    iter = segmenter.segment_str("\u{0029}  \u{203C}");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(None, iter.next());

    let input: [u16; 4] = [0x29, 0x20, 0x20, 0x203c];
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(4), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    // LB17
    iter = segmenter.segment_str("\u{2014}\u{2014}aa");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(8), iter.next());
    assert_eq!(None, iter.next());
    iter = segmenter.segment_str("\u{2014}  \u{2014}aa");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(8), iter.next());
    assert_eq!(Some(10), iter.next());
    assert_eq!(None, iter.next());

    iter = segmenter.segment_str("\u{2014}\u{2014}  \u{2014}\u{2014}123 abc");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(14), iter.next());
    assert_eq!(Some(18), iter.next());
    assert_eq!(Some(21), iter.next());
    assert_eq!(None, iter.next());

    // LB25
    let mut iter = segmenter.segment_str("(0,1)+(2,3)");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(11), iter.next());
    assert_eq!(None, iter.next());
    let input: [u16; 11] = [
        0x28, 0x30, 0x2C, 0x31, 0x29, 0x2B, 0x28, 0x32, 0x2C, 0x33, 0x29,
    ];
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(11), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    let input: [u16; 13] = [
        0x2014, 0x2014, 0x20, 0x20, 0x2014, 0x2014, 0x31, 0x32, 0x33, 0x20, 0x61, 0x62, 0x63,
    ];
    let mut iter_u16 = segmenter.segment_utf16(&input);
    assert_eq!(Some(0), iter_u16.next());
    assert_eq!(Some(6), iter_u16.next());
    assert_eq!(Some(10), iter_u16.next());
    assert_eq!(Some(13), iter_u16.next());
    assert_eq!(None, iter_u16.next());

    iter = segmenter.segment_str("\u{1F3FB} \u{1F3FB}");
    assert_eq!(Some(0), iter.next());
    assert_eq!(Some(5), iter.next());
    assert_eq!(Some(9), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
#[cfg(feature = "lstm")]
fn thai_line_break() {
    const TEST_STR: &str = "ภาษาไทยภาษาไทย";

    let segmenter = LineSegmenter::new_lstm(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
    assert_eq!(breaks, [0, 12, 21, 33, TEST_STR.len()], "Thai test");

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
    assert_eq!(breaks, [0, 4, 7, 11, utf16.len()], "Thai test");

    let utf16: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
    let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
    assert_eq!(breaks, [0, 4], "Thai test");
}

#[test]
#[cfg(feature = "lstm")]
fn burmese_line_break() {
    // "Burmese Language" in Burmese
    const TEST_STR: &str = "မြန်မာဘာသာစကား";

    let segmenter = LineSegmenter::new_lstm(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
    // LSTM model breaks more characters, but it is better to return [30].
    assert_eq!(breaks, [0, 12, 18, 30, TEST_STR.len()], "Burmese test");

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
    // LSTM model breaks more characters, but it is better to return [10].
    assert_eq!(breaks, [0, 4, 6, 10, utf16.len()], "Burmese utf-16 test");
}

#[test]
#[cfg(feature = "lstm")]
fn khmer_line_break() {
    const TEST_STR: &str = "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស";

    let segmenter = LineSegmenter::new_lstm(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
    // Note: This small sample matches the ICU dictionary segmenter
    assert_eq!(breaks, [0, 39, 48, 54, 72, TEST_STR.len()], "Khmer test");

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
    assert_eq!(
        breaks,
        [0, 13, 16, 18, 24, utf16.len()],
        "Khmer utf-16 test"
    );
}

#[test]
#[cfg(feature = "lstm")]
fn lao_line_break() {
    const TEST_STR: &str = "ກ່ຽວກັບສິດຂອງມະນຸດ";

    let segmenter = LineSegmenter::new_lstm(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
    // Note: LSTM finds a break at '12' that the dictionary does not find
    assert_eq!(breaks, [0, 12, 21, 30, 39, TEST_STR.len()], "Lao test");

    let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
    let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
    assert_eq!(breaks, [0, 4, 7, 10, 13, utf16.len()], "Lao utf-16 test");
}

#[test]
fn empty_string() {
    let segmenter = LineSegmenter::new_auto(Default::default());
    let breaks: Vec<usize> = segmenter.segment_str("").collect();
    assert_eq!(breaks, [0]);
}
