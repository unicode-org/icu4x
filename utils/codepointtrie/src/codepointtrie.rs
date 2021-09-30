// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::impl_const::*;

use core::fmt::Display;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use zerovec::ZeroVec;
use zerovec::ule::{AsULE, ULE};

// Enums

/// The width of the elements in the data array of a [`CodePointTrie`].
/// See [`UCPTrieValueWidth`](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/ucptrie_8h.html) in ICU4C.
#[derive(Clone, Copy, PartialEq)]
pub enum ValueWidthEnum {
    Bits16 = 0,
    Bits32 = 1,
    Bits8 = 2,
}

/// The type of trie represents whether the trie has an optimization that
/// would make it small or fast.
/// See [`UCPTrieType`](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/ucptrie_8h.html) in ICU4C.
#[derive(Clone, Copy, PartialEq)]
pub enum TrieTypeEnum {
    Fast = 0,
    Small = 1,
}

// ValueWidth trait

// AsULE is AsUnalignedLittleEndian, i.e. "allowed in a zerovec"

/// A trait representing the width of the values stored in the data array of a
/// [`CodePointTrie`]. This trait is used as a type parameter in constructing
/// a `CodePointTrie`.
pub trait ValueWidth: Copy + zerovec::ule::AsULE {
    /// This enum variant represents the specific instance of `ValueWidth` such
    /// that the enum discriminant values matches ICU4C's enum integer value.
    const ENUM_VALUE: ValueWidthEnum;
    /// This value is used to indicate an error in the Rust code in accessing
    /// a position in the trie's `data` array. In normal cases, the position in
    /// the `data` array will return either the correct value, or in case of a
    /// logical error in the trie's computation, the trie's own error value
    /// which is stored that in the `data` array.
    const DATA_GET_ERROR_VALUE: Self;
    fn cast_to_widest(self) -> u32;
}

impl ValueWidth for u8 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits8;
    const DATA_GET_ERROR_VALUE: u8 = u8::MAX;

    fn cast_to_widest(self) -> u32 {
        self as u32
    }
}

impl ValueWidth for u16 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits16;
    const DATA_GET_ERROR_VALUE: u16 = u16::MAX;

    fn cast_to_widest(self) -> u32 {
        self as u32
    }
}

impl ValueWidth for u32 {
    const ENUM_VALUE: ValueWidthEnum = ValueWidthEnum::Bits32;
    const DATA_GET_ERROR_VALUE: u32 = u32::MAX;

    fn cast_to_widest(self) -> u32 {
        self
    }
}

// TrieType trait

/// A trait representing the "trie type" of a [`CodePointTrie`].
///
/// Currently, the options are "fast" and "small", which differ in the "fast max"
/// limit.
pub trait TrieType {
    /// All code points up to the fast max limit are represented
    /// individually in the `index` array to hold their `data` array position, and
    /// thus only need 2 lookups for a [CodePointTrie::get()](`crate::codepointtrie::CodePointTrie::get`).
    /// Code points above the "fast max" limit require 4 lookups.
    const FAST_MAX: u32;
    /// This enum variant represents the specific instance of `TrieType` such
    /// that the enum discriminant values matches ICU4C's enum integer value.
    const ENUM_VALUE: TrieTypeEnum;
}

/// An empty struct to represent "fast" type code point tries for the
///  [`TrieType`] trait. The "fast max" limit is set to `0xffff`.
pub struct Fast;

impl TrieType for Fast {
    const FAST_MAX: u32 = FAST_TYPE_FAST_INDEXING_MAX;
    const ENUM_VALUE: TrieTypeEnum = TrieTypeEnum::Fast;
}

/// An empty struct to represent "small" type code point tries for the
///  [`TrieType`] trait. The "fast max" limit is set to `0x0fff`.
pub struct Small;

impl TrieType for Small {
    const FAST_MAX: u32 = SMALL_TYPE_FAST_INDEXING_MAX;
    const ENUM_VALUE: TrieTypeEnum = TrieTypeEnum::Small;
}

/// This struct represents a de-serialized CodePointTrie that was exported from
/// ICU binary data.
///
/// For more information:
/// - [ICU Site design doc](http://site.icu-project.org/design/struct/utrie)
/// - [ICU User Guide section on Properties lookup](https://unicode-org.github.io/icu/userguide/strings/properties.html#lookup)
pub struct CodePointTrie<'trie, W: ValueWidth>
{
    header: CodePointTrieHeader,
    index: ZeroVec<'trie, u16>,
    data: ZeroVec<'trie, W>,
}

