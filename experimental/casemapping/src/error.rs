// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::char::DecodeUtf16Error;
use displaydoc::Display;
use icu_collections::codepointtrie::CodePointTrieError;

/// A list of possible errors for the [`CaseMapping`](crate::CaseMapping) struct
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[derive(Clone, Display, Debug, PartialEq)]
pub enum Error {
    /// An error occurred while building and validating the data
    #[displaydoc("Failed to validate: {0}")]
    Validation(&'static str),
    /// A UTF16 string in the data contained an unpaired surrogate
    #[displaydoc("Unpaired surrogate")]
    DecodeUtf16(DecodeUtf16Error),
    /// An error occurred while building the code point trie
    #[displaydoc("Failed to build code point trie: {0}")]
    CodePointTrie(CodePointTrieError),
}

impl Error {
    /// Creates a new validation error with the given reason
    pub(crate) fn invalid<T>(reason: &'static str) -> Result<T, Self> {
        Err(Self::Validation(reason))
    }
}

impl From<DecodeUtf16Error> for Error {
    fn from(e: DecodeUtf16Error) -> Self {
        Error::DecodeUtf16(e)
    }
}

impl From<CodePointTrieError> for Error {
    fn from(e: CodePointTrieError) -> Self {
        Error::CodePointTrie(e)
    }
}
