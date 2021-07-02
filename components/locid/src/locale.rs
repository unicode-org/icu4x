// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::{get_subtag_iterator, parse_locale, ParserError};
use crate::{extensions, subtags, LanguageIdentifier};
use std::str::FromStr;

/// A core struct representing a [`Unicode Locale Identifier`].
///
/// A locale is made of two parts:
///  * Unicode Language Identifier
///  * A set of Unicode Extensions
///
/// [`Locale`] exposes all of the same fields and methods as [`LanguageIdentifier`], and
/// on top of that is able to parse, manipulate and serialize unicode extension fields.
///
///
/// # Examples
///
/// ```
/// use icu::locid::Locale;
/// use icu::locid::extensions::unicode::{Key, Value};
///
/// let loc: Locale = "en-US-u-ca-buddhist".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.id.language, "en");
/// assert_eq!(loc.id.script, None);
/// assert_eq!(loc.id.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.id.variants.len(), 0);
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
/// use icu::locid::Locale;
///
/// let loc: Locale = "eN_latn_Us-Valencia_u-hC-H12".parse()
///     .expect("Failed to parse.");
///
/// assert_eq!(loc.id.language, "en");
/// assert_eq!(loc.id.script, Some("Latn".parse().unwrap()));
/// assert_eq!(loc.id.region, Some("US".parse().unwrap()));
/// assert_eq!(loc.id.variants.get(0).unwrap(), "valencia");
/// ```
/// [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_locale_identifier
#[derive(Default, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Locale {
    // Language component of the Locale
    pub id: LanguageIdentifier,
    // Unicode Locale Extensions
    pub extensions: extensions::Extensions,
}

impl Locale {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`Locale`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    ///
    /// let loc = Locale::from_bytes("en-US-u-hc-h12".as_bytes())
    ///     .expect("Parsing failed.");
    ///
    /// assert_eq!(loc.to_string(), "en-US-u-hc-h12");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_locale(v)
    }

    /// Returns the default undefined locale "und". Same as [`default()`](Default::default()), but is `const`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    ///
    /// const loc: Locale = Locale::und();
    /// assert_eq!(Locale::default(), loc);
    /// assert_eq!("und", loc.to_string());
    /// ```
    #[inline]
    pub const fn und() -> Self {
        Self {
            id: LanguageIdentifier::und(),
            extensions: extensions::Extensions::new(),
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
    /// use icu::locid::Locale;
    ///
    /// assert_eq!(Locale::canonicalize("pL_latn_pl-U-HC-H12"), Ok("pl-Latn-PL-u-hc-h12".to_string()));
    /// ```
    pub fn canonicalize<S: AsRef<[u8]>>(input: S) -> Result<String, ParserError> {
        let locale = Self::from_bytes(input.as_ref())?;
        Ok(locale.to_string())
    }

    /// Accessor method for unicode extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::{extensions::unicode::Key, Locale};
    ///
    /// let loc = Locale::from_bytes("en-US-u-hc-h12".as_bytes())
    ///     .expect("Parsing failed.");
    /// let key = "hc".parse::<Key>()
    ///     .expect("Invalid key.");
    /// let ext = loc.get_unicode_extension(&key)
    ///     .expect("Extension not defined");
    /// assert_eq!(ext.to_string(), "h12");
    /// ```
    pub fn get_unicode_extension(
        &self,
        key: &extensions::unicode::Key,
    ) -> Option<&extensions::unicode::Value> {
        self.extensions.unicode.keywords.get(key)
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
        Self {
            id,
            extensions: extensions::Extensions::default(),
        }
    }
}

impl From<Locale> for LanguageIdentifier {
    fn from(loc: Locale) -> Self {
        loc.id
    }
}

impl AsRef<LanguageIdentifier> for Locale {
    fn as_ref(&self) -> &LanguageIdentifier {
        &self.id
    }
}

impl AsMut<LanguageIdentifier> for Locale {
    fn as_mut(&mut self) -> &mut LanguageIdentifier {
        &mut self.id
    }
}

impl std::fmt::Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for Locale {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        writeable::Writeable::write_to(&self.id, sink)?;
        writeable::Writeable::write_to(&self.extensions, sink)?;
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = writeable::Writeable::write_len(&self.id);
        result += writeable::Writeable::write_len(&self.extensions);
        result
    }
}

#[test]
fn test_writeable() {
    use writeable::assert_writeable_eq;
    assert_writeable_eq!("und", Locale::und());
    assert_writeable_eq!("und-001", Locale::from_str("und-001").unwrap());
    assert_writeable_eq!("und-Mymr", Locale::from_str("und-Mymr").unwrap());
    assert_writeable_eq!("my-Mymr-MM", Locale::from_str("my-Mymr-MM").unwrap());
    assert_writeable_eq!(
        "my-Mymr-MM-posix",
        Locale::from_str("my-Mymr-MM-posix").unwrap()
    );
    assert_writeable_eq!(
        "zh-macos-posix",
        Locale::from_str("zh-macos-posix").unwrap()
    );
    assert_writeable_eq!(
        "my-t-my-d0-zawgyi",
        Locale::from_str("my-t-my-d0-zawgyi").unwrap()
    );
    assert_writeable_eq!(
        "ar-SA-u-ca-islamic-civil",
        Locale::from_str("ar-SA-u-ca-islamic-civil").unwrap()
    );
    assert_writeable_eq!(
        "en-001-x-foo-bar",
        Locale::from_str("en-001-x-foo-bar").unwrap()
    );
}

impl PartialEq<&str> for Locale {
    fn eq(&self, other: &&str) -> bool {
        self == *other
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

impl PartialEq<str> for Locale {
    fn eq(&self, other: &str) -> bool {
        let mut iter = get_subtag_iterator(other.as_bytes()).peekable();
        if !subtag_matches!(subtags::Language, iter, self.id.language) {
            return false;
        }
        if let Some(ref script) = self.id.script {
            if !subtag_matches!(subtags::Script, iter, *script) {
                return false;
            }
        }
        if let Some(ref region) = self.id.region {
            if !subtag_matches!(subtags::Region, iter, *region) {
                return false;
            }
        }
        for variant in self.id.variants.iter() {
            if !subtag_matches!(subtags::Variant, iter, *variant) {
                return false;
            }
        }
        if !self.extensions.is_empty() {
            match extensions::Extensions::try_from_iter(&mut iter) {
                Ok(exts) => {
                    if self.extensions != exts {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        }
        iter.next() == None
    }
}
