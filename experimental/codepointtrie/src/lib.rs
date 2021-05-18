// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

const CODEPOINTTRIE_FAST_TYPE_SHIFT: i32 = 6;

/// Number of entries in a data block for code points below the fast limit. 64=0x40
const CODEPOINTTRIE_FAST_TYPE_DATA_BLOCK_LENGTH: u32 = 1 << CODEPOINTTRIE_FAST_TYPE_SHIFT;

/// Mask for getting the lower bits for the in-fast-data-block offset.
const CODEPOINTTRIE_FAST_TYPE_DATA_MASK: u32 = CODEPOINTTRIE_FAST_TYPE_DATA_BLOCK_LENGTH - 1;

// Fast indexing limit for "fast"-type trie
const CODEPOINTTRIE_FAST_TYPE_FAST_INDEXING_MAX: u32 = 0xffff;

// Fast indexing limit for "small"-type trie
const CODEPOINTTRIE_SMALL_TYPE_FAST_INDEXING_MAX: u32 = 0xfff;

/// Offset from dataLength (to be subtracted) for fetching the
/// value returned for out-of-range code points and ill-formed UTF-8/16.
const CODEPOINTTRIE_ERROR_VALUE_NEG_DATA_OFFSET: u32 = 1;

/// Offset from dataLength (to be subtracted) for fetching the
/// value returned for code points highStart..U+10FFFF.
const CODEPOINTTRIE_HIGH_VALUE_NEG_DATA_OFFSET: u32 = 2;

/// The length of the BMP index table. 1024=0x400
const CODEPOINTTRIE_BMP_INDEX_LENGTH: u32 = 0x10000 >> CODEPOINTTRIE_FAST_TYPE_SHIFT;

const CODEPOINTTRIE_SMALL_LIMIT: u32 = 0x10000;

const CODEPOINTTRIE_SMALL_INDEX_LENGTH: u32 =
    CODEPOINTTRIE_SMALL_LIMIT >> CODEPOINTTRIE_FAST_TYPE_SHIFT;

/// Shift size for getting the index-3 table offset.
const CODEPOINTTRIE_SHIFT_3: u32 = 4;

/// Shift size for getting the index-2 table offset.
const CODEPOINTTRIE_SHIFT_2: u32 = 5 + CODEPOINTTRIE_SHIFT_3;

/// Shift size for getting the index-1 table offset.
const CODEPOINTTRIE_SHIFT_1: u32 = 5 + CODEPOINTTRIE_SHIFT_2;

/// Difference between two shift sizes,
///  for getting an index-2 offset from an index-3 offset. 5=9-4
const CODEPOINTTRIE_SHIFT_2_3: u32 = CODEPOINTTRIE_SHIFT_2 - CODEPOINTTRIE_SHIFT_3;

/// Difference between two shift sizes,
/// for getting an index-1 offset from an index-2 offset. 5=14-9
const CODEPOINTTRIE_SHIFT_1_2: u32 = CODEPOINTTRIE_SHIFT_1 - CODEPOINTTRIE_SHIFT_2;

/// Number of index-1 entries for the BMP. (4)
/// This part of the index-1 table is omitted from the serialized form.
const CODEPOINTTRIE_OMITTED_BMP_INDEX_1_LENGTH: u32 = 0x10000 >> CODEPOINTTRIE_SHIFT_1;

/// Number of entries in an index-2 block. 32=0x20
const CODEPOINTTRIE_INDEX_2_BLOCK_LENGTH: u32 = 1 << CODEPOINTTRIE_SHIFT_2;

/// Mask for getting the lower bits for the in-index-2-block offset.
const CODEPOINTTRIE_INDEX_2_MASK: u32 = CODEPOINTTRIE_INDEX_2_BLOCK_LENGTH - 1;

/// Number of code points per index-2 table entry. 512=0x200
const CODEPOINTTRIE_CP_PER_INDEX_2_ENTRY: u32 = 1 << CODEPOINTTRIE_SHIFT_2;

