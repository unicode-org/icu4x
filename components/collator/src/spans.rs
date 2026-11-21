// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-cost span-tracking abstractions for collation element iteration.

use crate::elements::CollationElement;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
#[cfg(feature = "latin1")]
use icu_collections::codepointtrie::iter::Latin1CharsWithTrie;
use icu_collections::codepointtrie::iter::{
    CharsWithTrieDefaultForAscii, Utf8CharsWithTrieDefaultForAscii, Utf16CharsWithTrie, WithTrie,
};
use icu_collections::codepointtrie::{AbstractCodePointTrie, TrieValue};

/// Half-open `start..end` offset range in the source text
/// (UTF-8 bytes, UTF-16 code units, or Latin-1 bytes).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    #[inline(always)]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[inline(always)]
    pub const fn empty_at(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Pairs an internal buffer element with a zero-sized `()` or a `SourceSpan`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Spanned<T, Span> {
    pub value: T,
    pub span: Span,
}

impl<T, Span> Spanned<T, Span> {
    #[inline(always)]
    pub const fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}

impl<T, Span> Deref for Spanned<T, Span> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T, Span> DerefMut for Spanned<T, Span> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

/// Abstracts span propagation so `NoSpans` compiles away to zero overhead while
/// `TrackSpans` preserves gap-free source spans for search collation.
pub(crate) trait SpanMode: Copy + Debug + 'static {
    type Span: Copy + Clone + Debug + PartialEq + Eq + Default;
    type Output: Copy + Clone + Debug + PartialEq;

    fn make_output(ce: CollationElement, span: Self::Span) -> Self::Output;
    fn before_span(span: Self::Span) -> Self::Span;
    fn combine_spans(first: Self::Span, second: Self::Span) -> Self::Span;
    fn continuation_span(span: Self::Span) -> Self::Span;
    fn after_starter_span(starter_span: Self::Span, mark_span: Self::Span) -> Self::Span;
}

/// Zero-cost span mode used by `Collator::compare` and sort key generation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct NoSpans;

impl SpanMode for NoSpans {
    type Span = ();
    type Output = CollationElement;

    #[inline(always)]
    fn make_output(ce: CollationElement, _span: ()) -> CollationElement {
        ce
    }

    #[inline(always)]
    fn before_span(_span: ()) {}

    #[inline(always)]
    fn combine_spans(_first: (), _second: ()) {}

    #[inline(always)]
    fn continuation_span(_span: ()) {}

    #[inline(always)]
    fn after_starter_span(_starter_span: (), _mark_span: ()) {}
}

/// Span-tracking mode pairing each `CollationElement` with its `SourceSpan`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct TrackSpans;

impl SpanMode for TrackSpans {
    type Span = SourceSpan;
    type Output = (CollationElement, SourceSpan);

    #[inline(always)]
    fn make_output(ce: CollationElement, span: SourceSpan) -> (CollationElement, SourceSpan) {
        (ce, span)
    }

    #[inline(always)]
    fn before_span(span: SourceSpan) -> SourceSpan {
        SourceSpan::empty_at(span.start)
    }

    #[inline(always)]
    fn combine_spans(first: SourceSpan, second: SourceSpan) -> SourceSpan {
        SourceSpan::new(first.start.min(second.start), first.end.max(second.end))
    }

    #[inline(always)]
    fn continuation_span(span: SourceSpan) -> SourceSpan {
        SourceSpan::empty_at(span.end)
    }

    #[inline(always)]
    fn after_starter_span(starter_span: SourceSpan, mark_span: SourceSpan) -> SourceSpan {
        // Clamp start to `starter_span.end` and extend end when `mark_span` sits later so
        // contracted, decomposed, and CCC-reordered marks form a monotonic, gap-free partition.
        SourceSpan::new(starter_span.end, mark_span.end.max(starter_span.end))
    }
}

/// Fused character-and-trie stream parameterized over [`SpanMode`].
pub(crate) trait CharWithTrieStream<'trie, T, Mode: SpanMode>:
    WithTrie<'trie, T, u32>
where
    T: AbstractCodePointTrie<'trie, u32>,
{
    fn next_char_with_trie(&mut self) -> Option<(char, u32, Mode::Span)>;
}

