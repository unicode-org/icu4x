// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::grapheme::*;
use crate::indices::*;
use crate::provider::*;
use crate::scaffold::{PotentiallyIllFormedUtf8, RuleBreakType, Utf8, Utf16};
use icu_collections::char16trie::{Char16Trie, TrieResult};

/// Lifetimes:
/// - `'data` = lifetime of the data
/// - `'s` = lifetime of the string being segmented
#[derive(Debug)]
pub(super) struct DictionaryBreakIterator<'data, 's, R: RuleBreakType> {
    trie: Char16Trie<'data>,
    iter: R::IterAttr<'s>,
    len: usize,
    grapheme_iter: GraphemeClusterBreakIterator<'data, 's, R>,
    // TODO transform value for byte trie
}

/// Implement the [`Iterator`] trait over the segmenter break opportunities of the given string.
/// Please see the [module-level documentation](crate) for its usages.
///
/// [`Iterator`]: core::iter::Iterator
impl<Y: RuleBreakType> Iterator for DictionaryBreakIterator<'_, '_, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut trie_iter = self.trie.iter();
        let mut intermediate_length = 0;
        let mut not_match = false;
        let mut previous_match = None;
        let mut last_grapheme_offset = 0;

        while let Some(next) = self.iter.next() {
            match trie_iter.next32(next.1.into()) {
                TrieResult::FinalValue(_) => {
                    return Some(next.0 + Y::char_len(next.1));
                }
                TrieResult::Intermediate(_) => {
                    // Dictionary has to match with grapheme cluster segment.
                    // If not, we ignore it.
                    while last_grapheme_offset < next.0 + Y::char_len(next.1) {
                        if let Some(offset) = self.grapheme_iter.next() {
                            last_grapheme_offset = offset;
                            continue;
                        }
                        last_grapheme_offset = self.len;
                        break;
                    }
                    if last_grapheme_offset != next.0 + Y::char_len(next.1) {
                        continue;
                    }

                    intermediate_length = next.0 + Y::char_len(next.1);
                    previous_match = Some((self.iter.clone(), self.grapheme_iter.clone_internal()));
                }
                TrieResult::NoMatch => {
                    if intermediate_length > 0 {
                        if let Some((prev_iter, prev_grapheme_iter)) = previous_match {
                            // Rewind previous match point
                            self.iter = prev_iter;
                            self.grapheme_iter = prev_grapheme_iter;
                        }
                        return Some(intermediate_length);
                    }
                    // Not found
                    return Some(next.0 + Y::char_len(next.1));
                }
                TrieResult::NoValue => {
                    // Prefix string is matched
                    not_match = true;
                }
            }
        }

        if intermediate_length > 0 {
            Some(intermediate_length)
        } else if not_match {
            // no match by scanning text
            Some(self.len)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub(super) struct DictionarySegmenter<'data> {
    dict: &'data UCharDictionaryBreakData<'data>,
    grapheme: GraphemeClusterSegmenterBorrowed<'data>,
}

impl<'data> DictionarySegmenter<'data> {
    pub(super) fn new(
        dict: &'data UCharDictionaryBreakData<'data>,
        grapheme: GraphemeClusterSegmenterBorrowed<'data>,
    ) -> Self {
        // TODO: no way to verify trie data
        Self { dict, grapheme }
    }

