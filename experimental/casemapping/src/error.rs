// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "datagen")]
use core::char::DecodeUtf16Error;
use displaydoc::Display;
#[cfg(feature = "datagen")]
use icu_collections::codepointtrie::CodePointTrieError;

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[derive(Clone, Display, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred while building and validating the data
    #[displaydoc("Failed to validate: {0}")]
    #[cfg(any(feature = "serde", feature = "datagen"))]
    Validation(&'static str),
    /// A UTF16 string in the data contained an unpaired surrogate
    #[displaydoc("Unpaired surrogate")]
    #[cfg(feature = "datagen")]
    DecodeUtf16(DecodeUtf16Error),
    /// An error occurred while building the code point trie
    #[displaydoc("Failed to build code point trie: {0}")]
    #[cfg(feature = "datagen")]
    CodePointTrie(CodePointTrieError),
}

#[cfg(any(feature = "serde", feature = "datagen"))]
impl Error {
    /// Creates a new validation error with the given reason
    pub(crate) fn invalid<T>(reason: &'static str) -> Result<T, Self> {
        Err(Self::Validation(reason))
    }
}

#[cfg(feature = "datagen")]
impl From<DecodeUtf16Error> for Error {
    fn from(e: DecodeUtf16Error) -> Self {
        Error::DecodeUtf16(e)
    }
}

#[cfg(feature = "datagen")]
impl From<CodePointTrieError> for Error {
    fn from(e: CodePointTrieError) -> Self {
        Error::CodePointTrie(e)
    }
}
