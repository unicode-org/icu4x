// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;

use crate::complex::{Dictionary, LstmPayloads};
use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;
use crate::{provider::*, SegmenterError};
use utf8_iter::Utf8CharIndices;

/// Grapheme cluster break iterator for an `str` (a UTF-8 string).
pub type GraphemeClusterBreakIteratorUtf8<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypeUtf8>;

/// Grapheme cluster break iterator for a potentially invalid UTF-8 string.
pub type GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypePotentiallyIllFormedUtf8>;

/// Grapheme cluster break iterator for a Latin-1 (8-bit) string.
pub type GraphemeClusterBreakIteratorLatin1<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypeLatin1>;

/// Grapheme cluster break iterator for a UTF-16 string.
pub type GraphemeClusterBreakIteratorUtf16<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypeUtf16>;

/// Segments a string into grapheme clusters.
///
/// Supports loading grapheme cluster break data, and creating grapheme cluster break iterators for
/// different string encodings.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2259">#2259</a>
/// </div>
///
/// # Examples
///
/// Segment a string:
///
/// ```rust
/// use icu_segmenter::GraphemeClusterSegmenter;
/// let segmenter = GraphemeClusterSegmenter::try_new_unstable(
///     &icu_testdata::unstable(),
/// )
/// .expect("Data exists");
///
/// let breakpoints: Vec<usize> = segmenter.segment_str("Hello 🗺").collect();
/// // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
/// assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 10]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu_segmenter::GraphemeClusterSegmenter;
/// let segmenter = GraphemeClusterSegmenter::try_new_unstable(
///     &icu_testdata::unstable(),
/// )
/// .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
/// ```
pub struct GraphemeClusterSegmenter {
    payload: DataPayload<GraphemeClusterBreakDataV1Marker>,
    dictionary: Dictionary,
    lstm: LstmPayloads,
}

impl GraphemeClusterSegmenter {
    /// Construct a [`GraphemeClusterSegmenter`].
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<GraphemeClusterBreakDataV1Marker> + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        let dictionary = Dictionary::default();
        let lstm = LstmPayloads::default();
        Ok(Self {
            payload,
            dictionary,
            lstm,
        })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: SegmenterError);

    /// Create a grapheme cluster break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(
        &'l self,
        input: &'s str,
    ) -> GraphemeClusterBreakIteratorUtf8<'l, 's> {
        GraphemeClusterBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: None,
        }
    }

    /// Create a grapheme cluster break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    pub fn segment_utf8<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8<'l, 's> {
        GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8 {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: None,
        }
    }
    /// Create a grapheme cluster break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> GraphemeClusterBreakIteratorLatin1<'l, 's> {
        GraphemeClusterBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: None,
        }
    }

    /// Create a grapheme cluster break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(
        &'l self,
        input: &'s [u16],
    ) -> GraphemeClusterBreakIteratorUtf16<'l, 's> {
        GraphemeClusterBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            dictionary: &self.dictionary,
            lstm: &self.lstm,
            grapheme: None,
        }
    }
}

pub struct GraphemeClusterBreakTypeUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypeUtf8 {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().len_utf8()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        unreachable!()
    }
}

pub struct GraphemeClusterBreakTypePotentiallyIllFormedUtf8;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypePotentiallyIllFormedUtf8 {
    type IterAttr = Utf8CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.get_current_codepoint().len_utf8()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        unreachable!()
    }
}

pub struct GraphemeClusterBreakTypeLatin1;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8;

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        unreachable!()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        unreachable!()
    }
}

pub struct GraphemeClusterBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.get_current_codepoint();
        if ch >= 0x10000 {
            2
        } else {
            1
        }
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        unreachable!()
    }
}
