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
/// an argument when creating a line breaker.
///
/// Each enum value has the same meaning with respect to the `line-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#line-break-property>.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LineBreakRule {
    /// Breaks text using the least restrictive set of line-breaking rules.
    /// Typically used for short lines, such as in newspapers.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-loose>
    Loose,

    /// Breaks text using the most common set of line-breaking rules.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-normal>
    Normal,

    /// Breaks text using the most stringent set of line-breaking rules.
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-strict>
    Strict,

    /// Breaks text assuming there is a soft wrap opportunity around every
    /// typographic character unit, disregarding any prohibition against line
    /// breaks. See more details in
    /// <https://drafts.csswg.org/css-text-3/#valdef-line-break-anywhere>.
    Anywhere,
}

/// An enum specifies the line break opportunities between letters. It can be
/// passed as an argument when creating a line breaker.
///
/// Each enum value has the same meaning with respect to the `word-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#word-break-property>
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WordBreakRule {
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

/// Options to tailor line breaking behavior, such as for CSS.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LineBreakOptions {
    /// Strictness of line-breaking rules. See [`LineBreakRule`].
    pub line_break_rule: LineBreakRule,

    /// Line break opportunities between letters. See [`WordBreakRule`].
    pub word_break_rule: WordBreakRule,

    /// Use `true` as a hint to the line breaker that the writing
    /// system is Chinese or Japanese. This allows more break opportunities when
    /// `LineBreakRule` is `Normal` or `Loose`. See
    /// <https://drafts.csswg.org/css-text-3/#line-break-property> for details.
    ///
    /// This option has no effect in Latin-1 mode.
    pub ja_zh: bool,
}

impl Default for LineBreakOptions {
    fn default() -> Self {
        Self {
            line_break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
        }
    }
}

/// Line break iterator for an `str` (a UTF-8 string).
pub type LineBreakIteratorUtf8<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeUtf8>;

/// Line break iterator for a potentially invalid UTF-8 string
pub type LineBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    LineBreakIterator<'l, 's, LineBreakTypePotentiallyIllFormedUtf8>;

/// Line break iterator for a Latin-1 (8-bit) string.
pub type LineBreakIteratorLatin1<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeLatin1>;

/// Line break iterator for a UTF-16 string.
pub type LineBreakIteratorUtf16<'l, 's> = LineBreakIterator<'l, 's, LineBreakTypeUtf16>;

/// Supports loading line break data, and creating line break iterators for different string
/// encodings.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
///
/// # Examples
///
/// Segment a string with default options:
///
/// ```rust
/// use icu_segmenter::LineSegmenter;
///
/// let segmenter = LineSegmenter::try_new_auto_unstable(&icu_testdata::unstable())
///     .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[6, 11]);
/// ```
///
/// Segment a string with CSS option overrides:
///
/// ```rust
/// use icu_segmenter::{
///     LineBreakOptions, LineBreakRule, LineSegmenter, WordBreakRule,
/// };
///
/// let mut options = LineBreakOptions::default();
/// options.line_break_rule = LineBreakRule::Strict;
/// options.word_break_rule = WordBreakRule::BreakAll;
/// options.ja_zh = false;
/// let segmenter = LineSegmenter::try_new_auto_with_options_unstable(
///     &icu_testdata::unstable(),
///     options,
/// )
/// .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu_segmenter::LineSegmenter;
///
/// let segmenter = LineSegmenter::try_new_auto_unstable(&icu_testdata::unstable())
///     .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[6, 11]);
/// ```
#[derive(Debug)]
pub struct LineSegmenter {
    options: LineBreakOptions,
    payload: DataPayload<LineBreakDataV1Marker>,
    dictionary: Dictionary,
    lstm: LstmPayloads,
    grapheme: DataPayload<GraphemeClusterBreakDataV1Marker>,
}

