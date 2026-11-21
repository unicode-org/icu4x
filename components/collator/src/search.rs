// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Forward-streaming collation element iterator with source spans for string search.

// Consumed by subsequent search matcher PRs (#7944).
#![allow(dead_code)]

use crate::comparison::{CollatorBorrowed, NormTrie};
use crate::elements::{CollationElement, CollationElements, NO_CE};
use crate::spans::{RemainingLen, SourceSpan, SpanStream, TrackSpans};
use core::iter::FusedIterator;
#[cfg(feature = "latin1")]
use icu_collections::codepointtrie::iter::Latin1CharsWithTrie;
use icu_collections::codepointtrie::iter::{
    CharsWithTrieDefaultForAscii, Utf8CharsWithTrieDefaultForAscii, Utf16CharsWithTrie, WithTrie,
};

/// Forward-streaming iterator yielding `(CollationElement, SourceSpan)` pairs
/// over any slice-backed `WithTrie` character stream.
///
/// Each emitted `SourceSpan` gives the half-open byte or code-unit range in the
/// source slice corresponding to that collation element, partitioning `0..len`
/// without gaps across contractions, expansions, Hangul syllables, numeric runs,
/// and reordered combining marks.
pub(crate) struct SearchCollationElements<'data, I>
where
    I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen,
{
    elements: CollationElements<'data, SpanStream<I>, NormTrie<'data>, TrackSpans>,
}

impl<'data, I> SearchCollationElements<'data, I>
where
    I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen,
{
    /// Creates a forward-streaming search collation element iterator from a
    /// slice-backed `WithTrie` iterator carrying `collator.norm_trie()`.
    #[inline(always)]
    pub(crate) fn new(collator: &CollatorBorrowed<'data>, iter: I) -> Self {
        let mut ret = Self {
            elements: collator.collation_elements_with_spans(SpanStream::new(iter)),
        };
        // Initialize in place so `upcoming` does not move after population.
        ret.elements.init();
        ret
    }
}

impl<'data, I> core::fmt::Debug for SearchCollationElements<'data, I>
where
    I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SearchCollationElements")
            .finish_non_exhaustive()
    }
}

impl<'data, I> Iterator for SearchCollationElements<'data, I>
where
    I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen,
{
    type Item = (CollationElement, SourceSpan);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let (ce, span) = self.elements.next();
        if ce == NO_CE { None } else { Some((ce, span)) }
    }
}

impl<'data, I> FusedIterator for SearchCollationElements<'data, I> where
    I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen
{
}

impl<'data> CollatorBorrowed<'data> {
    /// Streams `(CollationElement, SourceSpan)` pairs forward over a UTF-8 `&str`.
    #[inline(always)]
    pub(crate) fn search_elements<'s>(
        &self,
        s: &'s str,
    ) -> SearchCollationElements<'data, CharsWithTrieDefaultForAscii<'s, 'data, NormTrie<'data>, u32>>
    {
        self.search_elements_iter(CharsWithTrieDefaultForAscii::new(s, self.norm_trie()))
    }

    /// Streams `(CollationElement, SourceSpan)` pairs forward over potentially ill-formed UTF-8.
    #[inline(always)]
    pub(crate) fn search_elements_utf8<'s>(
        &self,
        slice: &'s [u8],
    ) -> SearchCollationElements<
        'data,
        Utf8CharsWithTrieDefaultForAscii<'s, 'data, NormTrie<'data>, u32>,
    > {
        self.search_elements_iter(Utf8CharsWithTrieDefaultForAscii::new(
            slice,
            self.norm_trie(),
        ))
    }

    /// Streams `(CollationElement, SourceSpan)` pairs forward over potentially ill-formed UTF-16.
    #[inline(always)]
    pub(crate) fn search_elements_utf16<'s>(
        &self,
        slice: &'s [u16],
    ) -> SearchCollationElements<'data, Utf16CharsWithTrie<'s, 'data, NormTrie<'data>, u32>> {
        self.search_elements_iter(Utf16CharsWithTrie::new(slice, self.norm_trie()))
    }

    /// Streams `(CollationElement, SourceSpan)` pairs forward over a Latin-1 byte slice.
    #[cfg(feature = "latin1")]
    #[inline(always)]
    pub(crate) fn search_elements_latin1<'s>(
        &self,
        slice: &'s [u8],
    ) -> SearchCollationElements<'data, Latin1CharsWithTrie<'s, 'data, NormTrie<'data>, u32>> {
        self.search_elements_iter(Latin1CharsWithTrie::new(slice, self.norm_trie()))
    }

    /// Streams `(CollationElement, SourceSpan)` pairs forward over any `SpanStream`-compatible iterator.
    #[inline(always)]
    pub(crate) fn search_elements_iter<I>(&self, iter: I) -> SearchCollationElements<'data, I>
    where
        I: Iterator<Item = (char, u32)> + WithTrie<'data, NormTrie<'data>, u32> + RemainingLen,
    {
        SearchCollationElements::new(self, iter)
    }
}
