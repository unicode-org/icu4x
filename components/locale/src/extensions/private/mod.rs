// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Private Use Extensions is a list of extensions intended for
//! private use.
//!
//! Those extensions are treated as a pass-through, and no Unicode related
//! behavior depends on them.
//!
//! The main struct for this extension is [`Private`] which is a list of [`Keys`].
//!
//! # Examples
//!
//! ```
//! use icu_locale::Locale;
//! use icu_locale::extensions::private::{Private, Key};
//!
//! let mut loc: Locale = "en-US-x-foo-faa".parse()
//!     .expect("Parsing failed.");
//!
//! let key: Key = "foo".parse()
//!     .expect("Parsing key failed.");
//! assert_eq!(loc.extensions.private.contains(&key), true);
//! assert_eq!(loc.extensions.private.iter().next(), Some(&key));
//! loc.extensions.private.clear();
//! assert_eq!(loc.to_string(), "en-US");
//! ```
//!
//! [`Private`]: ./struct.Private.html
//! [`Keys`]: ./struct.Key.html
mod key;
use std::ops::Deref;

pub use key::Key;

use crate::parser::ParserError;

/// A list of [`Private Use Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Those extensions are treated as a pass-through, and no Unicode related
/// behavior depends on them.
///
/// # Examples
///
/// ```
/// use icu_locale::extensions::private::{Private, Key};
///
/// let key1: Key = "foo".parse()
///     .expect("Failed to parse a Key.");
/// let key2: Key = "bar".parse()
///     .expect("Failed to parse a Key.");
///
/// let private = Private::from_vec_unchecked(vec![key1, key2]);
/// assert_eq!(&private.to_string(), "-x-foo-bar");
/// ```
///
/// [`Private Use Extensions`]: https://unicode.org/reports/tr35/#pu_extensions
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Private(Box<[Key]>);

impl Private {
    /// A constructor which takes a pre-sorted list of `Key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::private::{Private, Key};
    ///
    /// let key1: Key = "foo".parse()
    ///     .expect("Failed to parse a Key.");
    /// let key2: Key = "bar".parse()
    ///     .expect("Failed to parse a Key.");
    ///
    /// let private = Private::from_vec_unchecked(vec![key1, key2]);
    /// assert_eq!(&private.to_string(), "-x-foo-bar");
    /// ```
    pub fn from_vec_unchecked(v: Vec<Key>) -> Self {
        Self(v.into_boxed_slice())
    }

    /// Empties the `Private` list.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::extensions::private::{Private, Key};
    ///
    /// let key1: Key = "foo".parse()
    ///     .expect("Failed to parse a Key.");
    /// let key2: Key = "bar".parse()
    ///     .expect("Failed to parse a Key.");
    /// let mut private = Private::from_vec_unchecked(vec![key1, key2]);
    ///
    /// assert_eq!(&private.to_string(), "-x-foo-bar");
    ///
    /// private.clear();
    ///
    /// assert_eq!(&private.to_string(), "");
    /// ```
    pub fn clear(&mut self) {
        self.0 = Box::new([]);
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a [u8]>,
    ) -> Result<Self, ParserError> {
        let keys = iter
            .map(|subtag| Key::from_bytes(subtag))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(keys.into_boxed_slice()))
    }
}

impl std::fmt::Display for Private {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-x")?;

        for key in self.iter() {
            write!(f, "-{}", key)?;
        }
        Ok(())
    }
}

impl Deref for Private {
    type Target = [Key];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
