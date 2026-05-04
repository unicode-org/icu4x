// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::*;
use crate::indices::*;
use crate::provider::*;
use crate::SegmenterError;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::char;
use core::str::CharIndices;
use icu_provider::prelude::*;
use utf8_iter::Utf8CharIndices;

// TODO(#1637): These constants should be data driven.
#[allow(dead_code)]
const UNKNOWN: u8 = 0;
#[allow(dead_code)]
const AI: u8 = 1;
#[allow(dead_code)]
const AL: u8 = 2;
#[allow(dead_code)]
const B2: u8 = 3;
#[allow(dead_code)]
const BA: u8 = 4;
#[allow(dead_code)]
const BB: u8 = 5;
#[allow(dead_code)]
const BK: u8 = 6;
#[allow(dead_code)]
const CB: u8 = 7;
#[allow(dead_code)]
const CJ: u8 = 8;
#[allow(dead_code)]
const CL: u8 = 9;
#[allow(dead_code)]
const CM: u8 = 10;
#[allow(dead_code)]
const CP: u8 = 11;
#[allow(dead_code)]
const CR: u8 = 12;
#[allow(dead_code)]
const EB: u8 = 13;
#[allow(dead_code)]
const EM: u8 = 14;
#[allow(dead_code)]
const EX: u8 = 15;
#[allow(dead_code)]
const GL: u8 = 16;
#[allow(dead_code)]
const H2: u8 = 17;
#[allow(dead_code)]
const H3: u8 = 18;
#[allow(dead_code)]
const HL: u8 = 19;
#[allow(dead_code)]
const HY: u8 = 20;
#[allow(dead_code)]
const ID: u8 = 21;
#[allow(dead_code)]
const ID_CN: u8 = 22;
#[allow(dead_code)]
const IN: u8 = 23;
#[allow(dead_code)]
const IS: u8 = 24;
#[allow(dead_code)]
const JL: u8 = 25;
#[allow(dead_code)]
const JT: u8 = 26;
#[allow(dead_code)]
const JV: u8 = 27;
#[allow(dead_code)]
const LF: u8 = 28;
#[allow(dead_code)]
const NL: u8 = 29;
#[allow(dead_code)]
const NS: u8 = 30;
#[allow(dead_code)]
const NU: u8 = 31;
#[allow(dead_code)]
const OP_EA: u8 = 32;
#[allow(dead_code)]
const OP_OP30: u8 = 33;
#[allow(dead_code)]
const PO: u8 = 34;
#[allow(dead_code)]
const PO_EAW: u8 = 35;
#[allow(dead_code)]
const PR: u8 = 36;
#[allow(dead_code)]
const PR_EAW: u8 = 37;
#[allow(dead_code)]
const QU: u8 = 38;
#[allow(dead_code)]
const RI: u8 = 39;
#[allow(dead_code)]
const SA: u8 = 40;
#[allow(dead_code)]
const SG: u8 = 41;
#[allow(dead_code)]
const SP: u8 = 42;
#[allow(dead_code)]
const SY: u8 = 43;
#[allow(dead_code)]
const WJ: u8 = 44;
#[allow(dead_code)]
const XX: u8 = 45;
#[allow(dead_code)]
const ZW: u8 = 46;
#[allow(dead_code)]
const ZWJ: u8 = 47;

