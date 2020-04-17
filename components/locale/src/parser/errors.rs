use std::error::Error;
use std::fmt::{self, Display};

/// List of parser errors that can be generated
/// while parsing [`LanguageIdentifier`] or its [`subtags`].
///
/// [`LanguageIdentifier`]: ./struct.LanguageIdentifier.html
/// [`subtags`]: ./subtags/index.html
#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Invalid language subtag.
    ///
    /// # Example
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
    /// # Example
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
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            ParserError::InvalidLanguage => "The given language subtag is invalid",
            ParserError::InvalidSubtag => "Invalid subtag",
        };
        f.write_str(value)
    }
}
