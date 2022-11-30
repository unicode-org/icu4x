// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use icu_provider::prelude::*;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;
use crate::{provider::*, SegmenterError};
use utf8_iter::Utf8CharIndices;

/// Grapheme cluster break iterator for an `str` (a UTF-8 string).
pub type GraphemeClusterBreakIteratorUtf8<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeUtf8>;

/// Grapheme cluster break iterator for a potentially invalid UTF-8 string.
pub type GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    RuleBreakIterator<'l, 's, RuleBreakTypePotentiallyIllFormedUtf8>;

/// Grapheme cluster break iterator for a Latin-1 (8-bit) string.
pub type GraphemeClusterBreakIteratorLatin1<'l, 's> =
    RuleBreakIterator<'l, 's, RuleBreakTypeLatin1>;

/// Grapheme cluster break iterator for a UTF-16 string.
pub type GraphemeClusterBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeUtf16>;

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
}

impl GraphemeClusterSegmenter {
    /// Construct a [`GraphemeClusterSegmenter`].
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<GraphemeClusterBreakDataV1Marker> + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        Ok(Self { payload })
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
            dictionary: None,
            lstm: None,
            grapheme: None,
        }
    }

    /// Create a grapheme cluster break iterator from grapheme cluster rule payload
    pub(crate) fn new_and_segment_str<'l, 's>(
        input: &'s str,
        payload: &'l RuleBreakDataV1<'l>,
    ) -> GraphemeClusterBreakIteratorUtf8<'l, 's> {
        GraphemeClusterBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: payload,
            dictionary: None,
            lstm: None,
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
            dictionary: None,
            lstm: None,
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
            dictionary: None,
            lstm: None,
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
            dictionary: None,
            lstm: None,
            grapheme: None,
        }
    }

    /// Create a grapheme cluster break iterator from grapheme cluster rule payload
    pub(crate) fn new_and_segment_utf16<'l, 's>(
        input: &'s [u16],
        payload: &'l RuleBreakDataV1<'l>,
    ) -> GraphemeClusterBreakIteratorUtf16<'l, 's> {
        GraphemeClusterBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: payload,
            dictionary: None,
            lstm: None,
            grapheme: None,
        }
    }
}
