// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::indices::*;
use crate::provider::*;
use crate::rule_segmenter::*;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::char;
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, language};
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

#[doc(hidden)]
impl RuleBreakData<'_> {
    pub const LINE_PROPERTY_AI: u8 = 1;
    pub const LINE_PROPERTY_AL: u8 = 3;
    pub const LINE_PROPERTY_BA: u8 = 8;
    pub const LINE_PROPERTY_BK: u8 = 10;
    pub const LINE_PROPERTY_CJ: u8 = 12;
    pub const LINE_PROPERTY_CM: u8 = 14;
    pub const LINE_PROPERTY_CR: u8 = 16;
    pub const LINE_PROPERTY_EX: u8 = 19;
    pub const LINE_PROPERTY_H2: u8 = 21;
    pub const LINE_PROPERTY_H3: u8 = 22;
    pub const LINE_PROPERTY_HY: u8 = 24;
    pub const LINE_PROPERTY_ID: u8 = 25;
    pub const LINE_PROPERTY_IN: u8 = 27;
    pub const LINE_PROPERTY_JL: u8 = 29;
    pub const LINE_PROPERTY_JT: u8 = 30;
    pub const LINE_PROPERTY_JV: u8 = 31;
    pub const LINE_PROPERTY_LF: u8 = 32;
    pub const LINE_PROPERTY_NL: u8 = 33;
    pub const LINE_PROPERTY_NS: u8 = 34;
    pub const LINE_PROPERTY_NU: u8 = 35;
    pub const LINE_PROPERTY_PO_EAW: u8 = 39;
    pub const LINE_PROPERTY_PR_EAW: u8 = 41;
    pub const LINE_PROPERTY_SP: u8 = 47;
    pub const LINE_PROPERTY_ZW: u8 = 53;
    pub const LINE_PROPERTY_ZWJ: u8 = 54;
}

#[cfg_attr(not(test), allow(dead_code))]
#[doc(hidden)]
impl RuleBreakData<'_> {
    pub const LINE_PROPERTY_AK: u8 = 2;
    pub const LINE_PROPERTY_AL_DOTTED_CIRCLE: u8 = 4;
    pub const LINE_PROPERTY_AP: u8 = 5;
    pub const LINE_PROPERTY_AS: u8 = 6;
    pub const LINE_PROPERTY_B2: u8 = 7;
    pub const LINE_PROPERTY_BB: u8 = 9;
    pub const LINE_PROPERTY_CB: u8 = 11;
    pub const LINE_PROPERTY_CL: u8 = 13;
    pub const LINE_PROPERTY_CP: u8 = 15;
    pub const LINE_PROPERTY_EB: u8 = 17;
    pub const LINE_PROPERTY_EM: u8 = 18;
    pub const LINE_PROPERTY_GL: u8 = 20;
    pub const LINE_PROPERTY_HL: u8 = 23;
    pub const LINE_PROPERTY_ID_CN: u8 = 26;
    pub const LINE_PROPERTY_IS: u8 = 28;
    pub const LINE_PROPERTY_OP_EA: u8 = 36;
    pub const LINE_PROPERTY_OP_OP30: u8 = 37;
    pub const LINE_PROPERTY_PO: u8 = 38;
    pub const LINE_PROPERTY_PR: u8 = 40;
    pub const LINE_PROPERTY_QU: u8 = 42;
    pub const LINE_PROPERTY_QU_PF: u8 = 43;
    pub const LINE_PROPERTY_QU_PI: u8 = 44;
    pub const LINE_PROPERTY_RI: u8 = 45;
    pub const LINE_PROPERTY_SY: u8 = 48;
    pub const LINE_PROPERTY_VF: u8 = 49;
    pub const LINE_PROPERTY_VI: u8 = 50;
    pub const LINE_PROPERTY_WJ: u8 = 51;
    pub const LINE_PROPERTY_XX: u8 = 52;
}

