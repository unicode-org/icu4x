// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for building and reading from a ZeroTrie.
//!
//! These options are internal to the crate. A small selection of options
//! are exported by way of the different public types on this crate.

/// Whether to use the perfect hash function in the ZeroTrie.
pub(crate) enum PhfMode {
    /// Use binary search for all branch nodes.
    BinaryOnly,
    /// Use the perfect hash function for large branch nodes.
    UsePhf,
}

/// Whether to support non-ASCII data in the ZeroTrie.
pub(crate) enum AsciiMode {
    /// Support only ASCII, returning an error if non-ASCII is found.
    AsciiOnly,
    /// Support all data, creating span nodes for non-ASCII bytes.
    BinarySpans,
}

/// Whether to enforce a limit to the capacity of the ZeroTrie.
pub(crate) enum CapacityMode {
    /// Return an error if the trie requires a branch of more than 2^32 bytes.
    Normal,
    /// Construct the trie without returning an error.
    Extended,
}

/// How to handle strings with mixed ASCII case at a node, such as "abc" and "Abc"
pub(crate) enum CaseSensitivity {
    /// Allow all strings and sort them by byte value.
    Sensitive,
    /// Reject strings with different case and sort them as if `to_ascii_lowercase` is called.
    IgnoreCase,
}

pub(crate) struct ZeroTrieBuilderOptions {
    pub phf_mode: PhfMode,
    pub ascii_mode: AsciiMode,
    pub capacity_mode: CapacityMode,
    pub case_sensitivity: CaseSensitivity,
}

impl<S: ?Sized> crate::ZeroTrieSimpleAscii<S> {
    pub(crate) const OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::BinaryOnly,
        ascii_mode: AsciiMode::AsciiOnly,
        capacity_mode: CapacityMode::Normal,
        case_sensitivity: CaseSensitivity::Sensitive,
    };
}

impl<S: ?Sized> crate::ZeroAsciiIgnoreCaseTrie<S> {
    pub(crate) const OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::BinaryOnly,
        ascii_mode: AsciiMode::AsciiOnly,
        capacity_mode: CapacityMode::Normal,
        case_sensitivity: CaseSensitivity::IgnoreCase,
    };
}

impl<S: ?Sized> crate::ZeroTriePerfectHash<S> {
    pub(crate) const OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::UsePhf,
        ascii_mode: AsciiMode::BinarySpans,
        capacity_mode: CapacityMode::Normal,
        case_sensitivity: CaseSensitivity::Sensitive,
    };
}

impl<S: ?Sized> crate::ZeroTrieExtendedCapacity<S> {
    pub(crate) const OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::UsePhf,
        ascii_mode: AsciiMode::BinarySpans,
        capacity_mode: CapacityMode::Extended,
        case_sensitivity: CaseSensitivity::Sensitive,
    };
}