    /// Create a dictionary based break iterator for an `str` (a UTF-8 string).
    pub(super) fn segment_str<'s>(
        self,
        input: &'s str,
    ) -> DictionaryBreakIterator<'data, 's, Utf8> {
        let grapheme_iter = self.grapheme.segment_str(input);
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.dict.trie_data.clone()),
            iter: input.char_indices(),
            len: input.len(),
            grapheme_iter,
        }
    }

    /// Create a dictionary based break iterator for a UTF-8 string.
    pub(super) fn segment_utf8<'s>(
        self,
        input: &'s [u8],
    ) -> DictionaryBreakIterator<'data, 's, PotentiallyIllFormedUtf8> {
        let grapheme_iter = self.grapheme.segment_utf8(input);
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.dict.trie_data.clone()),
            iter: Utf8CharIndices::new(input),
            len: input.len(),
            grapheme_iter,
        }
    }

    /// Create a dictionary based break iterator for a UTF-16 string.
    pub(super) fn segment_utf16<'s>(
        self,
        input: &'s [u16],
    ) -> DictionaryBreakIterator<'data, 's, Utf16> {
        let grapheme_iter = self.grapheme.segment_utf16(input);
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.dict.trie_data.clone()),
            iter: Utf16Indices::new(input),
            len: input.len(),
            grapheme_iter,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
    use super::*;
    use crate::GraphemeClusterSegmenter;
    use crate::complex::ComplexPayloadsBorrowed;
    use icu_provider::prelude::*;

    use super::super::check_complex;

    #[test]
    fn burmese_dictionary_test() {
        let mut segmenter = ComplexPayloadsBorrowed::new();
        segmenter.with_southeast_asian_dictionaries();
        let segmenter = segmenter.select(ComplexScript::Myanmar).unwrap();

        // From css/css-text/word-break/word-break-normal-my-000.html
        check_complex(
            "မြန်မာစာမြန်မာစာမြန်မာစာ",
            &["မြန်မာ", "စာ", "မြန်မာ", "စာ", "မြန်မာ", "စာ"],
            segmenter,
        );
    }

    #[test]
    fn cj_dictionary_test() {
        let mut segmenter = ComplexPayloadsBorrowed::new();
        segmenter.with_japanese_dictionary();
        let segmenter = segmenter.select(ComplexScript::ChineseOrJapanese).unwrap();

        // Match case
        check_complex("龟山岛龟山岛", &["龟山岛", "龟山岛"], segmenter);

        // Match case, then no match case
        check_complex("エディターエディ", &["エディター", "エディ"], segmenter);
    }

    #[test]
    fn khmer_dictionary_test() {
        let mut segmenter = ComplexPayloadsBorrowed::new();
        segmenter.with_southeast_asian_dictionaries();
        let segmenter = segmenter.select(ComplexScript::Khmer).unwrap();

        check_complex(
            "ភាសាខ្មែរភាសាខ្មែរភាសាខ្មែរ",
            &["ភាសាខ្មែរ", "ភាសាខ្មែរ", "ភាសាខ្មែរ"],
            segmenter,
        );
    }

    #[test]
    fn lao_dictionary_test() {
        let mut segmenter = ComplexPayloadsBorrowed::new();
        segmenter.with_southeast_asian_dictionaries();
        let segmenter = segmenter.select(ComplexScript::Lao).unwrap();
        check_complex(
            "ພາສາລາວພາສາລາວພາສາລາວ",
            &["ພາສາ", "ລາວ", "ພາສາ", "ລາວ", "ພາສາ", "ລາວ"],
            segmenter,
        );
    }

    #[test]
    fn test_dictionary_grapheme_rewind() {
        let response: DataResponse<SegmenterDictionaryAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic("cjdict"),
                ),
                ..Default::default()
            })
            .unwrap();
        let dict_segmenter =
            DictionarySegmenter::new(response.payload.get(), GraphemeClusterSegmenter::new());

        // Test that grapheme_iter is correctly rewound when trie traversal backtracks.
        // In "エディターエディター":
        // 1. "エディター" (15 bytes) is matched as an Intermediate dictionary word at byte 15.
        // 2. Trie traversal continues exploring whether the compound prefix "エディターエディ..."
        //    forms a longer dictionary word, advancing grapheme_iter past byte 15 in the process.
        // 3. When trie traversal eventually hits NoMatch, self.iter rewinds to the last match
        //    at byte 15.
        // 4. Without rewinding grapheme_iter alongside self.iter, on the next call to next()
        //    (starting from byte 15), grapheme_iter is already positioned ahead of byte 15.
        //    When the second word's prefix "エディ" hits an Intermediate match at byte 24,
        //    the grapheme boundary check fails because grapheme_iter skips past 24, causing
        //    incorrect segmentation.
        let s = "エディターエディター";
        let result: Vec<usize> = dict_segmenter.segment_str(s).collect();
        assert_eq!(result, vec![15, 30]);
    }
}