/// Number of entries in an index-3 block. 32=0x20
const CODEPOINTTRIE_INDEX_3_BLOCK_LENGTH: u32 = 1 << CODEPOINTTRIE_SHIFT_2_3;

/// Mask for getting the lower bits for the in-index-3-block offset.
const CODEPOINTTRIE_INDEX_3_MASK: u32 = CODEPOINTTRIE_INDEX_3_BLOCK_LENGTH - 1;

/// Number of entries in a small data block. 16=0x10
const CODEPOINTTRIE_SMALL_DATA_BLOCK_LENGTH: u32 = 1 << CODEPOINTTRIE_SHIFT_3;

/// Mask for getting the lower bits for the in-small-data-block offset.
const CODEPOINTTRIE_SMALL_DATA_MASK: u32 = CODEPOINTTRIE_SMALL_DATA_BLOCK_LENGTH - 1;




// trait+impl fn polymorphism testing code

struct MyStruct<T> {
    x: T,
}

fn trait_impl_polymorphism_scratch_code() {
    let xyz = MyStruct { x: 42 };

    let xyz = MyStruct::<CodePointTrieValueWidth> { x: CodePointTrieValueWidth::Bits8 };
}

trait CodePointTrieReader<'t> {
    fn get_trie_type(trie_type_int: u8) -> CodePointTrieType;
    fn get_value_width(value_width_int: u8) -> CodePointTrieValueWidth;
    fn internal_small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32;
    fn small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32;
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

struct CodePointTrie<'trie, CodePointTrieType, CodePointTrieValueWidth> {
    index_length: u32,
    data_length: u32,
    high_start: u32,
    shifted12_high_start: u16,
    trie_type: CodePointTrieType,

    // can't do this because:
    //
    // the size for values of type `(dyn TrieType + 'static)` cannot be known at compilation time
    // doesn't have a size known at compile-time
    // help: the trait `std::marker::Sized` is not implemented for `(dyn TrieType + 'static)`
    //
    // trie_type2: TrieType,

    value_width: CodePointTrieValueWidth,
    index3_null_offset: u16,
    data_null_offset: u32,
    null_value: u32,
    index: &'trie [u16],
    data: &'trie CodePointTrieData<'trie>,
}

// can't do this because:
//
// return type cannot have an unboxed trait object
// doesn't have a size known at compile-time
//
// fn get_trie_type(trie_type_int: u8) -> TrieType {
//     match trie_type_int {
//         0 => FastType,
//         1 => SmallType,
//         _ => AnyType,
//     }
// }


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
        _ => CodePointTrieValueWidth::BitsAny,
    }
}

fn trie_internal_small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    let mut i1: u32 = c >> CODEPOINTTRIE_SHIFT_1;
    if trie.trie_type == CodePointTrieType::Fast {
        assert!(0xffff < c && c < trie.high_start);
        i1 = i1 + CODEPOINTTRIE_BMP_INDEX_LENGTH - CODEPOINTTRIE_OMITTED_BMP_INDEX_1_LENGTH;
    } else {
        assert!(c < trie.high_start && trie.high_start > CODEPOINTTRIE_SMALL_LIMIT);
        i1 = i1 + CODEPOINTTRIE_SMALL_INDEX_LENGTH;
    }
    let mut i3_block: u32 = trie.index[(trie.index[i1 as usize] as u32
        + ((c >> CODEPOINTTRIE_SHIFT_2) & CODEPOINTTRIE_INDEX_2_MASK))
        as usize] as u32;
    let mut i3: u32 = (c >> CODEPOINTTRIE_SHIFT_3) & CODEPOINTTRIE_INDEX_3_MASK;
    let mut data_block: u32;
    if i3_block & 0x8000 == 0 {
        // 16-bit indexes
        data_block = trie.index[(i3_block + i3) as usize] as u32;
    } else {
        // 18-bit indexes stored in groups of 9 entries per 8 indexes.
        i3_block = (i3_block & 0x7fff) + (i3 & !7) + (i3 >> 3);
        i3 = i3 & 7;
        data_block =
            ((trie.index[(i3_block + 1) as usize] << (2 + (2 * i3))) as u32 & 0x30000) as u32;
        data_block = data_block | trie.index[(i3_block + i3) as usize] as u32;
    }
    data_block + (c & CODEPOINTTRIE_SMALL_DATA_MASK)
}

