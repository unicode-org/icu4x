// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Debug;
use displaydoc::Display;
use icu_provider::DataError;

#[cfg(feature = "std")]
impl std::error::Error for TransliteratorError {}

/// A list of error outcomes for various operations in this module.
///
/// Re-exported as [`Error`](crate::Error).
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum TransliteratorError {
    /// An error originating inside of the [data provider](icu_provider).
    #[displaydoc("{0}")]
    Data(DataError),
    /// The requested transliterator is marked as internal-only.
    InternalOnly,
    #[cfg(feature = "datagen")]
    /// The variant returned by [`RuleBasedTransliterator::compile`](crate::provider::RuleBasedTransliterator::compile).
    Compile {
        /// offset is the index to an arbitrary byte in the last character in the source that makes sense
        /// to display as location for the error, e.g., the unexpected character itself or
        /// for an unknown property name the last character of the name.
        offset: Option<usize>,
        /// The type of compile error
        kind: CompileErrorKind,
    },
}

impl From<DataError> for TransliteratorError {
    fn from(e: DataError) -> Self {
        Self::Data(e)
    }
}

/// The kind of compile error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg(feature = "datagen")]
pub enum CompileErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// A reference to an unknown variable.
    UnknownVariable,
    /// The source is incomplete.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal(&'static str),
    /// The provided syntax is not supported by us. Please file an issue on GitHub if you need
    /// this feature.
    Unimplemented,
    /// The provided escape sequence is not a valid Unicode code point.
    InvalidEscape,
    /// The provided transform ID is invalid.
    InvalidId,
    /// The provided number is invalid, which likely means it's too big.
    InvalidNumber,
    /// Duplicate variable definition.
    DuplicateVariable,
    /// Invalid UnicodeSet syntax. See `icu_unicodeset_parser`'s [`ParseError`](icu_unicodeset_parser::ParseError).
    UnicodeSetError(icu_unicodeset_parser::ParseError),

    // errors originating from compilation step
    /// A global filter (forward or backward) in an unexpected position.
    UnexpectedGlobalFilter,
    /// A global filter (forward or backward) may not contain strings.
    GlobalFilterWithStrings,
    /// An element appeared on the source side of a rule, but that is prohibited.
    UnexpectedElementInSource(&'static str),
    /// An element appeared on the target side of a rule, but that is prohibited.
    UnexpectedElementInTarget(&'static str),
    /// An element appeared in a variable definition, but that is prohibited.
    UnexpectedElementInVariableDefinition(&'static str),
    /// The start anchor `^` was not placed at the beginning of a source.
    AnchorStartNotAtStart,
    /// The end anchor `$` was not placed at the end of a source.
    AnchorEndNotAtEnd,
    /// A variable that contains source-only matchers (e.g., UnicodeSets) was used on the target side.
    SourceOnlyVariable,
    /// No matching segment for this backreference was found.
    BackReferenceOutOfRange,
    /// The cursor is in an invalid position.
    InvalidCursor,
    /// Multiple cursors were defined.
    DuplicateCursor,
    /// There are too many special matchers/replacers/variables in the source.
    TooManySpecials,
}

#[cfg(feature = "datagen")]
impl CompileErrorKind {
    pub(crate) fn with_offset(self, offset: usize) -> TransliteratorError {
        TransliteratorError::Compile {
            offset: Some(offset),
            kind: self,
        }
    }
}

#[cfg(feature = "datagen")]
impl From<CompileErrorKind> for TransliteratorError {
    fn from(kind: CompileErrorKind) -> Self {
        TransliteratorError::Compile { offset: None, kind }
    }
}
