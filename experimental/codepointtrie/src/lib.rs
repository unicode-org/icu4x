pub fn hello() {
    println!("Hello, world!");
}

const CODE_POINT_TRIE_FAST_TYPE_SHIFT: i32 = 6;

const CODE_POINT_TRIE_FAST_TYPE_DATA_BLOCK_LENGTH: u32 = 1 << CODE_POINT_TRIE_FAST_TYPE_SHIFT;

const CODE_POINT_TRIE_FAST_TYPE_DATA_MASK: u32 = CODE_POINT_TRIE_FAST_TYPE_DATA_BLOCK_LENGTH - 1;

// Fast indexing limit for "fast"-type trie
const CODE_POINT_TRIE_FAST_TYPE_FAST_INDEXING_MAX: u32 = 0xffff;

// Fast indexing limit for "small"-type trie
const CODE_POINT_TRIE_SMALL_TYPE_FAST_INDEXING_MAX: u32 = 0xfff;

const CODE_POINT_TRIE_ERROR_VALUE_NEG_DATA_OFFSET: u32 = 1;

const CODE_POINT_TRIE_HIGH_VALUE_NEG_DATA_OFFSET: u32 = 2;

const CODE_POINT_TRIE_BMP_INDEX_LENGTH: u32 = 0x10000 >> CODE_POINT_TRIE_FAST_TYPE_SHIFT;

const CODE_POINT_TRIE_SMALL_LIMIT: u32 = 0x10000;

const CODE_POINT_TRIE_SMALL_INDEX_LENGTH: u32 = CODE_POINT_TRIE_SMALL_LIMIT >> CODE_POINT_TRIE_FAST_TYPE_SHIFT;

const CODE_POINT_TRIE_SHIFT_3: u32 = 4;

const CODE_POINT_TRIE_SHIFT_2: u32 = 5 + CODE_POINT_TRIE_SHIFT_3;

const CODE_POINT_TRIE_SHIFT_1: u32 = 5 + CODE_POINT_TRIE_SHIFT_2;

const CODE_POINT_TRIE_SHIFT_2_3: u32 = CODE_POINT_TRIE_SHIFT_2 - CODE_POINT_TRIE_SHIFT_3;

const CODE_POINT_TRIE_SHIFT_1_2: u32 = CODE_POINT_TRIE_SHIFT_1 - CODE_POINT_TRIE_SHIFT_2;

const CODE_POINT_TRIE_OMITTED_BMP_INDEX_1_LENGTH: u32 = 0x10000 >> CODE_POINT_TRIE_SHIFT_1;

const CODE_POINT_TRIE_INDEX_2_BLOCK_LENGTH: u32 = 1 << CODE_POINT_TRIE_SHIFT_2;

const CODE_POINT_TRIE_INDEX_2_MASK: u32 = CODE_POINT_TRIE_INDEX_2_BLOCK_LENGTH - 1;

const CODE_POINT_TRIE_CP_PER_INDEX_2_ENTRY: u32 = 1 << CODE_POINT_TRIE_SHIFT_2;

const CODE_POINT_TRIE_INDEX_3_BLOCK_LENGTH: u32 = 1 << CODE_POINT_TRIE_SHIFT_2_3;

const CODE_POINT_TRIE_INDEX_3_MASK: u32 = CODE_POINT_TRIE_INDEX_3_BLOCK_LENGTH - 1;

const CODE_POINT_TRIE_SMALL_DATA_BLOCK_LENGTH: u32 = 1 << CODE_POINT_TRIE_SHIFT_3;

const CODE_POINT_TRIE_SMALL_DATA_MASK: u32 = CODE_POINT_TRIE_SMALL_DATA_BLOCK_LENGTH - 1;

/// The width of the elements in the data array of a CodePointTrie.
/// See UCPTrieValueWidth in ICU4C.
enum CodePointTrieValueWidth {
    BitsAny = -1,
    Bits16 = 0,
    Bits32 = 1,
    Bits8 = 2,
}

/// The type of trie represents whether the trie has an optimization that
/// would make it small or fast.
/// See UCPTrieType in ICU4C.
#[derive(PartialEq)]
enum CodePointTrieType {
    Any = -1,
    Fast = 0,
    Small = 1,
}

struct CodePointTrieData<'trie> {
    // void: bool,  // Do we need an equivalent to the `void *ptr0` field in
    // UCPTrieData? Is it derivative of a None value for the other fields? Is
    // there a performance / convenience to having it?
    data_8_bit: Option<&'trie [u8]>,
    data_16_bit: Option<&'trie [u16]>,
    data_32_bit: Option<&'trie [u32]>,
}

struct CodePointTrie<'trie> {
    index_length: u32,
    data_length: u32,
    high_start: u32,
    shifted12_high_start: u16,
    trie_type: CodePointTrieType,
    value_width: CodePointTrieValueWidth,
    index3_null_offset: u16,
    data_null_offset: u32,
    null_value: u32,
    index: &'trie [u16],
    data: &'trie CodePointTrieData<'trie>,
}

fn get_code_point_trie_type(trie_type_int: u8) -> CodePointTrieType {
    match trie_type_int {
        0 => CodePointTrieType::Fast,
        1 => CodePointTrieType::Small,
        _ => CodePointTrieType::Any,
    }
}

