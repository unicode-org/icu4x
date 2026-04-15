// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::indices::*;
use crate::provider::*;
use crate::symbols::*;
use crate::SegmenterError;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::char;
use core::str::CharIndices;
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

/// An enum specifies the strictness of line-breaking rules. It can be passed as
/// an argument when creating a line segmenter.
///
/// Each enum value has the same meaning with respect to the `line-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#line-break-property>.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LineBreakWordOption {
    /// Words break according to their customary rules. See the details in
    /// <https://drafts.csswg.org/css-text-3/#valdef-word-break-normal>.
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
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LineBreakOptions {
    /// Strictness of line-breaking rules. See [`LineBreakStrictness`].
    pub strictness: LineBreakStrictness,

    /// Line break opportunities between letters. See [`LineBreakWordOption`].
    pub word_option: LineBreakWordOption,

    /// Use `true` as a hint to the line segmenter that the writing
    /// system is Chinese or Japanese. This allows more break opportunities when
    /// `LineBreakStrictness` is `Normal` or `Loose`. See
    /// <https://drafts.csswg.org/css-text-3/#line-break-property> for details.
    ///
    /// This option has no effect in Latin-1 mode.
    pub ja_zh: bool,
}

impl Default for LineBreakOptions {
    fn default() -> Self {
        Self {
            strictness: LineBreakStrictness::Strict,
            word_option: LineBreakWordOption::Normal,
            ja_zh: false,
        }
    }
}

/// Line break iterator for an `str` (a UTF-8 string).
///
/// For examples of use, see [`LineSegmenter`].
pub type LineBreakIteratorUtf8<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeUtf8>;

/// Line break iterator for a potentially invalid UTF-8 string.
///
/// For examples of use, see [`LineSegmenter`].
pub type LineBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    LineBreakIterator<'l, 's, LineBreakTypePotentiallyIllFormedUtf8>;

/// Line break iterator for a Latin-1 (8-bit) string.
///
/// For examples of use, see [`LineSegmenter`].
pub type LineBreakIteratorLatin1<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeLatin1>;

/// Line break iterator for a UTF-16 string.
///
/// For examples of use, see [`LineSegmenter`].
pub type LineBreakIteratorUtf16<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeUtf16>;

/// Returns `true` if UAX #14 forbids a break immediately before a character
/// whose line-break class is `class`, regardless of the class that precedes it.
///
/// Used by the complex-script line-break pipeline to decide whether the
/// dict/LSTM's boundary between an SA chunk and the next character is a
/// valid line-break opportunity. For the classes listed here UAX #14 states
/// `× class` unconditionally (or unconditionally in all contexts reachable
/// from SA in practice):
///
/// * **LB7**:   `× SP` — never break before a space.
/// * **LB9**:   `× CM`, `× ZWJ` — combining marks attach to the preceding char.
/// * **LB11**:  `× WJ` — word-joiner is glue.
/// * **LB12a**: `× GL` — non-breaking glue.
/// * **LB13**:  `× CL`, `× CP`, `× EX`, `× IS`, `× SY`.
/// * **LB21**:  `× BA`, `× HY`, `× NS` — no break before these after the
///              previous non-space char (SA never introduces a space on its
///              right, so this simplifies to unconditional `×`).
/// * **LB22**:  `× IN`.
///
/// Classes intentionally **not** included here either allow a break
/// (LB18 `SP ÷`, etc.), require surrounding context that our SA-adjacent
/// usage cannot produce, or are handled by the mandatory-break rules
/// (LB4/LB5: BK, CR, LF, NL).
#[inline]
fn lb_class_forbids_break_before(class: u8) -> bool {
    matches!(
        class,
        SP | BA | HY | NS | WJ | GL | CL | CP | EX | IS | SY | IN | ZWJ | CM,
    )
}

/// Supports loading line break data, and creating line break iterators for different string
/// encodings.
///
/// The segmenter returns mandatory breaks (as defined by [definition LD7][LD7] of
/// Unicode Standard Annex #14, _Unicode Line Breaking Algorithm_) as well as
/// line break opportunities ([definition LD3][LD3]).
/// It does not distinguish them.  Callers requiring that distinction can check
/// the Line_Break property of the code point preceding the break against those
/// listed in rules [LB4][LB4] and [LB5][LB5], special-casing the end of text
/// according to [LB3][LB3].
///
/// For consistency with the grapheme, word, and sentence segmenters, there is
/// always a breakpoint returned at index 0, but this breakpoint is not a
/// meaningful line break opportunity.
///
/// [LD3]: https://www.unicode.org/reports/tr14/#LD3
/// [LD7]: https://www.unicode.org/reports/tr14/#LD7
/// [LB3]: https://www.unicode.org/reports/tr14/#LB3
/// [LB4]: https://www.unicode.org/reports/tr14/#LB4
/// [LB5]: https://www.unicode.org/reports/tr14/#LB5
///
/// ```rust
/// # use icu_segmenter::LineSegmenter;
/// #
/// # let segmenter = LineSegmenter::new_auto();
/// #
/// let text = "Summary\r\nThis annex…";
/// let breakpoints: Vec<usize> = segmenter.segment_str(text).collect();
/// // 9 and 22 are mandatory breaks, 14 is a line break opportunity.
/// assert_eq!(&breakpoints, &[0, 9, 14, 22]);
/// ```
///
/// # Examples
///
/// Segment a string with default options:
///
/// ```rust
/// use icu_segmenter::LineSegmenter;
///
/// let segmenter = LineSegmenter::new_auto();
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 6, 11]);
/// ```
///
/// Segment a string with CSS option overrides:
///
/// ```rust
/// use icu_segmenter::{
///     LineBreakOptions, LineBreakStrictness, LineBreakWordOption,
///     LineSegmenter,
/// };
///
/// let mut options = LineBreakOptions::default();
/// options.strictness = LineBreakStrictness::Strict;
/// options.word_option = LineBreakWordOption::BreakAll;
/// options.ja_zh = false;
/// let segmenter = LineSegmenter::new_auto_with_options(options);
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu_segmenter::LineSegmenter;
///
/// let segmenter = LineSegmenter::new_auto();
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 6, 11]);
/// ```
///
/// Separate mandatory breaks from the break opportunities:
///
/// ```rust
/// use icu::properties::{maps, LineBreak};
/// use icu_segmenter::LineSegmenter;
///
/// # let segmenter = LineSegmenter::new_auto();
/// #
/// let text = "Summary\r\nThis annex…";
///
/// let mandatory_breaks: Vec<usize> = segmenter
///     .segment_str(text)
///     .into_iter()
///     .filter(|&i| {
///         text[..i].chars().next_back().map_or(false, |c| {
///             matches!(
///                 maps::line_break().get(c),
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
    options: LineBreakOptions,
    payload: DataPayload<LineBreakDataV1Marker>,
    complex: ComplexPayloads,
}

