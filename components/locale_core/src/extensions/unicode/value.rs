// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::{ParseError, SubtagIterator};
use crate::shortvec::ShortBoxSlice;
use crate::subtags::{subtag, Subtag};
use alloc::vec::Vec;
use core::str::FromStr;

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
/// use icu::locale::extensions::unicode::{value, Value};
/// use writeable::assert_writeable_eq;
///
/// assert_writeable_eq!(value!("gregory"), "gregory");
/// assert_writeable_eq!(
///     "islamic-civil".parse::<Value>().unwrap(),
///     "islamic-civil"
/// );
///
/// // The value "true" has the special, empty string representation
/// assert_eq!(value!("true").to_string(), "");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Default)]
pub struct Value(ShortBoxSlice<Subtag>);

const TRUE_VALUE: Subtag = subtag!("true");

impl Value {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Value;
    ///
    /// Value::try_from_bytes(b"buddhist").expect("Parsing failed.");
    /// ```
    pub fn try_from_bytes(input: &[u8]) -> Result<Self, ParseError> {
        let mut v = ShortBoxSlice::new();

        if !input.is_empty() {
            for chunk in SubtagIterator::new(input) {
                let subtag = Subtag::try_from_bytes(chunk)?;
                if subtag != TRUE_VALUE {
                    v.push(subtag);
                }
            }
        }
        Ok(Self(v))
    }

    #[doc(hidden)]
    pub const fn as_single_subtag(&self) -> Option<&Subtag> {
        self.0.single()
    }

    #[doc(hidden)]
    pub fn as_subtags_slice(&self) -> &[Subtag] {
        &self.0
    }

    #[doc(hidden)]
    pub const fn from_subtag(subtag: Option<Subtag>) -> Self {
        match subtag {
            None | Some(TRUE_VALUE) => Self(ShortBoxSlice::new()),
            Some(val) => Self(ShortBoxSlice::new_single(val)),
        }
    }

    /// A constructor which takes a pre-sorted list of [`Value`] elements.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Value;
    /// use icu::locale::subtags::subtag;
    ///
    /// let subtag1 = subtag!("foobar");
    /// let subtag2 = subtag!("testing");
    /// let mut v = vec![subtag1, subtag2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let value = Value::from_vec_unchecked(v);
    /// ```
    ///
    /// Notice: For performance- and memory-constrained environments, it is recommended
    /// for the caller to use [`binary_search`](slice::binary_search) instead of [`sort`](slice::sort)
    /// and [`dedup`](Vec::dedup()).
    pub fn from_vec_unchecked(input: Vec<Subtag>) -> Self {
        Self(input.into())
    }

    pub(crate) fn from_short_slice_unchecked(input: ShortBoxSlice<Subtag>) -> Self {
        Self(input)
    }

    pub(crate) fn parse_subtag(t: &[u8]) -> Result<Option<Subtag>, ParseError> {
        Self::parse_subtag_from_bytes_manual_slice(t, 0, t.len())
    }

    pub(crate) const fn parse_subtag_from_bytes_manual_slice(
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<Subtag>, ParseError> {
        match Subtag::try_from_bytes_manual_slice(bytes, start, end) {
            Ok(TRUE_VALUE) => Ok(None),
            Ok(s) => Ok(Some(s)),
            Err(_) => Err(ParseError::InvalidSubtag),
        }
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        self.0.iter().map(Subtag::as_str).try_for_each(f)
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::try_from_bytes(source.as_bytes())
    }
}

impl_writeable_for_subtag_list!(Value, "islamic", "civil");

/// A macro allowing for compile-time construction of valid Unicode [`Value`] subtag.
///
/// The macro only supports single-subtag values.
///
/// # Examples
///
/// ```
/// use icu::locale::extensions::unicode::{key, value};
/// use icu::locale::Locale;
///
/// let loc: Locale = "de-u-ca-buddhist".parse().unwrap();
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&key!("ca")),
///     Some(&value!("buddhist"))
/// );
/// ```
///
/// [`Value`]: crate::extensions::unicode::Value
#[macro_export]
#[doc(hidden)] // macro
macro_rules! extensions_unicode_value {
    ($value:literal) => {{
        // What we want:
        // const R: $crate::extensions::unicode::Value =
        //     match $crate::extensions::unicode::Value::try_from_single_subtag($value.as_bytes()) {
        //         Ok(r) => r,
        //         #[allow(clippy::panic)] // const context
        //         _ => panic!(concat!("Invalid Unicode extension value: ", $value)),
        //     };
        // Workaround until https://github.com/rust-lang/rust/issues/73255 lands:
        const R: $crate::extensions::unicode::Value =
            $crate::extensions::unicode::Value::from_subtag(
                match $crate::subtags::Subtag::try_from_bytes($value.as_bytes()) {
                    Ok(r) => Some(r),
                    _ => panic!(concat!("Invalid Unicode extension value: ", $value)),
                },
            );
        R
    }};
}
#[doc(inline)]
pub use extensions_unicode_value as value;