impl LineSegmenter {
    /// Construct a [`LineSegmenter`] via [`Self::try_new_auto_with_options_unstable`] with default
    /// [`LineBreakOptions`].
    #[cfg(feature = "auto")]
    pub fn try_new_auto_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_auto_with_options_unstable(provider, Default::default())
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        functions: [
            Self::try_new_auto_unstable,
            try_new_auto_with_any_provider,
            try_new_auto_with_buffer_provider
        ]
    );

    /// Construct a [`LineSegmenter`] via [`Self::try_new_lstm_with_options_unstable`] with default
    /// [`LineBreakOptions`].
    #[cfg(feature = "lstm")]
    pub fn try_new_lstm_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_lstm_with_options_unstable(provider, Default::default())
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        functions: [
            Self::try_new_lstm_unstable,
            try_new_lstm_with_any_provider,
            try_new_lstm_with_buffer_provider
        ]
    );

    /// Construct a [`LineSegmenter`] via [`Self::try_new_dictionary_with_options_unstable`] with
    /// default [`LineBreakOptions`].
    pub fn try_new_dictionary_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_dictionary_with_options_unstable(provider, Default::default())
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: skip,
        error: SegmenterError,
        functions: [
            Self::try_new_dictionary_unstable,
            try_new_dictionary_with_any_provider,
            try_new_dictionary_with_buffer_provider
        ]
    );

    /// Construct a [`LineSegmenter`] with custom [`LineBreakOptions`]. It automatically loads the
    /// best available payload data for Burmese, Khmer, Lao, and Thai.
    #[cfg(feature = "auto")]
    pub fn try_new_auto_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_lstm_with_options_unstable(provider, options)
    }

    #[cfg(feature = "auto")]
    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        functions: [
            Self::try_new_auto_with_options_unstable,
            try_new_auto_with_options_with_any_provider,
            try_new_auto_with_options_with_buffer_provider
        ]
    );

    /// Construct a [`LineSegmenter`] with custom [`LineBreakOptions`] and LSTM payload data for
    /// Burmese, Khmer, Lao, and Thai.
    #[cfg(feature = "lstm")]
    pub fn try_new_lstm_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<LstmDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let grapheme = provider.load(Default::default())?.take_payload()?;

        Ok(Self {
            options,
            payload,
            dictionary: Dictionary::default(),
            lstm: LstmPayloads::try_new(provider)?,
            grapheme,
        })
    }

    #[cfg(feature = "lstm")]
    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        functions: [
            Self::try_new_lstm_with_options_unstable,
            try_new_lstm_with_options_with_any_provider,
            try_new_lstm_with_options_with_buffer_provider
        ]
    );

    /// Construct a [`LineSegmenter`] with custom [`LineBreakOptions`] and dictionary payload data
    /// for Burmese, Khmer, Lao, and Thai.
    pub fn try_new_dictionary_with_options_unstable<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, SegmenterError>
    where
        D: DataProvider<LineBreakDataV1Marker>
            + DataProvider<UCharDictionaryBreakDataV1Marker>
            + DataProvider<GraphemeClusterBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let grapheme = provider.load(Default::default())?.take_payload()?;

        Ok(Self {
            options,
            payload,
            // Line segmenter doesn't need to load CJ dictionary because UAX 14 rules handles CJK
            // characters [1]. Southeast Asian languages however require complex context analysis
            // [2].
            //
            // [1]: https://www.unicode.org/reports/tr14/#ID
            // [2]: https://www.unicode.org/reports/tr14/#SA
            dictionary: Dictionary::new_southeast_asian(provider),
            lstm: LstmPayloads::default(),
            grapheme,
        })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: LineBreakOptions,
        error: SegmenterError,
        functions: [
            Self::try_new_dictionary_with_options_unstable,
            try_new_dictionary_with_options_with_any_provider,
            try_new_dictionary_with_options_with_buffer_provider
        ]
    );

    /// Create a line break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> LineBreakIteratorUtf8<'l, 's> {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.get(),
        }
    }
    /// Create a line break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
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
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.get(),
        }
    }
    /// Create a line break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(&'l self, input: &'s [u8]) -> LineBreakIteratorLatin1<'l, 's> {
        LineBreakIterator {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.get(),
        }
    }

    /// Create a line break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> LineBreakIteratorUtf16<'l, 's> {
        LineBreakIterator {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: self.grapheme.get(),
        }
    }
}