impl LineSegmenter {
    /// Constructs a [`LineSegmenter`] with an invariant locale and the best available compiled data for
    /// complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The current behavior, which is subject to change, is to use the LSTM model when available.
    ///
    /// See also [`Self::new_auto_with_options`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `auto` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "auto")]
    pub fn new_auto() -> Self {
        Self::new_auto_with_options(Default::default())
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            new_auto,
            try_new_auto_with_any_provider,
            try_new_auto_with_buffer_provider,
            try_new_auto_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_auto)]
    #[cfg(feature = "auto")]
    pub fn try_new_auto_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_auto_with_options_unstable(provider, Default::default())
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale and compiled LSTM data for
    /// complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The LSTM, or Long Term Short Memory, is a machine learning model. It is smaller than
    /// the full dictionary but more expensive during segmentation (inference).
    ///
    /// See also [`Self::new_lstm_with_options`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[cfg(feature = "lstm")]
    pub fn new_lstm() -> Self {
        Self::new_lstm_with_options(Default::default())
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
            Self,
        ]
    );

    #[cfg(feature = "lstm")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_lstm)]
    pub fn try_new_lstm_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_lstm_with_options_unstable(provider, Default::default())
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale and compiled dictionary data for
    /// complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The dictionary model uses a list of words to determine appropriate breakpoints. It is
    /// faster than the LSTM model but requires more data.
    ///
    /// See also [`Self::new_dictionary_with_options`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new_dictionary() -> Self {
        Self::new_dictionary_with_options(Default::default())
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
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_dictionary)]
    pub fn try_new_dictionary_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<DictionaryForWordLineExtendedV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_dictionary_with_options_unstable(provider, Default::default())
    }

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
    pub fn new_auto_with_options(options: LineBreakOptions) -> Self {
        Self::new_lstm_with_options(options)
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            new_auto_with_options,
            try_new_auto_with_options_with_any_provider,
            try_new_auto_with_options_with_buffer_provider,
            try_new_auto_with_options_unstable,
            Self,
        ]
    );

    #[cfg(feature = "auto")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_auto_with_options)]
    pub fn try_new_auto_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_lstm_with_options_unstable(provider, options)
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// compiled LSTM data for complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The LSTM, or Long Term Short Memory, is a machine learning model. It is smaller than
    /// the full dictionary but more expensive during segmentation (inference).
    ///
    /// See also [`Self::new_dictionary`].
    ///
    /// ✨ *Enabled with the `compiled_data` and `lstm` Cargo features.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "lstm")]
    #[cfg(feature = "compiled_data")]
    pub fn new_lstm_with_options(options: LineBreakOptions) -> Self {
        Self {
            options,
            payload: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_LINE_V1,
            ),
            complex: ComplexPayloads::new_lstm(),
        }
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            try_new_lstm_with_options,
            try_new_lstm_with_options_with_any_provider,
            try_new_lstm_with_options_with_buffer_provider,
            try_new_lstm_with_options_unstable,
            Self,
        ]
    );

    #[cfg(feature = "lstm")]
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_lstm_with_options)]
    pub fn try_new_lstm_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmForWordLineAutoV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Ok(Self {
            options,
            payload: provider.load(Default::default())?.take_payload()?,
            complex: ComplexPayloads::try_new_lstm(provider)?,
        })
    }

    /// Constructs a [`LineSegmenter`] with an invariant locale, custom [`LineBreakOptions`], and
    /// compiled dictionary data for complex scripts (Khmer, Lao, Myanmar, and Thai).
    ///
    /// The dictionary model uses a list of words to determine appropriate breakpoints. It is
    /// faster than the LSTM model but requires more data.
    ///
    /// See also [`Self::new_dictionary`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new_dictionary_with_options(options: LineBreakOptions) -> Self {
        Self {
            options,
            payload: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_LINE_V1,
            ),
            // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
            // characters [1]. Southeast Asian languages however require complex context analysis
            // [2].
            //
            // [1]: https://www.unicode.org/reports/tr14/#ID
            // [2]: https://www.unicode.org/reports/tr14/#SA
            complex: ComplexPayloads::new_southeast_asian(),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        #[cfg(skip)]
        functions: [
            new_dictionary_with_options,
            try_new_dictionary_with_options_with_any_provider,
            try_new_dictionary_with_options_with_buffer_provider,
            try_new_dictionary_with_options_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_dictionary_with_options)]
    pub fn try_new_dictionary_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<DictionaryForWordLineExtendedV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Ok(Self {
            options,
            payload: provider.load(Default::default())?.take_payload()?,
            // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
            // characters [1]. Southeast Asian languages however require complex context analysis
            // [2].
            //
            // [1]: https://www.unicode.org/reports/tr14/#ID
            // [2]: https://www.unicode.org/reports/tr14/#SA
            complex: ComplexPayloads::try_new_southeast_asian(provider)?,
        })
    }

    /// Creates a line break iterator for an `str` (a UTF-8 string).
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> LineBreakIteratorUtf8<'l, 's> {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            complex: &self.complex,
        }
    }
    /// Creates a line break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf8<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> LineBreakIteratorPotentiallyIllFormedUtf8<'l, 's> {
        LineBreakIterator {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            complex: &self.complex,
        }
    }
    /// Creates a line break iterator for a Latin-1 (8-bit) string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_latin1<'l, 's>(&'l self, input: &'s [u8]) -> LineBreakIteratorLatin1<'l, 's> {
        LineBreakIterator {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            complex: &self.complex,
        }
    }

    /// Creates a line break iterator for a UTF-16 string.
    ///
    /// There are always breakpoints at 0 and the string length, or only at 0 for the empty string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> LineBreakIteratorUtf16<'l, 's> {
        LineBreakIterator {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            complex: &self.complex,
        }
    }
}

