// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures combining [`ZeroTrie`] and [`ZeroVec`] to store data
//! more efficiently.
//!
//! [`ZeroTrie`]: crate::ZeroTrie

use crate::ZeroTrieSimpleAscii;
use zerovec::ule::tuplevar::Tuple3VarULE;
use zerovec::ule::vartuple::VarTuple;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::ule::EncodeAsVarULE;
use zerovec::vecs::Index32;
use zerovec::ZeroSlice;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use zerovec::ZeroVec;

pub(crate) type DenseType = u16;

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity (owned version).
///
/// It stores prefixes and suffixes separated by a delimiter, which must not
/// occur in any of the prefix or suffix strings.
///
/// This type can deliver data size savings if the suffixes are commonly
/// repeated. If they are mostly unique, a simple trie is likely better.
///
/// # Examples
///
/// ```
/// use std::collections::BTreeMap;
/// use zerotrie::dense::ZeroAsciiDenseSparse2dTrieOwned;
///
/// let mut data: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
///
/// data.entry("aaa").or_default().insert("BBB", 1);
/// data.entry("aaa").or_default().insert("CCC", 2);
/// data.entry("ddd").or_default().insert("BBB", 3);
/// data.entry("ddd").or_default().insert("EEE", 4);
/// data.entry("fff").or_default().insert("BBB", 5);
///
/// let trie = ZeroAsciiDenseSparse2dTrieOwned::try_from_btree_map_str(&data, b'/').unwrap();
/// let trie = trie.as_borrowed();
///
/// assert_eq!(trie.get("aaa", "CCC"), Some(2));
/// assert_eq!(trie.get("aaa", "EEE"), None);
/// ```
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Eq)]
pub struct ZeroAsciiDenseSparse2dTrieOwned {
    /// The main trie, including all data that isn't in the dense matrix.
    ///
    /// There are three types of values stored here:
    ///
    /// 1. Prefix: value is row index
    /// 2. Prefix followed by delimiter: value is row value offset
    /// 3. Prefix followed by delimiter followed by suffix: a value not in the dense matrix
    pub(crate) primary: ZeroTrieSimpleAscii<Vec<u8>>,
    /// A trie mapping suffixes to column IDs in the dense matrix.
    pub(crate) suffixes: ZeroTrieSimpleAscii<Vec<u8>>,
    /// The dense matrix.
    pub(crate) dense: ZeroVec<'static, DenseType>,
    /// The number of columns in the dense matrix.
    pub(crate) suffix_count: u16,
    /// The delimiter byte.
    pub(crate) delimiter: u8,
}

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity (ULE version).
pub type ZeroAsciiDenseSparse2dTrieVarULE = VarTupleULE<
    (u16, u8),
    Tuple3VarULE<ZeroSlice<u8>, ZeroSlice<u8>, ZeroSlice<DenseType>, Index32>,
>;

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity (borrowed version).
#[derive(Debug, PartialEq, Eq)]
pub struct ZeroAsciiDenseSparse2dTrieBorrowed<'a> {
    primary: &'a ZeroTrieSimpleAscii<[u8]>,
    suffixes: &'a ZeroTrieSimpleAscii<[u8]>,
    dense: &'a ZeroSlice<DenseType>,
    suffix_count: u16,
    delimiter: u8,
}

#[cfg(feature = "alloc")]
impl ZeroAsciiDenseSparse2dTrieOwned {
    /// Borrows the structure for querying.
    pub fn as_borrowed(&self) -> ZeroAsciiDenseSparse2dTrieBorrowed<'_> {
        ZeroAsciiDenseSparse2dTrieBorrowed {
            primary: self.primary.as_borrowed(),
            suffixes: self.suffixes.as_borrowed(),
            dense: self.dense.as_slice(),
            suffix_count: self.suffix_count,
            delimiter: self.delimiter,
        }
    }
}

impl<'a> From<&'a ZeroAsciiDenseSparse2dTrieVarULE> for ZeroAsciiDenseSparse2dTrieBorrowed<'a> {
    fn from(ule: &ZeroAsciiDenseSparse2dTrieVarULE) -> ZeroAsciiDenseSparse2dTrieBorrowed<'_> {
        ZeroAsciiDenseSparse2dTrieBorrowed {
            primary: ZeroTrieSimpleAscii::from_bytes(ule.variable.a().as_ule_slice()),
            suffixes: ZeroTrieSimpleAscii::from_bytes(ule.variable.b().as_ule_slice()),
            dense: ule.variable.c(),
            suffix_count: ule.sized.0.as_unsigned_int(),
            delimiter: ule.sized.1,
        }
    }
}

impl<'a> ZeroAsciiDenseSparse2dTrieBorrowed<'a> {
    /// Queries the data structure for a value
    pub fn get(&self, prefix: &str, suffix: &str) -> Option<usize> {
        let mut cursor = self.primary.cursor();
        for b in prefix.as_bytes().iter() {
            if cursor.is_empty() {
                return None;
            }
            cursor.step(*b);
        }
        let row_index = cursor.take_value();
        cursor.step(self.delimiter);
        let row_value_offset = cursor.take_value();
        // Check for a sparse value
        for b in suffix.as_bytes().iter() {
            if cursor.is_empty() {
                break;
            }
            cursor.step(*b);
        }
        if let Some(value) = cursor.take_value() {
            // Found a sparse value
            return Some(value);
        }
        let (Some(row_index), Some(row_value_offset)) = (row_index, row_value_offset) else {
            // There is no dense row for this prefix
            return None;
        };
        let Some(column_index) = self.suffixes.get(suffix) else {
            // There is no dense column for this suffix
            return None;
        };
        let suffix_count = usize::from(self.suffix_count);
        let Some(offset) = self.dense.get(suffix_count * row_index + column_index) else {
            // The row and column indexes should be in-range
            debug_assert!(false);
            return None;
        };
        if offset == DenseType::MAX {
            // There is an entry in the dense matrix but it is a None value
            return None;
        }
        Some(usize::from(offset) + row_value_offset)
    }

    /// Borrows the structure for encoding into [`zerovec`].
    pub fn as_encodeable(&self) -> impl EncodeAsVarULE<ZeroAsciiDenseSparse2dTrieVarULE> + '_ {
        VarTuple {
            sized: (self.suffix_count, self.delimiter),
            variable: (
                self.primary.as_bytes(),
                self.suffixes.as_bytes(),
                self.dense,
            ),
        }
    }
}
