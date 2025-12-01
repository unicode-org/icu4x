// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ZeroTrieSimpleAscii;
use zerovec::ZeroSlice;

/// A data structure designed for 2-dimensional ASCII keys with a fixed
/// delimiter that exhibit a mix of density and sparsity.
pub struct DenseSparse2dAsciiWithFixedDelimiterBorrowed<'a> {
    primary: &'a ZeroTrieSimpleAscii<[u8]>,
	suffixes: &'a ZeroTrieSimpleAscii<[u8]>,
	dense: &'a ZeroSlice<u16>,
	suffix_count: u16,
    delimiter: u8,
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
        let Some(offset) = self.dense.get(suffix_count*row_index + column_index) else {
        	// The row and column indexes should be in-range
        	debug_assert!(false);
        	return None;
        };
        Some(usize::from(offset) + usize::from(row_value_offset))
    }
}
