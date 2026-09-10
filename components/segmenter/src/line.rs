// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::indices::*;
use crate::provider::*;
use crate::scaffold::*;
use icu_locale_core::LanguageIdentifier;
use icu_locale_core::subtags::{Language, language};
use icu_provider::prelude::*;

mod v1;
/// The Unicode 17 line breaking implementation, used with `SegmenterBreakLineV3` data.
#[cfg(any(feature = "unstable", feature = "datagen"))]
#[cfg_attr(not(feature = "unstable"), allow(dead_code))]
mod v3;

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
/// Line segmenter is currently compatible with [Unicode Standard Annex #14][UAX14] (version 15.1.0).
/// The `*_17_*` constructors, which require the `unstable` Cargo feature, implement version 17.0.0.
/// The `*_neo_*` constructors, which require the `unstable` Cargo feature, implement version
#[cfg_attr(feature = "compiled_data", doc = icu_segmenter_data::unicode_tag!())]
/// .
///
/// [UAX14]: https://www.unicode.org/reports/tr14/tr14-55.html
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
/// use icu::segmenter::LineSegmenter;
/// use icu::segmenter::options::{
///     LineBreakOptions, LineBreakStrictness, LineBreakWordOption,
/// };
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
/// use icu::properties::{CodePointMapData, props::LineBreak};
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
pub struct LineSegmenter(LineSegmenterInner);

#[derive(Debug)]
enum LineSegmenterInner {
    V1 {
        options: ResolvedLineBreakOptions,
        data: DataPayload<SegmenterBreakLineV1>,
        complex: ComplexPayloads,
    },
    #[cfg(feature = "unstable")]
    V2 {
        data: DataPayload<SegmenterBreakLineV2>,
        tailoring: Option<DataPayload<SegmenterBreakLineOverrideV2>>,
        complex: ComplexPayloads,
    },
    #[cfg(feature = "unstable")]
    V3 {
        options: ResolvedLineBreakOptions,
        data: DataPayload<SegmenterBreakLineV3>,
        complex: ComplexPayloads,
    },
}

/// Segments a string into lines (borrowed version).
///
/// See [`LineSegmenter`] for examples.
#[derive(Clone, Debug, Copy)]
pub struct LineSegmenterBorrowed<'data>(LineSegmenterBorrowedInner<'data>);