impl<'trie, T, I> CharWithTrieStream<'trie, T, NoSpans> for I
where
    T: AbstractCodePointTrie<'trie, u32>,
    I: Iterator<Item = (char, u32)> + WithTrie<'trie, T, u32>,
{
    #[inline(always)]
    fn next_char_with_trie(&mut self) -> Option<(char, u32, ())> {
        let (c, trie_val) = self.next()?;
        Some((c, trie_val, ()))
    }
}

/// Remaining byte or code-unit length of a slice-backed `WithTrie` iterator.
pub(crate) trait RemainingLen {
    fn remaining_len(&self) -> usize;
}

impl<'trie, T, V> RemainingLen for CharsWithTrieDefaultForAscii<'_, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.as_str().len()
    }
}

impl<'trie, T, V> RemainingLen for Utf8CharsWithTrieDefaultForAscii<'_, 'trie, T, V>
where
    V: TrieValue + Default,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<'trie, T, V> RemainingLen for Utf16CharsWithTrie<'_, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.as_slice().len()
    }
}

#[cfg(feature = "latin1")]
impl<'trie, T, V> RemainingLen for Latin1CharsWithTrie<'_, 'trie, T, V>
where
    V: TrieValue,
    T: AbstractCodePointTrie<'trie, V>,
{
    #[inline(always)]
    fn remaining_len(&self) -> usize {
        self.as_slice().len()
    }
}

/// Wraps any slice-backed `WithTrie` iterator to emit exact `SourceSpan`s
/// alongside `(char, u32)` with zero redundant offset bookkeeping.
#[derive(Debug)]
pub(crate) struct SpanStream<I> {
    iter: I,
    end_offset: usize,
}

impl<I: RemainingLen> SpanStream<I> {
    #[inline(always)]
    pub fn new(iter: I) -> Self {
        let end_offset = iter.remaining_len();
        Self { iter, end_offset }
    }
}

impl<'trie, T, I> WithTrie<'trie, T, u32> for SpanStream<I>
where
    T: AbstractCodePointTrie<'trie, u32>,
    I: WithTrie<'trie, T, u32>,
{
    #[inline(always)]
    fn trie(&self) -> &'trie T {
        self.iter.trie()
    }
}