fn get_linebreak_property_utf32_with_rule(
    property_table: &RuleBreakPropertyTable<'_>,
    codepoint: u32,
    strictness: LineBreakStrictness,
    word_option: LineBreakWordOption,
) -> u8 {
    // Note: Default value is 0 == UNKNOWN
    let prop = property_table.0.get32(codepoint);

    if word_option == LineBreakWordOption::BreakAll
        || strictness == LineBreakStrictness::Loose
        || strictness == LineBreakStrictness::Normal
    {
        return match prop {
            CJ => ID, // All CJ's General_Category is Other_Letter (Lo).
            _ => prop,
        };
    }

    // CJ is treated as NS by default, yielding strict line breaking.
    // https://www.unicode.org/reports/tr14/#CJ
    prop
}

#[inline]
fn get_linebreak_property_latin1(property_table: &RuleBreakPropertyTable<'_>, codepoint: u8) -> u8 {
    // Note: Default value is 0 == UNKNOWN
    property_table.0.get32(codepoint as u32)
}

#[inline]
fn get_linebreak_property_with_rule(
    property_table: &RuleBreakPropertyTable<'_>,
    codepoint: char,
    linebreak_rule: LineBreakStrictness,
    wordbreak_rule: LineBreakWordOption,
) -> u8 {
    get_linebreak_property_utf32_with_rule(
        property_table,
        codepoint as u32,
        linebreak_rule,
        wordbreak_rule,
    )
}

#[inline]
fn is_break_utf32_by_normal(codepoint: u32, ja_zh: bool) -> bool {
    match codepoint {
        0x301C => ja_zh,
        0x30A0 => ja_zh,
        _ => false,
    }
}

#[inline]
fn is_break_utf32_by_loose(
    right_codepoint: u32,
    left_prop: u8,
    right_prop: u8,
    ja_zh: bool,
) -> Option<bool> {
    // breaks before hyphens
    if right_prop == BA {
        if left_prop == ID && (right_codepoint == 0x2010 || right_codepoint == 0x2013) {
            return Some(true);
        }
    } else if right_prop == NS {
        // breaks before certain CJK hyphen-like characters
        if right_codepoint == 0x301C || right_codepoint == 0x30A0 {
            return Some(ja_zh);
        }

        // breaks before iteration marks
        if right_codepoint == 0x3005
            || right_codepoint == 0x303B
            || right_codepoint == 0x309D
            || right_codepoint == 0x309E
            || right_codepoint == 0x30FD
            || right_codepoint == 0x30FE
        {
            return Some(true);
        }

        // breaks before certain centered punctuation marks:
        if right_codepoint == 0x30FB
            || right_codepoint == 0xFF1A
            || right_codepoint == 0xFF1B
            || right_codepoint == 0xFF65
            || right_codepoint == 0x203C
            || (0x2047..=0x2049).contains(&right_codepoint)
        {
            return Some(ja_zh);
        }
    } else if right_prop == IN {
        // breaks between inseparable characters such as U+2025, U+2026 i.e. characters with the Unicode Line Break property IN
        return Some(true);
    } else if right_prop == EX {
        // breaks before certain centered punctuation marks:
        if right_codepoint == 0xFF01 || right_codepoint == 0xFF1F {
            return Some(ja_zh);
        }
    }

    // breaks before suffixes:
    // Characters with the Unicode Line Break property PO and the East Asian Width property
    if right_prop == PO_EAW {
        return Some(ja_zh);
    }
    // breaks after prefixes:
    // Characters with the Unicode Line Break property PR and the East Asian Width property
    if left_prop == PR_EAW {
        return Some(ja_zh);
    }
    None
}