#[derive(Debug, Clone, Copy)]
enum LineSegmenterBorrowedInner<'data> {
    V1 {
        options: ResolvedLineBreakOptions,
        data: &'data RuleBreakData<'data>,
        complex: ComplexPayloadsBorrowed<'data>,
    },
    #[cfg(feature = "unstable")]
    V2 {
        data: &'data SegmenterStateMachine<'data>,
        tailoring: Option<&'data SegmenterStateMachineOverride<'data>>,
        complex: ComplexPayloadsBorrowed<'data>,
    },
    #[cfg(feature = "unstable")]
    V3 {
        options: ResolvedLineBreakOptions,
        data: &'data RuleBreakData<'data>,
        complex: ComplexPayloadsBorrowed<'data>,
    },
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
        LineSegmenterBorrowed(LineSegmenterBorrowedInner::V1 {
            options: options.resolve(),
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V1,
            complex: ComplexPayloadsBorrowed::new(),
        })
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// no support for scripts requiring complex context dependent line breaks (Khmer, Lao, Myanmar, Thai).
    ///
    /// ✨ *Enabled with the `unstable` Cargo feature.*
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "unstable")]
    #[cfg(feature = "compiled_data")]
    pub const fn new_17_for_non_complex_scripts(
        options: LineBreakOptions,
    ) -> LineSegmenterBorrowed<'static> {
        LineSegmenterBorrowed(LineSegmenterBorrowedInner::V3 {
            options: options.resolve(),
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V3,
            complex: ComplexPayloadsBorrowed::new(),
        })
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
        Ok(Self(LineSegmenterInner::V1 {
            data: provider.load(Default::default())?.payload,
            options: options.resolve(),
            complex: ComplexPayloads::try_new(provider)?,
        }))
    }

    #[cfg(feature = "unstable")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_17_for_non_complex_scripts)]
    pub fn try_new_17_for_non_complex_script_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV3>
            + DataProvider<SegmenterBreakGraphemeClusterV1>
            + ?Sized,
    {
        Ok(Self(LineSegmenterInner::V3 {
            data: provider.load(Default::default())?.payload,
            options: options.resolve(),
            complex: ComplexPayloads::try_new(provider)?,
        }))
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// no support for scripts requiring complex context dependent line breaks (Khmer, Lao, Myanmar, Thai).
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "unstable")]
    pub const fn new_neo_for_non_complex_scripts(
        options: LineBreakOptions,
    ) -> LineSegmenterBorrowed<'static> {
        const _: () = assert!(
            Baked::SEGMENTER_BREAK_LINE_V2_CHECKSUM
                == Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_CHECKSUM
        );

        let options = options.resolve();
        LineSegmenterBorrowed(LineSegmenterBorrowedInner::V2 {
            data: Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2,
            tailoring: match (options.ja_zh, options.strictness, options.word_option) {
                (_, _, LineBreakWordOption::BreakAll) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_BREAKALL)
                }
                (_, _, LineBreakWordOption::KeepAll) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_KEEPALL)
                }
                (true, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_JA_LOOSE)
                }
                (false, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_LOOSE)
                }
                (true, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_JA_NORMAL)
                }
                (false, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_NORMAL)
                }
                (true, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => {
                    Some(Baked::SEGMENTER_BREAK_LINE_OVERRIDE_V2_UND_JA)
                }
                (false, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => None,
                (_, LineBreakStrictness::Anywhere, _) => {
                    // Return a line segmenter that is actually a grapheme cluster segmenter.
                    return LineSegmenterBorrowed(LineSegmenterBorrowedInner::V2 {
                        data: Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V2,
                        tailoring: None,
                        complex: ComplexPayloadsBorrowed::new_neo(),
                    });
                }
            },
            complex: ComplexPayloadsBorrowed::new_neo(),
        })
    }

    #[cfg(feature = "unstable")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new_neo_for_non_complex_scripts)]
    pub fn try_new_neo_for_non_complex_scripts_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: DataProvider<SegmenterBreakLineV2>
            + DataProvider<SegmenterBreakGraphemeClusterV2>
            + DataProvider<SegmenterBreakLineOverrideV2>
            + ?Sized,
    {
        let options = options.resolve();

        let data = provider.load(Default::default())?;

        let id = match (options.ja_zh, options.strictness, options.word_option) {
            (_, _, LineBreakWordOption::BreakAll) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("breakall") })
            }
            (_, _, LineBreakWordOption::KeepAll) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("keepall") })
            }
            (true, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("ja-loose") })
            }
            (false, LineBreakStrictness::Loose, LineBreakWordOption::Normal) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("loose") })
            }
            (true, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("ja-normal") })
            }
            (false, LineBreakStrictness::Normal, LineBreakWordOption::Normal) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("normal") })
            }
            (true, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => {
                Some(const { DataMarkerAttributes::from_str_or_panic("ja") })
            }
            (false, LineBreakStrictness::Strict, LineBreakWordOption::Normal) => None,
            (_, LineBreakStrictness::Anywhere, _) => {
                // Return a line segmenter that is actually a grapheme cluster segmenter.
                return Ok(Self(LineSegmenterInner::V2 {
                    data: DataProvider::<SegmenterBreakGraphemeClusterV2>::load(
                        provider,
                        Default::default(),
                    )?
                    .payload
                    .cast(),
                    tailoring: None,
                    complex: ComplexPayloads::try_new_neo(provider)?,
                }));
            }
        };

        let tailoring = id
            .map(|id| {
                provider.load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes(id),
                    metadata: Default::default(),
                })
            })
            .transpose()?;

        if let Some(ref tailoring) = tailoring
            && let Some(id) = id
            && data.metadata.checksum != tailoring.metadata.checksum
        {
            return Err(
                DataErrorKind::InconsistentData(SegmenterBreakLineV2::INFO).with_req(
                    SegmenterBreakLineOverrideV2::INFO,
                    DataRequest {
                        id: DataIdentifierBorrowed::for_marker_attributes(id),
                        metadata: Default::default(),
                    },
                ),
            );
        }

        Ok(Self(LineSegmenterInner::V2 {
            data: data.payload,
            complex: ComplexPayloads::try_new_neo(provider)?,
            tailoring: tailoring.map(|d| d.payload),
        }))
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
        match self.0 {
            LineSegmenterInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V2 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V3 {
                ref mut complex, ..
            } => complex,
        }
        .with_southeast_asian_lstms(provider)
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
        match self.0 {
            LineSegmenterInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V2 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V3 {
                ref mut complex, ..
            } => complex,
        }
        .with_southeast_asian_dictionaries(provider)
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
        LineSegmenterBorrowed(match &self.0 {
            LineSegmenterInner::V1 {
                data,
                options,
                complex,
            } => LineSegmenterBorrowedInner::V1 {
                options: *options,
                data: data.get(),
                complex: complex.as_borrowed(),
            },
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V2 {
                data,
                tailoring,
                complex,
            } => LineSegmenterBorrowedInner::V2 {
                data: data.get(),
                tailoring: tailoring.as_ref().map(|t| t.get()),
                complex: complex.as_borrowed(),
            },
            #[cfg(feature = "unstable")]
            LineSegmenterInner::V3 {
                data,
                options,
                complex,
            } => LineSegmenterBorrowedInner::V3 {
                options: *options,
                data: data.get(),
                complex: complex.as_borrowed(),
            },
        })
    }
}