/// Returns `true` if UAX #14 forbids a break immediately before a character
/// whose line-break class is `class`, regardless of the class that precedes it.
///
/// Used by the complex-script line-break pipeline to decide whether the
/// dict/LSTM's boundary between an SA chunk and the next character is a
/// valid line-break opportunity. The predicate runs only at the seam
/// between an SA chunk and the next character; the *preceding* char at
/// every callsite is therefore SA-class. For the classes listed below
/// UAX #14 states `× class` unconditionally in that context:
///
/// * **`LB7`**:   `× SP`, `× ZW` — never break before a space or zero-width
///   space.
/// * **`LB9`**:   `× CM`, `× ZWJ` — combining marks attach to the preceding char.
/// * **`LB11`**:  `× WJ` — word-joiner is glue.
/// * **`LB12a`**: `× GL` — non-breaking glue (the `SP|BA|HY` exception does
///   not apply when the previous class is SA).
/// * **`LB13`**:  `× CL`, `× CP`, `× EX`, `× IS`, `× SY`.
/// * **`LB19`**:  `× QU` — never break before a quotation mark. (1.5
///   classifies all quotation forms — initial `«` `"` `'`, final `»` `"`
///   `'`, and ambiguous `"` `'` — as `QU`; the newer LB19a East-Asian-
///   context tailoring does not apply when the previous class is SA.)
/// * **`LB21`**:  `× BA`, `× HY`, `× NS` — no break before these after the
///   previous non-space char (SA never introduces a space on its right,
///   so this simplifies to unconditional `×`).
/// * **`LB22`**:  `× IN`.
///
/// Classes intentionally **not** included here either allow a break
/// (LB18 `SP ÷`, LB20 `÷ CB`, etc.), require a *preceding-class* context
/// that SA cannot satisfy (LB14 `OP ×`, LB15 `QU ×`, LB17 `B2 SP* × B2`,
/// LB30 `(AL|HL|NU) × OP30`, LB30a/b stateful rules — all handled by the
/// outer line-break state machine, not this seam), or are handled by the
/// mandatory-break rules (LB4/LB5: BK, CR, LF, NL).
#[inline]
fn lb_class_forbids_break_before(class: u8) -> bool {
    matches!(
        class,
        SP | ZW | BA | HY | NS | WJ | GL | CL | CP | EX | IS | SY | IN | ZWJ | CM | QU,
    )
}

/// Returns `true` if `cp` is a complex-script (SA-class) iteration or
/// abbreviation marker that semantically attaches to the preceding
/// word and must therefore not have a line break inserted immediately
/// before it.
///
/// These characters are classified `SA` by Unicode (so they are
/// processed inside the dictionary/LSTM SA chunk rather than at the
/// SA→next-char seam), but the dict/LSTM may still emit a word
/// boundary right before them. UAX #14 itself says nothing about
/// internal SA boundaries; the dict's word list is the source of
/// truth there. For these specific marks, however, breaking the line
/// before them is wrong by definition of the character:
///
/// * **U+0E46** `ๆ` THAI CHARACTER MAIYAMOK — repetition mark; means
///   "repeat the previous word."
/// * **U+0E2F** `ฯ` THAI CHARACTER PAIYANNOI — abbreviation marker;
///   marks the elided continuation of the preceding word/phrase.
/// * **U+0EC6** `ໆ` LAO KO LA — Lao repetition mark, analogous to
///   THAI MAIYAMOK.
/// * **U+0EAF** `ຯ` LAO ELLIPSIS — Lao abbreviation marker, analogous
///   to THAI PAIYANNOI.
/// * **U+17D7** `ៗ` KHMER SIGN LEK TOO — Khmer repetition mark.
///
/// This is a typographic correction layered on top of the dict/LSTM
/// output, not a UAX #14 rule. It is applied only inside the line-
/// break pipeline; the word segmenter (which uses the word-boundary
/// entry point, not the line-break one) is unaffected.
#[inline]
fn sa_codepoint_must_attach_to_previous(cp: u32) -> bool {
    matches!(cp, 0x0E2F | 0x0E46 | 0x0EAF | 0x0EC6 | 0x17D7)
}

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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
/// # use icu::segmenter::LineSegmenter;
/// #
/// # let segmenter = LineSegmenter::new_auto();
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
/// use icu::segmenter::{
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
/// use icu::segmenter::LineSegmenter;
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
/// use icu::segmenter::LineSegmenter;
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