/// This struct contains the fixed-length header fields of a [`CodePointTrie`].
pub struct CodePointTrieHeader {
    /// Length of the trie's `index` array
    pub index_length: u32,
    /// Length of the trie's `data` array
    pub data_length: u32,
    /// The code point of the start of the last range of the trie. A
    /// range is defined as a partition of the code point space such that the
    /// value in this trie associated with all code points of the same range is
    /// the same.
    ///
    /// For the property value data for many Unicode properties,
    /// often times, `high_start` is `U+10000` or lower. In such cases, not
    /// reserving space in the `index` array for duplicate values is a large
    /// savings. The "highValue" associated with the `high_start` range is
    /// stored at the second-to-last position of the `data` array.
    /// (See `impl_const::HIGH_VALUE_NEG_DATA_OFFSET`.)
    pub high_start: u32,
    /// A version of the `high_start` value that is right-shifted 12 spaces,
    /// but is rounded up to a multiple `0x1000` for easy testing from UTF-8
    /// lead bytes.
    pub shifted12_high_start: u16,
    /// Offset for the null block in the "index-3" table of the `index` array.
    /// Set to an impossibly high value (e.g., `0xffff`) if there is no
    /// dedicated index-3 null block.
    pub index3_null_offset: u16,
    /// Internal data null block offset, not shifted.
    /// Set to an impossibly high value (e.g., `0xfffff`) if there is no
    /// dedicated data null block.
    pub data_null_offset: u32,
    /// The value stored in the trie that represents a null value being
    /// associated to a code point.
    pub null_value: u32,
    /// The enum value representing the type of trie, where trie type is as it
    /// is defined in ICU (ex: Fast, Small).
    pub trie_type: TrieTypeEnum,
}


#[cfg(feature = "serde")]
impl<'de: 'a, 'a, W: ValueWidth> serde::Serialize for CodePointTrie<'a, W> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CodePointTrie", 11)?;
        state.serialize_field("index", &self.index)?;
        match W::ENUM_VALUE {
            ValueWidthEnum::Bits8 => {
                state.serialize_field("data_8", &self.data)?;
            },
            ValueWidthEnum::Bits16 => {
                state.serialize_field("data_16", &self.data)?;
            },
            ValueWidthEnum::Bits32 => {
                state.serialize_field("data_32", &self.data)?;
            },
        };
        state.serialize_field("indexLength", &self.header.index_length)?;
        state.serialize_field("dataLength", &self.header.data_length)?;
        state.serialize_field("highStart", &self.header.high_start)?;
        state.serialize_field("shifted12HighStart", &self.header.shifted12_high_start)?;
        state.serialize_field("type", &self.header.trie_type as u8)?;
        state.serialize_field("valueWidth", W::ENUM_VALUE as u8)?;
        state.serialize_field("index3NullOffset", &self.header.index3_null_offset)?;
        state.serialize_field("dataNullOffset", &self.header.data_null_offset)?;
        state.serialize_field("nullValue", &self.header.null_value)?;
        state.end()
    }
}

/// Converts the serialized `u8` value for the trie type into a [`TrieTypeEnum`].
pub fn get_code_point_trie_type_enum(trie_type_int: u8) -> Option<TrieTypeEnum> {
    match trie_type_int {
        0 => Some(TrieTypeEnum::Fast),
        1 => Some(TrieTypeEnum::Small),
        _ => None,
    }
}