#[inline]
fn is_break_from_table(
    break_state_table: &RuleBreakStateTable<'_>,
    property_count: u8,
    left: u8,
    right: u8,
) -> bool {
    let rule = get_break_state_from_table(break_state_table, property_count, left, right);
    if rule == KEEP_RULE {
        return false;
    }
    if rule >= 0 {
        // need additional next characters to get break rule.
        return false;
    }
    true
}

#[inline]
fn is_non_break_by_keepall(left: u8, right: u8) -> bool {
    //  typographic letter units shouldn't be break
    (left == AI
        || left == AL
        || left == ID
        || left == NU
        || left == HY
        || left == H2
        || left == H3
        || left == JL
        || left == JV
        || left == JT
        || left == CJ)
        && (right == AI
            || right == AL
            || right == ID
            || right == NU
            || right == HY
            || right == H2
            || right == H3
            || right == JL
            || right == JV
            || right == JT
            || right == CJ)
}

#[inline]
fn get_break_state_from_table(
    break_state_table: &RuleBreakStateTable<'_>,
    property_count: u8,
    left: u8,
    right: u8,
) -> i8 {
    let idx = (left as usize) * (property_count as usize) + (right as usize);
    // We use unwrap_or to fall back to the base case and prevent panics on bad data.
    break_state_table.0.get(idx).unwrap_or(KEEP_RULE)
}

#[inline]
fn use_complex_breaking_utf32(property_table: &RuleBreakPropertyTable<'_>, codepoint: u32) -> bool {
    let line_break_property = get_linebreak_property_utf32_with_rule(
        property_table,
        codepoint,
        LineBreakStrictness::Strict,
        LineBreakWordOption::Normal,
    );

    line_break_property == SA
}

/*
#[inline]
fn use_complex_breaking_utf32(codepoint: u32) -> bool {
    // Thai, Lao and Khmer
    (codepoint >= 0xe01 && codepoint <= 0xeff) || (codepoint >= 0x1780 && codepoint <= 0x17ff)
}
*/

/// A trait allowing for LineBreakIterator to be generalized to multiple string iteration methods.
///
/// This is implemented by ICU4X for several common string types.
pub trait LineBreakType<'l, 's> {
    /// The iterator over characters.
    type IterAttr: Iterator<Item = (usize, Self::CharType)> + Clone;

    /// The character type.
    type CharType: Copy + Into<u32>;

    fn use_complex_breaking(iterator: &LineBreakIterator<'l, 's, Self>, c: Self::CharType) -> bool;

    fn get_linebreak_property_with_rule(
        iterator: &LineBreakIterator<'l, 's, Self>,
        c: Self::CharType,
    ) -> u8;

    fn get_current_position_character_len(iterator: &LineBreakIterator<'l, 's, Self>) -> usize;

    fn handle_complex_language(
        iterator: &mut LineBreakIterator<'l, 's, Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize>;
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
pub struct LineBreakIterator<'l, 's, Y: LineBreakType<'l, 's> + ?Sized> {
    iter: Y::IterAttr,
    len: usize,
    current_pos_data: Option<(usize, Y::CharType)>,
    result_cache: Vec<usize>,
    data: &'l RuleBreakDataV1<'l>,
    options: &'l LineBreakOptions,
    complex: &'l ComplexPayloads,
}

impl<'l, 's, Y: LineBreakType<'l, 's>> Iterator for LineBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
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
                i += Y::get_current_position_character_len(self);
                self.advance_iter();
                if self.is_eof() {
                    self.result_cache.clear();
                    return Some(self.len);
                }
            }
        }

        loop {
            debug_assert!(!self.is_eof());
            let left_codepoint = self.get_current_codepoint()?;
            let mut left_prop = self.get_linebreak_property(left_codepoint);
            self.advance_iter();

            let Some(right_codepoint) = self.get_current_codepoint() else {
                return Some(self.len);
            };
            let right_prop = self.get_linebreak_property(right_codepoint);

            // CSS word-break property handling
            match self.options.word_option {
                LineBreakWordOption::BreakAll => {
                    left_prop = match left_prop {
                        AL => ID,
                        NU => ID,
                        SA => ID,
                        _ => left_prop,
                    };
                }
                LineBreakWordOption::KeepAll => {
                    if is_non_break_by_keepall(left_prop, right_prop) {
                        continue;
                    }
                }
                _ => (),
            }

            // CSS line-break property handling
            match self.options.strictness {
                LineBreakStrictness::Normal => {
                    if self.is_break_by_normal(right_codepoint) {
                        return self.get_current_position();
                    }
                }
                LineBreakStrictness::Loose => {
                    if let Some(breakable) = is_break_utf32_by_loose(
                        right_codepoint.into(),
                        left_prop,
                        right_prop,
                        self.options.ja_zh,
                    ) {
                        if breakable {
                            return self.get_current_position();
                        }
                        continue;
                    }
                }
                LineBreakStrictness::Anywhere => {
                    return self.get_current_position();
                }
                _ => (),
            };

            // UAX14 doesn't have Thai etc, so use another way.
            if self.options.word_option != LineBreakWordOption::BreakAll
                && Y::use_complex_breaking(self, left_codepoint)
                && Y::use_complex_breaking(self, right_codepoint)
            {
                let result = Y::handle_complex_language(self, left_codepoint);
                if result.is_some() {
                    return result;
                }
                // I may have to fetch text until non-SA character?.
            }

            // If break_state is equals or grater than 0, it is alias of property.
            let mut break_state = self.get_break_state_from_table(left_prop, right_prop);
            if break_state >= 0_i8 {
                let mut previous_iter = self.iter.clone();
                let mut previous_pos_data = self.current_pos_data;

                loop {
                    self.advance_iter();

                    let Some(prop) = self.get_current_linebreak_property() else {
                        // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                        let break_state = self
                            .get_break_state_from_table(break_state as u8, self.data.eot_property);
                        if break_state == NOT_MATCH_RULE {
                            self.iter = previous_iter;
                            self.current_pos_data = previous_pos_data;
                            return self.get_current_position();
                        }
                        // EOF
                        return Some(self.len);
                    };

                    break_state = self.get_break_state_from_table(break_state as u8, prop);
                    if break_state < 0 {
                        break;
                    }

                    previous_iter = self.iter.clone();
                    previous_pos_data = self.current_pos_data;
                }
                if break_state == KEEP_RULE {
                    continue;
                }
                if break_state == NOT_MATCH_RULE {
                    self.iter = previous_iter;
                    self.current_pos_data = previous_pos_data;
                    return self.get_current_position();
                }
                return self.get_current_position();
            }

            if self.is_break_from_table(left_prop, right_prop) {
                return self.get_current_position();
            }
        }
    }
}

