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

const MIN_DENSE_PERCENT: usize = 2;
const FALLBACK_TOP_K: usize = 64;

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
    /// Helper function: finds best row offset when value range too large for dense matrix
    #[allow(clippy::indexing_slicing)] // bot < top and top < sorted_vals.len()
    fn find_window(values: &BTreeMap<&'a str, usize>) -> usize {
        let mut sorted_vals: Vec<usize> = values.values().cloned().collect();
        let row_width = usize::from(DenseType::MAX);
        sorted_vals.sort_unstable();
        let mut bot = 0;
        let mut best = 0;
        let mut best_index = 0;
        for top in 0..sorted_vals.len() {
            while bot < top {
                let top_val = sorted_vals[top];
                let bot_val = sorted_vals[bot];
                if top_val - bot_val >= row_width {
                    bot += 1;
                } else {
                    break;
                }
            }
            if (top - bot + 1) > best {
                best = top - bot + 1;
                best_index = bot;
            }
        }
        sorted_vals[best_index]
    }
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
        let mut row_value_offset = min;
        if max - min >= usize::from(DenseType::MAX) {
            row_value_offset = Self::find_window(values);
        }
        let sentinel = row_value_offset + usize::from(DenseType::MAX);
        // Partition the entries based on whether they can be encoded as dense
        let (dense_or_sparse, sparse_only) = values
            .iter()
            .map(|(suffix, value)| (*suffix, *value))
            .partition::<BTreeMap<&'a str, usize>, _>(|(suffix, value)| {
                self.suffixes.contains(suffix)
                    && *value >= row_value_offset
                    && *value < row_value_offset + usize::from(DenseType::MAX)
            });
        // Check whether the sparse trie is smaller than the dense row
        let sub_trie = dense_or_sparse
            .iter()
            .map(|(suffix, value)| (*suffix, *value))
            .collect::<ZeroTrieSimpleAscii<Vec<u8>>>();
        if sub_trie.byte_len() > self.suffixes.len() * core::mem::size_of::<DenseType>() {
            // Create a dense prefix
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
        // Validate prefixes for delimiter presence
        for prefix in entries.keys() {
            if prefix.contains(delimiter as char) {
                return Err(ZeroTrieBuildError::IllegalDelimiter);
            }
        }
        let mut suffix_prefix_count: BTreeMap<&str, usize> = BTreeMap::new();
        for (_, inner_map) in entries.iter() {
            for &suffix in inner_map.keys() {
                if suffix.contains(delimiter as char) {
                    return Err(ZeroTrieBuildError::IllegalDelimiter);
                }
                *suffix_prefix_count.entry(suffix).or_insert(0) += 1;
            }
        }

        let total_prefixes = entries.len();
        //ceil(a / b) = (a + b - 1) / b
        const PERCENT_DENOMINATOR: usize = 100;
        let computed_min = total_prefixes
            .saturating_mul(MIN_DENSE_PERCENT)
            .saturating_add(PERCENT_DENOMINATOR - 1)
            .saturating_div(PERCENT_DENOMINATOR);
        let min_prefixes = core::cmp::max(2, computed_min);
        let mut dense_candidates: Vec<(&str, usize)> = suffix_prefix_count
            .iter()
            .filter(|(_, &count)| count >= min_prefixes)
            .map(|(&suffix, &count)| (suffix, count))
            .collect();

        // If none meet the threshold, fallback to picking top-K by frequency.
        if dense_candidates.is_empty() {
            let mut all_suffixes: Vec<(&str, usize)> = suffix_prefix_count.into_iter().collect();

            // Sort by frequency descending 
            all_suffixes.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

            dense_candidates = all_suffixes.into_iter().take(FALLBACK_TOP_K).collect();
        }

        dense_candidates.sort_by(|a, b| a.0.cmp(&b.0));

        builder.suffixes = dense_candidates
            .into_iter()
            .map(|(suffix, _)| suffix)
            .collect();

        for (prefix, values) in entries.iter() {
            builder.add_prefix(prefix, values)?;
        }
        builder.build()
    }
}
