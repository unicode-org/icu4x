// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// The kind of error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// A reference to an unknown variable.
    UnknownVariable,
    /// The source is incomplete.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal,
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
pub(crate) use ParseErrorKind as PEK;

impl ParseErrorKind {
    pub(crate) fn with_offset(self, offset: usize) -> ParseError {
        ParseError {
            offset: Some(offset),
            kind: self,
        }
    }
}

/// The error type returned by the `parse` functions in this crate.
#[allow(unused)] // TODO(#3736): remove when parse error message tests are added
#[derive(Debug, Clone, Copy)]
pub struct ParseError {
    // offset is the index to an arbitrary byte in the last character in the source that makes sense
    // to display as location for the error, e.g., the unexpected character itself or
    // for an unknown property name the last character of the name.
    offset: Option<usize>,
    kind: ParseErrorKind,
}

impl ParseError {
    pub(crate) fn or_with_offset(self, offset: usize) -> Self {
        match self.offset {
            Some(_) => self,
            None => ParseError {
                offset: Some(offset),
                ..self
            },
        }
    }
}

impl From<ParseErrorKind> for ParseError {
    fn from(kind: ParseErrorKind) -> Self {
        ParseError { offset: None, kind }
    }
}

impl From<icu_unicodeset_parser::ParseError> for ParseError {
    fn from(e: icu_unicodeset_parser::ParseError) -> Self {
        ParseError {
            offset: None,
            kind: PEK::UnicodeSetError(e),
        }
    }
}

pub(crate) type Result<T, E = ParseError> = core::result::Result<T, E>;
