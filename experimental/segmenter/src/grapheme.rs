// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::provider::*;
use crate::rule_segmenter::*;

/// Grapheme cluster break iterator for an `str` (a UTF-8 string).
pub type GraphemeClusterBreakIterator<'l, 's> = RuleBreakIterator<'l, 's, GraphemeClusterBreakType>;

/// Grapheme cluster break iterator for a Latin-1 (8-bit) string.
pub type GraphemeClusterBreakIteratorLatin1<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypeLatin1>;

/// Grapheme cluster break iterator for a UTF-16 string.
pub type GraphemeClusterBreakIteratorUtf16<'l, 's> =
    RuleBreakIterator<'l, 's, GraphemeClusterBreakTypeUtf16>;

/// Supports loading grapheme cluster break data, and creating grapheme cluster break iterators for
/// different string encodings. Please see the [module-level documentation](crate) for its usages.
pub struct GraphemeClusterBreakSegmenter {
    payload: DataPayload<GraphemeClusterBreakDataV1Marker>,
}

impl GraphemeClusterBreakSegmenter {
    pub fn try_new<D>(provider: &D) -> Result<Self, DataError>
    where
        D: ResourceProvider<GraphemeClusterBreakDataV1Marker> + ?Sized,
    {
        let payload = provider
            .load_resource(&DataRequest::default())?
            .take_payload()?;
        Ok(Self { payload })
    }

    /// Create a grapheme cluster break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> GraphemeClusterBreakIterator<'l, 's> {
        GraphemeClusterBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
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
        }
    }
}

pub struct GraphemeClusterBreakType;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakType {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        iter.current_pos_data.unwrap().1.len_utf8()
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct GraphemeClusterBreakTypeLatin1;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypeLatin1 {
    type IterAttr = Latin1Indices<'s>;
    type CharType = u8; // TODO: Latin1Char

    fn get_current_position_character_len(_: &RuleBreakIterator<Self>) -> usize {
        panic!("not reachable")
    }

    fn handle_complex_language(
        _: &mut RuleBreakIterator<Self>,
        _: Self::CharType,
    ) -> Option<usize> {
        panic!("not reachable")
    }
}

pub struct GraphemeClusterBreakTypeUtf16;

impl<'l, 's> RuleBreakType<'l, 's> for GraphemeClusterBreakTypeUtf16 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn get_current_position_character_len(iter: &RuleBreakIterator<Self>) -> usize {
        let ch = iter.current_pos_data.unwrap().1;
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
        panic!("not reachable")
    }
}