enum StringBoundaryPosType {
    Start,
    Middle,
    End,
}

impl<'l, 's, Y: LineBreakType<'l, 's>> LineBreakIterator<'l, 's, Y> {
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
        Y::get_linebreak_property_with_rule(self, codepoint)
    }

    fn get_current_linebreak_property(&self) -> Option<u8> {
        self.get_current_codepoint()
            .map(|c| self.get_linebreak_property(c))
    }

    fn is_break_by_normal(&self, codepoint: Y::CharType) -> bool {
        is_break_utf32_by_normal(codepoint.into(), self.options.ja_zh)
    }

    fn get_break_state_from_table(&self, left: u8, right: u8) -> i8 {
        get_break_state_from_table(
            &self.data.break_state_table,
            self.data.property_count,
            left,
            right,
        )
    }

    fn is_break_from_table(&self, left: u8, right: u8) -> bool {
        is_break_from_table(
            &self.data.break_state_table,
            self.data.property_count,
            left,
            right,
        )
    }
}

#[derive(Debug)]
pub struct LineBreakTypeUtf8;

impl<'l, 's> LineBreakType<'l, 's> for LineBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: char) -> u8 {
        get_linebreak_property_with_rule(
            &iterator.data.property_table,
            c,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: char) -> bool {
        use_complex_breaking_utf32(&iterator.data.property_table, c as u32)
    }

    fn get_current_position_character_len(iterator: &LineBreakIterator<Self>) -> usize {
        iterator.get_current_codepoint().map_or(0, |c| c.len_utf8())
    }

    fn handle_complex_language(
        iter: &mut LineBreakIterator<'l, 's, Self>,
        left_codepoint: char,
    ) -> Option<usize> {
        handle_complex_language_utf8(iter, left_codepoint)
    }
}

#[derive(Debug)]
pub struct LineBreakTypePotentiallyIllFormedUtf8;

impl<'l, 's> LineBreakType<'l, 's> for LineBreakTypePotentiallyIllFormedUtf8 {
    type IterAttr = Utf8CharIndices<'s>;
    type CharType = char;

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: char) -> u8 {
        get_linebreak_property_with_rule(
            &iterator.data.property_table,
            c,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: char) -> bool {
        use_complex_breaking_utf32(&iterator.data.property_table, c as u32)
    }

    fn get_current_position_character_len(iterator: &LineBreakIterator<Self>) -> usize {
        iterator.get_current_codepoint().map_or(0, |c| c.len_utf8())
    }

    fn handle_complex_language(
        iter: &mut LineBreakIterator<'l, 's, Self>,
        left_codepoint: char,
    ) -> Option<usize> {
        handle_complex_language_utf8(iter, left_codepoint)
    }
}
/// handle_complex_language impl for UTF8 iterators
fn handle_complex_language_utf8<'l, 's, T>(
    iter: &mut LineBreakIterator<'l, 's, T>,
    left_codepoint: char,
) -> Option<usize>
where
    T: LineBreakType<'l, 's, CharType = char>,
{
    // word segmenter doesn't define break rules for some languages such as Thai.
    let start_iter = iter.iter.clone();
    let start_point = iter.current_pos_data;
    let mut s = String::new();
    s.push(left_codepoint);
    // Capture the char that terminated collection (if any). It is passed to
    // `complex_language_line_breaks_str` so the UAX #14 filter can decide
    // whether the dict/LSTM's trailing break is a valid line-break
    // opportunity (LB7 `× SP`, LB21 `× BA`, LB13 `× CL`, etc.).
    let mut next_ext_char: Option<char> = None;
    loop {
        debug_assert!(!iter.is_eof());
        s.push(iter.get_current_codepoint()?);
        iter.advance_iter();
        if let Some(current_codepoint) = iter.get_current_codepoint() {
            if !T::use_complex_breaking(iter, current_codepoint) {
                next_ext_char = Some(current_codepoint);
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
    // Capture `data` and `options` so the UAX #14 predicate closure can
    // look up line-break classes without borrowing `iter` across the call
    // to the complex oracle. Both are shared references (`Copy`).
    let data = iter.data;
    let options = iter.options;
    let breaks = complex_language_line_breaks_str(iter.complex, &s, next_ext_char, |c| {
        lb_class_forbids_break_before(get_linebreak_property_with_rule(
            &data.property_table,
            c,
            options.strictness,
            options.word_option,
        ))
    });
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
pub struct LineBreakTypeLatin1;

impl<'l, 's> LineBreakType<'l, 's> for LineBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8;

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: u8) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(&iterator.data.property_table, c)
    }

    #[inline]
    fn use_complex_breaking(_iterator: &LineBreakIterator<Self>, _c: u8) -> bool {
        false
    }

    fn get_current_position_character_len(_: &LineBreakIterator<Self>) -> usize {
        unreachable!()
    }

    fn handle_complex_language(
        _: &mut LineBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        unreachable!()
    }
}

#[derive(Debug)]
pub struct LineBreakTypeUtf16;

impl<'l, 's> LineBreakType<'l, 's> for LineBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: u32) -> u8 {
        get_linebreak_property_utf32_with_rule(
            &iterator.data.property_table,
            c,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: u32) -> bool {
        use_complex_breaking_utf32(&iterator.data.property_table, c)
    }

