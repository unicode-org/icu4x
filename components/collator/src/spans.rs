// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Zero-cost span-tracking abstractions for collation element iteration.

// Consumed by subsequent stacked PRs (#7944).
#![allow(dead_code)]

use crate::elements::CollationElement;
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};

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
}
