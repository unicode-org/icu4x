// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointtrie::impl_const::*;
use crate::codepointtrie::{
    CodePointTrie, CodePointTrieData, CodePointTrieType, CodePointTrieValueWidth,
};

// trait+impl fn polymorphism testing code

struct _MyStruct<T> {
    x: T,
}

fn _trait_impl_polymorphism_scratch_code() {
    let _xyz = _MyStruct { x: 42 };

    let _xyz = _MyStruct::<CodePointTrieValueWidth> {
        x: CodePointTrieValueWidth::Bits8,
    };
}

trait CodePointTrieReader<'t> {
    fn get_trie_type(trie_type_int: u8) -> CodePointTrieType;
    fn get_value_width(value_width_int: u8) -> CodePointTrieValueWidth;
    fn internal_small_index(
        trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
        c: u32,
    ) -> u32;
    fn small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32)
        -> u32;
    fn fast_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32;
}

// can't do this because:
//
// expected type, found variant `CodePointTrieType::Fast`
// not a type
// help: try using the variant's enum: `crate::CodePointTrieType`rustc(E0573)
//
// impl<'trie> CodePointTrieReader<'trie> for CodePointTrie<'trie, CodePointTrieType::Fast, CodePointTrieValueWidth::Bits8> {
//
// }

// empty trait+structs polymorphism testing code

// Using empty traits and structs instead of enums, following this strategy:
// https://stackoverflow.com/questions/59426358/how-to-make-a-struct-containing-an-enum-generic-over-the-enum-variant
// in order to allow polymorphism on methods for the `CodePointTrie` struct.
// This is because you cannot have `impl MyTrait for MyStruct<enum1::varianta, enum2::variantb> {...}`.

// alternative to having an enum `ValueWidth` with variants `Bits16`, `Bits32`, `Bits8`.

trait ValueWidth {}

struct Bits16;
struct Bits32;
struct Bits8;
struct BitsAny;

impl ValueWidth for Bits16 {}
impl ValueWidth for Bits32 {}
impl ValueWidth for Bits8 {}
impl ValueWidth for BitsAny {}

// alternative to having an enum `TrieType` with variants `Fast`, `Small`.

trait TrieType {}

struct FastType;
struct SmallType;
struct AnyType;

impl TrieType for FastType {}
impl TrieType for SmallType {}
impl TrieType for AnyType {}

fn trie_internal_small_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    c: u32,
) -> u32 {
    let mut i1: u32 = c >> SHIFT_1;
    if trie.trie_type() == CodePointTrieType::Fast {
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
fn trie_small_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
    c: u32,
) -> u32 {
    if c >= trie.high_start() {
        trie.data_length() - HIGH_VALUE_NEG_DATA_OFFSET
    } else {
        trie_internal_small_index(trie, c)
    }
}

/// Internal trie getter for a code point below the fast limit. Returns the data index.
fn trie_fast_index(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
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
fn trie_cp_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
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
fn trie_get_value(
    data: &CodePointTrieData,
    value_width: &CodePointTrieValueWidth,
    data_index: u32,
) -> u32 {
    let return_val_opt: Option<u32> = match value_width {
        &CodePointTrieValueWidth::Bits16 => match data.data_16_bit() {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        &CodePointTrieValueWidth::Bits32 => match data.data_32_bit() {
            Some(data_array) => Some(data_array[data_index as usize]),
            _ => None,
        },
        &CodePointTrieValueWidth::Bits8 => match data.data_8_bit() {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        _ => None, // Unreachable if the trie is properly initialized.
    };
    return_val_opt.unwrap_or(0xffffffff)
}

fn trie_get(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    let data_index: u32 = trie_cp_index(trie, c);
    let data_value: u32 = trie_get_value(&trie.data(), &trie.value_width(), data_index);
    data_value & 0xff
}

fn check_trie(
    trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>,
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

#[cfg(test)]
mod fast_8_test {
    use super::*;

    // Exported trie data from free-blocks.8.toml. This file represents a
    // fast-type trie with 8-bit width data.
    #[test]
    pub fn fast_type_8_bit_trie_test() {
        let index_length: u32 = 1024;
        let data_length: u32 = 260;
        // Question: in ICU4C, `highStart` is a `UChar32` type. Does it make sense
        // to represent it as a u32 since UnicodeSet deals with `u32` instead of
        // the Rust `char` type?
        let high_start: u32 = 0xa00;
        let shifted12_high_start: u16 = 0x1;
        let trie_type: u8 = 0;
        let value_width: u8 = 2;
        let index3_null_offset: u16 = 0x7fff;
        let data_null_offset: u32 = 0x0;
        let null_value: u32 = 0x1;

        let index: [u16; 1024] = [
            0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0x80, 0xc0, 0xc0, 0xc0, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let data_8: [u8; 260] = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 0xad,
        ];

        let trie: CodePointTrie<CodePointTrieType, CodePointTrieValueWidth> = CodePointTrie {
            index_length,
            data_length,
            high_start,
            shifted12_high_start,
            trie_type: crate::codepointtrie::get_code_point_trie_type(trie_type),
            value_width: crate::codepointtrie::get_code_point_trie_value_width(value_width),
            index3_null_offset,
            data_null_offset,
            null_value,
            index: &index,
            data: &CodePointTrieData {
                data_8_bit: Some(&data_8),
                data_16_bit: None,
                data_32_bit: None,
            },
        };

        assert_eq!(0, trie_cp_index(&trie, 0), "trie_cp_index(&trie, 0)");
        assert_eq!(1, trie_cp_index(&trie, 1), "trie_cp_index(&trie, 1)");
        assert_eq!(2, trie_cp_index(&trie, 2), "trie_cp_index(&trie, 2)");

        // fastIndex == index[64 >> FAST_SHIFT] + (64 & FAST_DATA_MASK)
        //   == index[63 >> 6] + (63 & 63)
        //   == index[0] + 63
        //   == 0 + 63
        //   == 63
        assert_eq!(63, trie_cp_index(&trie, 63), "trie_cp_index(&trie, 63)");

        // fastIndex == index[64 >> FAST_SHIFT] + (64 & FAST_DATA_MASK)
        //   == index[64 >> 6] + (64 & 63)
        //   == index[1] + 0
        //   == 64 + 0
        //   == 64
        assert_eq!(64, trie_cp_index(&trie, 64), "trie_cp_index(&trie, 64)");

        // fastIndex == index[127 >> FAST_SHIFT] + (127 & FAST_DATA_MASK)
        //   == index[127 >> 6] + (127 & 63)
        //   == index[1] + 63
        //   == 64 + 63
        //   == 127
        assert_eq!(127, trie_cp_index(&trie, 127), "trie_cp_index(&trie, 127)");

        // fastIndex == index[999 >> FAST_SHIFT] + (999 & FAST_DATA_MASK)
        //   == index[999 >> 6] + (999 & 63)
        //   == index[15] + 39
        //   == 39
        assert_eq!(trie_cp_index(&trie, 999), 39);

        assert_eq!(trie_get(&trie, 0), 1);
        assert_eq!(trie_get(&trie, 1), 1);
        assert_eq!(trie_get(&trie, 2), 1);
        assert_eq!(trie_get(&trie, 28), 1);
        assert_eq!(trie_get(&trie, 29), 1);

        let check_ranges: [u32; 10] = [0, 1, 0x740, 1, 0x780, 2, 0x880, 3, 0x110000, 1];

        check_trie(&trie, &check_ranges);
    }
}