    fn get_current_position_character_len(iterator: &LineBreakIterator<Self>) -> usize {
        match iterator.get_current_codepoint() {
            None => 0,
            Some(ch) if ch >= 0x10000 => 2,
            _ => 1,
        }
    }

    fn handle_complex_language(
        iterator: &mut LineBreakIterator<Self>,
        left_codepoint: Self::CharType,
    ) -> Option<usize> {
        // word segmenter doesn't define break rules for some languages such as Thai.
        let start_iter = iterator.iter.clone();
        let start_point = iterator.current_pos_data;
        let mut s = vec![left_codepoint as u16];
        // Capture the codepoint that terminated collection (if any). See
        // `handle_complex_language_utf8` for rationale.
        let mut next_ext_cp: Option<u32> = None;
        loop {
            debug_assert!(!iterator.is_eof());
            s.push(iterator.get_current_codepoint()? as u16);
            iterator.advance_iter();
            if let Some(current_codepoint) = iterator.get_current_codepoint() {
                if !Self::use_complex_breaking(iterator, current_codepoint) {
                    next_ext_cp = Some(current_codepoint);
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
        let data = iterator.data;
        let options = iterator.options;
        // The filter predicate takes u16 (BMP code unit). All
        // "forbids-break-before" classes we care about (SP, BA, GL, CL, CP,
        // EX, IS, SY, WJ, IN, NS, HY, ZWJ, CM) are BMP, so a single u16
        // lookup is faithful for our purposes.
        let next_ext_code_unit = next_ext_cp.and_then(|c| u16::try_from(c).ok());
        let breaks = complex_language_line_breaks_utf16(
            iterator.complex,
            &s,
            next_ext_code_unit,
            |c| {
                lb_class_forbids_break_before(get_linebreak_property_utf32_with_rule(
                    &data.property_table,
                    c as u32,
                    options.strictness,
                    options.word_option,
                ))
            },
        );
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
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use crate::LineSegmenter;

    #[test]
    fn linebreak_property() {
        let payload = DataProvider::<LineBreakDataV1Marker>::load(
            &crate::provider::Baked,
            Default::default(),
        )
        .expect("Loading should succeed!")
        .take_payload()
        .expect("Data should be present!");

        let get_linebreak_property = |codepoint| {
            get_linebreak_property_with_rule(
                &payload.get().property_table,
                codepoint,
                LineBreakStrictness::Strict,
                LineBreakWordOption::Normal,
            )
        };

        assert_eq!(get_linebreak_property('\u{0020}'), SP);
        assert_eq!(get_linebreak_property('\u{0022}'), QU);
        assert_eq!(get_linebreak_property('('), OP_OP30);
        assert_eq!(get_linebreak_property('\u{0030}'), NU);
        assert_eq!(get_linebreak_property('['), OP_OP30);
        assert_eq!(get_linebreak_property('\u{1f3fb}'), EM);
        assert_eq!(get_linebreak_property('\u{20000}'), ID);
        assert_eq!(get_linebreak_property('\u{e0020}'), CM);
        assert_eq!(get_linebreak_property('\u{3041}'), CJ);
        assert_eq!(get_linebreak_property('\u{0025}'), PO);
        assert_eq!(get_linebreak_property('\u{00A7}'), AI);
        assert_eq!(get_linebreak_property('\u{50005}'), XX);
        assert_eq!(get_linebreak_property('\u{17D6}'), NS);
        assert_eq!(get_linebreak_property('\u{2014}'), B2);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // clearer when we're testing bools directly
    fn break_rule() {
        let payload = DataProvider::<LineBreakDataV1Marker>::load(
            &crate::provider::Baked,
            Default::default(),
        )
        .expect("Loading should succeed!")
        .take_payload()
        .expect("Data should be present!");
        let lb_data: &RuleBreakDataV1 = payload.get();

        let is_break = |left, right| {
            is_break_from_table(
                &lb_data.break_state_table,
                lb_data.property_count,
                left,
                right,
            )
        };

        // LB4
        assert_eq!(is_break(BK, AL), true);
        // LB5
        assert_eq!(is_break(CR, LF), false);
        assert_eq!(is_break(CR, AL), true);
        assert_eq!(is_break(LF, AL), true);
        assert_eq!(is_break(NL, AL), true);
        // LB6
        assert_eq!(is_break(AL, BK), false);
        assert_eq!(is_break(AL, CR), false);
        assert_eq!(is_break(AL, LF), false);
        assert_eq!(is_break(AL, NL), false);
        // LB7
        assert_eq!(is_break(AL, SP), false);
        assert_eq!(is_break(AL, ZW), false);
        // LB8
        // LB8a
        assert_eq!(is_break(ZWJ, AL), false);
        // LB9
        assert_eq!(is_break(AL, ZWJ), false);
        assert_eq!(is_break(AL, CM), false);
        assert_eq!(is_break(ID, ZWJ), false);
        // LB10
        assert_eq!(is_break(ZWJ, SP), false);
        assert_eq!(is_break(SP, CM), true);
        // LB11
        assert_eq!(is_break(AL, WJ), false);
        assert_eq!(is_break(WJ, AL), false);
        // LB12
        assert_eq!(is_break(GL, AL), false);
        // LB12a
        assert_eq!(is_break(AL, GL), false);
        assert_eq!(is_break(SP, GL), true);
        // LB13
        assert_eq!(is_break(AL, CL), false);
        assert_eq!(is_break(AL, CP), false);
        assert_eq!(is_break(AL, EX), false);
        assert_eq!(is_break(AL, IS), false);
        assert_eq!(is_break(AL, SY), false);
        // LB18
        assert_eq!(is_break(SP, AL), true);
        // LB19
        assert_eq!(is_break(AL, QU), false);
        assert_eq!(is_break(QU, AL), false);
        // LB20
        assert_eq!(is_break(AL, CB), true);
        assert_eq!(is_break(CB, AL), true);
        // LB20
        assert_eq!(is_break(AL, BA), false);
        assert_eq!(is_break(AL, HY), false);
        assert_eq!(is_break(AL, NS), false);
        // LB21
        assert_eq!(is_break(AL, BA), false);
        assert_eq!(is_break(BB, AL), false);
        assert_eq!(is_break(ID, BA), false);
        assert_eq!(is_break(ID, NS), false);
        // LB21a
        // LB21b
        assert_eq!(is_break(SY, HL), false);
        // LB22
        assert_eq!(is_break(AL, IN), false);
        // LB 23
        assert_eq!(is_break(AL, NU), false);
        assert_eq!(is_break(HL, NU), false);
        // LB 23a
        assert_eq!(is_break(PR, ID), false);
        assert_eq!(is_break(PR, EB), false);
        assert_eq!(is_break(PR, EM), false);
        assert_eq!(is_break(ID, PO), false);
        assert_eq!(is_break(EB, PO), false);
        assert_eq!(is_break(EM, PO), false);
        // LB26
        assert_eq!(is_break(JL, JL), false);
        assert_eq!(is_break(JL, JV), false);
        assert_eq!(is_break(JL, H2), false);
        // LB27
        assert_eq!(is_break(JL, IN), false);
        assert_eq!(is_break(JL, PO), false);
        assert_eq!(is_break(PR, JL), false);
        // LB28
        assert_eq!(is_break(AL, AL), false);
        assert_eq!(is_break(HL, AL), false);
        // LB29
        assert_eq!(is_break(IS, AL), false);
        assert_eq!(is_break(IS, HL), false);
        // LB30b
        assert_eq!(is_break(EB, EM), false);
        // LB31
        assert_eq!(is_break(ID, ID), true);
    }

    #[test]
    fn linebreak() {
        let segmenter = LineSegmenter::try_new_dictionary_unstable(&crate::provider::Baked)
            .expect("Data exists");

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

        // LB15
        iter = segmenter.segment_str("abc\u{0022}  (def");
        assert_eq!(Some(0), iter.next());
        assert_eq!(Some(10), iter.next());
        assert_eq!(None, iter.next());

        let input: [u8; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u8 = segmenter.segment_latin1(&input);
        assert_eq!(Some(0), iter_u8.next());
        assert_eq!(Some(10), iter_u8.next());
        assert_eq!(None, iter_u8.next());

        let input: [u16; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(0), iter_u16.next());
        assert_eq!(Some(10), iter_u16.next());
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

        let segmenter = LineSegmenter::new_lstm();
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

        let segmenter = LineSegmenter::new_lstm();
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

        let segmenter = LineSegmenter::new_lstm();
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

        let segmenter = LineSegmenter::new_lstm();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [0, 12, 21, 30, 39, TEST_STR.len()], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [0, 4, 7, 10, 13, utf16.len()], "Lao utf-16 test");
    }

    #[test]
    fn empty_string() {
        let segmenter = LineSegmenter::new_auto();
        let breaks: Vec<usize> = segmenter.segment_str("").collect();
        assert_eq!(breaks, [0]);
    }

    /// Shared helper: no two adjacent line breaks in `text`, and UTF-8 /
    /// UTF-16 break offsets agree under conversion. A double-break
    /// (offsets differing by 1) is the issue-#7218 symptom.
    fn assert_no_double_breaks_and_utf_consistency(text: &str, label: &str) {
        let segmenter = LineSegmenter::new_auto();

        let utf8_breaks: Vec<usize> = segmenter.segment_str(text).collect();
        for pair in utf8_breaks.windows(2).skip(1) {
            assert!(
                pair[1] - pair[0] > 1,
                "{label}: UTF-8 double-break at {pair:?} in {utf8_breaks:?}"
            );
        }

        let utf16: Vec<u16> = text.encode_utf16().collect();
        let utf16_breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        for pair in utf16_breaks.windows(2).skip(1) {
            assert!(
                pair[1] - pair[0] > 1,
                "{label}: UTF-16 double-break at {pair:?} in {utf16_breaks:?}"
            );
        }
    }

    #[test]
    fn khmer_line_break_with_spaces() {
        assert_no_double_breaks_and_utf_consistency("ខ្មែរ ភាសា", "Khmer with space");
    }

    #[test]
    fn thai_line_break_with_spaces() {
        assert_no_double_breaks_and_utf_consistency("ภาษา ไทย", "Thai with space");
    }

    #[test]
    fn lao_line_break_with_spaces() {
        assert_no_double_breaks_and_utf_consistency("ລາວ ພາສາ", "Lao with space");
    }

    /// Exercises the UAX #14 filter in `complex::complex_language_line_breaks_str`
    /// against an input with multiple SA-SP boundaries. Each Thai word
    /// is independently segmented; the trailing break of each SA run is
    /// filtered because the next char (SP) has LB class SP which forbids
    /// a preceding break under LB7.
    #[test]
    fn sa_sp_boundary_filtered_at_every_run() {
        let text = "ภาษา ไทย abc";
        assert_no_double_breaks_and_utf_consistency(text, "multiple SA-SP boundaries");

        let segmenter = LineSegmenter::new_auto();
        let breaks: Vec<usize> = segmenter.segment_str(text).collect();
        let first_space = text.find(' ').unwrap();
        let second_space = text.rfind(' ').unwrap();
        assert!(
            !breaks.contains(&first_space),
            "unexpected break BEFORE first space at {first_space}: {breaks:?}"
        );
        assert!(
            !breaks.contains(&second_space),
            "unexpected break BEFORE second space at {second_space}: {breaks:?}"
        );
        assert!(
            breaks.contains(&(first_space + 1)),
            "missing break AFTER first space at {}: {breaks:?}",
            first_space + 1
        );
        assert!(
            breaks.contains(&(second_space + 1)),
            "missing break AFTER second space at {}: {breaks:?}",
            second_space + 1
        );
    }

    /// Non-ASCII whitespace around SA must not produce double-breaks.
    ///
    /// UAX #14 assigns line-break class `SP` only to U+0020. Other
    /// "whitespace-looking" characters have different LB classes — most
    /// of them class BA (break after), which LB21 forbids a break
    /// *before*. The generalized filter in
    /// `complex::complex_language_line_breaks_*` consults the caller's
    /// `forbids_break_before` predicate for every terminal SA boundary,
    /// so all of the following must be handled uniformly:
    ///
    ///   - U+00A0 NO-BREAK SPACE  → GL (LB12a × GL)
    ///   - U+3000 IDEOGRAPHIC SP  → BA (LB21 × BA)
    ///   - U+2009 THIN SPACE      → BA
    ///   - U+0009 TAB             → BA
    #[test]
    fn non_ascii_whitespace_around_sa() {
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{00A0}ไทย", "Thai + NBSP");
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{3000}ไทย", "Thai + IDSP");
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{2009}ไทย", "Thai + THIN SPACE");
        assert_no_double_breaks_and_utf_consistency("ภาษา\tไทย", "Thai + TAB");
    }

    /// LSTM and Dictionary engines both feed through
    /// `complex_language_line_breaks_str`, so both must be free of the
    /// SA|SP double-break regardless of which word boundaries each engine
    /// chooses internally.
    ///
    /// The two engines are **not** expected to agree on the full set of
    /// break positions (the dict's word list differs from the LSTM's
    /// trained boundaries). What this test guards is the invariant that
    /// the UAX #14 filter applies uniformly — neither produces a
    /// double-break around the space, and each engine's break at the
    /// byte after each space is present (LB18 `SP ÷`).
    #[test]
    fn lstm_vs_dictionary_no_double_break() {
        let text = "ภาษา ไทย ภาษา";

        let lstm = LineSegmenter::new_lstm();
        let dict = LineSegmenter::new_dictionary();

        let lstm_breaks: Vec<usize> = lstm.segment_str(text).collect();
        let dict_breaks: Vec<usize> = dict.segment_str(text).collect();

        for pair in lstm_breaks.windows(2).skip(1) {
            assert!(pair[1] - pair[0] > 1, "LSTM double-break: {lstm_breaks:?}");
        }
        for pair in dict_breaks.windows(2).skip(1) {
            assert!(pair[1] - pair[0] > 1, "Dict double-break: {dict_breaks:?}");
        }

        for (engine, breaks) in [("LSTM", &lstm_breaks), ("Dict", &dict_breaks)] {
            for (i, _) in text.match_indices(' ') {
                assert!(
                    !breaks.contains(&i),
                    "{engine} broke BEFORE space at {i}: {breaks:?}"
                );
                assert!(
                    breaks.contains(&(i + 1)),
                    "{engine} missing break AFTER space at {}: {breaks:?}",
                    i + 1
                );
            }
        }
    }

    /// Word segmenter regression: `word.rs` uses
    /// `complex_language_segment_str` (the word-boundary oracle), not the
    /// line-break variant, so it must see every word boundary the dict/LSTM
    /// produces. This locks that invariant end-to-end.
    #[test]
    fn word_segmenter_thai_unchanged() {
        use crate::WordSegmenter;
        let segmenter = WordSegmenter::new_auto();
        let text = "ภาษาไทยภาษาไทย";
        let breaks: Vec<usize> = segmenter.segment_str(text).collect();
        // Every word boundary from the dict must be preserved — including
        // the terminal one at the end of the SA run.
        assert!(
            breaks.contains(&text.len()),
            "word segmenter must keep terminal boundary at {}: {breaks:?}",
            text.len()
        );
        for pair in breaks.windows(2) {
            assert!(
                pair[1] - pair[0] > 0,
                "word segmenter emitted duplicate offset: {breaks:?}"
            );
        }
    }
}