impl<'data> LineSegmenterBorrowed<'data> {
    /// Creates a line break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'s>(self, input: &'s str) -> LineBreakIterator<'data, 's, Utf8> {
        LineBreakIterator(match self.0 {
            LineSegmenterBorrowedInner::V1 {
                options,
                data,
                complex,
            } => LineBreakIteratorInner::V1(v1::LineBreakIteratorV1::new(
                input.char_indices(),
                input.len(),
                data,
                options,
                complex,
                v1::line_handle_complex,
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                data,
                tailoring,
                complex,
            } => LineBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                input.char_indices(),
                data,
                tailoring,
                Some(complex),
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                options,
                data,
                complex,
            } => LineBreakIteratorInner::V3(v3::LineBreakIteratorV3::new(
                input.char_indices(),
                input.len(),
                data,
                options,
                complex,
                v3::line_handle_complex,
            )),
        })
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
        LineBreakIterator(match self.0 {
            LineSegmenterBorrowedInner::V1 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V1(v1::LineBreakIteratorV1::new(
                Utf8CharIndices::new(input),
                input.len(),
                data,
                options,
                complex,
                v1::line_handle_complex,
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                data,
                tailoring,
                complex,
            } => LineBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                Utf8CharIndices::new(input),
                data,
                tailoring,
                Some(complex),
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V3(v3::LineBreakIteratorV3::new(
                Utf8CharIndices::new(input),
                input.len(),
                data,
                options,
                complex,
                v3::line_handle_complex,
            )),
        })
    }

    /// Creates a line break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'s>(self, input: &'s [u8]) -> LineBreakIterator<'data, 's, Latin1> {
        LineBreakIterator(match self.0 {
            LineSegmenterBorrowedInner::V1 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V1(v1::LineBreakIteratorV1::new(
                Latin1Indices::new(input),
                input.len(),
                data,
                options,
                complex,
                |_, _| None,
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                data, tailoring, ..
            } => LineBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                Latin1Indices::new(input),
                data,
                tailoring,
                None,
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V3(v3::LineBreakIteratorV3::new(
                Latin1Indices::new(input),
                input.len(),
                data,
                options,
                complex,
                |_, _| None,
            )),
        })
    }

