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
//! use icu::locid::{LanguageIdentifier, Locale};
//! use icu::locid::extensions::unicode::{Unicode, Key, Value, Attribute};
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
/// use icu::locid::Locale;
/// use icu::locid::extensions::unicode::{Key, Value};
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
    /// Returns a new empty map of Unicode extensions. Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::extensions::unicode::Unicode;
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
    /// use icu::locid::Locale;
    ///
    /// let loc: Locale = "en-US-u-foo".parse()
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
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for Unicode {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        sink.write_str("-u")?;
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

    fn write_len(&self) -> writeable::LengthHint {
        if self.is_empty() {
            return writeable::LengthHint::Exact(0);
        }
        let mut result = writeable::LengthHint::Exact(2);
        if !self.attributes.is_empty() {
            result += writeable::Writeable::write_len(&self.attributes) + 1;
        }
        if !self.keywords.is_empty() {
            result += writeable::Writeable::write_len(&self.keywords) + 1;
        }
        result
    }
}