fn get_linebreak_property_utf32_with_rule(
    property_table: &RuleBreakPropertyTable<'_>,
    codepoint: u32,
    line_break_rule: LineBreakRule,
    word_break_rule: WordBreakRule,
) -> u8 {
    // Note: Default value is 0 == UNKNOWN
    let prop = property_table.0.get32(codepoint);

    if word_break_rule == WordBreakRule::BreakAll
        || line_break_rule == LineBreakRule::Loose
        || line_break_rule == LineBreakRule::Normal
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
    linebreak_rule: LineBreakRule,
    wordbreak_rule: WordBreakRule,
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
        LineBreakRule::Strict,
        WordBreakRule::Normal,
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

/// Implements the [`Iterator`] trait over the line break opportunities of the given string. Please
/// see the examples in [`LineSegmenter`] for its usages.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the [`LineSegmenter`] object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
#[derive(Debug)]
pub struct LineBreakIterator<'l, 's, Y: LineBreakType<'l, 's> + ?Sized> {
    iter: Y::IterAttr,
    len: usize,
    current_pos_data: Option<(usize, Y::CharType)>,
    result_cache: Vec<usize>,
    data: &'l RuleBreakDataV1<'l>,
    options: &'l LineBreakOptions,
    dictionary: &'l Dictionary,
    lstm: &'l LstmPayloads,
    grapheme: &'l RuleBreakDataV1<'l>,
}

impl<'l, 's, Y: LineBreakType<'l, 's>> Iterator for LineBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.check_eof() {
            return None;
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

            // Initializing right_codepoint can be simplified with a let-else statement in Rust 1.65.
            // https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#let-else-statements
            let right_codepoint = if let Some(right_codepoint) = self.get_current_codepoint() {
                right_codepoint
            } else {
                return Some(self.len);
            };
            let right_prop = self.get_linebreak_property(right_codepoint);

            // CSS word-break property handling
            match self.options.word_break_rule {
                WordBreakRule::BreakAll => {
                    left_prop = match left_prop {
                        AL => ID,
                        NU => ID,
                        SA => ID,
                        _ => left_prop,
                    };
                }
                WordBreakRule::KeepAll => {
                    if is_non_break_by_keepall(left_prop, right_prop) {
                        continue;
                    }
                }
                _ => (),
            }

            // CSS line-break property handling
            match self.options.line_break_rule {
                LineBreakRule::Normal => {
                    if self.is_break_by_normal(right_codepoint) {
                        return self.get_current_position();
                    }
                }
                LineBreakRule::Loose => {
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
                LineBreakRule::Anywhere => {
                    return self.get_current_position();
                }
                _ => (),
            };

            // UAX14 doesn't have Thai etc, so use another way.
            if self.options.word_break_rule != WordBreakRule::BreakAll
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

                    // Initializing prop can be simplified with a let-else statement in Rust 1.65.
                    // https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#let-else-statements
                    let prop = if let Some(prop) = self.get_current_linebreak_property() {
                        prop
                    } else {
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

impl<'l, 's, Y: LineBreakType<'l, 's>> LineBreakIterator<'l, 's, Y> {
    fn advance_iter(&mut self) {
        self.current_pos_data = self.iter.next();
    }

    fn is_eof(&self) -> bool {
        self.current_pos_data.is_none()
    }

    #[inline]
    fn check_eof(&mut self) -> bool {
        if self.is_eof() {
            self.advance_iter();
            if self.is_eof() {
                return true;
            }
        }
        false
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
            iterator.options.line_break_rule,
            iterator.options.word_break_rule,
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
            iterator.options.line_break_rule,
            iterator.options.word_break_rule,
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
    loop {
        debug_assert!(!iter.is_eof());
        s.push(iter.get_current_codepoint()?);
        iter.advance_iter();
        if let Some(current_codepoint) = iter.get_current_codepoint() {
            if !T::use_complex_breaking(iter, current_codepoint) {
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
    let breaks = complex_language_segment_str(
        Some(iter.dictionary),
        Some(iter.lstm),
        Some(iter.grapheme),
        &s,
    );
    iter.result_cache = breaks;
    let mut i = iter.get_current_codepoint()?.len_utf8();
    let first_pos = *iter.result_cache.first()?;
    loop {
        if i == first_pos {
            // Re-calculate breaking offset
            iter.result_cache = iter.result_cache.iter().skip(1).map(|r| r - i).collect();
            return iter.get_current_position();
        }
        iter.advance_iter();
        if iter.is_eof() {
            iter.result_cache.clear();
            return Some(iter.len);
        }
        i += T::get_current_position_character_len(iter);
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
            iterator.options.line_break_rule,
            iterator.options.word_break_rule,
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
        loop {
            debug_assert!(!iterator.is_eof());
            s.push(iterator.get_current_codepoint()? as u16);
            iterator.advance_iter();
            if let Some(current_codepoint) = iterator.get_current_codepoint() {
                if !Self::use_complex_breaking(iterator, current_codepoint) {
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
        let breaks = complex_language_segment_utf16(
            Some(iterator.dictionary),
            Some(iterator.lstm),
            Some(iterator.grapheme),
            &s,
        );
        let mut i = 1;
        iterator.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        let first_pos = *iterator.result_cache.first()?;
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
            iterator.advance_iter();
            if iterator.is_eof() {
                iterator.result_cache.clear();
                return Some(iterator.len);
            }
            i += 1;
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use crate::LineSegmenter;
    use icu_provider_adapters::fork::ForkByKeyProvider;
    use icu_provider_fs::FsDataProvider;
    use std::path::PathBuf;

    fn get_segmenter_testdata_provider() -> impl BufferProvider {
        let segmenter_fs_provider = FsDataProvider::try_new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/provider"),
        )
        .unwrap();
        ForkByKeyProvider::new(segmenter_fs_provider, icu_testdata::buffer())
    }

    #[test]
    fn linebreak_property() {
        let payload = DataProvider::<LineBreakDataV1Marker>::load(
            &get_segmenter_testdata_provider().as_deserializing(),
            Default::default(),
        )
        .expect("Loading should succeed!")
        .take_payload()
        .expect("Data should be present!");

        let get_linebreak_property = |codepoint| {
            get_linebreak_property_with_rule(
                &payload.get().property_table,
                codepoint,
                LineBreakRule::Strict,
                WordBreakRule::Normal,
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
            &get_segmenter_testdata_provider().as_deserializing(),
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
        let segmenter = LineSegmenter::try_new_dictionary_unstable(
            &get_segmenter_testdata_provider().as_deserializing(),
        )
        .expect("Data exists");

        let mut iter = segmenter.segment_str("hello world");
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(11), iter.next());
        assert_eq!(None, iter.next());

        iter = segmenter.segment_str("$10 $10");
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(7), iter.next());

        // LB10

        // LB14
        iter = segmenter.segment_str("[  abc def");
        assert_eq!(Some(7), iter.next());
        assert_eq!(Some(10), iter.next());
        assert_eq!(None, iter.next());

        let input: [u8; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
        let mut iter_u8 = segmenter.segment_latin1(&input);
        assert_eq!(Some(7), iter_u8.next());
        assert_eq!(Some(10), iter_u8.next());
        assert_eq!(None, iter_u8.next());

        let input: [u16; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(7), iter_u16.next());

        // LB15
        iter = segmenter.segment_str("abc\u{0022}  (def");
        assert_eq!(Some(10), iter.next());

        let input: [u8; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u8 = segmenter.segment_latin1(&input);
        assert_eq!(Some(10), iter_u8.next());

        let input: [u16; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(10), iter_u16.next());

        // LB16
        iter = segmenter.segment_str("\u{0029}\u{203C}");
        assert_eq!(Some(4), iter.next());
        iter = segmenter.segment_str("\u{0029}  \u{203C}");
        assert_eq!(Some(6), iter.next());

        let input: [u16; 4] = [0x29, 0x20, 0x20, 0x203c];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(4), iter_u16.next());

        // LB17
        iter = segmenter.segment_str("\u{2014}\u{2014}aa");
        assert_eq!(Some(6), iter.next());
        iter = segmenter.segment_str("\u{2014}  \u{2014}aa");
        assert_eq!(Some(8), iter.next());

        iter = segmenter.segment_str("\u{2014}\u{2014}  \u{2014}\u{2014}123 abc");
        assert_eq!(Some(14), iter.next());
        assert_eq!(Some(18), iter.next());
        assert_eq!(Some(21), iter.next());

        // LB25
        let mut iter = segmenter.segment_str("(0,1)+(2,3)");
        assert_eq!(Some(11), iter.next());
        let input: [u16; 11] = [
            0x28, 0x30, 0x2C, 0x31, 0x29, 0x2B, 0x28, 0x32, 0x2C, 0x33, 0x29,
        ];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(11), iter_u16.next());

        let input: [u16; 13] = [
            0x2014, 0x2014, 0x20, 0x20, 0x2014, 0x2014, 0x31, 0x32, 0x33, 0x20, 0x61, 0x62, 0x63,
        ];
        let mut iter_u16 = segmenter.segment_utf16(&input);
        assert_eq!(Some(6), iter_u16.next());

        iter = segmenter.segment_str("\u{1F3FB} \u{1F3FB}");
        assert_eq!(Some(5), iter.next());
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        assert_eq!(breaks, [12, 21, 33, TEST_STR.len()], "Thai test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 11, utf16.len()], "Thai test");

        let utf16: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4], "Thai test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks, [12, 18, 30, TEST_STR.len()], "Burmese test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks, [4, 6, 10, utf16.len()], "Burmese utf-16 test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn khmer_word_break() {
        const TEST_STR: &str = "សេចក្ដីប្រកាសជាសកលស្ដីពីសិទ្ធិមនុស្ស";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: This small sample matches the ICU dictionary segmenter
        assert_eq!(breaks, [39, 48, 54, 72, TEST_STR.len()], "Khmer test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [13, 16, 18, 24, utf16.len()], "Khmer utf-16 test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn lao_word_break() {
        const TEST_STR: &str = "ກ່ຽວກັບສິດຂອງມະນຸດ";

        let provider = get_segmenter_testdata_provider();
        let segmenter = LineSegmenter::try_new_lstm_with_buffer_provider(&provider).unwrap();
        let breaks: Vec<usize> = segmenter.segment_str(TEST_STR).collect();
        // Note: LSTM finds a break at '12' that the dictionary does not find
        assert_eq!(breaks, [12, 21, 30, 39, TEST_STR.len()], "Lao test");

        let utf16: Vec<u16> = TEST_STR.encode_utf16().collect();
        let breaks: Vec<usize> = segmenter.segment_utf16(&utf16).collect();
        assert_eq!(breaks, [4, 7, 10, 13, utf16.len()], "Lao utf-16 test");
    }
}
