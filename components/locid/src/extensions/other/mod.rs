// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Other Use Extensions is a list of extensions other than unicode,
//! transform or private.
//!
//! Those extensions are treated as a pass-through, and no Unicode related
//! behavior depends on them.
//!
//! The main struct for this extension is [`Other`] which is a list of [`Keys`].
//!
//! # Examples
//!
//! ```
//! use icu::locid::Locale;
//! use icu::locid::extensions::other::Other;
//!
//! let mut loc: Locale = "en-US-a-foo-faa".parse()
//!     .expect("Parsing failed.");
//! ```
//!
//! [`Keys`]: Key
mod key;
use alloc::boxed::Box;

use crate::parser::ParserError;
use alloc::vec::Vec;
use core::iter::Peekable;

pub use key::Key;

/// A list of [`Other Use Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Those extensions are treated as a pass-through, and no Unicode related
/// behavior depends on them.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::other::{Other, Key};
///
/// let key1: Key = "foo".parse()
///     .expect("Failed to parse a Key.");
/// let key2: Key = "bar".parse()
///     .expect("Failed to parse a Key.");
///
/// let other = Other::from_vec_unchecked(b'a', vec![key1, key2]);
/// assert_eq!(&other.to_string(), "-a-foo-bar");
/// ```
///
/// [`Other Use Extensions`]: https://unicode.org/reports/tr35/#other_extensions
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Other((u8, Box<[Key]>));

impl Other {
    /// A constructor which takes a pre-sorted list of [`Key`].
    ///
    /// # Panics
    ///
    /// Panics if `ext` is not ASCII alphabetic.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::other::{Other, Key};
    ///
    /// let key1: Key = "foo".parse()
    ///     .expect("Failed to parse a Key.");
    /// let key2: Key = "bar".parse()
    ///     .expect("Failed to parse a Key.");
    ///
    /// let other = Other::from_vec_unchecked(b'a', vec![key1, key2]);
    /// assert_eq!(&other.to_string(), "-a-foo-bar");
    /// ```
    pub fn from_vec_unchecked(ext: u8, input: Vec<Key>) -> Self {
        assert!(ext.is_ascii_alphabetic());
        Self((ext, input.into_boxed_slice()))
    }

    pub(crate) fn try_from_iter<'a>(
        ext: u8,
        iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        debug_assert!(ext.is_ascii_alphabetic());

        let mut keys = Vec::new();
        while let Some(subtag) = iter.peek() {
            if !Key::valid_key(subtag) {
                break;
            }
            if let Ok(key) = Key::from_bytes(subtag) {
                keys.push(key);
            }
            iter.next();
        }

        Ok(Self::from_vec_unchecked(ext, keys))
    }

    #[allow(missing_docs)] // TODO(#1028) - Add missing docs.
    pub fn get_ext(&self) -> char {
        self.0 .0 as char
    }

    pub(crate) fn for_each_subtag_str<E, F>(
        &self,
        f: &mut F,
    ) -> Result<(), E> where F: FnMut(&str) -> Result<(), E> {
        let (ext, keys) = &self.0;
        debug_assert!(self.0 .0.is_ascii_alphabetic());
        // Safety: ext is ascii_alphabetic, so it is valid UTF-8
        let ext_str = unsafe { core::str::from_utf8_unchecked(core::slice::from_ref(&ext)) };
        f(ext_str)?;
        keys.iter().map(|t| t.as_str()).try_for_each(f)
    }
}

impl core::fmt::Display for Other {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for Other {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        let (ext, keys) = &self.0;
        sink.write_char('-')?;
        sink.write_char(*ext as char)?;
        for key in keys.iter() {
            sink.write_char('-')?;
            writeable::Writeable::write_to(key, sink)?;
        }

        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = writeable::LengthHint::exact(2);
        for key in self.0 .1.iter() {
            result += writeable::Writeable::write_len(key) + 1;
        }
        result
    }
}
