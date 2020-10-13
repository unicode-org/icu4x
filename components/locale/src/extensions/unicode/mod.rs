// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Unicode Extensions provide information about user preferences in a given locale.
//!
//! The main struct for this extension is [`Unicode`] which contains [`Keywords`] and
//! [`Attributes`].
//!
//! [`Unicode`]: ./struct.Unicode.html
//! [`Keywords`]: ./struct.Keywords.html
//! [`Attributes`]: ./struct.Attributes.html
//!
//! # Examples
//!
//! ```
//! use icu_locale::{LanguageIdentifier, Locale};
//! use icu_locale::extensions::unicode::{Unicode, Key, Value, Attribute};
//!
//! let mut loc: Locale = "en-US-u-foobar-hc-h12".parse()
//!     .expect("Parsing failed.");
//!
//! let key: Key = "hc".parse()
//!     .expect("Parsing key failed.");
//! let value: Value = "h12".parse()
//!     .expect("Parsing value failed.");
//! let attribute: Attribute = "foobar".parse()
//!     .expect("Parsing attribute failed.");
//!
//! assert_eq!(loc.extensions.unicode.keywords.get(&key), Some(&value));
//! assert!(loc.extensions.unicode.attributes.contains(&attribute));
//!
//! assert_eq!(&loc.extensions.unicode.to_string(), "-u-foobar-hc-h12");
//! ```
mod attribute;
mod attributes;
mod key;
mod keywords;
mod value;

pub use attribute::Attribute;
pub use attributes::Attributes;
pub use key::Key;
pub use keywords::Keywords;
pub use value::Value;

use crate::parser::ParserError;

use std::iter::Peekable;

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
/// use icu_locale::Locale;
/// use icu_locale::extensions::unicode::{Key, Value};
///
/// let mut loc: Locale = "de-u-hc-h12-ca-buddhist".parse()
///     .expect("Parsing failed.");
///
/// let key: Key = "ca".parse().expect("Parsing key failed.");
/// let value: Value = "buddhist".parse().expect("Parsing value failed.");
/// assert_eq!(loc.extensions.unicode.keywords.get(key),
///            Some(&value));
/// ```
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash, PartialOrd, Ord)]
pub struct Unicode {
    pub keywords: Keywords,
    pub attributes: Attributes,
}

impl Unicode {
    /// Returns `true` if there list of keywords and attributes is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::Locale;
    ///
    /// let mut loc: Locale = "en-US-u-foo".parse()
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.extensions.unicode.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keywords.is_empty() && self.attributes.is_empty()
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut Peekable<impl Iterator<Item = &'a [u8]>>,
    ) -> Result<Self, ParserError> {
        let mut attributes = vec![];
        let mut keywords = vec![];

        let mut current_keyword = None;
        let mut current_type = vec![];

        while let Some(subtag) = iter.peek() {
            if let Ok(attr) = Attribute::from_bytes(subtag) {
                attributes.push(attr);
            } else {
                break;
            }
            iter.next();
        }

        while let Some(subtag) = iter.peek() {
            let slen = subtag.len();
            if slen == 2 {
                if let Some(kw) = current_keyword.take() {
                    keywords.push((kw, Value::from_vec_unchecked(current_type)));
                    current_type = vec![];
                }
                current_keyword = Some(Key::from_bytes(subtag)?);
            } else if current_keyword.is_some() {
                match Value::parse_subtag(subtag) {
                    Ok(Some(t)) => current_type.push(t),
                    Ok(None) => {}
                    Err(_) => break,
                }
            } else {
                break;
            }
            iter.next();
        }

        if let Some(kw) = current_keyword.take() {
            keywords.push((kw, Value::from_vec_unchecked(current_type)));
        }

        keywords.sort_by_key(|i| i.0);

        Ok(Self {
            keywords: Keywords::from_vec_unchecked(keywords),
            attributes: Attributes::from_vec_unchecked(attributes),
        })
    }
}

impl std::fmt::Display for Unicode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-u")?;

        if !self.attributes.is_empty() {
            write!(f, "-{}", self.attributes)?;
        }

        if !self.keywords.is_empty() {
            write!(f, "-{}", self.keywords)?;
        }
        Ok(())
    }
}
