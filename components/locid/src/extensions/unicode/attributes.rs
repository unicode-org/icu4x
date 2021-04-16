// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::Attribute;
use std::ops::Deref;

/// A set of [`Attribute`] elements as defined in [`Unicode Extension Attributes`].
///
/// [`Unicode Extension Attributes`]: https://unicode.org/reports/tr35/tr35.html#u_Extension
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::{Attribute, Attributes};
///
/// let attribute1: Attribute = "foobar".parse()
///     .expect("Failed to parse a variant subtag.");
///
/// let attribute2: Attribute = "testing".parse()
///     .expect("Failed to parse a variant subtag.");
/// let mut v = vec![attribute1, attribute2];
/// v.sort();
/// v.dedup();
///
/// let attributes: Attributes = Attributes::from_vec_unchecked(v);
/// assert_eq!(attributes.to_string(), "foobar-testing");
/// ```
///
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Attributes(Option<Box<[Attribute]>>);

impl Attributes {
    /// Returns a new empty set of attributes. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Attributes;
    ///
    /// assert_eq!(Attributes::new(), Attributes::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self(None)
    }

    /// A constructor which takes a pre-sorted list of [`Attribute`] elements.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Attribute, Attributes};
    ///
    /// let attribute1: Attribute = "foobar".parse()
    ///     .expect("Parsing failed.");
    /// let attribute2: Attribute = "testing".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![attribute1, attribute2];
    /// v.sort();
    /// v.dedup();
    ///
    /// let attributes = Attributes::from_vec_unchecked(v);
    /// ```
    ///
    /// Notice: For performance- and memory-constrained environments, it is recommended
    /// for the caller to use [`binary_search`](slice::binary_search) instead of [`sort`](slice::sort)
    /// and [`dedup`](Vec::dedup()).
    pub fn from_vec_unchecked(input: Vec<Attribute>) -> Self {
        if input.is_empty() {
            Self(None)
        } else {
            Self(Some(input.into_boxed_slice()))
        }
    }

    /// Empties the [`Attributes`] list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::{Attribute, Attributes};
    ///
    /// let attribute1: Attribute = "foobar".parse()
    ///     .expect("Parsing failed.");
    /// let attribute2: Attribute = "testing".parse()
    ///     .expect("Parsing failed.");
    /// let mut v = vec![attribute1, attribute2];
    ///
    /// let mut attributes: Attributes = Attributes::from_vec_unchecked(v);
    ///
    /// assert_eq!(attributes.to_string(), "foobar-testing");
    ///
    /// attributes.clear();
    ///
    /// assert_eq!(attributes.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl_writeable_for_subtag_list!(Attributes, "foobar", "testing");

impl Deref for Attributes {
    type Target = [Attribute];

    fn deref(&self) -> &[Attribute] {
        if let Some(ref data) = self.0 {
            data
        } else {
            &[]
        }
    }
}
