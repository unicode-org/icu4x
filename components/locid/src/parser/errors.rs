// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

/// List of parser errors that can be generated
/// while parsing [`LanguageIdentifier`](crate::LanguageIdentifier), [`Locale`](crate::Locale),
/// [`subtags`](crate::subtags) or [`extensions`](crate::extensions).
#[derive(Display, Debug, PartialEq, Copy, Clone)]
pub enum ParserError {
    /// Invalid language subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::str::FromStr;
    ///
    /// use icu::locid::subtags::Language;
    /// use icu::locid::ParserError;
    ///
    /// assert_eq!(Language::from_str("x2"), Err(ParserError::InvalidLanguage));
    /// ```
    #[displaydoc("The given language subtag is invalid")]
    InvalidLanguage,

    /// Invalid script, region or variant subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::str::FromStr;
    ///
    /// use icu::locid::subtags::Region;
    /// use icu::locid::ParserError;
    ///
    /// assert_eq!(Region::from_str("#@2X"), Err(ParserError::InvalidSubtag));
    /// ```
    #[displaydoc("Invalid subtag")]
    InvalidSubtag,

    /// Invalid extension subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::str::FromStr;
    ///
    /// use icu::locid::extensions::unicode::Key;
    /// use icu::locid::ParserError;
    ///
    /// assert_eq!(Key::from_str("#@2X"), Err(ParserError::InvalidExtension));
    /// ```
    #[displaydoc("Invalid extension")]
    InvalidExtension,
}

#[cfg(feature = "std")]
impl std::error::Error for ParserError {}
