// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dictionary::DictionarySegmenter;
use crate::indices::*;
use crate::language::*;
use crate::provider::*;
use crate::symbols::*;

use alloc::vec;
use alloc::vec::Vec;
use core::char;
use core::str::CharIndices;
use icu_provider::prelude::*;

// Use the LSTM when the feature is enabled.
#[cfg(feature = "lstm")]
use crate::lstm::get_line_break_utf16;

// No-op function when LSTM is disabled.
#[cfg(not(feature = "lstm"))]
fn get_line_break_utf16(_: &[u16]) -> Option<Vec<usize>> {
    None
}

/// An enum specifies the strictness of line-breaking rules. It can be passed as
/// an argument when creating a line breaker.
///
/// Each enum value has the same meaning with respect to the `line-break`
/// property values in the CSS Text spec. See the details in
/// <https://drafts.csswg.org/css-text-3/#line-break-property>.
#[derive(Copy, Clone, PartialEq, Eq)]
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
#[derive(Copy, Clone, PartialEq, Eq)]
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
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq)]
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

/// Supports loading line break data, and creating line break iterators for different string
/// encodings. Please see the [module-level documentation](crate) for its usages.
pub struct LineBreakSegmenter {
    options: LineBreakOptions,
    payload: DataPayload<LineBreakDataV1Marker>,
    dictionary_payload: DataPayload<UCharDictionaryBreakDataV1Marker>,
}

impl LineBreakSegmenter {
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: ResourceProvider<LineBreakDataV1Marker>
            + ResourceProvider<UCharDictionaryBreakDataV1Marker>
            + ?Sized,
    {
        Self::try_new_with_options(provider, Default::default())
    }

    pub fn try_new_with_options<D>(
        provider: &D,
        options: LineBreakOptions,
    ) -> Result<Self, DataError>
    where
        D: ResourceProvider<LineBreakDataV1Marker>
            + ResourceProvider<UCharDictionaryBreakDataV1Marker>
            + ?Sized,
    {
        let payload = provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;

        // TODO: Use `provider` parameter after we support loading dictionary data from the
        // production-ready providers.
        let inv_provider = icu_provider::inv::InvariantDataProvider;
        let dictionary_payload = inv_provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        Ok(Self {
            options,
            payload,
            dictionary_payload,
        })
    }

    /// Create a line break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> LineBreakIterator<'l, 's, char> {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary_payload: &self.dictionary_payload,
        }
    }

    /// Create a line break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> LineBreakIterator<'l, 's, Latin1Char> {
        LineBreakIterator {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary_payload: &self.dictionary_payload,
        }
    }

    /// Create a line break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(
        &'l self,
        input: &'s [u16],
    ) -> LineBreakIterator<'l, 's, Utf16Char> {
        LineBreakIterator {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            options: &self.options,
            dictionary_payload: &self.dictionary_payload,
        }
    }
}

fn get_linebreak_property_utf32_with_rule(
    property_table: &RuleBreakPropertyTable<'_>,
    codepoint: u32,
    line_break_rule: LineBreakRule,
    word_break_rule: WordBreakRule,
) -> u8 {
    if codepoint < 0x20000 {
        let codepoint = codepoint as usize;
        let prop = property_table.0.get(codepoint).unwrap_or(0);

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
        return prop;
    }

    match codepoint {
        0x20000..=0x2fffd => ID,
        0x30000..=0x3fffd => ID,
        0xe0001 => CM,
        0xe0020..=0xe007f => CM,
        0xe0100..=0xe01ef => CM,
        _ => XX,
    }
}

#[inline]
fn get_linebreak_property_latin1(property_table: &RuleBreakPropertyTable<'_>, codepoint: u8) -> u8 {
    let codepoint = codepoint as usize;
    property_table.0.get(codepoint).unwrap_or(0)
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
        && matches!(get_language(codepoint), Language::Thai | Language::Burmese)
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

    fn compute_line_break_for_complex_language(
        iterator: &LineBreakIterator<'l, 's, Self>,
        input: &[u16],
    ) -> Vec<usize>;
}

