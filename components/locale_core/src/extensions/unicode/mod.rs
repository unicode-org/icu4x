// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Unicode Extensions provide information about user preferences in a given locale.
//!
//! The main struct for this extension is [`Unicode`] which contains [`Keywords`] and
//! [`Attributes`].
//!
//!
//! # Examples
//!
//! ```
//! use icu::locale::extensions::unicode::{attribute, key, value, Unicode};
//! use icu::locale::Locale;
//!
//! let loc: Locale = "en-US-u-foobar-hc-h12".parse().expect("Parsing failed.");
//!
//! assert_eq!(
//!     loc.extensions.unicode.keywords.get(&key!("hc")),
//!     Some(&value!("h12"))
//! );
//! assert!(loc
//!     .extensions
//!     .unicode
//!     .attributes
//!     .contains(&attribute!("foobar")));
//! ```
mod attribute;
mod attributes;
mod key;
mod keywords;
mod value;

use core::cmp::Ordering;
use core::str::FromStr;

#[doc(inline)]
pub use attribute::{attribute, Attribute};
pub use attributes::Attributes;
#[doc(inline)]
pub use key::{key, Key};
pub use keywords::Keywords;
#[doc(inline)]
pub use value::{value, Value};

use super::ExtensionType;
use crate::parser::ParserError;
use crate::parser::SubtagIterator;

pub(crate) const UNICODE_EXT_CHAR: char = 'u';
pub(crate) const UNICODE_EXT_STR: &str = "u";

/// Unicode Extensions provide information about user preferences in a given locale.
///
/// A list of [`Unicode BCP47 U Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Unicode extensions provide subtags that specify language and/or locale-based behavior
/// or refinements to language tags, according to work done by the Unicode Consortium.
/// (See [`RFC 6067`] for details).
///
/// [`Unicode BCP47 U Extensions`]: https://unicode.org/reports/tr35/#u_Extension
/// [`RFC 6067`]: https://www.ietf.org/rfc/rfc6067.txt
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
///
/// # Examples
///
/// ```
/// use icu::locale::extensions::unicode::{key, value};
/// use icu::locale::Locale;
///
/// let loc: Locale =
///     "de-u-hc-h12-ca-buddhist".parse().expect("Parsing failed.");
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&key!("ca")),
///     Some(&value!("buddhist"))
/// );
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
#[allow(clippy::exhaustive_structs)] // spec-backed stable datastructure
pub struct Unicode {
    /// The key-value pairs present in this locale extension, with each extension key subtag
    /// associated to its provided value subtag.
    pub keywords: Keywords,
    /// A canonically ordered sequence of single standalone subtags for this locale extension.
    pub attributes: Attributes,
}

impl Unicode {
    /// Returns a new empty map of Unicode extensions. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::extensions::unicode::Unicode;
    ///
    /// assert_eq!(Unicode::new(), Unicode::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            keywords: Keywords::new(),
            attributes: Attributes::new(),
        }
    }

    /// Returns [`true`] if there list of keywords and attributes is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::Locale;
    ///
    /// let loc: Locale = "en-US-u-foo".parse().expect("Parsing failed.");
    ///
    /// assert!(!loc.extensions.unicode.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    pub(crate) fn try_from_bytes(t: &[u8]) -> Result<Self, ParserError> {
        let mut iter = SubtagIterator::new(t);

        let ext = iter.next().ok_or(ParserError::InvalidExtension)?;
        if let ExtensionType::Unicode = ExtensionType::try_from_byte_slice(ext)? {
            return Self::try_from_iter(&mut iter);
        }

        Err(ParserError::InvalidExtension)
    }

    /// Clears all Unicode extension keywords and attributes, effectively removing
    /// the Unicode extension.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::locale::Locale;
    ///
    /// let mut loc: Locale =
    ///     "und-t-mul-u-hello-ca-buddhist-hc-h12".parse().unwrap();
    /// loc.extensions.unicode.clear();
    /// assert_eq!(loc, "und-t-mul".parse().unwrap());
    /// ```
    pub fn clear(&mut self) {
        self.keywords.clear();
        self.attributes.clear();
    }

    pub(crate) fn as_tuple(&self) -> (&Attributes, &Keywords) {
        (&self.attributes, &self.keywords)
    }

    /// Returns an ordering suitable for use in [`BTreeSet`].
    ///
    /// The ordering may or may not be equivalent to string ordering, and it
    /// may or may not be stable across ICU4X releases.
    ///
    /// [`BTreeSet`]: alloc::collections::BTreeSet
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        self.as_tuple().cmp(&other.as_tuple())
    }

    pub(crate) fn try_from_iter(iter: &mut SubtagIterator) -> Result<Self, ParserError> {
        let attributes = Attributes::try_from_iter(iter)?;
        let keywords = Keywords::try_from_iter(iter)?;

        // Ensure we've defined at least one attribute or keyword
        if attributes.is_empty() && keywords.is_empty() {
            return Err(ParserError::InvalidExtension);
        }

        Ok(Self {
            keywords,
            attributes,
        })
    }

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F, with_ext: bool) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        if !self.is_empty() {
            if with_ext {
                f(UNICODE_EXT_STR)?;
            }
            self.attributes.for_each_subtag_str(f)?;
            self.keywords.for_each_subtag_str(f)?;
        }
        Ok(())
    }
}

impl FromStr for Unicode {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::try_from_bytes(source.as_bytes())
    }
}

writeable::impl_display_with_writeable!(Unicode);

impl writeable::Writeable for Unicode {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        sink.write_char(UNICODE_EXT_CHAR)?;

        if !self.attributes.is_empty() {
            sink.write_char('-')?;
            writeable::Writeable::write_to(&self.attributes, sink)?;
        }
        if !self.keywords.is_empty() {
            sink.write_char('-')?;
            writeable::Writeable::write_to(&self.keywords, sink)?;
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        if self.is_empty() {
            return writeable::LengthHint::exact(0);
        }
        let mut result = writeable::LengthHint::exact(1);
        if !self.attributes.is_empty() {
            result += writeable::Writeable::writeable_length_hint(&self.attributes) + 1;
        }
        if !self.keywords.is_empty() {
            result += writeable::Writeable::writeable_length_hint(&self.keywords) + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_extension_fromstr() {
        let ue: Unicode = "u-foo-hc-h12".parse().expect("Failed to parse Unicode");
        assert_eq!(ue.to_string(), "u-foo-hc-h12");
    }
}
