// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::helpers::ShortVec;
use crate::parser::{get_subtag_iterator, ParserError};
use alloc::vec::Vec;
use core::ops::RangeInclusive;
use core::str::FromStr;
use tinystr::TinyAsciiStr;

/// A value used in a list of [`Keywords`](super::Keywords).
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
/// use icu::locid::extensions::unicode::Value;
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
pub struct Value(ShortVec<TinyAsciiStr<{ *VALUE_LENGTH.end() }>>);

const VALUE_LENGTH: RangeInclusive<usize> = 3..=8;
const TRUE_VALUE: TinyAsciiStr<8> = tinystr::tinystr!(8, "true");

impl Value {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Value;
    ///
    /// let value = Value::from_bytes(b"buddhist")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(&value.to_string(), "buddhist");
    /// ```
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParserError> {
        let mut v = ShortVec::new();

        if !input.is_empty() {
            for subtag in get_subtag_iterator(input) {
                let val = Self::subtag_from_bytes(subtag)?;
                if let Some(val) = val {
                    v.push(val);
                }
            }
        }
        Ok(Self(v))
    }

    /// Const constructor for when the value contains only a single subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Value;
    ///
    /// Value::try_from_single_subtag(b"buddhist").expect("valid subtag");
    /// Value::try_from_single_subtag(b"#####").expect_err("invalid subtag");
    /// Value::try_from_single_subtag(b"foo-bar").expect_err("not a single subtag");
    /// ```
    pub const fn try_from_single_subtag(subtag: &[u8]) -> Result<Self, ParserError> {
        match Self::subtag_from_bytes(subtag) {
            Err(_) => Err(ParserError::InvalidExtension),
            Ok(option) => Ok(Self::from_tinystr(option)),
        }
    }

    #[doc(hidden)]
    pub const fn from_tinystr(subtag: Option<TinyAsciiStr<8>>) -> Self {
        match subtag {
            None => Self(ShortVec::new()),
            Some(val) => {
                debug_assert!(val.is_ascii_alphanumeric());
                debug_assert!(!matches!(val, TRUE_VALUE));
                Self(ShortVec::new_single(val))
            }
        }
    }

    pub(crate) fn from_vec_unchecked(input: Vec<TinyAsciiStr<8>>) -> Self {
        Self(input.into())
    }

    #[doc(hidden)]
    pub const fn subtag_from_bytes(bytes: &[u8]) -> Result<Option<TinyAsciiStr<8>>, ParserError> {
        if *VALUE_LENGTH.start() > bytes.len() || *VALUE_LENGTH.end() < bytes.len() {
            return Err(ParserError::InvalidExtension);
        };
        match TinyAsciiStr::from_bytes(bytes) {
            Ok(TRUE_VALUE) => Ok(None),
            Ok(val) if val.is_ascii_alphanumeric() => Ok(Some(val)),
            _ => Err(ParserError::InvalidExtension),
        }
    }

    pub(crate) fn parse_subtag(t: &[u8]) -> Result<Option<TinyAsciiStr<8>>, ParserError> {
        let s = TinyAsciiStr::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
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

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        self.0.as_slice().iter().map(|t| t.as_str()).try_for_each(f)
    }
}

impl FromStr for Value {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_tinystr_list!(Value, "", "islamic", "civil");
