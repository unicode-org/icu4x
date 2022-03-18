// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::parser::{get_subtag_iterator, parse_locale, ParserError};
use crate::{extensions, subtags, LanguageIdentifier};
use alloc::string::String;
use alloc::string::ToString;
use core::cmp::Ordering;
use core::str::FromStr;

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
#[allow(missing_docs)] // TODO(#1028) - Add missing docs.
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

    /// Compare this `Locale` with a BCP-47 string.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// `Locale` to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::Locale;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&[u8]] = &[
    ///     b"pl-Latn-PL",
    ///     b"und",
    ///     b"und-fonipa",
    ///     b"und-t-m0-true",
    ///     b"und-u-ca-hebrew",
    ///     b"und-u-ca-japanese",
    ///     b"zh",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert!(a.cmp(b) == Ordering::Less);
    ///     let a_langid = Locale::from_bytes(a).unwrap();
    ///     assert!(a_langid.cmp_bytes(b) == Ordering::Less);
    /// }
    /// ```
    pub fn cmp_bytes(&self, other: &[u8]) -> Ordering {
        let base_iter = self.iter_subtags().map(str::as_bytes);
        // Note: This does not use get_subtag_iterator because we want to guarantee
        // perfect lexicographic ordering of the strings.
        let other_iter = other.split(|b| *b == b'-');
        base_iter.cmp(other_iter)
    }

    pub(crate) fn iter_subtags(&self) -> impl Iterator<Item = &str> {
        self.id.iter_subtags().chain(self.extensions.iter_subtags())
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

impl core::fmt::Debug for Locale {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl core::fmt::Display for Locale {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for Locale {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
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
    assert_writeable_eq!(Locale::und(), "und");
    assert_writeable_eq!(Locale::from_str("und-001").unwrap(), "und-001");
    assert_writeable_eq!(Locale::from_str("und-Mymr").unwrap(), "und-Mymr");
    assert_writeable_eq!(Locale::from_str("my-Mymr-MM").unwrap(), "my-Mymr-MM");
    assert_writeable_eq!(
        Locale::from_str("my-Mymr-MM-posix").unwrap(),
        "my-Mymr-MM-posix",
    );
    assert_writeable_eq!(
        Locale::from_str("zh-macos-posix").unwrap(),
        "zh-macos-posix",
    );
    assert_writeable_eq!(
        Locale::from_str("my-t-my-d0-zawgyi").unwrap(),
        "my-t-my-d0-zawgyi",
    );
    assert_writeable_eq!(
        Locale::from_str("ar-SA-u-ca-islamic-civil").unwrap(),
        "ar-SA-u-ca-islamic-civil",
    );
    assert_writeable_eq!(
        Locale::from_str("en-001-x-foo-bar").unwrap(),
        "en-001-x-foo-bar",
    );
    assert_writeable_eq!(Locale::from_str("und-t-m0-true").unwrap(), "und-t-m0-true",);
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
