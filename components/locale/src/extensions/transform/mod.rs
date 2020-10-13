// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Transform Extensions provide information on content transformations in a given locale.
//!
//! The main struct for this extension is [`Transform`] which contains [`Fields`] and an
//! optional [`LanguageIdentifier`].
//!
//! [`Transform`]: ./struct.Transform.html
//! [`Fields`]: ./struct.Fields.html
//! [`LanguageIdentifier`]: ../../struct.LanguageIdentifier.html
//!
//! # Examples
//!
//! ```
//! use icu_locale::{LanguageIdentifier, Locale};
//! use icu_locale::extensions::transform::{Transform, Fields, Key, Value};
//!
//! let mut loc: Locale = "en-US-t-es-AR-h0-hybrid".parse()
//!     .expect("Parsing failed.");
//!
//! let lang: LanguageIdentifier = "es-AR".parse()
//!     .expect("Parsing LanguageIdentifier failed.");
//!
//! let key: Key = "h0".parse()
//!     .expect("Parsing key failed.");
//! let value: Value = "hybrid".parse()
//!     .expect("Parsing value failed.");
//!
//! assert_eq!(loc.extensions.transform.lang, Some(lang));
//! assert!(loc.extensions.transform.fields.contains_key(key));
//! assert_eq!(loc.extensions.transform.fields.get(key), Some(&value));
//!
//! assert_eq!(&loc.extensions.transform.to_string(), "-t-es-AR-h0-hybrid");
//! ```
mod fields;
mod key;
mod value;

pub use fields::Fields;
pub use key::Key;
pub use value::Value;

use crate::parser::{parse_language_identifier_from_iter, ParserError, ParserMode};
use crate::subtags::Language;
use crate::LanguageIdentifier;

use std::iter::Peekable;

/// A list of [`Unicode BCP47 T Extensions`] as defined in [`Unicode Locale
/// Identifier`] specification.
///
/// Transform extension carries information about source language or script of
/// transformed content, including content that has been transliterated, transcribed,
/// or translated, or in some other way influenced by the source (See [`RFC 6497`] for details).
///
/// # Examples
///
/// ```
/// use icu_locale::{Locale, LanguageIdentifier};
/// use icu_locale::extensions::transform::{Key, Value};
///
/// let mut loc: Locale = "de-t-en-US-h0-hybrid".parse()
///     .expect("Parsing failed.");
///
/// let en_us: LanguageIdentifier = "en-US".parse()
///     .expect("Parsing failed.");
///
/// assert_eq!(loc.extensions.transform.lang, Some(en_us));
/// let key: Key = "h0".parse().expect("Parsing key failed.");
/// let value: Value = "hybrid".parse().expect("Parsing value failed.");
/// assert_eq!(
///     loc.extensions.transform.fields.get(&key),
///     Some(&value)
/// );
/// ```
/// [`Unicode BCP47 T Extensions`]: https://unicode.org/reports/tr35/#t_Extension
/// [`RFC 6497`]: https://www.ietf.org/rfc/rfc6497.txt
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Transform {
    pub lang: Option<LanguageIdentifier>,
    pub fields: Fields,
}

impl Transform {
    /// Returns `true` if there are no tfields and no tlang in
    /// the `TransformExtensionList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::Locale;
    ///
    /// let mut loc: Locale = "en-US-t-es-AR".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.transform.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.lang.is_none() && self.fields.is_empty()
    }

    pub(crate) fn try_from_iter<'a>(
        mut iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut tlang = None;
        let mut tfields = vec![];

        if let Some(subtag) = iter.peek() {
            if Language::from_bytes(subtag).is_ok() {
                tlang = Some(parse_language_identifier_from_iter(
                    &mut iter,
                    ParserMode::Partial,
                )?);
            }
        }

        let mut current_tkey = None;
        let mut current_tvalue = vec![];

        while let Some(subtag) = iter.peek() {
            if let Some(tkey) = current_tkey {
                if let Ok(val) = Value::parse_subtag(subtag) {
                    current_tvalue.push(val);
                } else if let Ok(new_key) = Key::from_bytes(subtag) {
                    tfields.push((
                        tkey,
                        Value::from_vec_unchecked(
                            current_tvalue.drain(..).filter_map(|s| s).collect(),
                        ),
                    ));
                    current_tkey = Some(new_key);
                } else if current_tvalue.is_empty() {
                    return Err(ParserError::InvalidExtension);
                } else {
                    break;
                }
            } else if let Ok(tkey) = Key::from_bytes(subtag) {
                current_tkey = Some(tkey);
            } else {
                break;
            }

            iter.next();
        }

        if let Some(tkey) = current_tkey {
            if current_tvalue.is_empty() {
                return Err(ParserError::InvalidExtension);
            }
            tfields.push((
                tkey,
                Value::from_vec_unchecked(current_tvalue.into_iter().filter_map(|s| s).collect()),
            ));
        }

        tfields.sort_by_key(|f| f.0);

        Ok(Self {
            lang: tlang,
            fields: Fields::from_vec_unchecked(tfields),
        })
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-t")?;

        if let Some(lang) = &self.lang {
            write!(f, "-{}", lang)?;
        }

        if !self.fields.is_empty() {
            write!(f, "-{}", self.fields)?;
        }

        Ok(())
    }
}
