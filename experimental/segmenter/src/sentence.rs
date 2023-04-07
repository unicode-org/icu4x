// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec::Vec;
use icu_provider::prelude::*;

use crate::indices::{Latin1Indices, Utf16Indices};
use crate::rule_segmenter::*;
use crate::{provider::*, SegmenterError};
use utf8_iter::Utf8CharIndices;

/// Sentence break iterator for an `str` (a UTF-8 string).
///
/// For more information, see [`SentenceSegmenter`].
pub type SentenceBreakIteratorUtf8<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeUtf8>;

/// Sentence break iterator for a potentially invalid UTF-8 string.
///
/// For more information, see [`SentenceSegmenter`].
pub type SentenceBreakIteratorPotentiallyIllFormedUtf8<'l, 's> =
    RuleBreakIterator<'l, 's, RuleBreakTypePotentiallyIllFormedUtf8>;

/// Sentence break iterator for a Latin-1 (8-bit) string.
///
/// For more information, see [`SentenceSegmenter`].
pub type SentenceBreakIteratorLatin1<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeLatin1>;

/// Sentence break iterator for a UTF-16 string.
///
/// For more information, see [`SentenceSegmenter`].
pub type SentenceBreakIteratorUtf16<'l, 's> = RuleBreakIterator<'l, 's, RuleBreakTypeUtf16>;

/// Supports loading sentence break data, and creating sentence break iterators for different string
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
/// Segment a string:
///
/// ```rust
/// use icu_segmenter::SentenceSegmenter;
/// let segmenter =
///     SentenceSegmenter::try_new_unstable(&icu_testdata::unstable())
///         .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_str("Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 11]);
/// ```
///
/// Segment a Latin1 byte string:
///
/// ```rust
/// use icu_segmenter::SentenceSegmenter;
/// let segmenter =
///     SentenceSegmenter::try_new_unstable(&icu_testdata::unstable())
///         .expect("Data exists");
///
/// let breakpoints: Vec<usize> =
///     segmenter.segment_latin1(b"Hello World").collect();
/// assert_eq!(&breakpoints, &[0, 11]);
/// ```
///
/// Successive boundaries can be used to retrieve the sentences.
/// In particular, the first boundary is always 0, and the last one is the
/// length of the segmented text in code units.
///
/// ```rust
/// # use icu_segmenter::SentenceSegmenter;
/// # let segmenter =
/// #     SentenceSegmenter::try_new_unstable(&icu_testdata::unstable())
/// #         .expect("Data exists");
/// let text = "Ceci tuera cela. Le livre tuera l’édifice.";
/// let words: Vec<&str> = segmenter
///    .segment_str(text)
///    .collect::<Vec<_>>()
///    .windows(2)
///    .map(|i| &text[i[0]..i[1]])
///    .collect();
/// assert_eq!(&words, &["Ceci tuera cela. ", "Le livre tuera l’édifice."]);
/// ```
#[derive(Debug)]
pub struct SentenceSegmenter {
    payload: DataPayload<SentenceBreakDataV1Marker>,
}

impl SentenceSegmenter {
    /// Construct a [`SentenceSegmenter`].
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, SegmenterError>
    where
        D: DataProvider<SentenceBreakDataV1Marker> + ?Sized,
    {
        let payload = provider.load(Default::default())?.take_payload()?;
        Ok(Self { payload })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: SegmenterError);

    /// Create a sentence break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> SentenceBreakIteratorUtf8<'l, 's> {
        SentenceBreakIteratorUtf8 {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: None,
            boundary_property: 0,
        }
    }
    /// Create a sentence break iterator for a potentially ill-formed UTF8 string
    ///
    /// Invalid characters are treated as REPLACEMENT CHARACTER
    pub fn segment_utf8<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> SentenceBreakIteratorPotentiallyIllFormedUtf8<'l, 's> {
        SentenceBreakIteratorPotentiallyIllFormedUtf8 {
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: None,
            boundary_property: 0,
        }
    }
    /// Create a sentence break iterator for a Latin-1 (8-bit) string.
    pub fn segment_latin1<'l, 's>(
        &'l self,
        input: &'s [u8],
    ) -> SentenceBreakIteratorLatin1<'l, 's> {
        SentenceBreakIteratorLatin1 {
            iter: Latin1Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: None,
            boundary_property: 0,
        }
    }

    /// Create a sentence break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(&'l self, input: &'s [u16]) -> SentenceBreakIteratorUtf16<'l, 's> {
        SentenceBreakIteratorUtf16 {
            iter: Utf16Indices::new(input),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            data: self.payload.get(),
            complex: None,
            boundary_property: 0,
        }
    }
}