fn get_code_point_trie_value_width(value_width_int: u8) -> CodePointTrieValueWidth {
    match value_width_int {
        0 => CodePointTrieValueWidth::Bits16,
        1 => CodePointTrieValueWidth::Bits32,
        2 => CodePointTrieValueWidth::Bits8,
        _ => CodePointTrieValueWidth::BitsAny
    }
}

fn trie_internal_small_index(trie: &CodePointTrie, c: u32) -> u32 {
    let mut i1: u32 = c >> CODE_POINT_TRIE_SHIFT_1;
    if trie.trie_type == CodePointTrieType::Fast {
        assert!(0xffff < c && c < trie.high_start);
        i1 = i1 + CODE_POINT_TRIE_BMP_INDEX_LENGTH - CODE_POINT_TRIE_OMITTED_BMP_INDEX_1_LENGTH;
    } else {
        assert!(c < trie.high_start && trie.high_start > CODE_POINT_TRIE_SMALL_LIMIT);
        i1 = i1 + CODE_POINT_TRIE_SMALL_INDEX_LENGTH;
    }
    let mut i3_block: u32 = trie.index[ (trie.index[i1 as usize] as u32 + ((c >> CODE_POINT_TRIE_SHIFT_2) & CODE_POINT_TRIE_INDEX_2_MASK)) as usize] as u32;
    let mut i3: u32 = (c >> CODE_POINT_TRIE_SHIFT_3) & CODE_POINT_TRIE_INDEX_3_MASK;
    let mut data_block: u32;
    if i3_block & 0x8000 == 0 {
        // 16-bit indexes
        data_block = trie.index[(i3_block + i3) as usize] as u32;
    } else {
        // 18-bit indexes stored in groups of 9 entries per 8 indexes.
        i3_block = (i3_block & 0x7fff) + (i3 & !7) + (i3 >> 3);
        i3 = i3 & 7;
        data_block = ((trie.index[(i3_block + 1) as usize] << (2 + (2 * i3))) as u32 & 0x30000) as u32;
        data_block = data_block | trie.index[(i3_block + i3) as usize] as u32;
    }
    data_block + (c & CODE_POINT_TRIE_SMALL_DATA_MASK)
}

fn trie_small_index(trie: &CodePointTrie, c: u32) -> u32 {
    if c >= trie.high_start {
        trie.data_length - CODE_POINT_TRIE_HIGH_VALUE_NEG_DATA_OFFSET
    } else {
        trie_internal_small_index(trie, c)
    }
}

fn trie_fast_index(trie: &CodePointTrie, c: u32) -> u32 {
    let index_array_pos: u32 = (c >> CODE_POINT_TRIE_FAST_TYPE_SHIFT) +
        (c & CODE_POINT_TRIE_FAST_TYPE_DATA_MASK);
    trie.index[index_array_pos as usize] as u32
}

/// Get trie data array index position for code point value `c` that is beyond
/// ASCII range.
fn trie_cp_index(trie: &CodePointTrie, fast_max: u32, c: u32) -> u32 {
    if c <= fast_max {
        trie_fast_index(trie, c)
    } else if c < 0x10ffff {
        trie_small_index(trie, c)
    } else {
        trie.data_length - CODE_POINT_TRIE_ERROR_VALUE_NEG_DATA_OFFSET
    }
}

/// Get trie data array index position for code point value `c`.
fn trie_index(trie: &CodePointTrie, c: u32) -> u32 {
    if c <= 0x7f {
        c
    } else {
        let fast_indexing_limit: u32 =
            if trie.trie_type == CodePointTrieType::Fast {
                CODE_POINT_TRIE_FAST_TYPE_FAST_INDEXING_MAX // 0xffff
            } else {
                CODE_POINT_TRIE_SMALL_TYPE_FAST_INDEXING_MAX // 0xfff
            };
        trie_cp_index(trie, fast_indexing_limit, c)
    }
}

// pub fn trie_get(trie: &CodePointTrie, c: u32) -> u32 {
//     if c <= 0x7f {

//     } else {
//         trie.null_value
//     }
// }

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
        0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0x80, 0xc0, 0xc0, 0xc0, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let data_8: [u8; 260] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 0xad,
    ];

    let trie: CodePointTrie =
        CodePointTrie {
            index_length,
            data_length,
            high_start,
            shifted12_high_start,
            trie_type: get_code_point_trie_type(trie_type),
            value_width: get_code_point_trie_value_width(value_width),
            index3_null_offset,
            data_null_offset,
            null_value,
            index: &index,
            data: 
                &CodePointTrieData {
                    data_8_bit: Some(&data_8),
                    data_16_bit: None,
                    data_32_bit: None,
                },
        };

    assert_eq!(trie_index(&trie, 1), 1, "ASCII range code points index pos is code point value");
    assert_eq!(trie_index(&trie, 65), 65, "ASCII range code points index pos is code point value");
    assert_eq!(trie_index(&trie, 127), 127, "ASCII range code points index pos is code point value");

    assert_eq!(trie_index(&trie, 999), 0);

    // TODO: add more test cases for index values

    // TODO: add impl & tests for data-getting fns using the index fns


    let _check_ranges: [u32; 10] = [0, 1, 0x740, 1, 0x780, 2, 0x880, 3, 0x110000, 1];

}
