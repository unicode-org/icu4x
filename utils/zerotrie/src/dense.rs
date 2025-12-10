// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures combining [`ZeroTrie`] and [`ZeroVec`] to store data
//! more efficiently.

use crate::ZeroTrieSimpleAscii;
use zerovec::ule::tuplevar::Tuple3VarULE;
use zerovec::ule::vartuple::VarTuple;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::ule::EncodeAsVarULE;
use zerovec::ZeroSlice;
use zerovec::ZeroVec;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub(crate) type DenseType = u16;

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Eq)]
pub struct DenseSparse2dAsciiWithFixedDelimiterOwned {
    pub(crate) primary: ZeroTrieSimpleAscii<Vec<u8>>,
    pub(crate) suffixes: ZeroTrieSimpleAscii<Vec<u8>>,
    pub(crate) dense: ZeroVec<'static, DenseType>,
    pub(crate) suffix_count: u16,
    pub(crate) delimiter: u8,
}

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity.
pub type DenseSparse2dAsciiWithFixedDelimiterVarULE =
    VarTupleULE<(u16, u8), Tuple3VarULE<ZeroSlice<u8>, ZeroSlice<u8>, ZeroSlice<DenseType>>>;

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity.
#[derive(Debug, PartialEq, Eq)]
pub struct DenseSparse2dAsciiWithFixedDelimiterBorrowed<'a> {
    primary: &'a ZeroTrieSimpleAscii<[u8]>,
    suffixes: &'a ZeroTrieSimpleAscii<[u8]>,
    dense: &'a ZeroSlice<DenseType>,
    suffix_count: u16,
    delimiter: u8,
}

#[cfg(feature = "alloc")]
impl DenseSparse2dAsciiWithFixedDelimiterOwned {
    /// Borrows the structure for querying.
    pub fn as_borrowed(&self) -> DenseSparse2dAsciiWithFixedDelimiterBorrowed<'_> {
        DenseSparse2dAsciiWithFixedDelimiterBorrowed {
            primary: self.primary.as_borrowed(),
            suffixes: self.suffixes.as_borrowed(),
            dense: self.dense.as_slice(),
            suffix_count: self.suffix_count,
            delimiter: self.delimiter,
        }
    }

    /// Borrows the structure for encoding into [`zerovec`].
    pub fn as_encodeable(
        &self,
    ) -> impl EncodeAsVarULE<DenseSparse2dAsciiWithFixedDelimiterVarULE> + '_ {
        VarTuple {
            sized: (self.suffix_count, self.delimiter),
            variable: (
                self.primary.as_bytes(),
                self.suffixes.as_bytes(),
                self.dense.as_slice(),
            ),
        }
    }
}

impl<'a> From<&'a DenseSparse2dAsciiWithFixedDelimiterVarULE>
    for DenseSparse2dAsciiWithFixedDelimiterBorrowed<'a>
{
    fn from(
        ule: &DenseSparse2dAsciiWithFixedDelimiterVarULE,
    ) -> DenseSparse2dAsciiWithFixedDelimiterBorrowed<'_> {
        DenseSparse2dAsciiWithFixedDelimiterBorrowed {
            primary: ZeroTrieSimpleAscii::from_bytes(ule.variable.a().as_ule_slice()),
            suffixes: ZeroTrieSimpleAscii::from_bytes(ule.variable.b().as_ule_slice()),
            dense: ule.variable.c(),
            suffix_count: ule.sized.0.as_unsigned_int(),
            delimiter: ule.sized.1,
        }
    }
}

impl<'a> DenseSparse2dAsciiWithFixedDelimiterBorrowed<'a> {
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
        Some(usize::from(offset) + usize::from(row_value_offset))
    }
}
