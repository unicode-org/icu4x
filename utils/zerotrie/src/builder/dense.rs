// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::dense::DenseType;
use crate::dense::ZeroAsciiDenseSparse2dTrieOwned;
use crate::ZeroTrieBuildError;
use crate::ZeroTrieSimpleAscii;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;
use zerovec::ZeroVec;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Row<'a> {
    prefix: &'a str,
    row_value_offset: usize,
    offsets: Vec<DenseType>,
}

#[derive(Default)]
pub(crate) struct DenseSparse2dAsciiWithFixedDelimiterBuilder<'a> {
    primary: Vec<(&'a str, &'a str, usize)>,
    suffixes: BTreeSet<&'a str>,
    dense: Vec<Row<'a>>,
    delimiter: u8,
}

impl<'a> DenseSparse2dAsciiWithFixedDelimiterBuilder<'a> {
    /// Add a prefix and all values associated with the prefix to the builder.
    pub(crate) fn add_prefix(
        &mut self,
        prefix: &'a str,
        values: &BTreeMap<&'a str, usize>,
    ) -> Result<(), ZeroTrieBuildError> {
        // Is there a more Rusty way to compute min and max together?
        let mut min = usize::MAX;
        let mut max = 0;
        for value in values.values() {
            min = core::cmp::min(min, *value);
            max = core::cmp::max(max, *value);
        }
        // >= because DenseType::MAX is the sentinel
        if max - min >= usize::from(DenseType::MAX) {
            // TODO(#7303): How to implement this when we need it:
            // 1. Select the row offset that gets the greatest number of values into the dense matrix.
            // 2. Put all out-of-range values into the primary trie, as we do with sparse rows.
            todo!("values in row are not in a sufficiently compact range");
        }
        let sentinel = min + usize::from(DenseType::MAX);
        // Partition the entries based on whether they can be encoded as dense
        let (dense_or_sparse, sparse_only) = values
            .iter()
            .map(|(suffix, value)| (*suffix, *value))
            .partition::<BTreeMap<&'a str, usize>, _>(|(suffix, _value)| {
                // TODO(#7302): Also filter for suffixes that are out of range of the dense row offset
                self.suffixes.contains(suffix)
            });
        // Check whether the sparse trie is smaller than the dense row
        let sub_trie = dense_or_sparse
            .iter()
            .map(|(suffix, value)| (*suffix, *value))
            .collect::<ZeroTrieSimpleAscii<Vec<u8>>>();
        if sub_trie.byte_len() > self.suffixes.len() * core::mem::size_of::<DenseType>() {
            // Create a dense prefix
            let row_value_offset = min;
            let offsets = self
                .suffixes
                .iter()
                .map(|suffix| {
                    let value = sub_trie.get(suffix).unwrap_or(sentinel);
                    let Ok(offset) = DenseType::try_from(value - row_value_offset) else {
                        unreachable!("this should have been handled earlier");
                    };
                    offset
                })
                .collect::<Vec<DenseType>>();
            self.dense.push(Row {
                prefix,
                row_value_offset,
                offsets,
            });
            for (suffix, value) in sparse_only.iter() {
                self.primary.push((prefix, *suffix, *value));
            }
            Ok(())
        } else {
            // Create a sparse prefix
            for (suffix, value) in values.iter() {
                self.primary.push((prefix, *suffix, *value));
            }
            Ok(())
        }
    }

    /// Assemble the intermediate structures into the final layout.
    pub(crate) fn build(mut self) -> Result<ZeroAsciiDenseSparse2dTrieOwned, ZeroTrieBuildError> {
        self.dense.sort();
        let Ok(suffix_count) = DenseType::try_from(self.suffixes.len()) else {
            return Err(ZeroTrieBuildError::CapacityExceeded);
        };
        let delimiter = self.delimiter as char;
        let mut primary_contents = BTreeMap::new();
        for (prefix, suffix, value) in self.primary.iter() {
            if prefix.contains(delimiter) || suffix.contains(delimiter) {
                debug_assert!(false, "handled earlier");
                return Err(ZeroTrieBuildError::IllegalDelimiter);
            }
            let mut delimited = String::with_capacity(prefix.len() + suffix.len() + 1);
            delimited.push_str(prefix);
            delimited.push(delimiter);
            delimited.push_str(suffix);
            primary_contents.insert(delimited, *value);
        }
        let mut dense = Vec::<DenseType>::with_capacity(self.dense.len() * self.suffixes.len());
        for (
            row_index,
            Row {
                prefix,
                row_value_offset,
                offsets,
            },
        ) in self.dense.iter().enumerate()
        {
            primary_contents.insert(String::from(*prefix), row_index);
            let mut prefix_with_delim = String::with_capacity(prefix.len() + 1);
            prefix_with_delim.push_str(prefix);
            prefix_with_delim.push(delimiter);
            primary_contents.insert(prefix_with_delim, *row_value_offset);
            dense.extend(offsets);
        }
        let suffixes = self
            .suffixes
            .iter()
            .enumerate()
            .map(|(column_index, suffix)| (*suffix, column_index))
            .collect::<BTreeMap<&str, usize>>();
        Ok(ZeroAsciiDenseSparse2dTrieOwned {
            primary: ZeroTrieSimpleAscii::try_from_btree_map_str(&primary_contents)?,
            suffixes: ZeroTrieSimpleAscii::try_from_btree_map_str(&suffixes)?,
            dense: ZeroVec::from_slice_or_alloc(dense.as_slice()).into_owned(),
            suffix_count,
            delimiter: self.delimiter,
        })
    }
}

impl ZeroAsciiDenseSparse2dTrieOwned {
    /// Builds one of these from a two-dimensional BTreeMap and a delimiter.
    ///
    /// Keep in mind the recommendations for optimal data size described in
    /// the [class docs].
    ///
    /// [class docs]: ZeroAsciiDenseSparse2dTrieOwned
    pub fn try_from_btree_map_str(
        entries: &BTreeMap<&str, BTreeMap<&str, usize>>,
        delimiter: u8,
    ) -> Result<Self, ZeroTrieBuildError> {
        let mut builder = DenseSparse2dAsciiWithFixedDelimiterBuilder {
            delimiter,
            ..Default::default()
        };
        // TODO(#7302): Prune low-frequency suffixes.
        // For now, build with all suffixes.
        builder.suffixes = entries
            .values()
            .flat_map(|inner| inner.keys())
            .copied()
            .map(|s| {
                if s.contains(delimiter as char) {
                    Err(ZeroTrieBuildError::IllegalDelimiter)
                } else {
                    Ok(s)
                }
            })
            .collect::<Result<_, ZeroTrieBuildError>>()?;
        for (prefix, values) in entries.iter() {
            if prefix.contains(delimiter as char) {
                return Err(ZeroTrieBuildError::IllegalDelimiter);
            }
            builder.add_prefix(prefix, values)?;
        }
        builder.build()
    }
}
