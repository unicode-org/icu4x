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

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::*;
    use crate::CollatorPreferences;
    use crate::options::CollatorOptions;
    use crate::preferences::CollationNumericOrdering;
    use crate::spans::NoSpans;
    use icu_locale_core::locale;

    fn make_collator(
        prefs: impl Into<CollatorPreferences>,
        numeric: bool,
    ) -> CollatorBorrowed<'static> {
        let mut prefs = prefs.into();
        if numeric {
            prefs.numeric_ordering = Some(CollationNumericOrdering::True);
        }
        CollatorBorrowed::try_new(prefs, CollatorOptions::default()).unwrap()
    }

    fn collect_and_verify<I>(
        mut iter: SearchCollationElements<'static, I>,
        mut baseline: CollationElements<'static, I, NormTrie<'static>, NoSpans>,
        total_len: usize,
    ) -> Vec<(CollationElement, SourceSpan)>
    where
        I: Iterator<Item = (char, u32)> + WithTrie<'static, NormTrie<'static>, u32> + RemainingLen,
    {
        baseline.init();
        let mut out = Vec::new();
        let mut prev_end = 0;

        for (ce, span) in &mut iter {
            let base_ce = baseline.next();
            assert_ne!(base_ce, NO_CE, "baseline ended early");
            assert_eq!(ce, base_ce);
            assert_eq!(span.start, prev_end, "span gap/overlap at {span:?}");
            assert!(span.end >= span.start, "backward span {span:?}");
            assert!(span.end <= total_len, "span past end {span:?}");
            prev_end = span.end;
            out.push((ce, span));
        }

        assert_eq!(baseline.next(), NO_CE, "baseline had extra CEs");
        assert_eq!(prev_end, total_len, "spans did not cover entire input");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        out
    }

    #[test]
    fn test_forward_str_contractions_prefixes_and_reordering() {
        let cases: &[(&str, icu_locale_core::Locale, bool)] = &[
            ("", locale!("en"), false),
            ("abc", locale!("en"), false),
            ("c'h", locale!("br"), false),
            ("cha", locale!("cs"), false),
            ("baad", locale!("da"), false),
            ("o\u{0323}\u{0308}", locale!("sv"), false),
            ("xはゞyばゞ", locale!("ja"), false),
            ("Straße", locale!("de"), false),
            ("각", locale!("en"), false),
            ("가\u{0301}", locale!("en"), false),
            ("a42b", locale!("en"), true),
            ("e\u{0301}\u{0323}", locale!("en"), false),
            ("\u{0301}\u{0323}", locale!("en"), false),
            ("i\u{0307}\u{0301}", locale!("lt"), false),
            ("a😀b", locale!("en"), false),
        ];

        for &(input, ref loc, numeric) in cases {
            let collator = make_collator(loc.clone(), numeric);
            let iter = collator.search_elements(input);
            let baseline = collator.collation_elements_with_spans(
                CharsWithTrieDefaultForAscii::new(input, collator.norm_trie()),
            );
            let items = collect_and_verify(iter, baseline, input.len());
            if input.is_empty() {
                assert!(items.is_empty());
            } else {
                assert!(!items.is_empty());
            }
        }
    }

    #[test]
    fn test_utf8_utf16_and_generic_iter() {
        let collator_cs = make_collator(locale!("cs"), false);
        let utf8 = b"a\xF0\x9Fb\x80\x80c\xCC\x81cha";
        let utf8_items = collect_and_verify(
            collator_cs.search_elements_utf8(utf8),
            collator_cs.collation_elements_with_spans(Utf8CharsWithTrieDefaultForAscii::new(
                utf8,
                collator_cs.norm_trie(),
            )),
            utf8.len(),
        );
        assert!(!utf8_items.is_empty());

        let utf16: &[u16] = &[
            u16::from(b'a'),
            0xD83D,
            0xDE00,
            0xD800,
            u16::from(b'c'),
            u16::from(b'h'),
            u16::from(b'a'),
        ];
        let utf16_items = collect_and_verify(
            collator_cs.search_elements_utf16(utf16),
            collator_cs.collation_elements_with_spans(Utf16CharsWithTrie::new(
                utf16,
                collator_cs.norm_trie(),
            )),
            utf16.len(),
        );
        assert!(!utf16_items.is_empty());

        let via_iter = collator_cs.search_elements_iter(CharsWithTrieDefaultForAscii::new(
            "chacha",
            collator_cs.norm_trie(),
        ));
        let via_str = collator_cs.search_elements("chacha");
        assert_eq!(via_iter.collect::<Vec<_>>(), via_str.collect::<Vec<_>>());
    }

    #[cfg(feature = "latin1")]
    #[test]
    fn test_latin1_search_elements() {
        let collator_fi = make_collator(locale!("fi"), false);
        let latin1: &[u8] = &[b'a', 0xE9, b'y', 0xE4];
        let items = collect_and_verify(
            collator_fi.search_elements_latin1(latin1),
            collator_fi.collation_elements_with_spans(Latin1CharsWithTrie::new(
                latin1,
                collator_fi.norm_trie(),
            )),
            latin1.len(),
        );
        assert!(!items.is_empty());
    }

    #[test]
    fn test_non_static_haystack_lifetime_and_debug() {
        let collator = make_collator(locale!("en"), false);
        let owned = String::from("e\u{0301}x");
        let iter = collator.search_elements(owned.as_str());
        assert!(format!("{iter:?}").contains("SearchCollationElements"));
        let collected: Vec<_> = iter.collect();
        assert_eq!(
            collected.iter().map(|&(_, s)| s).collect::<Vec<_>>(),
            vec![
                SourceSpan::new(0, 1),
                SourceSpan::new(1, 3),
                SourceSpan::new(3, 4),
            ]
        );
    }
}