impl RuleBreakDataV1<'_> {
    fn get_linebreak_property_utf32_with_rule(
        &self,
        codepoint: u32,
        strictness: LineBreakStrictness,
        word_option: LineBreakWordOption,
    ) -> u8 {
        // Note: Default value is 0 == UNKNOWN
        let prop = self.property_table.get32(codepoint);

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
    fn get_break_state_from_table(&self, left: u8, right: u8) -> BreakState {
        let idx = (left as usize) * (self.property_count as usize) + (right as usize);
        // We use unwrap_or to fall back to the base case and prevent panics on bad data.
        self.break_state_table.get(idx).unwrap_or(BreakState::Keep)
    }

    #[inline]
    fn use_complex_breaking_utf32(&self, codepoint: u32) -> bool {
        let line_break_property = self.get_linebreak_property_utf32_with_rule(
            codepoint,
            LineBreakStrictness::Strict,
            LineBreakWordOption::Normal,
        );

        line_break_property == SA
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

        'a: loop {
            debug_assert!(!self.is_eof());
            let left_codepoint = self.get_current_codepoint()?;
            let mut left_prop = self.get_linebreak_property(left_codepoint);
            self.advance_iter();

            let Some(right_codepoint) = self.get_current_codepoint() else {
                return Some(self.len);
            };
            let right_prop = self.get_linebreak_property(right_codepoint);

            // CSS word-break property handling
            match (self.options.word_option, left_prop, right_prop) {
                (LineBreakWordOption::BreakAll, AL | NU | SA, _) => {
                    left_prop = ID;
                }
                //  typographic letter units shouldn't be break
                (
                    LineBreakWordOption::KeepAll,
                    AI | AL | ID | NU | HY | H2 | H3 | JL | JV | JT | CJ,
                    AI | AL | ID | NU | HY | H2 | H3 | JL | JV | JT | CJ,
                ) => {
                    continue;
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
            let mut index = match self.data.get_break_state_from_table(left_prop, right_prop) {
                BreakState::Index(index) => index,
                // Line break uses more that 64 states, so they spill over into the intermediate range,
                // and we cannot change that at the moment
                BreakState::Intermediate(index) => index + 64,
                BreakState::Break | BreakState::NoMatch => return self.get_current_position(),
                BreakState::Keep => continue,
            };

            let mut previous_iter = self.iter.clone();
            let mut previous_pos_data = self.current_pos_data;

            loop {
                self.advance_iter();

                let Some(prop) = self.get_current_linebreak_property() else {
                    // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                    let break_state = self
                        .data
                        .get_break_state_from_table(index, self.data.eot_property);
                    if break_state == BreakState::NoMatch {
                        self.iter = previous_iter;
                        self.current_pos_data = previous_pos_data;
                        return self.get_current_position();
                    }
                    // EOF
                    return Some(self.len);
                };

                match self.data.get_break_state_from_table(index, prop) {
                    BreakState::Keep => continue 'a,
                    BreakState::NoMatch => {
                        self.iter = previous_iter;
                        self.current_pos_data = previous_pos_data;
                        return self.get_current_position();
                    }
                    BreakState::Break => return self.get_current_position(),
                    BreakState::Index(i) => {
                        index = i;
                        previous_iter = self.iter.clone();
                        previous_pos_data = self.current_pos_data;
                    }
                    BreakState::Intermediate(i) => {
                        index = i + 64;
                        previous_iter = self.iter.clone();
                        previous_pos_data = self.current_pos_data;
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
        match codepoint.into() {
            0x301C | 0x30A0 => self.options.ja_zh,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct LineBreakTypeUtf8;

impl<'l, 's> LineBreakType<'l, 's> for LineBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: char) -> u8 {
        iterator.data.get_linebreak_property_utf32_with_rule(
            c as u32,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: char) -> bool {
        iterator.data.use_complex_breaking_utf32(c as u32)
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
        iterator.data.get_linebreak_property_utf32_with_rule(
            c as u32,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: char) -> bool {
        iterator.data.use_complex_breaking_utf32(c as u32)
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
        lb_class_forbids_break_before(data.get_linebreak_property_utf32_with_rule(
            c as u32,
            options.strictness,
            options.word_option,
        ))
    });
    // Drop dict/LSTM-emitted boundaries that fall immediately before an
    // SA iteration/abbreviation mark (THAI MAIYAMOK ๆ, KHMER LEK TOO ៗ,
    // etc.). These are typographic corrections layered on top of the
    // dict's word boundaries; see `sa_codepoint_must_attach_to_previous`.
    let breaks: Vec<usize> = breaks
        .into_iter()
        .filter(|&offset| {
            offset >= s.len()
                || !s[offset..]
                    .chars()
                    .next()
                    .is_some_and(|c| sa_codepoint_must_attach_to_previous(c as u32))
        })
        .collect();
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
        // Note: Default value is 0 == UNKNOWN
        iterator.data.property_table.get32(c as u32)
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
        iterator.data.get_linebreak_property_utf32_with_rule(
            c,
            iterator.options.strictness,
            iterator.options.word_option,
        )
    }

    #[inline]
    fn use_complex_breaking(iterator: &LineBreakIterator<Self>, c: u32) -> bool {
        iterator.data.use_complex_breaking_utf32(c)
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
        // Capture the codepoint that terminated collection (if any). See the
        // UTF-8 counterpart for rationale.
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
        // "forbids-break-before" classes we care about are BMP, so a single
        // u16 lookup is faithful for our purposes.
        let next_ext_code_unit = next_ext_cp.and_then(|c| u16::try_from(c).ok());
        let breaks =
            complex_language_line_breaks_utf16(iterator.complex, &s, next_ext_code_unit, |c| {
                lb_class_forbids_break_before(data.get_linebreak_property_utf32_with_rule(
                    c as u32,
                    options.strictness,
                    options.word_option,
                ))
            });
        // Drop dict/LSTM-emitted boundaries immediately before an SA
        // iteration/abbreviation mark. All such marks are BMP, so a
        // single-u16 lookup at the boundary is faithful.
        let breaks: Vec<usize> = breaks
            .into_iter()
            .filter(|&offset| {
                offset >= s.len()
                    || !sa_codepoint_must_attach_to_previous(s[offset] as u32)
            })
            .collect();
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
            payload.get().get_linebreak_property_utf32_with_rule(
                codepoint as u32,
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
            matches!(
                lb_data.get_break_state_from_table(left, right),
                BreakState::Break | BreakState::NoMatch
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

    /// Helper: assert no double-breaks (adjacent break positions) except at
    /// the mandatory start-of-text position 0, and that UTF-8 and UTF-16
    /// produce the same number of break segments.
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

    /// Original fix (issue #7218): SA followed by space emitted a
    /// double-break under the old pipeline (the dict's terminal
    /// boundary plus the LB18 `SP ÷` break). The UAX #14 oracle now
    /// drops the dict's terminal break per LB7 `× SP`.
    #[test]
    fn khmer_line_break_with_spaces() {
        assert_no_double_breaks_and_utf_consistency("ខ្មែរ ភាសា ខ្មែរ", "Khmer with single space");
        assert_no_double_breaks_and_utf_consistency("ខ្មែរ  ភាសា", "Khmer with double space");
        assert_no_double_breaks_and_utf_consistency(" ខ្មែរ ", "Khmer leading/trailing space");
    }

    #[test]
    fn thai_line_break_with_spaces() {
        assert_no_double_breaks_and_utf_consistency("ภาษา ไทย", "Thai single space");
        assert_no_double_breaks_and_utf_consistency("ภาษา  ไทย", "Thai double space");
    }

    /// Non-ASCII whitespace around SA also must not produce double-breaks.
    /// UAX #14 assigns `SP` only to U+0020. Other whitespace-looking
    /// characters have different LB classes — most class BA (break-after)
    /// or GL (glue) — but `lb_class_forbids_break_before` covers all of
    /// them uniformly:
    ///
    ///   - U+00A0 NO-BREAK SPACE  → GL (`LB12a` × GL)
    ///   - U+3000 IDEOGRAPHIC SP  → BA (`LB21` × BA)
    ///   - U+2009 THIN SPACE      → BA
    ///   - U+0009 TAB             → BA
    #[test]
    fn non_ascii_whitespace_around_sa() {
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{00A0}ไทย", "Thai + NBSP");
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{3000}ไทย", "Thai + IDSP");
        assert_no_double_breaks_and_utf_consistency("ภาษา\u{2009}ไทย", "Thai + THIN SPACE");
        assert_no_double_breaks_and_utf_consistency("ภาษา\tไทย", "Thai + TAB");
    }

    /// LB7 (`× ZW`) and LB19 (`× QU`) at the SA→next-char seam. The
    /// dict/LSTM emits a terminal break at the end of every SA run; the
    /// predicate must drop that break when the next character is a
    /// zero-width space or any quotation mark, regardless of script.
    ///
    /// 1.5 classifies all quotation forms — initial `«` `"` `'`, final
    /// `»` `"` `'`, and ambiguous `"` `'` — as a single `QU` class
    /// (the QU_PI / QU_PF split was added later), so adding `QU` to
    /// the predicate covers every quote form here.
    #[test]
    fn complex_script_punctuation_no_break_before() {
        let segmenter = LineSegmenter::new_auto();
        let cases: &[(&str, &str, char)] = &[
            ("Thai + ASCII dquote", "ภาษาไทย", '"'),
            ("Thai + ASCII squote", "ภาษาไทย", '\''),
            ("Thai + smart-left dquote", "ภาษาไทย", '\u{201C}'),
            ("Thai + smart-right dquote", "ภาษาไทย", '\u{201D}'),
            ("Thai + smart-left squote", "ภาษาไทย", '\u{2018}'),
            ("Thai + smart-right squote", "ภาษาไทย", '\u{2019}'),
            ("Thai + left guillemet", "ภาษาไทย", '\u{00AB}'),
            ("Thai + right guillemet", "ภาษาไทย", '\u{00BB}'),
            ("Thai + ZWSP", "ภาษาไทย", '\u{200B}'),
            ("Khmer + ASCII dquote", "ខ្មែរ", '"'),
            ("Khmer + smart-right dquote", "ខ្មែរ", '\u{201D}'),
            ("Khmer + left guillemet", "ខ្មែរ", '\u{00AB}'),
            ("Khmer + right guillemet", "ខ្មែរ", '\u{00BB}'),
            ("Khmer + ZWSP", "ខ្មែរ", '\u{200B}'),
            ("Lao + ASCII dquote", "ລາວ", '"'),
            ("Lao + right guillemet", "ລາວ", '\u{00BB}'),
            ("Lao + ZWSP", "ລາວ", '\u{200B}'),
        ];

        for (label, sa, punct) in cases {
            let mut text = String::from(*sa);
            let punct_byte_offset = text.len();
            text.push(*punct);

            let breaks: Vec<usize> = segmenter.segment_str(&text).collect();
            assert!(
                !breaks.contains(&punct_byte_offset),
                "{label}: unexpected break BEFORE '{}' (U+{:04X}) at byte {}: {breaks:?}",
                punct,
                *punct as u32,
                punct_byte_offset,
            );

            let utf16: Vec<u16> = text.encode_utf16().collect();
            let utf16_punct_offset = sa.encode_utf16().count();
            let utf16_breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
            assert!(
                !utf16_breaks.contains(&utf16_punct_offset),
                "{label} (utf16): unexpected break BEFORE '{}' (U+{:04X}) at u16 index {}: {utf16_breaks:?}",
                punct,
                *punct as u32,
                utf16_punct_offset,
            );
        }
    }

    /// LSTM-only complement to `complex_script_punctuation_no_break_before`
    /// for Burmese (whose bundled coverage in `new_auto` depends on the
    /// LSTM model).
    #[test]
    fn complex_script_punctuation_no_break_before_burmese() {
        let segmenter = LineSegmenter::new_lstm();
        let cases: &[(&str, &str, char)] = &[
            ("Burmese + ASCII squote", "မြန်မာ", '\''),
            ("Burmese + smart-right dquote", "မြန်မာ", '\u{201D}'),
            ("Burmese + left guillemet", "မြန်မာ", '\u{00AB}'),
            ("Burmese + right guillemet", "မြန်မာ", '\u{00BB}'),
            ("Burmese + ZWSP", "မြန်မာ", '\u{200B}'),
        ];

        for (label, sa, punct) in cases {
            let mut text = String::from(*sa);
            let punct_byte_offset = text.len();
            text.push(*punct);

            let breaks: Vec<usize> = segmenter.segment_str(&text).collect();
            assert!(
                !breaks.contains(&punct_byte_offset),
                "{label}: unexpected break BEFORE '{}' (U+{:04X}) at byte {}: {breaks:?}",
                punct,
                *punct as u32,
                punct_byte_offset,
            );

            let utf16: Vec<u16> = text.encode_utf16().collect();
            let utf16_punct_offset = sa.encode_utf16().count();
            let utf16_breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
            assert!(
                !utf16_breaks.contains(&utf16_punct_offset),
                "{label} (utf16): unexpected break BEFORE '{}' (U+{:04X}) at u16 index {}: {utf16_breaks:?}",
                punct,
                *punct as u32,
                utf16_punct_offset,
            );
        }
    }

    /// SA-internal iteration/abbreviation marks must not have a line
    /// break inserted immediately before them (THAI MAIYAMOK ๆ "repeat
    /// previous word", THAI PAIYANNOI ฯ abbreviation, LAO KO LA ໆ, LAO
    /// ELLIPSIS ຯ, KHMER LEK TOO ៗ). These are SA-class so processed
    /// inside the dict/LSTM chunk, not at the seam — the post-filter
    /// in line.rs drops dict-emitted boundaries before them.
    #[test]
    fn complex_script_sa_iteration_marks_no_break_before() {
        let segmenter = LineSegmenter::new_auto();
        let cases: &[(&str, &str, char)] = &[
            ("Thai + MAIYAMOK", "ภาษาไทย", '\u{0E46}'),
            ("Thai + PAIYANNOI", "ภาษาไทย", '\u{0E2F}'),
            ("Lao + KO LA", "ກ່ຽວກັບ", '\u{0EC6}'),
            ("Lao + ELLIPSIS", "ກ່ຽວກັບ", '\u{0EAF}'),
            ("Khmer + LEK TOO", "ខ្មែរ", '\u{17D7}'),
        ];

        for (label, sa, mark) in cases {
            let mut text = String::from(*sa);
            let mark_byte_offset = text.len();
            text.push(*mark);

            let breaks: Vec<usize> = segmenter.segment_str(&text).collect();
            assert!(
                !breaks.contains(&mark_byte_offset),
                "{label}: unexpected break BEFORE '{}' (U+{:04X}) at byte {}: {breaks:?}",
                mark,
                *mark as u32,
                mark_byte_offset,
            );

            let utf16: Vec<u16> = text.encode_utf16().collect();
            let utf16_mark_offset = sa.encode_utf16().count();
            let utf16_breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
            assert!(
                !utf16_breaks.contains(&utf16_mark_offset),
                "{label} (utf16): unexpected break BEFORE '{}' (U+{:04X}) at u16 index {}: {utf16_breaks:?}",
                mark,
                *mark as u32,
                utf16_mark_offset,
            );
        }
    }

    /// The SA-internal mark filter applies only to the *line*
    /// segmenter; UAX #29 word boundaries before iteration/abbreviation
    /// marks may still be valid (the dict/LSTM's word list is the
    /// source of truth there).
    #[test]
    fn word_segmenter_preserves_sa_iteration_marks() {
        use crate::WordSegmenter;
        let segmenter = WordSegmenter::new_auto();
        for text in &["ภาษาไทยๆ", "ภาษาฯ", "ខ្មែរៗ"] {
            let breaks: Vec<usize> = segmenter.segment_str(text).collect();
            assert!(
                breaks.first() == Some(&0) && breaks.last() == Some(&text.len()),
                "word segmenter failed on {text:?}: {breaks:?}",
            );
        }
    }

    /// Counter-test: confirm the predicate has not been over-broadened.
    /// Letters (LB class AL) immediately following an SA run *should*
    /// retain the dict/LSTM's terminal break — there's no UAX #14 rule
    /// of the form `× AL` at this seam.
    #[test]
    fn complex_script_letter_keeps_break_before() {
        let segmenter = LineSegmenter::new_auto();
        let text = "ภาษาไทยabc";
        let thai_end = "ภาษาไทย".len();
        let breaks: Vec<usize> = segmenter.segment_str(text).collect();
        assert!(
            breaks.contains(&thai_end),
            "expected break at SA→AL seam (byte {thai_end}): {breaks:?}",
        );
    }
}