/// Internal trie getter for a code point at or above the fast limit. Returns the data index.
fn trie_small_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    if c >= trie.high_start {
        trie.data_length - CODEPOINTTRIE_HIGH_VALUE_NEG_DATA_OFFSET
    } else {
        trie_internal_small_index(trie, c)
    }
}

/// Internal trie getter for a code point below the fast limit. Returns the data index.
fn trie_fast_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    let index_array_pos: u32 =
        (c >> CODEPOINTTRIE_FAST_TYPE_SHIFT) + (c & CODEPOINTTRIE_FAST_TYPE_DATA_MASK);
    trie.index[index_array_pos as usize] as u32
}

/// Internal trie getter to get trie data array index position for code point
/// value `c` that is beyond ASCII range. Also checks that c is in
/// U+0000..10FFFF.
fn trie_cp_index(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    if c < 0 {
        trie.data_length - CODEPOINTTRIE_ERROR_VALUE_NEG_DATA_OFFSET
    } else if c <= 0xffff {
        trie_fast_index(trie, c)
    } else if c <= 0x10ffff {
        trie_small_index(trie, c)
    } else {
        trie.data_length - CODEPOINTTRIE_ERROR_VALUE_NEG_DATA_OFFSET
    }
}

/// Helper function that gets the data array value at the provided index
fn trie_get_value(
    data: &CodePointTrieData,
    value_width: &CodePointTrieValueWidth,
    data_index: u32,
) -> u32 {
    let return_val_opt: Option<u32> = match value_width {
        &CodePointTrieValueWidth::Bits16 => match data.data_16_bit {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        &CodePointTrieValueWidth::Bits32 => match data.data_32_bit {
            Some(data_array) => Some(data_array[data_index as usize]),
            _ => None,
        },
        &CodePointTrieValueWidth::Bits8 => match data.data_8_bit {
            Some(data_array) => Some(data_array[data_index as usize] as u32),
            _ => None,
        },
        _ => None, // Unreachable if the trie is properly initialized.
    };
    return_val_opt.unwrap_or(0xffffffff)
}

fn trie_get(trie: &CodePointTrie<CodePointTrieType, CodePointTrieValueWidth>, c: u32) -> u32 {
    let data_index: u32 = trie_cp_index(trie, c);
    let data_value: u32 = trie_get_value(&trie.data, &trie.value_width, data_index);
    data_value
}

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

    let trie: CodePointTrie<CodePointTrieType, CodePointTrieValueWidth> =  CodePointTrie {
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
        data: &CodePointTrieData {
            data_8_bit: Some(&data_8),
            data_16_bit: None,
            data_32_bit: None,
        },
    };

    assert_eq!(
        0,
        trie_cp_index(&trie, 0),
        "trie_cp_index(&trie, 0)"
    );
    assert_eq!(
        64,
        trie_cp_index(&trie, 1),
        "trie_cp_index(&trie, 1)"
    );
    assert_eq!(
        0,
        trie_cp_index(&trie, 2),
        "trie_cp_index(&trie, 2)"
    );
    assert_eq!(
        0,
        trie_cp_index(&trie, 127),
        "trie_cp_index(&trie, 127)"
    );

    assert_eq!(trie_cp_index(&trie, 999), 0);

    // TODO: add more test cases for index values

    // TODO: add impl & tests for data-getting fns using the index fns

    let _check_ranges: [u32; 10] = [0, 1, 0x740, 1, 0x780, 2, 0x880, 3, 0x110000, 1];
}
