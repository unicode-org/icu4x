// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::indices::Utf16Indices;
use crate::provider::*;

use core::str::CharIndices;
use icu_char16trie::char16trie::{Char16Trie, TrieResult};
use icu_provider::prelude::*;

/// A trait for dictionary based iterator
pub trait DictionaryType<'l, 's> {
    /// The iterator over characters.
    type IterAttr: Iterator<Item = (usize, Self::CharType)> + Clone;

    /// The character type.
    type CharType: Copy + Into<u32>;

    fn to_char(c: Self::CharType) -> char;
    fn char_len(c: Self::CharType) -> usize;
}

#[derive(Clone)]
pub struct DictionaryBreakIterator<'l, 's, Y: DictionaryType<'l, 's> + ?Sized> {
    trie: Char16Trie<'l>,
    iter: Y::IterAttr,
    len: usize,
    // TODO transform value for byte trie
}

impl<'l, 's, Y: DictionaryType<'l, 's> + ?Sized> Iterator for DictionaryBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut trie_iter = self.trie.iter();
        let mut intermediate_length = 0;
        let mut not_match = false;
        let mut previous_match = None;

        while let Some(next) = self.iter.next() {
            let ch = Y::to_char(next.1);
            match trie_iter.next(ch) {
                TrieResult::FinalValue(_) => {
                    return Some(next.0 + Y::char_len(next.1));
                }
                TrieResult::Intermediate(_) => {
                    intermediate_length = next.0 + Y::char_len(next.1);
                    previous_match = Some(self.iter.clone());
                }
                TrieResult::NoMatch => {
                    if intermediate_length > 0 {
                        if let Some(previous_match) = previous_match {
                            // Rewind previous match point
                            self.iter = previous_match;
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

impl<'l, 's> DictionaryType<'l, 's> for u32 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn to_char(c: u32) -> char {
        char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER)
    }

    fn char_len(c: u32) -> usize {
        if c >= 0x10000 {
            2
        } else {
            1
        }
    }
}

impl<'l, 's> DictionaryType<'l, 's> for char {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn to_char(c: char) -> char {
        c
    }

    fn char_len(c: char) -> usize {
        c.len_utf8()
    }
}

pub struct DictionarySegmenter<'a> {
    payload: &'a DataPayload<UCharDictionaryBreakDataV1Marker>,
}

impl<'a> DictionarySegmenter<'a> {
    pub fn try_new(
        payload: &'a DataPayload<UCharDictionaryBreakDataV1Marker>,
    ) -> Result<Self, DataError> {
        // TODO: no way to verify trie data
        Ok(Self { payload })
    }

    /// Create a dictionary based break iterator for an `str` (a UTF-8 string).
    #[allow(dead_code)]
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> DictionaryBreakIterator<'l, 's, char> {
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.payload.get().trie_data.clone()),
            iter: input.char_indices(),
            len: input.len(),
        }
    }

    /// Create a dictionary based break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(
        &'l self,
        input: &'s [u16],
    ) -> DictionaryBreakIterator<'l, 's, u32> {
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.payload.get().trie_data.clone()),
            iter: Utf16Indices::new(input),
            len: input.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zerovec::ZeroSlice;

    #[test]
    fn cj_dictionary_test() {
        // This test data is created by the following using ICU4C tools
        // LD_LIBRARY_PATH=lib bin/gendict --uchars data/brkitr/dictionaries/cjdict.txt tmp.bin
        // dd if=tmp.bin of=cjdict.dict bs=1 skip=64
        const CJ_DICTIONARY: &ZeroSlice<u16> = match ZeroSlice::<u16>::try_from_bytes(
            include_bytes!("../tests/testdata/cjdic.dict"),
        ) {
            Ok(s) => s,
            Err(_) => panic!("invalid dictionary data"),
        };

        let data = UCharDictionaryBreakDataV1 {
            trie_data: CJ_DICTIONARY.as_zerovec(),
        };
        let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
        let segmenter = DictionarySegmenter::try_new(&payload).expect("Data exists");

        // Match case
        let s = "龟山岛龟山岛";
        let result: Vec<usize> = segmenter.segment_str(s).collect();
        assert_eq!(result, vec![9, 18]);

        let s_utf16: Vec<u16> = s.encode_utf16().collect();
        let result: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
        assert_eq!(result, vec![3, 6]);

        // Match case, then no match case
        let s = "エディターエディ";
        let result: Vec<usize> = segmenter.segment_str(s).collect();
        assert_eq!(result, vec![15, 24]);

        let s_utf16: Vec<u16> = s.encode_utf16().collect();
        let result: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
        assert_eq!(result, vec![5, 8]);
    }

    #[test]
    fn khmer_dictionary_test() {
        // This test data is created by the following using ICU4C tools
        // LD_LIBRARY_PATH=lib bin/gendict --uchars data/brkitr/dictionaries/khmerdict.txt tmp.bin
        // dd if=tmp.bin of=khmer.dict bs=1 skip=64
        const KHMER_DICTIONARY: &ZeroSlice<u16> = match ZeroSlice::<u16>::try_from_bytes(
            include_bytes!("../tests/testdata/khmer.dict"),
        ) {
            Ok(s) => s,
            Err(_) => panic!("invalid dictionary data"),
        };

        let data = UCharDictionaryBreakDataV1 {
            trie_data: KHMER_DICTIONARY.as_zerovec(),
        };
        let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
        let segmenter = DictionarySegmenter::try_new(&payload).expect("Data exists");
        let s = "ភាសាខ្មែរភាសាខ្មែរភាសាខ្មែរ";
        let result: Vec<usize> = segmenter.segment_str(s).collect();
        assert_eq!(result, vec![27, 54, 81]);

        let s_utf16: Vec<u16> = s.encode_utf16().collect();
        let result: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
        assert_eq!(result, vec![9, 18, 27]);
    }

    #[test]
    fn lao_dictionary_test() {
        // This test data is created by the following using ICU4C tools
        // LD_LIBRARY_PATH=lib bin/gendict --uchars data/brkitr/dictionaries/laodict.txt tmp.bin
        // dd if=tmp.bin of=lao.dict bs=1 skip=64
        static LAO_DICTIONARY: &ZeroSlice<u16> =
            match ZeroSlice::<u16>::try_from_bytes(include_bytes!("../tests/testdata/lao.dict")) {
                Ok(s) => s,
                Err(_) => panic!("invalid dictionary data"),
            };

        let data = UCharDictionaryBreakDataV1 {
            trie_data: LAO_DICTIONARY.as_zerovec(),
        };
        let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
        let segmenter = DictionarySegmenter::try_new(&payload).expect("Data exists");
        let s = "ພາສາລາວພາສາລາວພາສາລາວ";
        let r: Vec<usize> = segmenter.segment_str(s).collect();
        assert_eq!(r, vec![12, 21, 33, 42, 54, 63]);

        let s_utf16: Vec<u16> = s.encode_utf16().collect();
        let r: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
        assert_eq!(r, vec![4, 7, 11, 14, 18, 21]);
    }
}