    /// Creates a line break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'s>(self, input: &'s [u16]) -> LineBreakIterator<'data, 's, Utf16> {
        LineBreakIterator(match self.0 {
            LineSegmenterBorrowedInner::V1 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V1(v1::LineBreakIteratorV1::new(
                Utf16Indices::new(input),
                input.len(),
                data,
                options,
                complex,
                v1::line_handle_complex,
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                data,
                tailoring,
                complex,
            } => LineBreakIteratorInner::V2(crate::rule_segmenter_v2::RuleBreakIterator::new(
                Utf16Indices::new(input),
                data,
                tailoring,
                Some(complex),
            )),
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                data,
                options,
                complex,
            } => LineBreakIteratorInner::V3(v3::LineBreakIteratorV3::new(
                Utf16Indices::new(input),
                input.len(),
                data,
                options,
                complex,
                v3::line_handle_complex,
            )),
        })
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
        // [2]: https://www.unicode.org/reports/tr14/#SA,
        match self.0 {
            LineSegmenterBorrowedInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                ref mut complex, ..
            } => complex,
        }
        .with_southeast_asian_lstms()
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
        match self.0 {
            LineSegmenterBorrowedInner::V1 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                ref mut complex, ..
            } => complex,
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                ref mut complex, ..
            } => complex,
        }
        .with_southeast_asian_dictionaries()
    }

    /// Cheaply converts a [`LineSegmenterBorrowed<'static>`] into a [`LineSegmenter`].
    ///
    /// Note: Due to branching and indirection, using [`LineSegmenter`] might inhibit some
    /// compile-time optimizations that are possible with [`LineSegmenterBorrowed`].
    pub fn static_to_owned(self) -> LineSegmenter {
        LineSegmenter(match self.0 {
            LineSegmenterBorrowedInner::V1 {
                data,
                options,
                complex,
            } => LineSegmenterInner::V1 {
                data: DataPayload::from_static_ref(data),
                complex: complex.static_to_owned(),
                options,
            },
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V2 {
                data,
                tailoring,
                complex,
            } => LineSegmenterInner::V2 {
                data: DataPayload::from_static_ref(data),
                complex: complex.static_to_owned(),
                tailoring: tailoring.map(DataPayload::from_static_ref),
            },
            #[cfg(feature = "unstable")]
            LineSegmenterBorrowedInner::V3 {
                data,
                options,
                complex,
            } => LineSegmenterInner::V3 {
                data: DataPayload::from_static_ref(data),
                complex: complex.static_to_owned(),
                options,
            },
        })
    }
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
pub struct LineBreakIterator<'data, 's, Y: RuleBreakType>(LineBreakIteratorInner<'data, 's, Y>);

#[derive(Debug)]
enum LineBreakIteratorInner<'data, 's, Y: RuleBreakType> {
    V1(v1::LineBreakIteratorV1<'data, 's, Y>),
    #[cfg(feature = "unstable")]
    V2(crate::rule_segmenter_v2::RuleBreakIterator<'data, 's, Y, ComplexLine<Y>>),
    #[cfg(feature = "unstable")]
    V3(v3::LineBreakIteratorV3<'data, 's, Y>),
}

impl<Y: RuleBreakType> Iterator for LineBreakIterator<'_, '_, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            LineBreakIteratorInner::V1(ref mut iter) => iter.next(),
            #[cfg(feature = "unstable")]
            LineBreakIteratorInner::V2(ref mut iter) => iter.next(),
            #[cfg(feature = "unstable")]
            LineBreakIteratorInner::V3(ref mut iter) => iter.next(),
        }
    }
}

