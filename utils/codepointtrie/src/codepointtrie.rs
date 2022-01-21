// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::impl_const::*;

use core::convert::TryFrom;
use core::fmt::Display;
use core::num::TryFromIntError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use yoke::{Yokeable, ZeroCopyFrom};
use zerovec::ZeroVec;
use zerovec::ZeroVecError;

/// The type of trie represents whether the trie has an optimization that
/// would make it small or fast.
/// See [`UCPTrieType`](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4c/ucptrie_8h.html) in ICU4C.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TrieType {
    /// Represents the "fast" type code point tries for the
    /// [`TrieType`] trait. The "fast max" limit is set to `0xffff`.
    Fast = 0,
    /// Represents the "small" type code point tries for the
    /// [`TrieType`] trait. The "fast max" limit is set to `0x0fff`.
    Small = 1,
}

// TrieValue trait

// AsULE is AsUnalignedLittleEndian, i.e. "allowed in a zerovec"

/// A trait representing the values stored in the data array of a [`CodePointTrie`].
/// This trait is used as a type parameter in constructing a `CodePointTrie`.
pub trait TrieValue: Copy + Eq + PartialEq + zerovec::ule::AsULE + 'static {
    /// Last-resort fallback value to return if we cannot read data from the trie.
    ///
    /// In most cases, the error value is read from the last element of the `data` array.
    const DATA_GET_ERROR_VALUE: Self;
    /// Error type when converting from a u32 to this TrieValue.
    type TryFromU32Error: Display;
    /// A parsing function that is primarily motivated by deserialization contexts.
    /// When the serialization type width is smaller than 32 bits, then it is expected
    /// that the call site will widen the value to a `u32` first.
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error>;
}

impl TrieValue for u8 {
    const DATA_GET_ERROR_VALUE: u8 = u8::MAX;
    type TryFromU32Error = TryFromIntError;
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        Self::try_from(i)
    }
}

impl TrieValue for u16 {
    const DATA_GET_ERROR_VALUE: u16 = u16::MAX;
    type TryFromU32Error = TryFromIntError;
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        Self::try_from(i)
    }
}

impl TrieValue for u32 {
    const DATA_GET_ERROR_VALUE: u32 = u32::MAX;
    type TryFromU32Error = TryFromIntError;
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        Ok(i)
    }
}

impl TrieValue for char {
    const DATA_GET_ERROR_VALUE: char = '\0';
    type TryFromU32Error = core::char::CharTryFromError;
    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        char::try_from(i)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct CodePointMapRange {
    start: u32,
    end: u32,
    value: u32,
}

impl CodePointMapRange {
    pub fn get_start(&self) -> u32 {
        self.start
    }

    pub fn get_end(&self) -> u32 {
        self.end
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

/// Helper function used by [`get_range`]. Converts occurrences of trie's null
/// value into the provided null_value.
///
/// Note: the ICU version of this helper function uses a ValueFilter function
/// to apply a transform on a non-null value. But currently, this implementation
/// stops short of that functionality, and instead leaves the non-null trie value
/// untouched. This is equivalent to having a ValueFilter function that is the
/// identity function.
fn maybe_filter_value(value: u32, trie_null_value: u32, null_value: u32) -> u32 {
    if value == trie_null_value {
        null_value
    } else {
        value
    }
}

/// This struct represents a de-serialized CodePointTrie that was exported from
/// ICU binary data.
///
/// For more information:
/// - [ICU Site design doc](http://site.icu-project.org/design/struct/utrie)
/// - [ICU User Guide section on Properties lookup](https://unicode-org.github.io/icu/userguide/strings/properties.html#lookup)
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
pub struct CodePointTrie<'trie, T: TrieValue> {
    header: CodePointTrieHeader,
    #[cfg_attr(feature = "serde", serde(borrow))]
    index: ZeroVec<'trie, u16>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    data: ZeroVec<'trie, T>,
}

/// This struct contains the fixed-length header fields of a [`CodePointTrie`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
pub struct CodePointTrieHeader {
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
    pub trie_type: TrieType,
}

impl TryFrom<u8> for TrieType {
    type Error = crate::error::Error;

