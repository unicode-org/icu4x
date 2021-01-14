// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::fmt::Write;
use std::str::FromStr;

use crate::parser::{get_subtag_iterator, parse_language_identifier, ParserError, ParserMode};
use crate::subtags;

/// `LanguageIdentifier` is a core struct representing a [`Unicode BCP47 Language Identifier`].
///
/// # Examples
///
/// ```
/// use icu_locid::LanguageIdentifier;
///
/// let li: LanguageIdentifier = "en-US".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(li.language, "en");
/// assert_eq!(li.script, None);
/// assert_eq!(li.region.unwrap(), "US");
/// assert_eq!(li.variants.len(), 0);
/// assert_eq!(li, "en-US");
/// ```
///
/// # Parsing
///
/// Unicode recognizes three levels of standard conformance for any language identifier:
///
///  * *well-formed* - syntactically correct
///  * *valid* - well-formed and only uses registered language, region, script and variant subtags...
///  * *canonical* - valid and no deprecated codes or structure.
///
/// At the moment parsing normalizes a well-formed language identifier converting
/// `_` separators to `-` and adjusting casing to conform to the Unicode standard.
///
/// Any bogus subtags will cause the parsing to fail with an error.
/// No subtag validation is performed.
///
/// # Examples
///
/// ```
/// use icu_locid::LanguageIdentifier;
///
/// let li: LanguageIdentifier = "eN_latn_Us-Valencia".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(li.language, "en");
/// assert_eq!(li.script.unwrap(), "Latn");
/// assert_eq!(li.region.unwrap(), "US");
/// assert_eq!(li.variants.get(0).unwrap(), "valencia");
/// ```
///
/// [`Unicode BCP47 Language Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier
#[derive(Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct LanguageIdentifier {
    /// Language subtag of the LanguageIdentifier
    pub language: subtags::Language,
    /// Script subtag of the LanguageIdentifier
    pub script: Option<subtags::Script>,
    /// Region subtag of the LanguageIdentifier
    pub region: Option<subtags::Region>,
    /// Variant subtags of the LanguageIdentifier
    pub variants: subtags::Variants,
}

impl LanguageIdentifier {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `LanguageIdentifier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::LanguageIdentifier;
    ///
    /// let li = LanguageIdentifier::from_bytes(b"en-US")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(li.to_string(), "en-US");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_language_identifier(v, ParserMode::LanguageIdentifier)
    }

    /// A constructor which takes a utf8 slice which may contain extension keys,
    /// parses it and produces a well-formed `LanguageIdentifier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::LanguageIdentifier;
    ///
    /// let li = LanguageIdentifier::from_locale_bytes(b"en-US-x-posix")
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(li.to_string(), "en-US");
    /// ```
    ///
    /// This method should be used for input that may be a locale identifier.
    /// All extensions will be lost.
    pub fn from_locale_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_language_identifier(v, ParserMode::Locale)
    }

    /// Returns the default undefined language "und". Same as `Default`, but is `const`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_locid::LanguageIdentifier;
    ///
    /// assert_eq!(LanguageIdentifier::und(), LanguageIdentifier::default());
    /// assert_eq!("und", LanguageIdentifier::und().to_string());
    /// ```
    #[inline]
    pub const fn und() -> Self {
        Self {
            language: subtags::Language::und(),
            script: None,
            region: None,
            variants: subtags::Variants::const_default(),
        }
    }

    /// This is a best-effort operation that performs all available levels of canonicalization.
    ///
    /// At the moment the operation will normalize casing and the separator, but in the future
    /// it may also validate and update from deprecated subtags to canonical ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::LanguageIdentifier;
    ///
    /// assert_eq!(LanguageIdentifier::canonicalize("pL_latn_pl"), Ok("pl-Latn-PL".to_string()));
    /// ```
    pub fn canonicalize<S: AsRef<[u8]>>(input: S) -> Result<String, ParserError> {
        let lang_id = Self::from_bytes(input.as_ref())?;
        Ok(lang_id.to_string())
    }
}

impl std::fmt::Debug for LanguageIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl FromStr for LanguageIdentifier {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl std::fmt::Display for LanguageIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.language.fmt(f)?;
        if let Some(ref script) = self.script {
            f.write_char('-')?;
            script.fmt(f)?;
        }
        if let Some(ref region) = self.region {
            f.write_char('-')?;
            region.fmt(f)?;
        }
        if !self.variants.is_empty() {
            f.write_char('-')?;
            self.variants.fmt(f)?;
        }
        Ok(())
    }
}

macro_rules! subtag_matches {
    ($T:ty, $iter:ident, $expected:expr) => {
        $iter
            .next()
            .map(|b| <$T>::from_bytes(b) == Ok($expected))
            .unwrap_or(false)
    };
}

impl PartialEq<&str> for LanguageIdentifier {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<str> for LanguageIdentifier {
    fn eq(&self, other: &str) -> bool {
        let mut iter = get_subtag_iterator(other.as_bytes());
        if !subtag_matches!(subtags::Language, iter, self.language) {
            return false;
        }
        if let Some(ref script) = self.script {
            if !subtag_matches!(subtags::Script, iter, *script) {
                return false;
            }
        }
        if let Some(ref region) = self.region {
            if !subtag_matches!(subtags::Region, iter, *region) {
                return false;
            }
        }
        for variant in self.variants.iter() {
            if !subtag_matches!(subtags::Variant, iter, *variant) {
                return false;
            }
        }
        iter.next() == None
    }
}