#[cfg(feature = "serde")]
impl<'de: 'a, 'a, W: ValueWidth> serde::Deserialize<'de> for CodePointTrie<'a, W> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::fmt;

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Index,
            Data,
            IndexLength,
            DataLength,
            HighStart,
            Shifted12HighStart,
            TrieType,
            ValueWidth,
            Index3NullOffset,
            DataNullOffset,
            NullValue,
        }

        struct CodePointTrieVisitor;

        impl<'de> serde::de::Visitor<'de> for CodePointTrieVisitor {
            type Value = CodePointTrie<W>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid CodePointTrie struct")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let index = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let data = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let index_length = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let data_length = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                let high_start = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(4, &self))?;
                let shifted12_high_start = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(5, &self))?;
                let trie_type: TrieTypeEnum = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(6, &self))?;
                let value_width = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(7, &self))?;
                let index3_null_offset = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(8, &self))?;
                let data_null_offset = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(9, &self))?;
                let null_value = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(10, &self))?;
                
                let header = CodePointTrieHeader {
                    index_length,
                    data_length,
                    high_start,
                    shifted12_high_start,
                    index3_null_offset,
                    data_null_offset,
                    null_value,
                    trie_type,
                };
                Ok(CodePointTrie { header, index, data })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut inv_list = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        },
                        Field::InvList => {
                            if inv_list.is_some() {
                                return Err(serde::de::Error::duplicate_field("inv_list"));
                            }
                            inv_list = Some(map.next_value()?);
                        },
                    }
                }
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let inv_list =
                    inv_list.ok_or_else(|| serde::de::Error::missing_field("inv_list"))?;
                Ok(CodePointTrie { name, inv_list })
            }
        }

        const FIELDS: &[&str] = &["index", "data", "indexLength", "dataLength", "highStart",
            "shifted12HighStart", "trieType", "valueWidth", "index3NullOffset", "dataNullOffset",
            "nullValue"];
        deserializer.deserialize_struct("CodePointTrie", FIELDS, CodePointTrieVisitor)
    }
}

impl<'trie, W: ValueWidth> CodePointTrie<'trie, W>
{
    /// Returns a new [`CodePointTrie`] backed by borrowed data for the `index`
    /// array and `data` array, whose data values have width `W`.
    pub fn try_new(
        header: CodePointTrieHeader,
        index: ZeroVec<'trie, u16>,
        data: ZeroVec<'trie, W>,
    ) -> Result<CodePointTrie<'trie, W>, Error> {
        if header.data_length < ERROR_VALUE_NEG_DATA_OFFSET {
            return Err(Error::FromDeserialized {
                reason: "Data array must be large enough to contain error value",
            });
        }

        if header.data_length < HIGH_VALUE_NEG_DATA_OFFSET {
            return Err(Error::FromDeserialized {
                reason:
                    "Data array must be large enough to contain value for range highStart..U+10FFFF",
            });
        }

        if index.len() as u32 != header.index_length {
            return Err(Error::FromDeserialized {
                reason: "Length of index array does not match corresponding header value",
            });
        }

        if data.len() as u32 != header.data_length {
            return Err(Error::FromDeserialized {
                reason: "Length of data array does not match corresponding header value",
            });
        }