    fn try_from(trie_type_int: u8) -> Result<TrieType, crate::error::Error> {
        match trie_type_int {
            0 => Ok(TrieType::Fast),
            1 => Ok(TrieType::Small),
            _ => Err(crate::error::Error::FromDeserialized {
                reason: "Cannot parse value for trie_type",
            }),
        }
    }
}

impl<'trie, T: TrieValue> CodePointTrie<'trie, T> {
    /// Returns a new [`CodePointTrie`] backed by borrowed data for the `index`
    /// array and `data` array, whose data values have width `W`.
    pub fn try_new(
        header: CodePointTrieHeader,
        index: ZeroVec<'trie, u16>,
        data: ZeroVec<'trie, T>,
    ) -> Result<CodePointTrie<'trie, T>, Error> {
        // Validation invariants are not needed here when constructing a new
        // `CodePointTrie` because:
        //
        // - Rust includes the size of a slice (or Vec or similar), which allows it
        //   to prevent lookups at out-of-bounds indices, whereas in C++, it is the
        //   programmer's responsibility to keep track of length info.
        // - For lookups into collections, Rust guarantees that a fallback value will
        //   be returned in the case of `.get()` encountering a lookup error, via
        //   the `Option` type.
        // - The `ZeroVec` serializer stores the length of the array along with the
        //   ZeroVec data, meaning that a deserializer would also see that length info.

        let trie: CodePointTrie<'trie, T> = CodePointTrie {
            header,
            index,
            data,
        };
        Ok(trie)
    }

    /// Returns the position in the data array containing the trie's stored
    /// error value.
    fn trie_error_val_index(&self) -> u32 {
        self.data.len() as u32 - ERROR_VALUE_NEG_DATA_OFFSET
    }

