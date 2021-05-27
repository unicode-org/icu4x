// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ZeroVec;

pub mod impl_const {

    pub const FAST_TYPE_SHIFT: i32 = 6;

    /// Number of entries in a data block for code points below the fast limit. 64=0x40
    pub const FAST_TYPE_DATA_BLOCK_LENGTH: u32 = 1 << FAST_TYPE_SHIFT;

    /// Mask for getting the lower bits for the in-fast-data-block offset.
    pub const FAST_TYPE_DATA_MASK: u32 = FAST_TYPE_DATA_BLOCK_LENGTH - 1;

    // Fast indexing limit for "fast"-type trie
    pub const FAST_TYPE_FAST_INDEXING_MAX: u32 = 0xffff;

    // Fast indexing limit for "small"-type trie
    pub const SMALL_TYPE_FAST_INDEXING_MAX: u32 = 0xfff;

    /// Offset from dataLength (to be subtracted) for fetching the
    /// value returned for out-of-range code points and ill-formed UTF-8/16.
    pub const ERROR_VALUE_NEG_DATA_OFFSET: u32 = 1;

    /// Offset from dataLength (to be subtracted) for fetching the
    /// value returned for code points highStart..U+10FFFF.
    pub const HIGH_VALUE_NEG_DATA_OFFSET: u32 = 2;

    /// The length of the BMP index table. 1024=0x400
    pub const BMP_INDEX_LENGTH: u32 = 0x10000 >> FAST_TYPE_SHIFT;

    pub const SMALL_LIMIT: u32 = 0x10000;

    pub const SMALL_INDEX_LENGTH: u32 = SMALL_LIMIT >> FAST_TYPE_SHIFT;

    /// Shift size for getting the index-3 table offset.
    pub const SHIFT_3: u32 = 4;

    /// Shift size for getting the index-2 table offset.
    pub const SHIFT_2: u32 = 5 + SHIFT_3;

    /// Shift size for getting the index-1 table offset.
    pub const SHIFT_1: u32 = 5 + SHIFT_2;

    /// Difference between two shift sizes,
    ///  for getting an index-2 offset from an index-3 offset. 5=9-4
    pub const SHIFT_2_3: u32 = SHIFT_2 - SHIFT_3;

    /// Difference between two shift sizes,
    /// for getting an index-1 offset from an index-2 offset. 5=14-9
    pub const SHIFT_1_2: u32 = SHIFT_1 - SHIFT_2;

    /// Number of index-1 entries for the BMP. (4)
    /// This part of the index-1 table is omitted from the serialized form.
    pub const OMITTED_BMP_INDEX_1_LENGTH: u32 = 0x10000 >> SHIFT_1;

    /// Number of entries in an index-2 block. 32=0x20
    pub const INDEX_2_BLOCK_LENGTH: u32 = 1 << SHIFT_2;

    /// Mask for getting the lower bits for the in-index-2-block offset.
    pub const INDEX_2_MASK: u32 = INDEX_2_BLOCK_LENGTH - 1;

    /// Number of code points per index-2 table entry. 512=0x200
    pub const CP_PER_INDEX_2_ENTRY: u32 = 1 << SHIFT_2;

    /// Number of entries in an index-3 block. 32=0x20
    pub const INDEX_3_BLOCK_LENGTH: u32 = 1 << SHIFT_2_3;

    /// Mask for getting the lower bits for the in-index-3-block offset.
    pub const INDEX_3_MASK: u32 = INDEX_3_BLOCK_LENGTH - 1;

    /// Number of entries in a small data block. 16=0x10
    pub const SMALL_DATA_BLOCK_LENGTH: u32 = 1 << SHIFT_3;

    /// Mask for getting the lower bits for the in-small-data-block offset.
    pub const SMALL_DATA_MASK: u32 = SMALL_DATA_BLOCK_LENGTH - 1;

    pub const CODE_POINT_MAX: u32 = 0x10ffff;
}

// Enums

/// The width of the elements in the data array of a CodePointTrie.
/// See UCPTrieValueWidth in ICU4C.
#[derive(Clone, Copy)]
pub enum ValueWidthEnum {
    BitsAny = -1,
    Bits16 = 0,
    Bits32 = 1,
    Bits8 = 2,
}

/// The type of trie represents whether the trie has an optimization that
/// would make it small or fast.
/// See UCPTrieType in ICU4C.
#[derive(Clone, Copy, PartialEq)]
pub enum TrieTypeEnum {
    Any = -1,
    Fast = 0,
    Small = 1,
}

// ValueWidth trait

pub trait ValueWidth: Copy + zerovec::ule::AsULE { // AsUnalignedLittleEndian -> "allowed in a zerovec"
    const ENUM_VALUE: ValueWidthEnum;
    fn cast_to_widest(self) -> u32;
}

impl ValueWidth for u8 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits8;
    
    fn cast_to_widest(self) -> u32 { self as u32 }
}

impl ValueWidth for u16 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits16;
    
    fn cast_to_widest(self) -> u32 { self as u32 }
}

impl ValueWidth for u32 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits32;
    
    fn cast_to_widest(self) -> u32 { self }
}

// TrieType trait

pub trait TrieType {
    const FAST_MAX: u32;
    const ENUM_VALUE: TrieTypeEnum;
}

struct Fast;
impl TrieType for Fast {
    const FAST_MAX: u32 = 0xffff;
    const ENUM_VALUE: TrieTypeEnum = TrieTypeEnum::Fast;
}

struct Small;
impl TrieType for Small {
    const FAST_MAX: u32 = 0x0fff;
    const ENUM_VALUE: TrieTypeEnum = TrieTypeEnum::Small;
}


