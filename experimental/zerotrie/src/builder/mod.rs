// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod branch_meta;
pub(crate) mod bytestr;
pub(crate) mod konst;
#[cfg(feature = "litemap")]
mod litemap;
#[cfg(feature = "alloc")]
pub(crate) mod nonconst;

use bytestr::ByteStr;

use super::ZeroTrieSimpleAscii;

impl<const N: usize> ZeroTrieSimpleAscii<[u8; N]> {
    /// **Const Constructor:** Creates an [`ZeroTrieSimpleAscii`] from a sorted slice of keys and values.
    ///
    /// This function needs to know the exact length of the resulting trie at compile time.
    ///
    /// Also see [`Self::from_sorted_str_tuples`].
    ///
    /// # Panics
    ///
    /// Panics if `items` is not sorted or if `N` is not correct.
    ///
    /// # Examples
    ///
    /// Create a `const` ZeroTrieSimpleAscii at compile time:
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // The required capacity for this trie happens to be 17 bytes
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_sorted_u8_tuples(&[
    ///     (b"bar", 2),
    ///     (b"bazzoo", 3),
    ///     (b"foo", 1),
    /// ]);
    ///
    /// assert_eq!(TRIE.get(b"foo"), Some(1));
    /// assert_eq!(TRIE.get(b"bar"), Some(2));
    /// assert_eq!(TRIE.get(b"bazzoo"), Some(3));
    /// assert_eq!(TRIE.get(b"unknown"), None);
    /// ```
    ///
    /// Panics if strings are not sorted:
    ///
    /// ```compile_fail
    /// # use zerotrie::ZeroTrieSimpleAscii;
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_sorted_u8_tuples(&[
    ///     (b"foo", 1),
    ///     (b"bar", 2),
    ///     (b"bazzoo", 3),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too small:
    ///
    /// ```compile_fail
    /// # use zerotrie::ZeroTrieSimpleAscii;
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 15]> = ZeroTrieSimpleAscii::from_sorted_u8_tuples(&[
    ///     (b"bar", 2),
    ///     (b"bazzoo", 3),
    ///     (b"foo", 1),
    /// ]);
    /// ```
    ///
    /// Panics if capacity is too large:
    ///
    /// ```compile_fail
    /// # use zerotrie::{ZeroTrieSimpleAscii, AsciiStr};
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 20]> = ZeroTrieSimpleAscii::from_sorted_u8_tuples(&[
    ///     (b"bar", 2),
    ///     (b"bazzoo", 3),
    ///     (b"foo", 1),
    /// ]);
    /// ```
    pub const fn from_sorted_u8_tuples(tuples: &[(&[u8], usize)]) -> Self {
        use konst::*;
        let byte_str_slice = ByteStr::from_byte_slice_with_value(tuples);
        let result = ZeroTrieBuilderConst::<N>::from_tuple_slice::<100>(byte_str_slice);
        match result {
            Ok(s) => Self::from_store(s.take_or_panic()),
            Err(_) => panic!("Failed to build ZeroTrie"),
        }
    }

    /// **Const Constructor:** Creates an [`ZeroTrieSimpleAscii`] from a sorted slice of keys and values.
    ///
    /// This function needs to know the exact length of the resulting trie at compile time.
    ///
    /// Also see [`Self::from_sorted_u8_tuples`].
    ///
    /// # Panics
    ///
    /// Panics if `items` is not sorted, if `N` is not correct, or if any of the strings contain
    /// non-ASCII characters.
    ///
    /// # Examples
    ///
    /// Create a `const` ZeroTrieSimpleAscii at compile time:
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // The required capacity for this trie happens to be 17 bytes
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 17]> = ZeroTrieSimpleAscii::from_sorted_str_tuples(&[
    ///     ("bar", 2),
    ///     ("bazzoo", 3),
    ///     ("foo", 1),
    /// ]);
    ///
    /// assert_eq!(TRIE.get(b"foo"), Some(1));
    /// assert_eq!(TRIE.get(b"bar"), Some(2));
    /// assert_eq!(TRIE.get(b"bazzoo"), Some(3));
    /// assert_eq!(TRIE.get(b"unknown"), None);
    /// ```
    ///
    /// Panics if the strings are not ASCII:
    ///
    /// ```compile_fail
    /// # use zerotrie::ZeroTrieSimpleAscii;
    /// const TRIE: ZeroTrieSimpleAscii<[u8; 100]> = ZeroTrieSimpleAscii::from_sorted_str_tuples(&[
    ///     ("bár", 2),
    ///     ("båzzöo", 3),
    ///     ("foo", 1),
    /// ]);
    /// ```
    pub const fn from_sorted_str_tuples(tuples: &[(&str, usize)]) -> Self {
        use konst::*;
        let byte_str_slice = ByteStr::from_str_slice_with_value(tuples);
        // 100 is the value of `K`, the size of the lengths stack. If compile errors are
        // encountered, this number may need to be increased.
        let result = ZeroTrieBuilderConst::<N>::from_tuple_slice::<100>(byte_str_slice);
        match result {
            Ok(s) => Self::from_store(s.take_or_panic()),
            Err(_) => panic!("Failed to build ZeroTrie"),
        }
    }
}