    fn internal_small_index(&self, code_point: u32) -> u32 {
        let mut index1_pos: u32 = code_point >> SHIFT_1;
        if self.header.trie_type == TrieType::Fast {
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
            self.data.len() as u32 - HIGH_VALUE_NEG_DATA_OFFSET
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
    pub fn get(&self, code_point: u32) -> T {
        self.get_ule(code_point)
            .map(|t| T::from_unaligned(*t))
            .unwrap_or(T::DATA_GET_ERROR_VALUE)
    }

    /// Returns a reference to the ULE of the value that is associated with `code_point` in this [`CodePointTrie`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_codepointtrie::planes;
    /// let trie = planes::get_planes_trie();
    ///
    /// assert_eq!(Some(&0), trie.get_ule(0x41));  // 'A' as u32
    /// assert_eq!(Some(&0), trie.get_ule(0x13E0));  // 'Ꮰ' as u32
    /// assert_eq!(Some(&1), trie.get_ule(0x10044));  // '𐁄' as u32
    /// ```
    pub fn get_ule(&self, code_point: u32) -> Option<&T::ULE> {
        // All code points up to the fast max limit are represented
        // individually in the `index` array to hold their `data` array position, and
        // thus only need 2 lookups for a [CodePointTrie::get()](`crate::codepointtrie::CodePointTrie::get`).
        // Code points above the "fast max" limit require 4 lookups.
        let fast_max = match self.header.trie_type {
            TrieType::Fast => FAST_TYPE_FAST_INDEXING_MAX,
            TrieType::Small => SMALL_TYPE_FAST_INDEXING_MAX,
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
        // DATA_GET_ERROR_VALUE for the instance type for T: TrieValue.
        self.data.as_ule_slice().get(data_pos as usize)
    }

    /// Converts the CodePointTrie into one that returns another type of the same size.
    ///
    /// Borrowed data remains borrowed, and owned data remains owned.
    ///
    /// # Panics
    ///
    /// Panics if `T` and `P` are different sizes.
    ///
    /// More specifically, panics if [ZeroVec::try_into_converted()] panics when converting
    /// `ZeroVec<T>` into `ZeroVec<P>`, which happens if `T::ULE` and `P::ULE` differ in size.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use icu_codepointtrie::CodePointTrie;
    ///
    /// let cpt1: CodePointTrie<char> = unimplemented!();
    /// let cpt2: CodePointTrie<u32> = cpt1.try_into_converted()
    ///     .expect("infallible");
    /// ```
    pub fn try_into_converted<P>(self) -> Result<CodePointTrie<'trie, P>, ZeroVecError>
    where
        P: TrieValue,
    {
        let converted_data = self.data.try_into_converted()?;
        Ok(CodePointTrie {
            header: self.header,
            index: self.index,
            data: converted_data,
        })
    }
}

impl<'trie, T: TrieValue + Into<u32>> CodePointTrie<'trie, T> {
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
        self.get(code_point).into()
    }

    /// Returns a [`CodePointMapRange`] struct representing a range of code
    /// points associated with the same value.
    pub fn get_range(&self, start: u32) -> Option<CodePointMapRange> {
        if CODE_POINT_MAX < start {
            return None;
        }
        if start >= self.header.high_start {
            let di: usize = self.data.len() - (HIGH_VALUE_NEG_DATA_OFFSET as usize);
            let value: u32 = if let Some(v) = self.data.get(di) {
                v.into()
            } else {
                return None;
            };
            return Some(CodePointMapRange {
                start,
                end: CODE_POINT_MAX,
                value,
            });
        }

        let null_value: u32 = self.header.null_value;

        let mut prev_i3_block: u32 = u32::MAX; // using u32::MAX (instead of -1 as an i32 in ICU)
        let mut prev_block: u32 = u32::MAX; // using u32::MAX (instead of -1 as an i32 in ICU)
        let mut c: u32 = start;
        let mut trie_value: u32 = 0;
        let mut value: u32 = 0;
        let mut have_value: bool = false;

        loop {
            let i3_block: u32;
            let mut i3: u32;
            let i3_block_length: u32;
            let data_block_length: u32;

            if c <= 0xffff
                && (self.header.trie_type == TrieType::Fast || c <= SMALL_TYPE_FAST_INDEXING_MAX)
            {
                i3_block = 0;
                i3 = c >> FAST_TYPE_SHIFT;
                i3_block_length = if self.header.trie_type == TrieType::Fast {
                    BMP_INDEX_LENGTH
                } else {
                    SMALL_INDEX_LENGTH
                };
                data_block_length = FAST_TYPE_DATA_BLOCK_LENGTH;
            } else {
                // Use the multi-stage index.
                let mut i1: u32 = c >> SHIFT_1;
                if self.header.trie_type == TrieType::Fast {
                    debug_assert!(0xffff < c && c < self.header.high_start);
                    i1 = i1 + BMP_INDEX_LENGTH - OMITTED_BMP_INDEX_1_LENGTH;
                } else {
                    debug_assert!(
                        c < self.header.high_start && self.header.high_start > SMALL_LIMIT
                    );
                    i1 += SMALL_INDEX_LENGTH;
                }
                let i2: u16 = self.index.get(i1 as usize)?;
                let i3_block_idx: u32 = (i2 as u32) + ((c >> SHIFT_2) & INDEX_2_MASK);
                i3_block = if let Some(i3b) = self.index.get(i3_block_idx as usize) {
                    i3b as u32
                } else {
                    return None;
                };
                if i3_block == prev_i3_block && (c - start) >= CP_PER_INDEX_2_ENTRY {
                    // The index-3 block is the same as the previous one, and filled with value.
                    debug_assert!((c & (CP_PER_INDEX_2_ENTRY - 1)) == 0);
                    c += CP_PER_INDEX_2_ENTRY;

                    if c >= self.header.high_start {
                        break;
                    } else {
                        continue;
                    }
                }
                prev_i3_block = i3_block;
                if i3_block == self.header.index3_null_offset as u32 {
                    // This is the index-3 null block.
                    if have_value {
                        if null_value != value {
                            return Some(CodePointMapRange {
                                start,
                                end: c - 1,
                                value,
                            });
                        }
                    } else {
                        trie_value = self.header.null_value;
                        value = null_value;
                        have_value = true;
                    }
                    prev_block = self.header.data_null_offset;
                    c = (c + CP_PER_INDEX_2_ENTRY) & !(CP_PER_INDEX_2_ENTRY - 1);
                    
                    if c >= self.header.high_start {
                        break;
                    } else {
                        continue;
                    }
                }
                i3 = (c >> SHIFT_3) & INDEX_3_MASK;
                i3_block_length = INDEX_3_BLOCK_LENGTH;
                data_block_length = SMALL_DATA_BLOCK_LENGTH;
            }
            // Enumerate data blocks for one index-3 block.
            loop {
                let mut block: u32;
                if (i3_block & 0x8000) == 0 {
                    block = if let Some(b) = self.index.get((i3_block + i3) as usize) {
                        b as u32
                    } else {
                        return None;
                    };
                } else {
                    // 18-bit indexes stored in groups of 9 entries per 8 indexes.
                    let mut group: u32 = (i3_block & 0x7fff) + (i3 & !7) + (i3 >> 3);
                    let gi: u32 = i3 & 7;
                    let gi_val: u32 = if let Some(giv) = self.index.get(group as usize) {
                        giv.into()
                    } else {
                        return None;
                    };
                    block = (gi_val << (2 + (2 * gi))) & 0x30000;
                    group += 1;
                    let ggi_val: u32 = if let Some(ggiv) = self.index.get((group + gi) as usize) {
                        ggiv as u32
                    } else {
                        return None;
                    };
                    block |= ggi_val;
                }
                if block == prev_block && (c - start) >= data_block_length {
                    // The block is the same as the previous one, and filled with value.
                    debug_assert!((c & (data_block_length - 1)) == 0);
                    c += data_block_length;
                } else {
                    let data_mask: u32 = data_block_length - 1;
                    prev_block = block;
                    if block == self.header.data_null_offset {
                        // This is the data null block.
                        if have_value {
                            if null_value != value {
                                return Some(CodePointMapRange {
                                    start,
                                    end: c - 1,
                                    value,
                                });
                            }
                        } else {
                            trie_value = self.header.null_value;
                            value = null_value;
                            have_value = true;
                        }
                        c = (c + data_block_length) & !data_mask;
                    } else {
                        let mut di: u32 = block + (c & data_mask);
                        let mut trie_value_2: u32 = if let Some(trv2) = self.data.get(di as usize) {
                            trv2.into()
                        } else {
                            return None;
                        };
                        if have_value {
                            if trie_value_2 != trie_value {
                                if maybe_filter_value(
                                    trie_value_2,
                                    self.header.null_value,
                                    null_value,
                                ) != value
                                {
                                    return Some(CodePointMapRange {
                                        start,
                                        end: c - 1,
                                        value,
                                    });
                                }
                                trie_value = trie_value_2; // may or may not help
                            }
                        } else {
                            trie_value = trie_value_2;
                            value = maybe_filter_value(
                                trie_value_2,
                                self.header.null_value,
                                null_value,
                            );
                            have_value = true;
                        }

                        c += 1;
                        while (c & data_mask) != 0 {
                            di += 1;
                            trie_value_2 = if let Some(trv2) = self.data.get(di as usize) {
                                trv2.into()
                            } else {
                                return None;
                            };
                            if trie_value_2 != trie_value {
                                if maybe_filter_value(
                                    trie_value_2,
                                    self.header.null_value,
                                    null_value,
                                ) != value
                                {
                                    return Some(CodePointMapRange {
                                        start,
                                        end: c - 1,
                                        value,
                                    });
                                }
                                trie_value = trie_value_2; // may or may not help
                            }

                            c += 1;
                        }
                    }
                }

                i3 += 1;
                if i3 >= i3_block_length {
                    break;
                }
            }

            if c >= self.header.high_start {
                break;
            }
        }

        debug_assert!(have_value);

        let di: u32 = self.data.len() as u32 - HIGH_VALUE_NEG_DATA_OFFSET;
        let high_value: u32 = if let Some(hv) = self.data.get(di as usize) {
            hv.into()
        } else {
            return None;
        };
        if maybe_filter_value(high_value, self.header.null_value, null_value) != value {
            c -= 1;
        } else {
            c = CODE_POINT_MAX;
        }
        Some(CodePointMapRange {
            start,
            end: c,
            value,
        })
    }
}

impl<'trie, T: TrieValue> Clone for CodePointTrie<'trie, T>
where
    <T as zerovec::ule::AsULE>::ULE: Clone,
{
    fn clone(&self) -> Self {
        CodePointTrie {
            header: self.header,
            index: self.index.clone(),
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CodePointMapRange;
    #[cfg(feature = "serde")]
    use super::CodePointTrie;
    use crate::planes;
    use alloc::vec::Vec;
    #[cfg(feature = "serde")]
    use zerovec::ZeroVec;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_with_postcard_roundtrip() -> Result<(), postcard::Error> {
        let trie = crate::planes::get_planes_trie();
        let trie_serialized: Vec<u8> = postcard::to_allocvec(&trie).unwrap();

        // Assert an expected (golden data) version of the serialized trie.
        const EXP_TRIE_SERIALIZED: &[u8] = &[
            0, 0, 16, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 160, 18, 0, 0, 64, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 136, 2, 144, 2, 144, 2, 144, 2, 176, 2, 176, 2, 176, 2, 176, 2, 208, 2,
            208, 2, 208, 2, 208, 2, 240, 2, 240, 2, 240, 2, 240, 2, 16, 3, 16, 3, 16, 3, 16, 3, 48,
            3, 48, 3, 48, 3, 48, 3, 80, 3, 80, 3, 80, 3, 80, 3, 112, 3, 112, 3, 112, 3, 112, 3,
            144, 3, 144, 3, 144, 3, 144, 3, 176, 3, 176, 3, 176, 3, 176, 3, 208, 3, 208, 3, 208, 3,
            208, 3, 240, 3, 240, 3, 240, 3, 240, 3, 16, 4, 16, 4, 16, 4, 16, 4, 48, 4, 48, 4, 48,
            4, 48, 4, 80, 4, 80, 4, 80, 4, 80, 4, 112, 4, 112, 4, 112, 4, 112, 4, 0, 0, 16, 0, 32,
            0, 48, 0, 64, 0, 80, 0, 96, 0, 112, 0, 0, 0, 16, 0, 32, 0, 48, 0, 0, 0, 16, 0, 32, 0,
            48, 0, 0, 0, 16, 0, 32, 0, 48, 0, 0, 0, 16, 0, 32, 0, 48, 0, 0, 0, 16, 0, 32, 0, 48, 0,
            0, 0, 16, 0, 32, 0, 48, 0, 0, 0, 16, 0, 32, 0, 48, 0, 0, 0, 16, 0, 32, 0, 48, 0, 128,
            0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128,
            0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128,
            0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 144, 0, 144,
            0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144,
            0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144,
            0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 144, 0, 160, 0, 160, 0, 160,
            0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160,
            0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160,
            0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 160, 0, 176, 0, 176, 0, 176, 0, 176,
            0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176,
            0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176,
            0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 176, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192,
            0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192,
            0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 192,
            0, 192, 0, 192, 0, 192, 0, 192, 0, 192, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208,
            0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208,
            0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208, 0, 208,
            0, 208, 0, 208, 0, 208, 0, 208, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224,
            0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224,
            0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224, 0, 224,
            0, 224, 0, 224, 0, 224, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240,
            0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240,
            0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240, 0, 240,
            0, 240, 0, 240, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1,
            16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16,
            1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1, 16, 1,
            32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32,
            1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1,
            32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48,
            1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1,
            48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48,
            1, 48, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1,
            64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64,
            1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 80, 1, 80, 1, 80, 1, 80, 1,
            80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80,
            1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1, 80, 1,
            80, 1, 80, 1, 80, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96,
            1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1,
            96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 96, 1, 128, 0, 136, 0,
            136, 0, 136, 0, 136, 0, 136, 0, 136, 0, 136, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2,
            0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0,
            2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 168, 0, 168, 0, 168,
            0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168,
            0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168,
            0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 168, 0, 200, 0, 200, 0, 200, 0, 200,
            0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200,
            0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200,
            0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232,
            0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232,
            0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 232,
            0, 232, 0, 232, 0, 232, 0, 232, 0, 232, 0, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8,
            1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1,
            8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 40, 1, 40, 1, 40, 1, 40, 1,
            40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40,
            1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1, 40, 1,
            40, 1, 40, 1, 40, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72,
            1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1,
            72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 104, 1, 104, 1,
            104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1,
            104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1,
            104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 104, 1, 136, 1, 136, 1, 136, 1,
            136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1,
            136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1,
            136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 136, 1, 168, 1, 168, 1, 168, 1, 168, 1,
            168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1,
            168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1,
            168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 168, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1,
            200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1,
            200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 200, 1,
            200, 1, 200, 1, 200, 1, 200, 1, 200, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1,
            232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1,
            232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1, 232, 1,
            232, 1, 232, 1, 232, 1, 232, 1, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2,
            8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8,
            2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 8, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2,
            40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40,
            2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2, 40, 2,
            40, 2, 40, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72,
            2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2,
            72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 72, 2, 104, 2, 104, 2, 104, 2,
            104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2,
            104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2,
            104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 104, 2, 244, 2, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6,
            6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
            8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            9, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11,
            11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12, 12,
            12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
            13, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15,
            15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16, 16, 16, 0,
        ];
        assert_eq!(trie_serialized, EXP_TRIE_SERIALIZED);

        let trie_deserialized = postcard::from_bytes::<CodePointTrie<u8>>(&trie_serialized)?;

        assert_eq!(&trie.index, &trie_deserialized.index);
        assert_eq!(&trie.data, &trie_deserialized.data);

        assert!(matches!(trie_deserialized.index, ZeroVec::Borrowed(_)));
        assert!(matches!(trie_deserialized.data, ZeroVec::Borrowed(_)));

        Ok(())
    }

    #[test]
    fn test_get_range() {
        let planes_trie = planes::get_planes_trie();

        let first_range: Option<CodePointMapRange> = planes_trie.get_range(0x0);
        assert_eq!(
            first_range,
            Some(CodePointMapRange {
                start: 0x0,
                end: 0xffff,
                value: 0
            })
        );

        let second_range: Option<CodePointMapRange> = planes_trie.get_range(0x1_0000);
        assert_eq!(
            second_range,
            Some(CodePointMapRange {
                start: 0x10000,
                end: 0x1ffff,
                value: 1
            })
        );

        let last_range: Option<CodePointMapRange> = planes_trie.get_range(0x10_0000);
        assert_eq!(
            last_range,
            Some(CodePointMapRange {
                start: 0x10_0000,
                end: 0x10_ffff,
                value: 16
            })
        );
    }
}
