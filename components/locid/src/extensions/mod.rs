// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Unicode Extensions provide a mechanism to extend the [`LanguageIdentifier`] with
//! additional bits of information - a combination of a [`LanguageIdentifier`] and [`Extensions`]
//! is called [`Locale`].
//!
//! There are four types of extensions:
//!
//!  * [`Unicode Extensions`] - marked as `u`.
//!  * [`Transform Extensions`] - marked as `t`.
//!  * [`Private Use Extensions`] - marked as `x`.
//!  * Other extensions - marked as any `a-z` except of `u`, `t` and `x`.
//!
//! One can think of extensions as a bag of extra information on top of basic 4 [`subtags`].
//!
//! Notice: `Other` extension type is currently not supported.
//!
//! # Examples
//!
//! ```
//! use icu::locid::Locale;
//! use icu::locid::extensions::unicode::{Key, Value};
//!
//! let loc: Locale = "en-US-u-ca-buddhist-t-en-US-h0-hybrid-x-foo".parse()
//!     .expect("Failed to parse.");
//!
//! assert_eq!(loc.id.language, "en");
//! assert_eq!(loc.id.script, None);
//! assert_eq!(loc.id.region, Some("US".parse().unwrap()));
//! assert_eq!(loc.id.variants.len(), 0);
//!
//!
//! let key: Key = "ca".parse().expect("Parsing key failed.");
//! let value: Value = "buddhist".parse().expect("Parsing value failed.");
//! assert_eq!(loc.extensions.unicode.keywords.get(key),
//!            Some(&value));
//! ```
//!
//! [`LanguageIdentifier`]: super::LanguageIdentifier
//! [`Locale`]: super::Locale
//! [`subtags`]: super::subtags
//! [`Unicode Extensions`]: unicode
//! [`Transform Extensions`]: transform
//! [`Private Use Extensions`]: private
pub mod private;
pub mod transform;
pub mod unicode;

pub use private::Private;
pub use transform::Transform;
pub use unicode::Unicode;

use std::iter::Peekable;

use crate::parser::ParserError;

/// Defines the type of extension.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy)]
pub enum ExtensionType {
    /// Transform Extension Type marked as `t`.
    Transform,
    /// Unicode Extension Type marked as `u`.
    Unicode,
    /// Private Extension Type marked as `x`.
    Private,
}

impl ExtensionType {
    pub fn from_byte(key: u8) -> Result<Self, ParserError> {
        let key = key.to_ascii_lowercase();
        match key {
            b'u' => Ok(Self::Unicode),
            b't' => Ok(Self::Transform),
            b'x' => Ok(Self::Private),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

/// A map of extensions associated with a given [`Locale`](crate::Locale).
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Extensions {
    pub unicode: Unicode,
    pub transform: Transform,
    pub private: Private,
}

impl Extensions {
    /// Returns a new empty map of extensions. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::Extensions;
    ///
    /// assert_eq!(Extensions::new(), Extensions::default());
    /// ```
    #[inline]
    pub const fn new() -> Self {
        Self {
            unicode: Unicode::new(),
            transform: Transform::new(),
            private: Private::new(),
        }
    }

    /// Returns whether there are no extensions present.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    ///
    /// let loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.unicode.is_empty() && self.transform.is_empty() && self.private.is_empty()
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut unicode = None;
        let mut transform = None;
        let mut private = None;

        let mut st = iter.next();
        while let Some(subtag) = st {
            match subtag.get(0).map(|b| ExtensionType::from_byte(*b)) {
                Some(Ok(ExtensionType::Unicode)) => {
                    unicode = Some(Unicode::try_from_iter(iter)?);
                }
                Some(Ok(ExtensionType::Transform)) => {
                    transform = Some(Transform::try_from_iter(iter)?);
                }
                Some(Ok(ExtensionType::Private)) => {
                    private = Some(Private::try_from_iter(iter)?);
                }
                None => {}
                _ => return Err(ParserError::InvalidExtension),
            }

            st = iter.next();
        }

        Ok(Self {
            unicode: unicode.unwrap_or_default(),
            transform: transform.unwrap_or_default(),
            private: private.unwrap_or_default(),
        })
    }
}

impl std::fmt::Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for Extensions {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        // Alphabetic by singleton (t, u, x)
        writeable::Writeable::write_to(&self.transform, sink)?;
        writeable::Writeable::write_to(&self.unicode, sink)?;
        writeable::Writeable::write_to(&self.private, sink)?;
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = writeable::LengthHint::Exact(0);
        result += writeable::Writeable::write_len(&self.transform);
        result += writeable::Writeable::write_len(&self.unicode);
        result += writeable::Writeable::write_len(&self.private);
        result
    }
}