/// Implements the [`Iterator`] trait over the line break opportunities of the given string. Please
/// see the [module-level documentation](crate) for its usages.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the [`LineBreakSegmenter`] object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// [`Iterator`]: core::iter::Iterator
pub struct LineBreakIterator<'l, 's, Y: LineBreakType<'l, 's> + ?Sized> {
    iter: Y::IterAttr,
    len: usize,
    current_pos_data: Option<(usize, Y::CharType)>,
    result_cache: Vec<usize>,
    data: &'l RuleBreakDataV1<'l>,
    options: &'l LineBreakOptions,
    dictionary_payload: &'l DataPayload<UCharDictionaryBreakDataV1Marker>,
}

impl<'l, 's, Y: LineBreakType<'l, 's>> Iterator for LineBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }

        if !self.result_cache.is_empty() {
            // We have break point cache by previous run.
            let mut i = 0;
            loop {
                if i == *self.result_cache.first().unwrap() {
                    self.result_cache.remove(0);
                    self.result_cache = self.result_cache.iter().map(|r| r - i).collect();
                    return Some(self.current_pos_data.unwrap().0);
                }
                self.current_pos_data = self.iter.next();
                if self.current_pos_data.is_none() {
                    // Reach EOF
                    self.result_cache.clear();
                    return Some(self.len);
                }
                i += 1;
            }
        }

        loop {
            let mut left_prop = self.get_linebreak_property();
            let left_codepoint = self.current_pos_data;
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                // EOF
                return Some(self.len);
            }
            let right_prop = self.get_linebreak_property();

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
                    if self.is_break_by_normal() {
                        return Some(self.current_pos_data.unwrap().0);
                    }
                }
                LineBreakRule::Loose => {
                    if let Some(breakable) = is_break_utf32_by_loose(
                        self.current_pos_data.unwrap().1.into(),
                        left_prop,
                        right_prop,
                        self.options.ja_zh,
                    ) {
                        if breakable {
                            return Some(self.current_pos_data.unwrap().0);
                        }
                        continue;
                    }
                }
                LineBreakRule::Anywhere => {
                    return Some(self.current_pos_data.unwrap().0);
                }
                _ => (),
            };

            // UAX14 doesn't have Thai etc, so use another way.
            if self.options.word_break_rule != WordBreakRule::BreakAll
                && Y::use_complex_breaking(self, left_codepoint.unwrap().1)
                && Y::use_complex_breaking(self, self.current_pos_data.unwrap().1)
            {
                let result = self.handle_complex_language(left_codepoint.unwrap().1);
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
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                        let break_state = self
                            .get_break_state_from_table(break_state as u8, self.data.eot_property);
                        if break_state == NOT_MATCH_RULE {
                            self.iter = previous_iter;
                            self.current_pos_data = previous_pos_data;
                            return Some(previous_pos_data.unwrap().0);
                        }
                        // EOF
                        return Some(self.len);
                    }

                    let prop = self.get_linebreak_property();
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
                    return Some(previous_pos_data.unwrap().0);
                }
                return Some(self.current_pos_data.unwrap().0);
            }

            if self.is_break_from_table(left_prop, right_prop) {
                return Some(self.current_pos_data.unwrap().0);
            }
        }
    }
}

impl<'l, 's, Y: LineBreakType<'l, 's>> LineBreakIterator<'l, 's, Y> {
    #[inline]
    fn is_eof(&mut self) -> bool {
        if self.current_pos_data.is_none() {
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                return true;
            }
        }
        false
    }

    fn get_linebreak_property(&self) -> u8 {
        Y::get_linebreak_property_with_rule(self, self.current_pos_data.unwrap().1)
    }

    fn is_break_by_normal(&self) -> bool {
        is_break_utf32_by_normal(self.current_pos_data.unwrap().1.into(), self.options.ja_zh)
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

    // UAX14 doesn't define line break rules for some languages such as Thai.
    // These languages uses dictionary-based breaker, so we use OS's line breaker instead.
    fn handle_complex_language(&mut self, left_codepoint: Y::CharType) -> Option<usize> {
        let start_iter = self.iter.clone();
        let start_point = self.current_pos_data;
        let mut s = vec![left_codepoint.into() as u16];
        loop {
            s.push(self.current_pos_data.unwrap().1.into() as u16);
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                break;
            }
            if !Y::use_complex_breaking(self, self.current_pos_data.unwrap().1) {
                break;
            }
        }
        // Restore iterator to move to head of complex string
        self.iter = start_iter;
        self.current_pos_data = start_point;
        let breaks = Y::compute_line_break_for_complex_language(self, &s);
        let mut i = 1;
        self.result_cache = breaks;
        // result_cache vector is utf-16 index that is in BMP.
        loop {
            if i == *self.result_cache.first().unwrap() {
                self.result_cache.remove(0);
                self.result_cache = self.result_cache.iter().map(|r| r - i).collect();
                return Some(self.current_pos_data.unwrap().0);
            }
            self.current_pos_data = self.iter.next();
            if self.current_pos_data.is_none() {
                self.result_cache.clear();
                return Some(self.len);
            }
            i += 1;
        }
    }
}

