// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointtrie::impl_const::*;
use crate::codepointtrie::{
    CodePointTrie, TrieType, ValueWidth, TrieTypeEnum
};

pub(crate) fn trie_internal_small_index<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    c: u32,
) -> u32 {
    let mut i1: u32 = c >> SHIFT_1;
    if trie.trie_type() == TrieTypeEnum::Fast {
        assert!(0xffff < c && c < trie.high_start());
        i1 = i1 + BMP_INDEX_LENGTH - OMITTED_BMP_INDEX_1_LENGTH;
    } else {
        assert!(c < trie.high_start() && trie.high_start() > SMALL_LIMIT);
        i1 = i1 + SMALL_INDEX_LENGTH;
    }
    let mut i3_block: u32 = trie.index()
        [(trie.index()[i1 as usize] as u32 + ((c >> SHIFT_2) & INDEX_2_MASK)) as usize]
        as u32;
    let mut i3: u32 = (c >> SHIFT_3) & INDEX_3_MASK;
    let mut data_block: u32;
    if i3_block & 0x8000 == 0 {
        // 16-bit indexes
        data_block = trie.index()[(i3_block + i3) as usize] as u32;
    } else {
        // 18-bit indexes stored in groups of 9 entries per 8 indexes.
        i3_block = (i3_block & 0x7fff) + (i3 & !7) + (i3 >> 3);
        i3 = i3 & 7;
        data_block =
            ((trie.index()[(i3_block + 1) as usize] << (2 + (2 * i3))) as u32 & 0x30000) as u32;
        data_block = data_block | trie.index()[(i3_block + i3) as usize] as u32;
    }
    data_block + (c & SMALL_DATA_MASK)
}

/// Internal trie getter for a code point at or above the fast limit. Returns the data index.
pub(crate) fn trie_small_index<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    c: u32,
) -> u32 {
    if c >= trie.high_start() {
        trie.data_length() - HIGH_VALUE_NEG_DATA_OFFSET
    } else {
        trie_internal_small_index(trie, c)
    }
}

/// Internal trie getter for a code point below the fast limit. Returns the data index.
pub(crate) fn trie_fast_index<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    c: u32,
) -> u32 {
    let index_array_pos: u32 = c >> FAST_TYPE_SHIFT;
    let index_array_val: u16 = trie.index()[index_array_pos as usize];
    let fast_index_val: u32 = index_array_val as u32 + (c & FAST_TYPE_DATA_MASK);
    fast_index_val
}

/// Internal trie getter to get trie data array index position for code point
/// value `c` that is beyond ASCII range. Also checks that c is in
/// U+0000..10FFFF.
pub(crate) fn trie_cp_index<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    c: u32,
) -> u32 {
    if c < 0 {
        trie.data_length() - ERROR_VALUE_NEG_DATA_OFFSET
    } else if c <= 0xffff {
        trie_fast_index(trie, c)
    } else if c <= 0x10ffff {
        trie_small_index(trie, c)
    } else {
        trie.data_length() - ERROR_VALUE_NEG_DATA_OFFSET
    }
}

/// Helper function that gets the data array value at the provided index
pub(crate) fn trie_get_value<W: ValueWidth>(
    data: &[W],
    value_width: &W,
    data_index: u32,
) -> u32 {
    let return_val_opt: Option<u32> = match value_width {
        &ValueWidth::Bits16 => match data.data_16_bit() {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        &ValueWidth::Bits32 => match data.data_32_bit() {
            Some(data_array) => Some(data_array[data_index as usize]),
            _ => None,
        },
        &ValueWidth::Bits8 => match data.data_8_bit() {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        _ => None, // Unreachable if the trie is properly initialized.
    };
    return_val_opt.unwrap_or(0xffffffff)
}

pub(crate) fn trie_get<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    c: u32,
) -> u32 {
    let data_index: u32 = trie_cp_index(trie, c);
    let data_value: u32 = trie_get_value(&trie.data(), &trie.value_width(), data_index);
    data_value
}

pub(crate) fn check_trie<T: TrieType, W: ValueWidth>(
    trie: &CodePointTrie<W, T>,
    check_ranges: &[u32],
) {
    assert_eq!(
        0,
        check_ranges.len() % 2,
        "check_ranges must have an even number of 32-bit values in (limit,value) pairs"
    );

    let mut i: u32 = 0;
    let check_range_tuples = check_ranges.chunks(2);
    // Iterate over each check range
    for range_tuple in check_range_tuples {
        let range_end = range_tuple[0];
        let range_value = range_tuple[1];
        // Check all values in this range, one-by-one
        while i < range_end {
            assert_eq!(
                range_value,
                trie_get(trie, i),
                "expected trie_get({}) == {}",
                i,
                range_value
            );
            i = i + 1;
        }
    }
}
