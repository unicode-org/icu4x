// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::ZeroTrieSimpleAscii;
use zerovec::ZeroSlice;
use zerovec::ule::AsULE;

/// The key type for storage in a [`DenseSparse2d`].
pub trait DenseSparse2dConfig {
	/// The type of the key fields, probably `str` or `[u8]`.
    type Key: ?Sized;
    /// The type of the value, probably `u16` or `u32`.
    type Value: AsULE;
    /// The type of trie for inner storage.
    type TrieBorrowed<'a>;
    /// The type of dense inner storage.
    type DenseBorrowed<'a>;
    /// Returns a string to delimit key0 and key1 for sparse storage.
    fn delimit<'a>(&self, key0: &'a Self::Key, key1: &'a Self::Key) -> &'_ Self::Key;
}

/// A key type that allows ASCII strings with a fixed delimiter. No strings
/// can contain the delimiter.
#[non_exhaustive]
pub struct AsciiWithFixedDelimiter {
    /// The delimiter, used between fields.
    pub delimiter: &'static str,
}

impl DenseSparse2dConfig for AsciiWithFixedDelimiter {
	type Key = str;
	type Value = u32;
	type TrieBorrowed<'a> = &'a ZeroTrieSimpleAscii<[u8]>;
	type DenseBorrowed<'a> = &'a ZeroSlice<u16>;
	fn delimit<'a>(&self, key0: &'a str, key1: &'a str) -> &'_ str {
		debug_assert!(!key0.contains(self.delimiter));
		debug_assert!(!key1.contains(self.delimiter));
		&self.delimiter
	}
}

/// A data structure designed for 2-dimensional keys that exhibit a mix of
/// density and sparsity.
pub struct DenseSparse2dBorrowed<'a, Config: DenseSparse2dConfig> {
    primary: Config::TrieBorrowed<'a>,
	secondary: Config::TrieBorrowed<'a>,
	dense: Config::DenseBorrowed<'a>,
	offsets: &'a ZeroSlice<Config::Value>,
    config: Config,
}

impl<'a, Config: DenseSparse2dConfig> DenseSparse2dBorrowed<'a, Config> {
    pub fn get(key0: &Config::Key, key1: &Config::Key) -> usize {
        todo!()
    }
}