#[cfg(feature = "unstable")]
impl<Y: RuleBreakType> LineBreakIterator<'_, '_, Y> {
    /// Returns whether the last break was mandatory
    pub fn is_mandatory(&self) -> bool {
        match self.0 {
            LineBreakIteratorInner::V1(_) => false,
            #[cfg(feature = "unstable")]
            LineBreakIteratorInner::V2(ref iter) => iter.last_accepting_status() == (true as u8),
            #[cfg(feature = "unstable")]
            LineBreakIteratorInner::V3(_) => false,
        }
    }
}

#[derive(Debug)]
#[cfg(feature = "unstable")]
struct ComplexLine<Y>(core::marker::PhantomData<Y>);

#[cfg(feature = "unstable")]
impl<Y: RuleBreakType> crate::rule_segmenter_v2::ComplexHandler<Y> for ComplexLine<Y> {
    const BREAK_AT_BOUNDARIES: bool = false;
    const BREAK_STATUS: u8 = false as u8;
    type Cache = [usize; 16];

    type ComplexPayloads<'s> = Y::ComplexPayloads<'s>;
    type ComplexPayload<'s> = Y::ComplexPayload<'s>;

    fn select<'data>(
        complex_payloads: &Self::ComplexPayloads<'data>,
        complex_script: ComplexScript,
    ) -> Option<Self::ComplexPayload<'data>> {
        Y::select_complex(complex_payloads, complex_script)
    }

    fn handle<'data, 's>(
        complex_payload: &Self::ComplexPayload<'data>,
        iter: &Y::IterAttr<'s>,
        past_complex: &Y::IterAttr<'s>,
    ) -> ComplexIterator<'data, 's, Y> {
        Y::handle_complex(complex_payload, iter, past_complex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LineSegmenterBorrowed;
    use crate::*;

    include!("../tests/helpers.rs.raw");

    #[test]
    fn test_mandatory() {
        let mut actual_breaks = LineSegmenter::new_neo_for_non_complex_scripts(Default::default())
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
    fn linebreak_17() {
        let segmenter = LineSegmenter::new_17_for_non_complex_scripts(Default::default());

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
    fn linebreak_neo() {
        let segmenter = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());

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
    fn thai_line_break_17() {
        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );

        check_line("ภาษา", &["ภาษา"], {
            let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        });

        check_line("ภาษา", &["ภาษา"], {
            let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
            s.load_dictionary();
            s
        });
    }

    #[test]
    fn complex_line_break_encodings() {
        let segmenter = LineSegmenter::new_dictionary(Default::default());
        let input = "ภาษาไทย龟山岛";
        check_line(input, &["ภาษา", "ไทย", "龟", "山", "岛"], segmenter);

        let ill_formed =
            b"\xE0\xB8\xA0\xE0\xB8\xB2\xE0\xB8\xA9\xE0\xB8\xB2\xFF\xE0\xB9\x84\xE0\xB8\x97\xE0\xB8\xA2";
        let breaks: Vec<usize> = segmenter.segment_utf8(ill_formed).collect();
        assert_eq!(breaks, [0, 12, 22]);

        let unpaired_surrogate = [
            0x0E20, 0x0E32, 0x0E29, 0x0E32, 0xD800, 0x0E44, 0x0E17, 0x0E22,
        ];
        let breaks: Vec<usize> = segmenter.segment_utf16(&unpaired_surrogate).collect();
        assert_eq!(breaks, [0, 4, 8]);
    }

    #[test]
    fn complex_line_break_encodings_neo() {
        let mut segmenter = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
        segmenter.load_dictionary();
        let input = "ภาษาไทย龟山岛";
        check_line(input, &["ภาษา", "ไทย", "龟", "山", "岛"], segmenter);

        let ill_formed =
            b"\xE0\xB8\xA0\xE0\xB8\xB2\xE0\xB8\xA9\xE0\xB8\xB2\xFF\xE0\xB9\x84\xE0\xB8\x97\xE0\xB8\xA2";
        let breaks: Vec<usize> = segmenter.segment_utf8(ill_formed).collect();
        assert_eq!(breaks, [0, 22]);

        let unpaired_surrogate = [
            0x0E20, 0x0E32, 0x0E29, 0x0E32, 0xD800, 0x0E44, 0x0E17, 0x0E22,
        ];
        let breaks: Vec<usize> = segmenter.segment_utf16(&unpaired_surrogate).collect();
        assert_eq!(breaks, [0, 8]);
    }

    #[test]
    fn complex_line_break_encodings_17() {
        let mut segmenter = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
        segmenter.load_dictionary();
        let input = "ภาษาไทย龟山岛";
        check_line(input, &["ภาษา", "ไทย", "龟", "山", "岛"], segmenter);

        let ill_formed =
            b"\xE0\xB8\xA0\xE0\xB8\xB2\xE0\xB8\xA9\xE0\xB8\xB2\xFF\xE0\xB9\x84\xE0\xB8\x97\xE0\xB8\xA2";
        let breaks: Vec<usize> = segmenter.segment_utf8(ill_formed).collect();
        assert_eq!(breaks, [0, 12, 22]);

        let unpaired_surrogate = [
            0x0E20, 0x0E32, 0x0E29, 0x0E32, 0xD800, 0x0E44, 0x0E17, 0x0E22,
        ];
        let breaks: Vec<usize> = segmenter.segment_utf16(&unpaired_surrogate).collect();
        assert_eq!(breaks, [0, 4, 8]);
    }

    #[test]
    fn thai_line_break_neo() {
        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "ภาษาไทยภาษาไทย",
            &["ภาษา", "ไทย", "ภาษา", "ไทย"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );

        check_line("ภาษา", &["ภาษา"], {
            let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
            s.load_lstm();
            s
        });

        check_line("ภาษา", &["ภาษา"], {
            let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
            s.load_dictionary();
            s
        });

        // # 8243
        check_line(
            "ก\u{2060}รุ\u{2060}ง",
            &["ก\u{2060}รุ\u{2060}ง"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
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
    fn burmese_line_break_17() {
        // "Burmese Language" in Burmese

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်", "မာ", "ဘာသာ", "စကား"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်မာဘာသာ", "စကား"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );
    }

    #[test]
    fn burmese_line_break_neo() {
        // "Burmese Language" in Burmese

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်", "မာ", "ဘာသာ", "စကား"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "မြန်မာဘာသာစကား",
            &["မြန်မာဘာသာ", "စကား"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
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
    fn khmer_line_break_17() {
        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );
    }

    #[test]
    fn khmer_line_break_neo() {
        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស",
            &["សេចក្ដីប្រកាស", "ជាស", "កល", "ស្ដីពី", "សិទ្ធិមនុស្ស"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );

        // #7218
        check_line(
            "អស់ នឹង មាន",
            &["អស់ ", "នឹង ", "មាន"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
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
    fn lao_line_break_17() {
        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວ", "ກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            {
                let mut s = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );
    }

    #[test]
    fn lao_line_break_neo() {
        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວ", "ກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_lstm();
                s
            },
        );

        check_line(
            "ກ່ຽວກັບສິດຂອງມະນຸດ",
            &["ກ່ຽວກັບ", "ສິດ", "ຂອງ", "ມະນຸດ"],
            {
                let mut s = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
                s.load_dictionary();
                s
            },
        );
    }

    #[test]
    fn empty_string() {
        let segmenter = LineSegmenter::new_for_non_complex_scripts(Default::default());
        let breaks: Vec<usize> = segmenter.segment_str("").collect();
        assert_eq!(breaks, [0]);
    }

    #[test]
    fn empty_string_17() {
        let segmenter = LineSegmenter::new_17_for_non_complex_scripts(Default::default());
        let breaks: Vec<usize> = segmenter.segment_str("").collect();
        assert_eq!(breaks, [0]);
    }

    #[test]
    fn empty_string_neo() {
        let segmenter = LineSegmenter::new_neo_for_non_complex_scripts(Default::default());
        let breaks: Vec<usize> = segmenter.segment_str("").collect();
        assert_eq!(breaks, [0]);
    }
}