/// An enum specifies the strictness of line-breaking rules. It can be passed as
/// an argument when creating a line segmenter.
///
/// Each enum value has the same meaning with respect to the `line-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#line-break-property>.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum LineBreakStrictness {
    /// Breaks text using the least restrictive set of line-breaking rules.
    /// Typically used for short lines, such as in newspapers.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-loose>
    Loose,

    /// Breaks text using the most common set of line-breaking rules.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-normal>
    Normal,

    /// Breaks text using the most stringent set of line-breaking rules.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-strict>
    ///
    /// This is the default behaviour of the Unicode Line Breaking Algorithm,
    /// resolving class [CJ](https://www.unicode.org/reports/tr14/#CJ) to
    /// [NS](https://www.unicode.org/reports/tr14/#NS);
    /// see rule [LB1](https://www.unicode.org/reports/tr14/#LB1).
    #[default]
    Strict,

    /// Breaks text assuming there is a soft wrap opportunity around every
    /// typographic character unit, disregarding any prohibition against line
    /// breaks. See more details in
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-anywhere>.
    Anywhere,
}

/// An enum specifies the line break opportunities between letters. It can be
/// passed as an argument when creating a line segmenter.
///
/// Each enum value has the same meaning with respect to the `word-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#word-break-property>
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub enum LineBreakWordOption {
    /// Words break according to their customary rules. See the details in
    /// <https://drafts.csswg.org/css-text-3/#valdef-word-break-normal>.
    #[default]
    Normal,

    /// Breaking is allowed within "words".
    /// <https://drafts.csswg.org/css-text-3/#valdef-word-break-break-all>
    BreakAll,

    /// Breaking is forbidden within "word".
    /// <https://drafts.csswg.org/css-text-3/#valdef-word-break-keep-all>
    KeepAll,
}

/// Options to tailor line-breaking behavior.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct LineBreakOptions<'a> {
    /// Strictness of line-breaking rules. See [`LineBreakStrictness`].
    ///
    /// Default is [`LineBreakStrictness::Strict`]
    pub strictness: Option<LineBreakStrictness>,

    /// Line break opportunities between letters. See [`LineBreakWordOption`].
    ///
    /// Default is [`LineBreakStrictness::Normal`]
    pub word_option: Option<LineBreakWordOption>,

    /// Content locale for line segmenter
    ///
    /// This allows more break opportunities when `LineBreakStrictness` is
    /// `Normal` or `Loose`. See
    /// <https://drafts.csswg.org/css-text-3/#line-break-property> for details.
    /// This option has no effect in Latin-1 mode.
    pub content_locale: Option<&'a LanguageIdentifier>,
}

impl LineBreakOptions<'_> {
    /// `const` version of [`Default::default`]
    pub const fn default() -> Self {
        Self {
            strictness: None,
            word_option: None,
            content_locale: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolvedLineBreakOptions {
    pub(crate) strictness: LineBreakStrictness,
    pub(crate) word_option: LineBreakWordOption,
    pub(crate) ja_zh: bool,
}

impl LineBreakOptions<'_> {
    pub(crate) const fn resolve(self) -> ResolvedLineBreakOptions {
        ResolvedLineBreakOptions {
            strictness: match self.strictness {
                Some(s) => s,
                None => LineBreakStrictness::Strict,
            },
            word_option: match self.word_option {
                Some(s) => s,
                None => LineBreakWordOption::Normal,
            },
            ja_zh: if let Some(content_locale) = self.content_locale.as_ref() {
                const JA: Language = language!("ja");
                const ZH: Language = language!("zh");
                matches!(content_locale.language, JA | ZH)
            } else {
                false
            },
        }
    }
}

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
/// # use icu::segmenter::LineSegmenter;
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
/// use icu::segmenter::LineSegmenter;
///
/// let segmenter = LineSegmenter::new_auto(Default::default());
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 6, 11]);
/// ```
///
/// Segment a string with CSS option overrides:
///
/// ```rust
/// use icu::segmenter::options::{
///     LineBreakOptions, LineBreakStrictness, LineBreakWordOption,
/// };
/// use icu::segmenter::LineSegmenter;
///
/// let mut options = LineBreakOptions::default();
/// options.strictness = Some(LineBreakStrictness::Strict);
/// options.word_option = Some(LineBreakWordOption::BreakAll);
/// options.content_locale = None;
/// let segmenter = LineSegmenter::new_auto(options);
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu::segmenter::LineSegmenter;
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
/// use icu::segmenter::LineSegmenter;
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
    payload: DataPayload<SegmenterBreakLineV1>,
    complex: ComplexPayloads,
}