impl<'l, 's> LineBreakType<'l, 's> for char {
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

    fn compute_line_break_for_complex_language(
        iterator: &LineBreakIterator<Self>,
        input: &[u16],
    ) -> Vec<usize> {
        if let Some(mut ret) = get_line_break_utf16(input) {
            ret.push(input.len());
            return ret;
        }
        if let Ok(segmenter) = DictionarySegmenter::try_new(iterator.dictionary_payload) {
            let mut result: Vec<usize> = segmenter.segment_utf16(input).collect();
            result.push(input.len());
            return result;
        }
        [input.len()].to_vec()
    }

    /*
        fn handle_complex_language(iterator: &LineBreakIterator<char>, left_codepoint: char) -> Option<usize> {
            let start_iter = self.iter.clone();
            let start_point = self.current_pos_data;
            loop {
                self.current_pos_data = self.iter.next();
                if self.current_pos_data.is_none() {
                    break;
                }
                if !self.use_complex_breaking(self.current_pos_data.unwrap().1) {
                    break;
                }
            }
            if let Some(mut breaks) = get_line_break_utf8(&str) {
                breaks.push(str.len());
                return breaks;
            }
            [str.len_utf8()].to_vec()
        }
    */
}

pub struct Latin1Char(pub u8);

impl<'l, 's> LineBreakType<'l, 's> for Latin1Char {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8; // TODO: Latin1Char

    fn get_linebreak_property_with_rule(iterator: &LineBreakIterator<Self>, c: u8) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(&iterator.data.property_table, c)
    }

    #[inline]
    fn use_complex_breaking(_iterator: &LineBreakIterator<Self>, _c: u8) -> bool {
        false
    }

    fn compute_line_break_for_complex_language(
        _: &LineBreakIterator<Self>,
        _input: &[u16],
    ) -> Vec<usize> {
        panic!("not reachable");
    }
}

// TODO: This should be u16
pub struct Utf16Char(pub u32);

impl<'l, 's> LineBreakType<'l, 's> for Utf16Char {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32; // TODO: Utf16Char

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

    fn compute_line_break_for_complex_language(
        iterator: &LineBreakIterator<Self>,
        input: &[u16],
    ) -> Vec<usize> {
        if let Some(mut ret) = get_line_break_utf16(input) {
            ret.push(input.len());
            return ret;
        }
        if let Ok(segmenter) = DictionarySegmenter::try_new(iterator.dictionary_payload) {
            let mut result: Vec<usize> = segmenter.segment_utf16(input).collect();
            result.push(input.len());
            return result;
        }
        [input.len()].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_linebreak_property(codepoint: char) -> u8 {
        let provider = icu_testdata::get_provider();
        let payload: DataPayload<LineBreakDataV1Marker> = provider
            .load_resource(&DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let lb_data: &RuleBreakDataV1 = payload.get();
        get_linebreak_property_with_rule(
            &lb_data.property_table,
            codepoint,
            LineBreakRule::Strict,
            WordBreakRule::Normal,
        )
    }

    #[test]
    fn linebreak_propery() {
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

    fn is_break(left: u8, right: u8) -> bool {
        let provider = icu_testdata::get_provider();
        let payload: DataPayload<LineBreakDataV1Marker> = provider
            .load_resource(&DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let lb_data: &RuleBreakDataV1 = payload.get();
        is_break_from_table(
            &lb_data.break_state_table,
            lb_data.property_count,
            left,
            right,
        )
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // clearer when we're testing bools directly
    fn break_rule() {
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
        let provider = icu_testdata::get_provider();
        let segmenter = LineBreakSegmenter::try_new(&provider).expect("Data exists");

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
}
