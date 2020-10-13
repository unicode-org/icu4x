// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::error::Error;
use std::fmt::{self, Display};

/// List of parser errors that can be generated
/// while parsing [`LanguageIdentifier`], [`Locale`], [`subtags`] or [`extensions`].
///
/// [`LanguageIdentifier`]: ./struct.LanguageIdentifier.html
/// [`Locale`]: ./struct.Locale.html
/// [`subtags`]: ./subtags/index.html
/// [`extensions`]: ./extensions/index.html
#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Invalid language subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use icu_locale::subtags::Language;
    /// use icu_locale::ParserError;
    ///
    /// assert_eq!(Language::from_str("x2"), Err(ParserError::InvalidLanguage));
    /// ```
    InvalidLanguage,

    /// Invalid script, region or variant subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use icu_locale::subtags::Region;
    /// use icu_locale::ParserError;
    ///
    /// assert_eq!(Region::from_str("#@2X"), Err(ParserError::InvalidSubtag));
    /// ```
    InvalidSubtag,

    /// Invalid extension subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use icu_locale::extensions::unicode::Key;
    /// use icu_locale::ParserError;
    ///
    /// assert_eq!(Key::from_str("#@2X"), Err(ParserError::InvalidExtension));
    /// ```
    InvalidExtension,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ParserError::InvalidLanguage => "The given language subtag is invalid",
            ParserError::InvalidSubtag => "Invalid subtag",
            ParserError::InvalidExtension => "Invalid extension",
        };
        f.write_str(value)
    }
}
