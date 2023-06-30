// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::branch_meta::BranchMeta;
use super::super::bytestr::ByteStr;
use super::store::const_for_each;
use super::store::ConstArrayBuilder;
use super::store::ConstLengthsStack;
use super::store::ConstSlice;
use crate::error::Error;
use crate::varint;

/// A low-level builder for ZeroTrieSimpleAscii. Works in const contexts.
pub(crate) struct ZeroTrieBuilderConst<const N: usize> {
    data: ConstArrayBuilder<N, u8>,
}

impl<const N: usize> ZeroTrieBuilderConst<N> {
    #[cfg(feature = "litemap")]
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_const_slice().as_slice()
    }

    pub const fn take_or_panic(self) -> [u8; N] {
        self.data.const_take_or_panic()
    }

    pub const fn new() -> Self {
        Self {
            data: ConstArrayBuilder::new_empty([0; N], N),
        }
    }

    #[must_use]
    const fn prepend_ascii(self, ascii: u8) -> (Self, usize) {
        if ascii >= 128 {
            panic!("Non-ASCII not supported in ZeroTrieSimpleAscii");
        }
        let data = self.data.const_push_front_or_panic(ascii);
        (Self { data }, 1)
    }

    #[must_use]
    const fn prepend_value(self, value: usize) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_extended_varint(value);
        data = data.const_extend_front_or_panic(varint_array.as_const_slice());
        data = data.const_bitor_assign(0, 0b10000000);
        (Self { data }, varint_array.len())
    }

    #[must_use]
    const fn prepend_branch(self, value: usize) -> (Self, usize) {
        let mut data = self.data;
        let varint_array = varint::write_varint(value);
        data = data.const_extend_front_or_panic(varint_array.as_const_slice());
        data = data.const_bitor_assign(0, 0b11000000);
        (Self { data }, varint_array.len())
    }

    #[must_use]
    const fn prepend_slice(self, s: ConstSlice<u8>) -> (Self, usize) {
        let mut data = self.data;
        let mut i = s.len();
        while i > 0 {
            data = data.const_push_front_or_panic(*s.get_or_panic(i - 1));
            i -= 1;
        }
        (Self { data }, s.len())
    }

    #[must_use]
    const fn prepend_n_zeros(self, n: usize) -> Self {
        let mut data = self.data;
        let mut i = 0;
        while i < n {
            data = data.const_push_front_or_panic(0);
            i += 1;
        }
        Self { data }
    }

    const fn bitor_assign_at(self, index: usize, byte: u8) -> Self {
        let mut data = self.data;
        data = data.const_bitor_assign(index, byte);
        Self { data }
    }

    /// Panics if the items are not sorted
    pub const fn from_tuple_slice<'a, const K: usize>(
        items: &[(&'a ByteStr, usize)],
    ) -> Result<Self, Error> {
        let items = ConstSlice::from_slice(items);
        let mut prev: Option<&'a ByteStr> = None;
        const_for_each!(items, (ascii_str, _), {
            match prev {
                None => (),
                Some(prev) => {
                    if !prev.is_less_then(ascii_str) {
                        panic!("Strings in ByteStr constructor are not sorted");
                    }
                }
            };
            prev = Some(ascii_str)
        });
        Self::from_sorted_const_tuple_slice::<K>(items)
    }

    /// Assumes that the items are sorted
    pub const fn from_sorted_const_tuple_slice<const K: usize>(
        items: ConstSlice<(&ByteStr, usize)>,
    ) -> Result<Self, Error> {
        let mut result = Self::new();
        let total_size;
        (result, total_size) = result.create_or_panic::<K>(items);
        debug_assert!(total_size == result.data.len());
        Ok(result)
    }

    #[must_use]
    const fn create_or_panic<const K: usize>(
        mut self,
        all_items: ConstSlice<(&ByteStr, usize)>,
    ) -> (Self, usize) {
        let mut prefix_len = match all_items.last() {
            Some(x) => x.0.len(),
            // Empty slice:
            None => return (Self::new(), 0),
        };
        let mut lengths_stack = ConstLengthsStack::<K>::new();
        let mut i = all_items.len() - 1;
        let mut j = all_items.len();
        let mut current_len = 0;
        loop {
            let item_i = all_items.get_or_panic(i);
            let item_j = all_items.get_or_panic(j - 1);
            assert!(item_i.0.prefix_eq(item_j.0, prefix_len));
            if item_i.0.len() == prefix_len {
                let len;
                (self, len) = self.prepend_value(item_i.1);
                current_len += len;
            }
            if prefix_len == 0 {
                break;
            }
            prefix_len -= 1;
            let mut new_i = i;
            let mut new_j = j;
            let mut diff_i = 0;
            let mut diff_j = 0;
            let mut ascii_i = item_i.0.byte_at_or_panic(prefix_len);
            let mut ascii_j = item_j.0.byte_at_or_panic(prefix_len);
            assert!(ascii_i == ascii_j);
            let key_ascii = ascii_i;
            loop {
                if new_i == 0 {
                    break;
                }
                let candidate = all_items.get_or_panic(new_i - 1).0;
                if candidate.len() < prefix_len {
                    // Too short
                    break;
                }
                if item_i.0.prefix_eq(candidate, prefix_len) {
                    new_i -= 1;
                } else {
                    break;
                }
                if candidate.len() == prefix_len {
                    // A string of length prefix_len can't be preceded by another with that prefix
                    break;
                }
                let candidate = candidate.byte_at_or_panic(prefix_len);
                if candidate != ascii_i {
                    diff_i += 1;
                    ascii_i = candidate;
                }
            }
            loop {
                if new_j == all_items.len() {
                    break;
                }
                let candidate = all_items.get_or_panic(new_j).0;
                if candidate.len() < prefix_len {
                    // Too short
                    break;
                }
                if item_j.0.prefix_eq(candidate, prefix_len) {
                    new_j += 1;
                } else {
                    break;
                }
                if candidate.len() == prefix_len {
                    panic!("A shorter string should be earlier in the sequence");
                }
                let candidate = candidate.byte_at_or_panic(prefix_len);
                if candidate != ascii_j {
                    diff_j += 1;
                    ascii_j = candidate;
                }
            }
            if diff_i == 0 && diff_j == 0 {
                let len;
                (self, len) = self.prepend_ascii(ascii_i);
                current_len += len;
                assert!(i == new_i || i == new_i + 1);
                i = new_i;
                assert!(j == new_j);
                continue;
            }
            // Branch
            if diff_j == 0 {
                lengths_stack = lengths_stack.push_or_panic(BranchMeta {
                    ascii: key_ascii,
                    cumulative_length: current_len,
                    local_length: current_len,
                    count: 1,
                });
            } else {
                let BranchMeta {
                    cumulative_length,
                    count,
                    ..
                } = lengths_stack.peek_or_panic();
                lengths_stack = lengths_stack.push_or_panic(BranchMeta {
                    ascii: key_ascii,
                    cumulative_length: cumulative_length + current_len,
                    local_length: current_len,
                    count: count + 1,
                });
            }
            if diff_i != 0 {
                j = i;
                i -= 1;
                prefix_len = all_items.get_or_panic(i).0.len();
                current_len = 0;
                continue;
            }
            // Branch (first)
            let (total_length, total_count) = {
                let BranchMeta {
                    cumulative_length,
                    count,
                    ..
                } = lengths_stack.peek_or_panic();
                (cumulative_length, count)
            };
            let branch_metas;
            (lengths_stack, branch_metas) = lengths_stack.pop_many_or_panic(total_count);
            let original_keys = branch_metas.map_to_ascii_bytes();
            // Write out the offset table
            current_len = total_length;
            const USIZE_BITS: usize = core::mem::size_of::<usize>() * 8;
            let w = (USIZE_BITS - (total_length.leading_zeros() as usize) - 1) / 8;
            if w > 3 {
                panic!("ZeroTrie capacity exceeded");
            }
            let mut k = 0;
            while k <= w {
                self = self.prepend_n_zeros(total_count - 1);
                current_len += total_count - 1;
                let mut l = 0;
                let mut length_to_write = 0;
                while l < total_count {
                    let BranchMeta { local_length, .. } = *branch_metas
                        .as_const_slice()
                        .get_or_panic(total_count - l - 1);
                    let mut adjusted_length = length_to_write;
                    let mut m = 0;
                    while m < k {
                        adjusted_length >>= 8;
                        m += 1;
                    }
                    if l > 0 {
                        self = self.bitor_assign_at(l - 1, adjusted_length as u8);
                    }
                    l += 1;
                    length_to_write += local_length;
                }
                k += 1;
            }
            assert!(0 < total_count && total_count <= 256);
            let branch_value = (w << 8) + (total_count & 0xff);
            let slice_len;
            (self, slice_len) = self.prepend_slice(original_keys.as_const_slice());
            let branch_len;
            (self, branch_len) = self.prepend_branch(branch_value);
            current_len += slice_len + branch_len;
            i = new_i;
            j = new_j;
        }
        assert!(lengths_stack.is_empty());
        (self, current_len)
    }
}
