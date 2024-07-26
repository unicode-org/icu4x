// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::{ParseError, SubtagIterator};
use crate::shortvec::{ShortBoxSlice, ShortBoxSliceIntoIter};
use crate::subtags::{subtag, Subtag};
use alloc::vec::Vec;
use core::str::FromStr;
use writeable::Writeable;

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
    /// A constructor which str slice, parses it and
    /// produces a well-formed [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Value;
    ///
    /// Value::try_from_str("buddhist").expect("Parsing failed.");
    /// ```
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, ParseError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, ParseError> {
        let mut v = ShortBoxSlice::new();

        if !code_units.is_empty() {
            for chunk in SubtagIterator::new(code_units) {
                let subtag = Subtag::try_from_utf8(chunk)?;
                if subtag != TRUE_VALUE {
                    v.push(subtag);
                }
            }
        }
        Ok(Self(v))
    }

    /// Returns a reference to a single [`Subtag`] if the [`Value`] contains exactly one
    /// subtag, or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Value;
    /// use std::str::FromStr;
    ///
    /// let value1 = Value::from_str("foo")
    ///     .expect("failed to parse a Value");
    /// let value2 = Value::from_str("foo-bar")
    ///     .expect("failed to parse a Value");
    ///
    /// assert!(value1.as_single_subtag().is_some());
    /// assert!(value2.as_single_subtag().is_none());
    /// ```
    pub const fn as_single_subtag(&self) -> Option<&Subtag> {
        self.0.single()
    }

    /// Destructs into a single [`Subtag`] if the [`Value`] contains exactly one
    /// subtag, or returns `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Value;
    /// use std::str::FromStr;
    ///
    /// let value1 = Value::from_str("foo")
    ///     .expect("failed to parse a Value");
    /// let value2 = Value::from_str("foo-bar")
    ///     .expect("failed to parse a Value");
    ///
    /// assert!(value1.into_single_subtag().is_some());
    /// assert!(value2.into_single_subtag().is_none());
    /// ```
    pub fn into_single_subtag(self) -> Option<Subtag> {
        self.0.into_single()
    }

    #[doc(hidden)]
    pub fn as_subtags_slice(&self) -> &[Subtag] {
        &self.0
    }

    /// Appends a subtag to the back of a [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::Value,
    ///     subtags::subtag,
    /// };
    ///
    /// let mut v = Value::default();
    /// v.push_subtag(subtag!("foo"));
    /// v.push_subtag(subtag!("bar"));
    /// assert_eq!(v, "foo-bar");
    /// ```
    pub fn push_subtag(&mut self, subtag: Subtag) {
        self.0.push(subtag);
    }

    /// Returns the number of subtags in the [`Value`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::Value,
    ///     subtags::subtag,
    /// };
    ///
    /// let mut v = Value::default();
    /// assert_eq!(v.subtag_count(), 0);
    /// v.push_subtag(subtag!("foo"));
    /// assert_eq!(v.subtag_count(), 1);
    /// ```
    pub fn subtag_count(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the Value has no subtags.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::Value,
    ///     subtags::subtag,
    /// };
    ///
    /// let mut v = Value::default();
    /// assert_eq!(v.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Removes and returns the subtag at position `index` within the value,
    /// shifting all subtags after it to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::Value,
    ///     subtags::subtag,
    /// };
    /// let mut v = Value::default();
    /// v.push_subtag(subtag!("foo"));
    /// v.push_subtag(subtag!("bar"));
    /// v.push_subtag(subtag!("baz"));
    ///
    /// assert_eq!(v.remove_subtag(1), Some(subtag!("bar")));
    /// assert_eq!(v, "foo-baz");
    /// ```
    pub fn remove_subtag(&mut self, idx: usize) -> Option<Subtag> {
        if self.0.len() < idx {
            None
        } else {
            let item = self.0.remove(idx);
            Some(item)
        }
    }

    /// Returns a reference to a subtag at index.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::{
    ///     extensions::unicode::Value,
    ///     subtags::subtag,
    /// };
    /// let mut v = Value::default();
    /// v.push_subtag(subtag!("foo"));
    /// v.push_subtag(subtag!("bar"));
    /// v.push_subtag(subtag!("baz"));
    ///
    /// assert_eq!(v.get_subtag(1), Some(&subtag!("bar")));
    /// assert_eq!(v.get_subtag(3), None);
    /// ```
    pub fn get_subtag(&self, idx: usize) -> Option<&Subtag> {
        self.0.get(idx)
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

    pub(crate) const fn parse_subtag_from_utf8(t: &[u8]) -> Result<Option<Subtag>, ParseError> {
        match Subtag::try_from_utf8(t) {
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

impl IntoIterator for Value {
    type Item = Subtag;

    type IntoIter = ShortBoxSliceIntoIter<Subtag>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Subtag> for Value {
    fn from_iter<T: IntoIterator<Item = Subtag>>(iter: T) -> Self {
        Self(ShortBoxSlice::from_iter(iter))
    }
}

impl Extend<Subtag> for Value {
    fn extend<T: IntoIterator<Item = Subtag>>(&mut self, iter: T) {
        for i in iter {
            self.0.push(i);
        }
    }
}

impl FromStr for Value {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.writeable_cmp_bytes(other.as_bytes()).is_eq()
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
                match $crate::subtags::Subtag::try_from_utf8($value.as_bytes()) {
                    Ok(r) => Some(r),
                    _ => panic!(concat!("Invalid Unicode extension value: ", $value)),
                },
            );
        R
    }};
}
#[doc(inline)]
pub use extensions_unicode_value as value;
