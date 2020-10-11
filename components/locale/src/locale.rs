// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::parser::{parse_locale, ParserError};
use crate::{extensions, subtags, LanguageIdentifier};
use std::fmt::Write;
use std::str::FromStr;

/// `Locale` is a core struct representing a [`Unicode Locale Identifier`].
///
/// A locale is made of two parts:
///  * Unicode Language Identifier
///  * A set of Unicode Extensions
///
/// `Locale` exposes all of the same fields and methods as [`LanguageIdentifier`], and
/// on top of that is able to parse, manipulate and serialize unicode extension fields.
///
/// [`LanguageIdentifier`]: ./struct.LanguageIdentifier.html
///
/// # Examples
///
/// ```
/// use icu_locale::Locale;
/// use icu_locale::extensions::unicode::{Key, Value};
///
/// let loc: Locale = "en-US-u-ca-buddhist".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.language, "en");
/// assert_eq!(loc.script, None);
/// assert_eq!(loc.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.variants.len(), 0);
/// assert_eq!(loc, "en-US-u-ca-buddhist");
///
/// let key: Key = "ca".parse().expect("Parsing key failed.");
/// let value: Value = "buddhist".parse().expect("Parsing value failed.");
/// assert_eq!(loc.extensions.unicode.keywords.get(key),
///     Some(&value));
/// ```
///
/// # Parsing
///
/// Unicode recognizes three levels of standard conformance for a locale:
///
///  * *well-formed* - syntactically correct
///  * *valid* - well-formed and only uses registered language subtags, extensions, keywords, types...
///  * *canonical* - valid and no deprecated codes or structure.
///
/// At the moment parsing normalizes a well-formed locale identifier converting
/// `_` separators to `-` and adjusting casing to conform to the Unicode standard.
///
/// Any bogus subtags will cause the parsing to fail with an error.
/// No subtag validation is performed.
///
/// # Examples
///
/// ```
/// use icu_locale::Locale;
///
/// let loc: Locale = "eN_latn_Us-Valencia_u-hC-H12".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.language, "en");
/// assert_eq!(loc.script, Some("Latn".parse().unwrap()));
/// assert_eq!(loc.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.variants.get(0).unwrap(), "valencia");
/// ```
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
#[derive(Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Locale {
    /// Language subtag of the Locale
    pub language: subtags::Language,
    /// Script subtag of the Locale
    pub script: Option<subtags::Script>,
    /// Region subtag of the Locale
    pub region: Option<subtags::Region>,
    /// Variant subtags of the Locale
    pub variants: subtags::Variants,
    // Unicode Locale Extensions
    pub extensions: extensions::Extensions,
}

impl Locale {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed `Locale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::Locale;
    ///
    /// let loc = Locale::from_bytes("en-US-u-hc-h12".as_bytes())
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-hc-h12");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_locale(v)
    }

    /// This is a best-effort operation that performs all available levels of canonicalization.
    ///
    /// At the moment the operation will normalize casing and the separator, but in the future
    /// it may also validate and update from deprecated subtags to canonical ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::Locale;
    ///
    /// assert_eq!(Locale::canonicalize("pL_latn_pl-U-HC-H12"), Ok("pl-Latn-PL-u-hc-h12".to_string()));
    /// ```
    pub fn canonicalize<S: AsRef<[u8]>>(input: S) -> Result<String, ParserError> {
        let locale = Locale::from_bytes(input.as_ref())?;
        Ok(locale.to_string())
    }
}

impl FromStr for Locale {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(id: LanguageIdentifier) -> Self {
        Locale {
            language: id.language,
            script: id.script,
            region: id.region,
            variants: id.variants,
            extensions: extensions::Extensions::default(),
        }
    }
}

impl From<Locale> for LanguageIdentifier {
    fn from(loc: Locale) -> Self {
        Self {
            language: loc.language,
            script: loc.script,
            region: loc.region,
            variants: loc.variants,
        }
    }
}

impl std::fmt::Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Locale {
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
        self.extensions.fmt(f)?;
        Ok(())
    }
}

impl PartialEq<&str> for Locale {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(*other)
    }
}

impl PartialEq<str> for Locale {
    fn eq(&self, other: &str) -> bool {
        self.to_string().eq(other)
    }
}
