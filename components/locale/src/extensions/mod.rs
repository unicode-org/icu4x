// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Unicode Extensions provide a mechanism to extend the [`LanguageIdentifier`] with
//! additional bits of information - a combination of a [`LanguageIdentifier`] and `Extensions`
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
//! use icu_locale::Locale;
//! use icu_locale::extensions::unicode::{Key, Value};
//!
//! let loc: Locale = "en-US-u-ca-buddhist-t-en-US-h0-hybrid-x-foo".parse()
//!     .expect("Failed to parse.");
//!
//! assert_eq!(loc.language, "en");
//! assert_eq!(loc.script, None);
//! assert_eq!(loc.region, Some("US".parse().unwrap()));
//! assert_eq!(loc.variants.len(), 0);
//!
//!
//! let key: Key = "ca".parse().expect("Parsing key failed.");
//! let value: Value = "buddhist".parse().expect("Parsing value failed.");
//! assert_eq!(loc.extensions.unicode.keywords.get(key),
//!            Some(&value));
//! ```
//!
//! [`LanguageIdentifier`]: ../struct.LanguageIdentifier.html
//! [`Locale`]: ../struct.Locale.html
//! [`subtags`]: ../subtags/index.html
//! [`Unicode Extensions`]: ./unicode/index.html
//! [`Transform Extensions`]: ./transform/index.html
//! [`Private Use Extensions`]: ./private/index.html
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
            b'u' => Ok(ExtensionType::Unicode),
            b't' => Ok(ExtensionType::Transform),
            b'x' => Ok(ExtensionType::Private),
            _ => Err(ParserError::InvalidExtension),
        }
    }
}

/// A map of extensions associated with a given `Locale.
#[derive(Debug, Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Extensions {
    pub unicode: Unicode,
    pub transform: Transform,
    pub private: Private,
}

impl Extensions {
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
        // Alphabetic by singleton (t, u, x)
        write!(f, "{}{}{}", self.transform, self.unicode, self.private)?;

        Ok(())
    }
}
