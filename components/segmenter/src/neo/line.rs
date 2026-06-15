// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::complex::{ComplexPayloads, ComplexPayloadsBorrowed};
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::iterator_helpers::derive_usize_iterator_with_type;
use crate::line::{LineBreakOptions, LineBreakStrictness, LineBreakWordOption};
#[cfg(feature = "compiled_data")]
use crate::provider::Baked;
#[cfg(feature = "lstm")]
use crate::provider::SegmenterLstmAutoV1;
use crate::provider::*;
use crate::scaffold::{Latin1, PotentiallyIllFormedUtf8, RuleBreakType, Utf8, Utf16};
use core::marker::PhantomData;
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
    RuleBreakIterator<'data, 's, Y, ComplexLine<Y>>,
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
    payload: DataPayload<SegmenterBreakLineV2>,
    tailoring: Option<DataPayload<SegmenterBreakLineOverrideV2>>,
    complex: ComplexPayloads,
}

/// Segments a string into lines (borrowed version).
///
/// See [`LineSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct LineSegmenterBorrowed<'data> {
    data: &'data SegmenterStateMachine<'data>,
    tailoring: Option<&'data SegmenterStateMachineOverride<'data>>,
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
            + DataProvider<SegmenterBreakGraphemeClusterV2>
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
            + DataProvider<SegmenterBreakGraphemeClusterV2>
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
            + DataProvider<SegmenterBreakGraphemeClusterV2>
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
        let options = options.resolve();
        LineSegmenterBorrowed {
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2,
            tailoring: match (options.ja_zh, options.strictness, options.word_option) {
                (true, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_LOOSE_CJ)
                }
                (false, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_LOOSE)
                }
                (true, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_NORMAL_CJ)
                }
                (false, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_NORMAL)
                }
                (true, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_CJ)
                }
                (false, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => None,
                (_, LineBreakStrictness::Anywhere, _) => {
                    // Return a line segmenter that is actually a grapheme cluster segmenter.
                    return LineSegmenterBorrowed {
                        data: Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V2,
                        tailoring: None,
                        complex: ComplexPayloadsBorrowed::new(),
                    };
                }
                (false, LineBreakStrictness::Strict, LineBreakWordOption::BreakAll) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_WORD_BREAKALL)
                }
                (false, LineBreakStrictness::Strict, LineBreakWordOption::KeepAll) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_WORD_KEEPALL)
                }
                _ => unimplemented!(),
            },
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
            + DataProvider<SegmenterBreakGraphemeClusterV2>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + ?Sized,
    {
        let options = options.resolve();

        let tailoring = match (options.ja_zh, options.strictness, options.word_option) {
            (true, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("loose_cj")) }
            }
            (false, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("loose")) }
            }
            (true, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("normal_cj")) }
            }
            (false, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("normal")) }
            }
            (true, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("cj")) }
            }
            (_, LineBreakStrictness::Anywhere, _) => {
                // Return a line segmenter that is actually a grapheme cluster segmenter.
                return Ok(Self {
                    payload: DataProvider::<SegmenterBreakGraphemeClusterV2>::load(
                        provider,
                        Default::default(),
                    )?
                    .payload
                    .cast(),
                    tailoring: None,
                    complex: ComplexPayloads::try_new(provider)?,
                });
            }
            (false, LineBreakStrictness::Strict, LineBreakWordOption::BreakAll) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("word_breakall")) }
            }
            (false, LineBreakStrictness::Strict, LineBreakWordOption::KeepAll) => {
                const { Some(DataMarkerAttributes::from_str_or_panic("word_keepall")) }
            }
            _ => unimplemented!(),
        }
        .map(|a| {
            provider.load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(a),
                metadata: Default::default(),
            })
        })
        .transpose()?
        .map(|d| d.payload);

        Ok(Self {
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
        }
    }
}

#[derive(Debug)]
struct ComplexLine<Y>(PhantomData<Y>);

impl<Y: RuleBreakType> ComplexHandler<Y> for ComplexLine<Y> {
    const BREAK_AT_BOUNDARIES: bool = false;
    const BREAK_STATUS: u8 = false as u8;
    type Cache = [usize; 16];
    type Data<'s> = Y::ComplexData<'s>;

    fn resolve_symbol(symbol: Symbol) -> Symbol {
        if !matches!(
            symbol & 0b1111_1110,
            SegmenterStateMachine::LB_SA_KHMER_SYMBOL
                | SegmenterStateMachine::LB_SA_THAI_SYMBOL
                | SegmenterStateMachine::LB_SA_LAO_SYMBOL
                | SegmenterStateMachine::LB_SA_MYANMAR_SYMBOL
        ) {
            return symbol;
        }
        if symbol & 1 == 0 {
            SegmenterStateMachine::LB_SA_SYMBOL
        } else {
            SegmenterStateMachine::LB_SA_CM_SYMBOL
        }
    }