impl<'trie, T, I> CharWithTrieStream<'trie, T, TrackSpans> for SpanStream<I>
where
    T: AbstractCodePointTrie<'trie, u32>,
    I: Iterator<Item = (char, u32)> + WithTrie<'trie, T, u32> + RemainingLen,
{
    #[inline(always)]
    fn next_char_with_trie(&mut self) -> Option<(char, u32, SourceSpan)> {
        let start = self.end_offset - self.iter.remaining_len();
        let (c, trie_val) = self.iter.next()?;
        let end = self.end_offset - self.iter.remaining_len();
        Some((c, trie_val, SourceSpan::new(start, end)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn test_zero_cost_no_spans_layout() {
        assert_eq!(size_of::<()>(), 0);
        assert_eq!(size_of::<Spanned<u32, ()>>(), size_of::<u32>());
        assert_eq!(align_of::<Spanned<u32, ()>>(), align_of::<u32>());
        assert_eq!(size_of::<Spanned<u64, ()>>(), size_of::<u64>());
        assert_eq!(align_of::<Spanned<u64, ()>>(), align_of::<u64>());
        assert_eq!(
            size_of::<Spanned<CollationElement, ()>>(),
            size_of::<CollationElement>()
        );
    }

    #[test]
    fn test_track_spans_operations() {
        let starter = SourceSpan::new(2, 5);
        assert_eq!(TrackSpans::before_span(starter), SourceSpan::empty_at(2));
        assert_eq!(
            TrackSpans::continuation_span(starter),
            SourceSpan::empty_at(5)
        );
        assert_eq!(
            TrackSpans::combine_spans(SourceSpan::new(2, 4), SourceSpan::new(6, 9)),
            SourceSpan::new(2, 9)
        );
        // Contiguous mark following starter
        assert_eq!(
            TrackSpans::after_starter_span(SourceSpan::new(0, 1), SourceSpan::new(1, 3)),
            SourceSpan::new(1, 3)
        );
        // Decomposed mark with empty continuation span at starter end
        assert_eq!(
            TrackSpans::after_starter_span(SourceSpan::new(0, 2), SourceSpan::new(2, 2)),
            SourceSpan::new(2, 2)
        );
        // Skipped mark emitted after a discontiguous contraction
        assert_eq!(
            TrackSpans::after_starter_span(SourceSpan::new(0, 5), SourceSpan::new(1, 3)),
            SourceSpan::new(5, 5)
        );
        // CCC-reordered mark at higher offset absorbs intermediate bytes without a gap
        assert_eq!(
            TrackSpans::after_starter_span(SourceSpan::new(0, 1), SourceSpan::new(3, 5)),
            SourceSpan::new(1, 5)
        );
        assert_eq!(
            TrackSpans::after_starter_span(SourceSpan::new(0, 0), SourceSpan::new(2, 4)),
            SourceSpan::new(0, 4)
        );
    }

    #[cfg(feature = "compiled_data")]
    fn collect_stream_spans<'trie, I>(mut stream: SpanStream<I>) -> Vec<(char, SourceSpan)>
    where
        I: Iterator<Item = (char, u32)>
            + WithTrie<'trie, icu_collections::codepointtrie::FastCodePointTrie<'trie, u32>, u32>
            + RemainingLen,
    {
        let mut out = Vec::new();
        while let Some((c, _tv, span)) = stream.next_char_with_trie() {
            out.push((c, span));
        }
        out
    }

    #[cfg(feature = "compiled_data")]
    #[test]
    fn test_span_stream_with_nfd_trie() {
        use icu_collections::codepointtrie::FastCodePointTrie;
        use icu_normalizer::provider::Baked;

        let trie: &FastCodePointTrie<u32> = (&Baked::SINGLETON_NORMALIZER_NFD_DATA_V1.trie)
            .try_into()
            .unwrap();

        let s = "a\u{0301}😀";
        assert_eq!(
            collect_stream_spans(SpanStream::new(CharsWithTrieDefaultForAscii::new(s, trie))),
            vec![
                ('a', SourceSpan::new(0, 1)),
                ('\u{0301}', SourceSpan::new(1, 3)),
                ('😀', SourceSpan::new(3, 7)),
            ]
        );

        let utf8_bytes = b"a\xF0\x9Fb\x80\x80";
        assert_eq!(
            collect_stream_spans(SpanStream::new(Utf8CharsWithTrieDefaultForAscii::new(
                utf8_bytes, trie
            ))),
            vec![
                ('a', SourceSpan::new(0, 1)),
                (char::REPLACEMENT_CHARACTER, SourceSpan::new(1, 3)),
                ('b', SourceSpan::new(3, 4)),
                (char::REPLACEMENT_CHARACTER, SourceSpan::new(4, 5)),
                (char::REPLACEMENT_CHARACTER, SourceSpan::new(5, 6)),
            ]
        );

        let utf16_units: &[u16] = &[
            u16::from(b'a'),
            0xD83D,
            0xDE00,
            0xD800,
            0xDC00,
            0xD800,
            u16::from(b'b'),
        ];
        assert_eq!(
            collect_stream_spans(SpanStream::new(Utf16CharsWithTrie::new(utf16_units, trie))),
            vec![
                ('a', SourceSpan::new(0, 1)),
                ('😀', SourceSpan::new(1, 3)),
                ('\u{10000}', SourceSpan::new(3, 5)),
                (char::REPLACEMENT_CHARACTER, SourceSpan::new(5, 6)),
                ('b', SourceSpan::new(6, 7)),
            ]
        );
    }

    #[cfg(all(test, feature = "compiled_data", feature = "latin1"))]
    #[test]
    fn test_latin1_span_stream() {
        use icu_collections::codepointtrie::FastCodePointTrie;
        use icu_normalizer::provider::Baked;

        let trie: &FastCodePointTrie<u32> = (&Baked::SINGLETON_NORMALIZER_NFD_DATA_V1.trie)
            .try_into()
            .unwrap();

        let bytes = &[b'a', 0xE9, b'b'];
        assert_eq!(
            collect_stream_spans(SpanStream::new(Latin1CharsWithTrie::new(bytes, trie))),
            vec![
                ('a', SourceSpan::new(0, 1)),
                ('é', SourceSpan::new(1, 2)),
                ('b', SourceSpan::new(2, 3)),
            ]
        );
    }
}