pub struct CodePointTrie<'trie, W: ValueWidth, T: TrieType> {
    pub index_length: u32,
    pub data_length: u32,
    pub high_start: u32,
    pub shifted12_high_start: u16,
    // pub trie_type: TrieTypeEnum,
    pub index3_null_offset: u16,
    pub data_null_offset: u32,
    pub null_value: u32,
    pub index: ZeroVec<'trie, u16>,
    pub data: ZeroVec<'trie, W>,
}

pub fn get_code_point_trie_type(trie_type_int: u8) -> TrieTypeEnum {
    match trie_type_int {
        0 => TrieTypeEnum::Fast,
        1 => TrieTypeEnum::Small,
        _ => TrieTypeEnum::Any,
    }
}

pub fn get_code_point_trie_value_width(value_width_int: u8) -> ValueWidthEnum {
    match value_width_int {
        0 => ValueWidthEnum::Bits16,
        1 => ValueWidthEnum::Bits32,
        2 => ValueWidthEnum::Bits8,
        _ => ValueWidthEnum::BitsAny,
    }
}

// CPTAuto will only expose get_u32() (probably will call it `get()`), but will not / cannot expose get() without using an enum for W
// CPT<W1,S1> will expose get() and get_u32(). You may not want get_u32() at this point, but it is necessary in order to implement CPTAuto

// u32 vs. char in Rust - unpaired surrogates are valid code points but are not UTF-32 code units, therefore not allowed in `char` type in Rust.
// But for the CodePointTrie, we do want to be able to look up surrogate code points, therefore we want to use the u32 type. (Note: this is also what we do in the `uniset` crate for `UnicodeSet`.)
// struct CodePoint(u32) ← enforce whatever invariants you'd like

use impl_const::*;

impl<'trie, W: ValueWidth, T: TrieType> CodePointTrie<'trie, W, T> {

    pub fn trie_internal_small_index(
        &self,
        c: u32,
    ) -> u32 {
        let mut i1: u32 = c >> SHIFT_1;
        if self.trie_type() == TrieTypeEnum::Fast {
            assert!(0xffff < c && c < self.high_start());
            i1 = i1 + BMP_INDEX_LENGTH - OMITTED_BMP_INDEX_1_LENGTH;
        } else {
            assert!(c < self.high_start() && self.high_start() > SMALL_LIMIT);
            i1 = i1 + SMALL_INDEX_LENGTH;
        }
        let mut i3_block: u32 = self.index().get(
            (self.index().get(i1 as usize).unwrap() as u32 + ((c >> SHIFT_2) & INDEX_2_MASK)) as usize).unwrap()
            as u32;
        let mut i3: u32 = (c >> SHIFT_3) & INDEX_3_MASK;
        let mut data_block: u32;
        if i3_block & 0x8000 == 0 {
            // 16-bit indexes
            data_block = self.index().get((i3_block + i3) as usize).unwrap() as u32;
        } else {
            // 18-bit indexes stored in groups of 9 enselfs per 8 indexes.
            i3_block = (i3_block & 0x7fff) + (i3 & !7) + (i3 >> 3);
            i3 = i3 & 7;
            data_block =
                ((self.index().get((i3_block + 1) as usize).unwrap() << (2 + (2 * i3))) as u32 & 0x30000) as u32;
            data_block = data_block | self.index().get((i3_block + i3) as usize).unwrap() as u32;
        }
        data_block + (c & SMALL_DATA_MASK)
    }
    

    /// Internal trie getter for a code point at or above the fast limit. Returns the data index.
    pub fn trie_small_index(
        &self,
        c: u32,
    ) -> u32 {
        if c >= self.high_start() {
            self.data_length() - HIGH_VALUE_NEG_DATA_OFFSET
        } else {
            trie_internal_small_index(self, c)
        }
    }

    /// Internal trie getter for a code point below the fast limit. Returns the data index.
    pub fn trie_fast_index(
        &self,
        c: u32,
    ) -> u32 {
        let index_array_pos: u32 = c >> FAST_TYPE_SHIFT;
        let index_array_val: u16 = self.index().get(index_array_pos as usize).unwrap(); // 1. How to specify type parameter for .index() 2. How to avoid unwrap()?
        let fast_index_val: u32 = index_array_val as u32 + (c & FAST_TYPE_DATA_MASK);
        fast_index_val
    }

    pub fn get(&self, c: u32) -> W {
        let index: u32 = if c <= T::FAST_MAX {
            trie_fast_index(self, c)
        } else if c <= CODE_POINT_MAX {
            trie_small_index(self, c)
        } else {
            self.data_length() - ERROR_VALUE_NEG_DATA_OFFSET
        };
        self.data().get(index as usize).unwrap()  // need the unwrap because the default value is stored in the data array,
                                                  // and getting that default value always returns an Option<W>, but need to return W
    }

    pub fn get_u32(&self, c: u32) -> u32 {  // this is the consistent API that is public-facing for users
        self.get(c).cast_to_widest()
    }

    pub fn index(&self) -> ZeroVec<'trie, u16> {
        self.index
    }

    pub fn data(&self) -> ZeroVec<'trie, W> {
        self.data
    }

    pub fn trie_type(&self) -> TrieTypeEnum {
        T::ENUM_VALUE
    }

    pub fn value_width(&self) -> ValueWidthEnum {
        W::ENUM_VALUE
    }

    pub fn high_start(&self) -> u32 {
        self.high_start
    }

    pub fn data_length(&self) -> u32 {
        self.data_length
    }
}