    fn handle<'data, 's>(
        symbol: Symbol,
        dfa: &RuleBreakIterator<'_, '_, Y, Self>,
        data: &Self::Data<'data>,
        iter: Y::IterAttr<'s>,
    ) -> Option<(ComplexIterator<'data, 's, Y>, Y::IterAttr<'s>)> {
        use crate::complex::Language;

        let data = Y::select_complex(
            data,
            match symbol & 0b1111_1110 {
                SegmenterStateMachine::LB_SA_KHMER_SYMBOL => Language::Khmer,
                SegmenterStateMachine::LB_SA_THAI_SYMBOL => Language::Thai,
                SegmenterStateMachine::LB_SA_LAO_SYMBOL => Language::Lao,
                SegmenterStateMachine::LB_SA_MYANMAR_SYMBOL => Language::Burmese,
                _ => return None,
            },
        )?;

        let mut past_complex = iter.clone();
        let mut last_complex = past_complex.clone();
        past_complex.next();
        while past_complex.clone().next().is_some_and(|(_, cp)| {
            // Ignore the last bit, which is the difference between XX_SYMBOL and XX_CM_SYMBOL.
            dfa.symbol(cp.into()) & 0b1111_1110 == symbol & 0b1111_1110
        }) {
            past_complex.next();
            last_complex.next();
        }

        Some((Y::handle_complex(&data, &iter, &past_complex), last_complex))
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
        LineBreakIterator(RuleBreakIterator::new(
            input.char_indices(),
            self.data,
            self.tailoring,
            Some(self.complex),
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
        LineBreakIterator(RuleBreakIterator::new(
            Utf8CharIndices::new(input),
            self.data,
            self.tailoring,
            Some(self.complex),
        ))
    }
    /// Creates a line break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> LineBreakIterator<'data, 's, Latin1> {
        LineBreakIterator(RuleBreakIterator::new(
            Latin1Indices::new(input),
            self.data,
            self.tailoring,
            None,
        ))
    }

    /// Creates a line break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> LineBreakIterator<'data, 's, Utf16> {
        LineBreakIterator(RuleBreakIterator::new(
            Utf16Indices::new(input),
            self.data,
            self.tailoring,
            Some(self.complex),
        ))
    }
}

impl<Y: RuleBreakType> LineBreakIterator<'_, '_, Y> {
    /// Returns whether the last break was mandatory
    pub fn is_mandatory(&self) -> bool {
        self.0.last_accepting_status == (true as u8)
    }
}

#[cfg(test)]
mod tests {
    use crate::neo::*;

    include!("../../tests/helpers.rs.raw");

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

        check_line("hello world", &["hello ", "world"], segmenter);

        check_line("$10 $10", &["$10 ", "$10"], segmenter);

        // LB10

        // LB14
        check_line("[  abc def", &["[  abc ", "def"], segmenter);

        // LB15 used to prevent the break at 6, but has been removed in Unicode 15.1.
        check_line("abc\u{0022}  (def", &["abc\u{0022}  ", "(def"], segmenter);

        // Instead, in Unicode 15.1, LB15a and LB15b prevent these breaks.
        check_line("« miaou »", &["« miaou »"], segmenter);

        // But not these:
        check_line(
            "Die Katze hat »miau« gesagt.",
            &["Die ", "Katze ", "hat ", "»miau« ", "gesagt."],
            segmenter,
        );

        // LB16
        check_line("\u{0029}\u{203C}", &["\u{0029}\u{203C}"], segmenter);
        check_line("\u{0029}  \u{203C}", &["\u{0029}  \u{203C}"], segmenter);

        // LB17
        check_line("\u{2014}\u{2014}aa", &["\u{2014}\u{2014}", "aa"], segmenter);
        check_line(
            "\u{2014}  \u{2014}aa",
            &["\u{2014}  \u{2014}", "aa"],
            segmenter,
        );

        check_line(
            "\u{2014}\u{2014}  \u{2014}\u{2014}123 abc",
            &["\u{2014}\u{2014}  \u{2014}\u{2014}", "123 ", "abc"],
            segmenter,
        );

        // LB25
        check_line("(0,1)+(2,3)", &["(0,1)+(2,3)"], segmenter);

        check_line("——  ——123 abc", &["——  ——", "123 ", "abc"], segmenter);
        check_line(
            "\u{1F3FB} \u{1F3FB}",
            &["\u{1F3FB} ", "\u{1F3FB}"],
            segmenter,
        );
    }

    #[test]
    fn thai_line_break() {
        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            LineSegmenter::new_lstm(Default::default()),
        );

        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            LineSegmenter::new_dictionary(Default::default()),
        );

        check_line(
            "ภาษา",
            &["ภาษา"],
            LineSegmenter::new_lstm(Default::default()),
        );

        check_line(
            "ภาษา",
            &["ภาษา"],
            LineSegmenter::new_dictionary(Default::default()),
        );
    }

    #[test]
    fn burmese_line_break() {
        // "Burmese Language" in Burmese

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်", "မာ", "ဘာသာ", "စကား"],
            LineSegmenter::new_lstm(Default::default()),
        );

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်မာဘာသာ", "စကား"],
            LineSegmenter::new_dictionary(Default::default()),
        );
    }

    #[test]
    fn khmer_line_break() {
        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            LineSegmenter::new_lstm(Default::default()),
        );

        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            LineSegmenter::new_dictionary(Default::default()),
        );
    }

    #[test]
    fn lao_line_break() {
        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວ", "ກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            LineSegmenter::new_lstm(Default::default()),
        );

        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            LineSegmenter::new_dictionary(Default::default()),
        );
    }

    #[test]
    fn empty_string() {
        let segmenter = LineSegmenter::new_auto(Default::default());
        let breaks: Vec<usize> = segmenter.segment_str("").collect();
        assert_eq!(breaks, [0]);
    }
}
