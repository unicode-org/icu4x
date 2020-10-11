// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::Attribute;
use std::ops::Deref;

/// A set of [`Attribute`] elements as defined in [`Unicode Extension Attributes`].
///
/// [`Attribute`]: ./struct.Attribute.html
/// [`Unicode Extension Attributes`]: https://unicode.org/reports/tr35/tr35.html#u_Extension
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::unicode::{Attribute, Attributes};
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
/// [`Attribute`]: ./struct.Attribute.html
#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Attributes(Box<[Attribute]>);

impl Attributes {
    /// A constructor which takes a pre-sorted list of [`Attribute`] elements.
    ///
    /// [`Attribute`]: ./struct.Attribute.html
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::{Attribute, Attributes};
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
    /// Notice: For performance and memory constraint environments, it is recommended
    /// for the caller to use `slice::binary_search` instead of `sort` and `dedup`.
    pub fn from_vec_unchecked(input: Vec<Attribute>) -> Self {
        Self(input.into_boxed_slice())
    }

    /// Empties the `Attributes` list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::unicode::{Attribute, Attributes};
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
        self.0 = Box::new([]);
    }
}

impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut initial = true;
        for variant in self.0.iter() {
            if initial {
                initial = false;
            } else {
                f.write_str("-")?;
            }
            variant.fmt(f)?;
        }
        Ok(())
    }
}

impl Deref for Attributes {
    type Target = [Attribute];

    fn deref(&self) -> &[Attribute] {
        &self.0
    }
}