/// Segments a string into lines (borrowed version).
///
/// See [`LineSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct LineSegmenterBorrowed<'data> {
    options: ResolvedLineBreakOptions,
    data: &'data RuleBreakData<'data>,
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
        D: DataProvider<SegmenterBreakLineV1>
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
        D: DataProvider<SegmenterBreakLineV1>
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
        D: DataProvider<SegmenterBreakLineV1>
            + DataProvider<SegmenterDictionaryExtendedV1>
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
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V1,
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
        D: DataProvider<SegmenterBreakLineV1>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self {
            options: options.resolve(),
            payload: provider.load(Default::default())?.payload,
            complex: ComplexPayloads::try_new(provider)?,
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
            complex: self.complex.as_borrowed(),
        }
    }
}

impl<'data> LineSegmenterBorrowed<'data> {
    #[doc(hidden)]
    pub fn with_options(self, options: LineBreakOptions) -> Self {
        Self {
            options: options.resolve(),
            ..self
        }
    }

    /// Creates a line break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> LineBreakIterator<'data, 's, Utf8> {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.data,
            options: self.options,
            complex: self.complex,
            handle_complex: line_handle_complex_utf8,
        }
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
        LineBreakIterator {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.data,
            options: self.options,
            complex: self.complex,
            handle_complex: line_handle_complex_utf8,
        }
    }
    /// Creates a line break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> LineBreakIterator<'data, 's, Latin1> {
        LineBreakIterator {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.data,
            options: self.options,
            complex: self.complex,
            handle_complex: |_, _| None,
        }
    }

    /// Creates a line break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> LineBreakIterator<'data, 's, Utf16> {
        LineBreakIterator {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.data,
            options: self.options,
            complex: self.complex,
            handle_complex: line_handle_complex_utf16,
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
            complex: self.complex.static_to_owned(),
            options: self.options,
        }
    }
}

fn is_break_utf32_by_normal(codepoint: u32, ja_zh: bool) -> bool {
    matches!(codepoint, 0x301C | 0x30A0 if ja_zh)
}

#[inline]
fn is_break_utf32_by_loose(
    right_codepoint: u32,
    left_prop: u8,
    right_prop: u8,
    ja_zh: bool,
) -> Option<bool> {
    Some(match (right_prop, right_codepoint, left_prop) {
        // breaks before hyphens
        (RuleBreakData::LINE_PROPERTY_BA, 0x2010 | 0x2013, RuleBreakData::LINE_PROPERTY_ID) => true,
        // breaks before certain CJK hyphen-like characters
        (RuleBreakData::LINE_PROPERTY_NS, 0x301C | 0x30A0, _) => ja_zh,
        // breaks before iteration marks
        (
            RuleBreakData::LINE_PROPERTY_NS,
            0x3005 | 0x303B | 0x309D | 0x309E | 0x30FD | 0x30FE,
            _,
        ) => true,
        // breaks before certain centered punctuation marks:
        (
            RuleBreakData::LINE_PROPERTY_NS,
            0x30FB | 0xFF1A | 0xFF1B | 0xFF65 | 0x203C | 0x2047..=0x2049,
            _,
        ) => ja_zh,
        // breaks between inseparable characters such as U+2025, U+2026 i.e. characters with the Unicode Line Break property IN
        (RuleBreakData::LINE_PROPERTY_IN, _, RuleBreakData::LINE_PROPERTY_IN) => true,
        // breaks before certain centered punctuation marks:
        (RuleBreakData::LINE_PROPERTY_EX, 0xFF01 | 0xFF1F, _) => ja_zh,
        // breaks before suffixes:
        // Characters with the Unicode Line Break property PO and the East Asian Width property
        (RuleBreakData::LINE_PROPERTY_PO_EAW, _, _) => ja_zh,
        // breaks after prefixes:
        // Characters with the Unicode Line Break property PR and the East Asian Width property
        (_, _, RuleBreakData::LINE_PROPERTY_PR_EAW) => ja_zh,
        _ => return None,
    })
}

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
pub struct LineBreakIterator<'data, 's, Y: RuleBreakType> {
    iter: Y::IterAttr<'s>,
    len: usize,
    current_pos_data: Option<(usize, Y::CharType)>,
    result_cache: Vec<usize>,
    data: &'data RuleBreakData<'data>,
    options: ResolvedLineBreakOptions,
    complex: ComplexPayloadsBorrowed<'data>,
    // Should return None if there is no complex handling
    pub(crate) handle_complex:
        fn(&mut LineBreakIterator<'data, 's, Y>, Y::CharType) -> Option<usize>,
}

