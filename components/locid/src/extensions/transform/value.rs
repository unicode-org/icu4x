// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::{get_subtag_iterator, ParserError};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::RangeInclusive;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
#[allow(missing_docs)] // TODO(#1028) - Add missing docs.
pub struct Value(Box<[TinyAsciiStr<{ *TYPE_LENGTH.end() }>]>);

const TYPE_LENGTH: RangeInclusive<usize> = 3..=8;
const TRUE_TVALUE: TinyAsciiStr<8> = tinystr::tinystr!(8, "true");

/// A value used in a list of [`Fields`](super::Fields).
///
/// The value has to be a sequence of one or more alphanumerical strings
/// separated by `-`.
/// Each part of the sequence has to be no shorter than three characters and no
/// longer than 8.
///
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::transform::Value;
///
/// let value1: Value = "hybrid".parse()
///     .expect("Failed to parse a Value.");
/// let value2: Value = "hybrid-foobar".parse()
///     .expect("Failed to parse a Value.");
///
/// assert_eq!(&value1.to_string(), "hybrid");
/// assert_eq!(&value2.to_string(), "hybrid-foobar");
/// ```
impl Value {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::transform::Value;
    ///
    /// let value = Value::from_bytes(b"hybrid")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(&value.to_string(), "hybrid");
    /// ```
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParserError> {
        let mut v = vec![];
        let mut has_value = false;

        for subtag in get_subtag_iterator(input) {
            if !Self::is_type_subtag(subtag) {
                return Err(ParserError::InvalidExtension);
            }
            has_value = true;
            let val =
                TinyAsciiStr::from_bytes(subtag).map_err(|_| ParserError::InvalidExtension)?;
            if val != TRUE_TVALUE {
                v.push(val);
            }
        }

        if !has_value {
            return Err(ParserError::InvalidExtension);
        }
        Ok(Self(v.into_boxed_slice()))
    }

    pub(crate) fn from_vec_unchecked(input: Vec<TinyAsciiStr<{ *TYPE_LENGTH.end() }>>) -> Self {
        Self(input.into_boxed_slice())
    }

    pub(crate) fn is_type_subtag(t: &[u8]) -> bool {
        TYPE_LENGTH.contains(&t.len()) && !t.iter().any(|c: &u8| !c.is_ascii_alphanumeric())
    }

    pub(crate) fn parse_subtag(
        t: &[u8],
    ) -> Result<Option<TinyAsciiStr<{ *TYPE_LENGTH.end() }>>, ParserError> {
        let s = TinyAsciiStr::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
        if !TYPE_LENGTH.contains(&t.len()) || !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidExtension);
        }

        let s = s.to_ascii_lowercase();

        if s == TRUE_TVALUE {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }

    pub(crate) fn iter_subtags(&self) -> impl Iterator<Item = &str> {
        let prefix: &[&str] = if self.0.is_empty() { &["true"] } else { &[] };
        prefix
            .iter()
            .copied()
            .chain(self.0.iter().map(|t| t.as_str()))
    }
}

impl FromStr for Value {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_tinystr_list!(Value, "true", "hybrid", "foobar");