        let trie: CodePointTrie<'trie, W> = CodePointTrie {
            header,
            index,
            data,
        };
        Ok(trie)
    }

    /// Returns the position in the data array containing the trie's stored
    /// error value.
    fn trie_error_val_index(&self) -> u32 {
        self.header.data_length - ERROR_VALUE_NEG_DATA_OFFSET
    }

    fn internal_small_index(&self, code_point: u32) -> u32 {
        let mut index1_pos: u32 = code_point >> SHIFT_1;
        if self.header.trie_type == TrieTypeEnum::Fast {
            debug_assert!(
                FAST_TYPE_FAST_INDEXING_MAX < code_point && code_point < self.header.high_start
            );
            index1_pos = index1_pos + BMP_INDEX_LENGTH - OMITTED_BMP_INDEX_1_LENGTH;
        } else {
            assert!(code_point < self.header.high_start && self.header.high_start > SMALL_LIMIT);
            index1_pos += SMALL_INDEX_LENGTH;
        }
        let index1_val = if let Some(index1_val) = self.index.get(index1_pos as usize) {
            index1_val
        } else {
            return self.trie_error_val_index();
        };
        let index3_block_idx: u32 = (index1_val as u32) + ((code_point >> SHIFT_2) & INDEX_2_MASK);
        let mut index3_block: u32 =
            if let Some(index3_block) = self.index.get(index3_block_idx as usize) {
                index3_block as u32
            } else {
                return self.trie_error_val_index();
            };
        let mut index3_pos: u32 = (code_point >> SHIFT_3) & INDEX_3_MASK;
        let mut data_block: u32;
        if index3_block & 0x8000 == 0 {
            // 16-bit indexes
            data_block =
                if let Some(data_block) = self.index.get((index3_block + index3_pos) as usize) {
                    data_block as u32
                } else {
                    return self.trie_error_val_index();
                };
        } else {
            // 18-bit indexes stored in groups of 9 entries per 8 indexes.
            index3_block = (index3_block & 0x7fff) + (index3_pos & !7) + (index3_pos >> 3);
            index3_pos &= 7;
            data_block = if let Some(data_block) = self.index.get(index3_block as usize) {
                data_block as u32
            } else {
                return self.trie_error_val_index();
            };
            data_block = ((data_block << (2 + (2 * index3_pos))) as u32 & 0x30000) as u32;
            index3_block += 1;
            data_block =
                if let Some(index3_val) = self.index.get((index3_block + index3_pos) as usize) {
                    data_block | (index3_val as u32)
                } else {
                    return self.trie_error_val_index();
                };
        }
        // Returns data_pos == data_block (offset) +
        //     portion of code_point bit field for last (4th) lookup
        data_block + (code_point & SMALL_DATA_MASK)
    }

    /// Returns the position in the `data` array for the given code point,
    /// where this code point is at or above the fast limit associated for the
    /// `trie_type`. We will refer to that limit as "`fastMax`" here.
    ///
    /// A lookup of the value in the code point trie for a code point in the
    /// code point space range [`fastMax`, `high_start`) will be a 4-step
    /// lookup: 3 lookups in the `index` array and one lookup in the `data`
    /// array. Lookups for code points in the range [`high_start`,
    /// `CODE_POINT_MAX`] are short-circuited to be a single lookup, see
    /// [CodePointTrieHeader::high_start].
    fn small_index(&self, code_point: u32) -> u32 {
        if code_point >= self.header.high_start {
            self.header.data_length - HIGH_VALUE_NEG_DATA_OFFSET
        } else {
            self.internal_small_index(code_point) // helper fn
        }
    }

    /// Returns the position in the `data` array for the given code point,
    /// where this code point is below the fast limit associated for the
    /// `trie type`. We will refer to that limit as "`fastMax`" here.
    ///
    /// A lookup of the value in the code point trie for a code point in the
    /// code point space range [0, `fastMax`) will be a 2-step lookup: 1
    /// lookup in the `index` array and one lookup in the `data` array. By
    /// design, for trie type `T`, there is an element allocated in the `index`
    /// array for each block of code points in [0, `fastMax`), which in
    /// turn guarantees that those code points are represented and only need 1
    /// lookup.
    fn fast_index(&self, code_point: u32) -> u32 {
        let index_array_pos: u32 = code_point >> FAST_TYPE_SHIFT;
        let index_array_val: u16 =
            if let Some(index_array_val) = self.index.get(index_array_pos as usize) {
                index_array_val
            } else {
                return self.trie_error_val_index();
            };
        let fast_index_val: u32 = index_array_val as u32 + (code_point & FAST_TYPE_DATA_MASK);
        fast_index_val
    }

    /// Returns the value that is associated with `code_point` in this [`CodePointTrie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_codepointtrie::planes;
    /// let trie = planes::get_planes_trie();
    ///
    /// assert_eq!(0, trie.get(0x41));  // 'A' as u32
    /// assert_eq!(0, trie.get(0x13E0));  // 'Ꮰ' as u32
    /// assert_eq!(1, trie.get(0x10044));  // '𐁄' as u32
    /// ```
    pub fn get(&self, code_point: u32) -> W {
        let fast_max = match self.header.trie_type {
            TrieTypeEnum::Fast => FAST_TYPE_FAST_INDEXING_MAX,
            TrieTypeEnum::Small => SMALL_TYPE_FAST_INDEXING_MAX,
        };
        let data_pos: u32 = if code_point <= fast_max {
            Self::fast_index(self, code_point)
        } else if code_point <= CODE_POINT_MAX {
            Self::small_index(self, code_point)
        } else {
            self.trie_error_val_index()
        };
        // Returns the trie value (or trie's error value).
        // If we cannot read from the data array, then return the associated constant
        // DATA_GET_ERROR_VALUE for the instance type for W: ValueWidth.
        self.data
            .get(data_pos as usize)
            .unwrap_or(W::DATA_GET_ERROR_VALUE)
    }

    /// Returns the value that is associated with `code_point` for this [`CodePointTrie`]
    /// as a `u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_codepointtrie::planes;
    /// let trie = planes::get_planes_trie();
    ///
    /// let cp = '𑖎' as u32;
    /// assert_eq!(cp, 0x1158E);
    /// let trie = planes::get_planes_trie();
    /// let plane_num: u8 = trie.get(cp);
    /// assert_eq!(trie.get_u32(cp), plane_num as u32);
    /// ```
    ///
    // Note: This API method maintains consistency with the corresponding
    // original ICU APIs.
    pub fn get_u32(&self, code_point: u32) -> u32 {
        self.get(code_point).cast_to_widest()
    }
}
