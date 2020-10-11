// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::parser::ParserError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use tinystr::TinyStr8;

/// A value used in a list of [`Keywords`].
///
/// The value has to be a sequence of one or more alphanumerical strings
/// separated by `-`.
/// Each part of the sequence has to be no shorter than three characters and no
/// longer than 8.
///
/// [`Keywords`]: ./struct.Keywords.html
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::unicode::Value;
///
/// let value1: Value = "gregory".parse()
///     .expect("Failed to parse a Value.");
/// let value2: Value = "islamic-civil".parse()
///     .expect("Failed to parse a Value.");
///
/// assert_eq!(&value1.to_string(), "gregory");
/// assert_eq!(&value2.to_string(), "islamic-civil");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Value(Box<[TinyStr8]>);

const VALUE_LENGTH: RangeInclusive<usize> = 3..=8;
const TRUE_VALUE: TinyStr8 = unsafe { TinyStr8::new_unchecked(1_702_195_828u64) }; // "true"

impl Value {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::Value;
    ///
    /// let value = Value::from_bytes(b"buddhist")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(&value.to_string(), "buddhist");
    /// ```
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParserError> {
        let mut v = vec![];

        if !input.is_empty() {
            for subtag in input.split(|c| *c == b'-' || *c == b'_') {
                if !Self::is_type_subtag(subtag) {
                    return Err(ParserError::InvalidExtension);
                }
                let val =
                    TinyStr8::from_bytes(subtag).map_err(|_| ParserError::InvalidExtension)?;
                if val != TRUE_VALUE {
                    v.push(val);
                }
            }
        }
        Ok(Value(v.into_boxed_slice()))
    }

    pub(crate) fn from_vec_unchecked(input: Vec<TinyStr8>) -> Self {
        Self(input.into_boxed_slice())
    }

    pub(crate) fn is_type_subtag(t: &[u8]) -> bool {
        VALUE_LENGTH.contains(&t.len()) && !t.iter().any(|c: &u8| !c.is_ascii_alphanumeric())
    }

    pub(crate) fn parse_subtag(t: &[u8]) -> Result<Option<TinyStr8>, ParserError> {
        let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
        if !VALUE_LENGTH.contains(&t.len()) || !s.is_ascii_alphanumeric() {
            return Err(ParserError::InvalidExtension);
        }

        let s = s.to_ascii_lowercase();

        if s == TRUE_VALUE {
            Ok(None)
        } else {
            Ok(Some(s))
        }
    }
}

impl FromStr for Value {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;

        for subtag in self.0.iter() {
            if first {
                write!(f, "{}", subtag)?;
                first = false;
            } else {
                write!(f, "-{}", subtag)?;
            }
        }
        Ok(())
    }
}