impl<Y: RuleBreakType> Iterator for LineBreakIterator<'_, '_, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.options.strictness == LineBreakStrictness::Anywhere {
            let mut grapheme_iter: RuleBreakIterator<'_, '_, Y> = RuleBreakIterator {
                iter: self.iter.clone(),
                len: self.len,
                current_pos_data: self.current_pos_data,
                data: self.complex.grapheme.data,
                result_cache: Default::default(),
                complex: None,
                boundary_property: 0,
                locale_override: None,
                handle_complex: empty_handle_complex,
            };
            let r = grapheme_iter.next();
            self.iter = grapheme_iter.iter;
            self.len = grapheme_iter.len;
            self.current_pos_data = grapheme_iter.current_pos_data;
            return r;
        }

        match self.check_eof() {
            StringBoundaryPosType::Start => return Some(0),
            StringBoundaryPosType::End => return None,
            _ => (),
        }

        // If we have break point cache by previous run, return this result
        if let Some(&first_pos) = self.result_cache.first() {
            let mut i = 0;
            loop {
                if i == first_pos {
                    self.result_cache = self.result_cache.iter().skip(1).map(|r| r - i).collect();
                    return self.get_current_position();
                }
                i += self.get_current_codepoint().map_or(0, Y::char_len);
                self.advance_iter();
                if self.is_eof() {
                    self.result_cache.clear();
                    return Some(self.len);
                }
            }
        }

        // The state prior to a sequence of CM and ZWJ affected by rule LB9.
        let mut lb9_left: Option<u8> = None;
        // Whether LB9 was applied to a ZWJ, so that breaks at the current
        // position must be suppressed.
        let mut lb8a_after_lb9 = false;

        'a: loop {
            debug_assert!(!self.is_eof());

            let left_codepoint = self.get_current_codepoint()?;
            self.advance_iter();
            let Some(right_codepoint) = self.get_current_codepoint() else {
                return Some(self.len);
            };

            let left_prop = lb9_left.unwrap_or_else(|| self.get_linebreak_property(left_codepoint));
            let right_prop = self.get_linebreak_property(right_codepoint);

            // UAX14 doesn't have Thai etc, so use another way.
            if Y::CAN_CONTAIN_SA
                && self.get_linebreak_property(left_codepoint) == self.data.complex_property
                && right_prop == self.data.complex_property
            {
                let result = (self.handle_complex)(self, left_codepoint);
                if result.is_some() {
                    return result;
                }
                // I may have to fetch text until non-SA character?.
            }

            let after_zwj = lb8a_after_lb9
                || (lb9_left.is_none() && left_prop == RuleBreakData::LINE_PROPERTY_ZWJ);

            if (right_prop == RuleBreakData::LINE_PROPERTY_CM
                || right_prop == RuleBreakData::LINE_PROPERTY_ZWJ)
                && left_prop != RuleBreakData::LINE_PROPERTY_BK
                && left_prop != RuleBreakData::LINE_PROPERTY_CR
                && left_prop != RuleBreakData::LINE_PROPERTY_LF
                && left_prop != RuleBreakData::LINE_PROPERTY_NL
                && left_prop != RuleBreakData::LINE_PROPERTY_SP
                && left_prop != RuleBreakData::LINE_PROPERTY_ZW
            {
                lb9_left = Some(left_prop);
                lb8a_after_lb9 = right_prop == RuleBreakData::LINE_PROPERTY_ZWJ;
                continue;
            } else {
                lb9_left = None;
                lb8a_after_lb9 = false;
            }

            // CSS word-break property handling
            #[allow(clippy::single_match)]
            if self.options.word_option == LineBreakWordOption::KeepAll {
                //  typographic letter units shouldn't be break
                if matches!(
                    left_prop,
                    RuleBreakData::LINE_PROPERTY_AI
                        | RuleBreakData::LINE_PROPERTY_AL
                        | RuleBreakData::LINE_PROPERTY_ID
                        | RuleBreakData::LINE_PROPERTY_NU
                        | RuleBreakData::LINE_PROPERTY_HY
                        | RuleBreakData::LINE_PROPERTY_H2
                        | RuleBreakData::LINE_PROPERTY_H3
                        | RuleBreakData::LINE_PROPERTY_JL
                        | RuleBreakData::LINE_PROPERTY_JV
                        | RuleBreakData::LINE_PROPERTY_JT
                        | RuleBreakData::LINE_PROPERTY_CJ
                ) && matches!(
                    right_prop,
                    RuleBreakData::LINE_PROPERTY_AI
                        | RuleBreakData::LINE_PROPERTY_AL
                        | RuleBreakData::LINE_PROPERTY_ID
                        | RuleBreakData::LINE_PROPERTY_NU
                        | RuleBreakData::LINE_PROPERTY_HY
                        | RuleBreakData::LINE_PROPERTY_H2
                        | RuleBreakData::LINE_PROPERTY_H3
                        | RuleBreakData::LINE_PROPERTY_JL
                        | RuleBreakData::LINE_PROPERTY_JV
                        | RuleBreakData::LINE_PROPERTY_JT
                        | RuleBreakData::LINE_PROPERTY_CJ
                ) {
                    continue;
                }
            }

            // CSS line-break property handling
            match self.options.strictness {
                LineBreakStrictness::Normal
                    if is_break_utf32_by_normal(right_codepoint.into(), self.options.ja_zh)
                        && !after_zwj =>
                {
                    return self.get_current_position();
                }
                LineBreakStrictness::Loose => {
                    if let Some(breakable) = is_break_utf32_by_loose(
                        right_codepoint.into(),
                        left_prop,
                        right_prop,
                        self.options.ja_zh,
                    ) {
                        if breakable && !after_zwj {
                            return self.get_current_position();
                        }
                        continue;
                    }
                }
                _ => (),
            };

            // If break_state is equals or grater than 0, it is alias of property.
            match self.data.get_break_state_from_table(left_prop, right_prop) {
                BreakState::Break | BreakState::NoMatch => {
                    if after_zwj {
                        continue;
                    } else {
                        return self.get_current_position();
                    }
                }
                BreakState::Keep => continue,
                BreakState::Index(mut index) | BreakState::Intermediate(mut index) => {
                    let mut previous_iter = self.iter.clone();
                    let mut previous_pos_data = self.current_pos_data;
                    let mut previous_is_after_zwj = after_zwj;

                    // Since we are building up a state in this inner loop, we do not
                    // need an analogue of lb9_left; continuing the inner loop preserves
                    // `index` which is the current state, and thus implements the
                    // “treat as” rule.
                    let mut left_prop_pre_lb9 = right_prop;

                    // current state isn't resolved due to intermediating.
                    // Example, [AK] [AS] is processing LB28a, but if not matched after fetching
                    // data, we should break after [AK].
                    let is_intermediate_rule_no_match = if lb8a_after_lb9 {
                        // left was ZWJ so we don't break between ZWJ.
                        true
                    } else {
                        index > self.data.last_codepoint_property
                    };

                    loop {
                        self.advance_iter();
                        let after_zwj = left_prop_pre_lb9 == RuleBreakData::LINE_PROPERTY_ZWJ;

                        let previous_break_state_is_cp_prop =
                            index <= self.data.last_codepoint_property;

                        let Some(prop) = self.get_current_linebreak_property() else {
                            // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                            let break_state = self
                                .data
                                .get_break_state_from_table(index, self.data.eot_property);
                            if break_state == BreakState::NoMatch {
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                if previous_is_after_zwj {
                                    // Do not break [AK] [ZWJ] ÷ [AS] (eot).
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            // EOF
                            return Some(self.len);
                        };

                        if (prop == RuleBreakData::LINE_PROPERTY_CM
                            || prop == RuleBreakData::LINE_PROPERTY_ZWJ)
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_BK
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_CR
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_LF
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_NL
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_SP
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_ZW
                        {
                            left_prop_pre_lb9 = prop;
                            continue;
                        }

                        match self.data.get_break_state_from_table(index, prop) {
                            BreakState::Keep => continue 'a,
                            BreakState::NoMatch => {
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                if after_zwj {
                                    // Break [AK] ÷ [AS] [ZWJ] [XX],
                                    // but not [AK] [ZWJ] ÷ [AS] [ZWJ] [XX].
                                    if is_intermediate_rule_no_match && !previous_is_after_zwj {
                                        return self.get_current_position();
                                    }
                                    continue 'a;
                                } else if previous_is_after_zwj {
                                    // Do not break [AK] [ZWJ] ÷ [AS] [XX].
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            BreakState::Break => {
                                if after_zwj {
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            BreakState::Intermediate(i) => {
                                index = i;
                                previous_iter = self.iter.clone();
                                previous_pos_data = self.current_pos_data;
                                previous_is_after_zwj = after_zwj;
                            }
                            BreakState::Index(i) => {
                                index = i;
                                if previous_break_state_is_cp_prop {
                                    previous_iter = self.iter.clone();
                                    previous_pos_data = self.current_pos_data;
                                    previous_is_after_zwj = after_zwj;
                                }
                            }
                        }
                        left_prop_pre_lb9 = prop;
                    }
                }
            }
        }
    }
}

enum StringBoundaryPosType {
    Start,
    Middle,
    End,
}

impl<Y: RuleBreakType> LineBreakIterator<'_, '_, Y> {
    fn advance_iter(&mut self) {
        self.current_pos_data = self.iter.next();
    }

    fn is_eof(&self) -> bool {
        self.current_pos_data.is_none()
    }

    #[inline]
    fn check_eof(&mut self) -> StringBoundaryPosType {
        if self.is_eof() {
            self.advance_iter();
            if self.is_eof() {
                if self.len == 0 {
                    // Empty string. Since `self.current_pos_data` is always going to be empty,
                    // we never read `self.len` except for here, so we can use it to mark that
                    // we have already returned the single empty-string breakpoint.
                    self.len = 1;
                    StringBoundaryPosType::Start
                } else {
                    StringBoundaryPosType::End
                }
            } else {
                StringBoundaryPosType::Start
            }
        } else {
            StringBoundaryPosType::Middle
        }
    }

    fn get_current_position(&self) -> Option<usize> {
        self.current_pos_data.map(|(pos, _)| pos)
    }

    fn get_current_codepoint(&self) -> Option<Y::CharType> {
        self.current_pos_data.map(|(_, codepoint)| codepoint)
    }

    fn get_linebreak_property(&self, codepoint: Y::CharType) -> u8 {
        match (
            (self.options.word_option, self.options.strictness),
            self.data.property_table.get32(codepoint.into()),
        ) {
            // CJ is treated as NS by default, yielding strict line breaking.
            // https://www.unicode.org/reports/tr14/#CJ
            (
                (LineBreakWordOption::BreakAll, _)
                | (_, LineBreakStrictness::Loose | LineBreakStrictness::Normal),
                RuleBreakData::LINE_PROPERTY_CJ,
            ) => RuleBreakData::LINE_PROPERTY_ID, // All CJ's General_Category is Other_Letter (Lo).
            ((LineBreakWordOption::BreakAll, _), p) if p == self.data.complex_property => {
                RuleBreakData::LINE_PROPERTY_ID
            }
            (
                (LineBreakWordOption::BreakAll, _),
                RuleBreakData::LINE_PROPERTY_AL | RuleBreakData::LINE_PROPERTY_NU,
            ) => RuleBreakData::LINE_PROPERTY_ID,
            (_, prop) => prop,
        }
    }

    fn get_current_linebreak_property(&self) -> Option<u8> {
        self.get_current_codepoint()
            .map(|c| self.get_linebreak_property(c))
    }
}

fn line_handle_complex_utf8<T>(
    iter: &mut LineBreakIterator<'_, '_, T>,
    left_codepoint: char,
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
        if let Some(current_codepoint) = iter.get_current_codepoint() {
            if iter.get_linebreak_property(current_codepoint) != iter.data.complex_property {
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
    let breaks = iter.complex.segment_str(&s);
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

fn line_handle_complex_utf16<T>(
    iterator: &mut LineBreakIterator<'_, '_, T>,
    left_codepoint: T::CharType,
) -> Option<usize>
where
    T: RuleBreakType<CharType = u32>,
{
    // word segmenter doesn't define break rules for some scripts such as Thai.
    let start_iter = iterator.iter.clone();
    let start_point = iterator.current_pos_data;
    let mut s = vec![left_codepoint as u16];
    loop {
        debug_assert!(!iterator.is_eof());
        s.push(iterator.get_current_codepoint()? as u16);
        iterator.advance_iter();
        if let Some(current_codepoint) = iterator.get_current_codepoint() {
            if iterator.get_linebreak_property(current_codepoint) != iterator.data.complex_property
            {
                break;
            }
        } else {
            // EOF
            break;
        }
    }

    // Restore iterator to move to head of complex string
    iterator.iter = start_iter;
    iterator.current_pos_data = start_point;
    let breaks = iterator.complex.segment_utf16(&s);
    iterator.result_cache = breaks;
    // result_cache vector is utf-16 index that is in BMP.
    let first_pos = *iterator.result_cache.first()?;
    let mut i = 1;
    loop {
        if i == first_pos {
            // Re-calculate breaking offset
            iterator.result_cache = iterator
                .result_cache
                .iter()
                .skip(1)
                .map(|r| r - i)
                .collect();
            return iterator.get_current_position();
        }
        debug_assert!(
            i < first_pos,
            "we should always arrive at first_pos: near index {:?}",
            iterator.get_current_position()
        );
        i += 1;
        iterator.advance_iter();
        if iterator.is_eof() {
            iterator.result_cache.clear();
            return Some(iterator.len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    include!("../tests/helpers.rs.raw");

    #[test]
    fn linebreak_property() {
        let segmenter =
            LineSegmenter::new_for_non_complex_scripts(Default::default()).segment_str("input");

        assert_eq!(
            segmenter.get_linebreak_property('\u{0020}'),
            RuleBreakData::LINE_PROPERTY_SP
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{0022}'),
            RuleBreakData::LINE_PROPERTY_QU
        );
        assert_eq!(
            segmenter.get_linebreak_property('('),
            RuleBreakData::LINE_PROPERTY_OP_OP30
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{0030}'),
            RuleBreakData::LINE_PROPERTY_NU
        );
        assert_eq!(
            segmenter.get_linebreak_property('['),
            RuleBreakData::LINE_PROPERTY_OP_OP30
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{1f3fb}'),
            RuleBreakData::LINE_PROPERTY_EM
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{20000}'),
            RuleBreakData::LINE_PROPERTY_ID
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{e0020}'),
            RuleBreakData::LINE_PROPERTY_CM
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{3041}'),
            RuleBreakData::LINE_PROPERTY_CJ
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{0025}'),
            RuleBreakData::LINE_PROPERTY_PO
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{00A7}'),
            RuleBreakData::LINE_PROPERTY_AI
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{50005}'),
            RuleBreakData::LINE_PROPERTY_XX
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{17D6}'),
            RuleBreakData::LINE_PROPERTY_NS
        );
        assert_eq!(
            segmenter.get_linebreak_property('\u{2014}'),
            RuleBreakData::LINE_PROPERTY_B2
        );
    }

    #[test]
    fn break_rule() {
        let payload = DataProvider::<SegmenterBreakLineV1>::load(&Baked, Default::default())
            .expect("Loading should succeed!")
            .payload;
        let lb_data: &RuleBreakData = payload.get();

        let is_break = |left, right| {
            matches!(
                lb_data.get_break_state_from_table(left, right),
                BreakState::Break | BreakState::NoMatch
            )
        };

        // LB4
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_BK,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB5
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CR,
                RuleBreakData::LINE_PROPERTY_LF
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CR,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_LF,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_NL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB6
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BK
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CR
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_LF
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NL
            ),
            false
        );
        // LB7
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_SP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_ZW
            ),
            false
        );
        // LB8
        // LB8a and LB9 omitted: These are handled outside of the state table.
        // LB10
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ZWJ,
                RuleBreakData::LINE_PROPERTY_SP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_CM
            ),
            true
        );
        // LB11
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_WJ
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_WJ,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB12
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_GL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB12a
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_GL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_GL
            ),
            true
        );
        // LB13
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_EX
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_IS
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_SY
            ),
            false
        );
        // LB18
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB19
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_QU
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_QU,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB20
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CB
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CB,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB20
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_HY
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NS
            ),
            false
        );
        // LB21
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_BB,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_NS
            ),
            false
        );
        // LB21a
        // LB21b
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SY,
                RuleBreakData::LINE_PROPERTY_HL
            ),
            false
        );
        // LB22
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_IN
            ),
            false
        );
        // LB 23
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NU
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_HL,
                RuleBreakData::LINE_PROPERTY_NU
            ),
            false
        );
        // LB 23a
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_ID
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_EB
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_EM
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EB,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EM,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        // LB26
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_JL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_JV
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_H2
            ),
            false
        );
        // LB27
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_IN
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_JL
            ),
            false
        );
        // LB28
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_HL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB29
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_IS,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_IS,
                RuleBreakData::LINE_PROPERTY_HL
            ),
            false
        );
        // LB30b
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EB,
                RuleBreakData::LINE_PROPERTY_EM
            ),
            false
        );
        // LB31
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_ID
            ),
            true
        );
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
