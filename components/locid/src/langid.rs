// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;
use core::str::FromStr;

use crate::ordering::SubtagOrderingResult;
use crate::parser::{
    get_subtag_iterator, parse_language_identifier, parse_language_identifier_without_variants,
    ParserError, ParserMode,
};
use crate::subtags;
use alloc::string::String;
use alloc::string::ToString;

/// A core struct representing a [`Unicode BCP47 Language Identifier`].
///
/// # Examples
///
/// ```
/// use icu::locid::{LanguageIdentifier, subtags::*};
///
/// let li: LanguageIdentifier = "en-US".parse().expect("Failed to parse.");
///
/// assert_eq!(li.language, "en".parse::<Language>().unwrap());
/// assert_eq!(li.script, None);
/// assert_eq!(li.region.unwrap(), "US".parse::<Region>().unwrap());
/// assert_eq!(li.variants.len(), 0);
/// assert_eq!(li.to_string(), "en-US");
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
/// use icu::locid::{LanguageIdentifier, subtags::*};
///
/// let li: LanguageIdentifier = "eN_latn_Us-Valencia".parse().expect("Failed to parse.");
///
/// assert_eq!(li.language, "en".parse::<Language>().unwrap());
/// assert_eq!(li.script, "Latn".parse::<Script>().ok());
/// assert_eq!(li.region, "US".parse::<Region>().ok());
/// assert_eq!(li.variants.get(0), "valencia".parse::<Variant>().ok().as_ref());
/// ```
///
/// [`Unicode BCP47 Language Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier
#[derive(Default, PartialEq, Eq, Clone, Hash)]
#[allow(clippy::exhaustive_structs)] // This struct is stable (and invoked by a macro)
pub struct LanguageIdentifier {
    /// Language subtag of the language identifier.
    pub language: subtags::Language,
    /// Script subtag of the language identifier.
    pub script: Option<subtags::Script>,
    /// Region subtag of the language identifier.
    pub region: Option<subtags::Region>,
    /// Variant subtags of the language identifier.
    pub variants: subtags::Variants,
}

impl LanguageIdentifier {
    /// A constructor which takes a utf8 slice, parses it and
    /// produces a well-formed [`LanguageIdentifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    ///
    /// let li = LanguageIdentifier::from_bytes(b"en-US").expect("Parsing failed.");
    ///
    /// assert_eq!(li.to_string(), "en-US");
    /// ```
    pub fn from_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_language_identifier(v, ParserMode::LanguageIdentifier)
    }

    #[doc(hidden)]
    // The return type should be `Result<Self, ParserError>` once the `const_precise_live_drops`
    // is stabilized ([rust-lang#73255](https://github.com/rust-lang/rust/issues/73255)).
    pub const fn from_bytes_without_variants(
        v: &[u8],
    ) -> Result<
        (
            subtags::Language,
            Option<subtags::Script>,
            Option<subtags::Region>,
        ),
        ParserError,
    > {
        parse_language_identifier_without_variants(v, ParserMode::LanguageIdentifier)
    }

    /// A constructor which takes a utf8 slice which may contain extension keys,
    /// parses it and produces a well-formed [`LanguageIdentifier`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    ///
    /// let li = LanguageIdentifier::from_locale_bytes(b"en-US-x-posix").expect("Parsing failed.");
    ///
    /// assert_eq!(li.to_string(), "en-US");
    /// ```
    ///
    /// This method should be used for input that may be a locale identifier.
    /// All extensions will be lost.
    pub fn from_locale_bytes(v: &[u8]) -> Result<Self, ParserError> {
        parse_language_identifier(v, ParserMode::Locale)
    }

    /// The default undefined language "und". Same as [`default()`](Default::default()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    ///
    /// assert_eq!(LanguageIdentifier::default(), LanguageIdentifier::UND);
    /// assert_eq!("und", LanguageIdentifier::UND.to_string());
    /// ```
    pub const UND: Self = Self {
        language: subtags::Language::UND,
        script: None,
        region: None,
        variants: subtags::Variants::new(),
    };

    /// This is a best-effort operation that performs all available levels of canonicalization.
    ///
    /// At the moment the operation will normalize casing and the separator, but in the future
    /// it may also validate and update from deprecated subtags to canonical ones.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    ///
    /// assert_eq!(
    ///     LanguageIdentifier::canonicalize("pL_latn_pl"),
    ///     Ok("pl-Latn-PL".to_string())
    /// );
    /// ```
    pub fn canonicalize<S: AsRef<[u8]>>(input: S) -> Result<String, ParserError> {
        let lang_id = Self::from_bytes(input.as_ref())?;
        Ok(lang_id.to_string())
    }

    /// Compare this [`LanguageIdentifier`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`LanguageIdentifier`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "pl-Latn-PL",
    ///     "und",
    ///     "und-Adlm",
    ///     "und-GB",
    ///     "und-ZA",
    ///     "und-fonipa",
    ///     "zh",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert!(a.cmp(b) == Ordering::Less);
    ///     let a_langid = a.parse::<LanguageIdentifier>().unwrap();
    ///     assert_eq!(a, a_langid.to_string());
    ///     assert!(a_langid.strict_cmp(a.as_bytes()) == Ordering::Equal);
    ///     assert!(a_langid.strict_cmp(b.as_bytes()) == Ordering::Less);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        self.strict_cmp_iter(other.split(|b| *b == b'-')).end()
    }

    /// Compare this [`LanguageIdentifier`] with an iterator of BCP-47 subtags.
    ///
    /// This function has the same equality semantics as [`LanguageIdentifier::strict_cmp`]. It is intended as
    /// a more modular version that allows multiple subtag iterators to be chained together.
    ///
    /// For an additional example, see [`SubtagOrderingResult`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    /// use std::cmp::Ordering;
    ///
    /// let subtags: &[&[u8]] = &[&*b"ca", &*b"ES", &*b"valencia"];
    ///
    /// let loc = "ca-ES-valencia".parse::<LanguageIdentifier>().unwrap();
    /// assert_eq!(
    ///     Ordering::Equal,
    ///     loc.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    ///
    /// let loc = "ca-ES".parse::<LanguageIdentifier>().unwrap();
    /// assert_eq!(
    ///     Ordering::Less,
    ///     loc.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    ///
    /// let loc = "ca-ZA".parse::<LanguageIdentifier>().unwrap();
    /// assert_eq!(
    ///     Ordering::Greater,
    ///     loc.strict_cmp_iter(subtags.iter().copied()).end()
    /// );
    /// ```
    pub fn strict_cmp_iter<'l, I>(&self, mut subtags: I) -> SubtagOrderingResult<I>
    where
        I: Iterator<Item = &'l [u8]>,
    {
        let r = self.for_each_subtag_str(&mut |subtag| {
            if let Some(other) = subtags.next() {
                match subtag.as_bytes().cmp(other) {
                    Ordering::Equal => Ok(()),
                    not_equal => Err(not_equal),
                }
            } else {
                Err(Ordering::Greater)
            }
        });
        match r {
            Ok(_) => SubtagOrderingResult::Subtags(subtags),
            Err(o) => SubtagOrderingResult::Ordering(o),
        }
    }

    /// Compare this `LanguageIdentifier` with a potentially unnormalized BCP-47 string.
    ///
    /// The return value is equivalent to what would happen if you first parsed the
    /// BCP-47 string to a `LanguageIdentifier` and then performed a structucal comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::LanguageIdentifier;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "pl-LaTn-pL",
    ///     "uNd",
    ///     "UnD-adlm",
    ///     "uNd-GB",
    ///     "UND-FONIPA",
    ///     "ZH",
    /// ];
    ///
    /// for a in bcp47_strings {
    ///     assert!(a.parse::<LanguageIdentifier>().unwrap().normalizing_eq(a));
    /// }
    /// ```
    pub fn normalizing_eq(&self, other: &str) -> bool {
        macro_rules! subtag_matches {
            ($T:ty, $iter:ident, $expected:expr) => {
                $iter
                    .next()
                    .map(|b| <$T>::from_bytes(b) == Ok($expected))
                    .unwrap_or(false)
            };
        }

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

    pub(crate) fn for_each_subtag_str<E, F>(&self, f: &mut F) -> Result<(), E>
    where
        F: FnMut(&str) -> Result<(), E>,
    {
        f(self.language.as_str())?;
        if let Some(ref script) = self.script {
            f(script.as_str())?;
        }
        if let Some(ref region) = self.region {
            f(region.as_str())?;
        }
        for variant in self.variants.iter() {
            f(variant.as_str())?;
        }
        Ok(())
    }
}

impl AsRef<LanguageIdentifier> for LanguageIdentifier {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<LanguageIdentifier> for LanguageIdentifier {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl core::fmt::Debug for LanguageIdentifier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}

impl FromStr for LanguageIdentifier {
    type Err = ParserError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(source.as_bytes())
    }
}

impl_writeable_for_each_subtag_str_no_test!(LanguageIdentifier);

#[test]
fn test_writeable() {
    use writeable::assert_writeable_eq;
    assert_writeable_eq!(LanguageIdentifier::UND, "und");
    assert_writeable_eq!(LanguageIdentifier::from_str("und-001").unwrap(), "und-001");
    assert_writeable_eq!(
        LanguageIdentifier::from_str("und-Mymr").unwrap(),
        "und-Mymr",
    );
    assert_writeable_eq!(
        LanguageIdentifier::from_str("my-Mymr-MM").unwrap(),
        "my-Mymr-MM",
    );
    assert_writeable_eq!(
        LanguageIdentifier::from_str("my-Mymr-MM-posix").unwrap(),
        "my-Mymr-MM-posix",
    );
    assert_writeable_eq!(
        LanguageIdentifier::from_str("zh-macos-posix").unwrap(),
        "zh-macos-posix",
    );
}

/// # Examples
///
/// ```
/// use icu::locid::subtags_language as language;
/// use icu::locid::LanguageIdentifier;
///
/// let language = language!("en");
/// let li = LanguageIdentifier::from(language);
///
/// assert_eq!(li.language, language);
/// assert_eq!(li.to_string(), "en");
/// ```
impl From<subtags::Language> for LanguageIdentifier {
    fn from(language: subtags::Language) -> Self {
        Self {
            language,
            ..Default::default()
        }
    }
}

/// # Examples
///
/// ```
/// use icu::locid::subtags_script as script;
/// use icu::locid::LanguageIdentifier;
///
/// let script = script!("latn");
/// let li = LanguageIdentifier::from(Some(script));
///
/// assert_eq!(li.script.unwrap(), script);
/// assert_eq!(li.to_string(), "und-Latn");
/// ```
impl From<Option<subtags::Script>> for LanguageIdentifier {
    fn from(script: Option<subtags::Script>) -> Self {
        Self {
            script,
            ..Default::default()
        }
    }
}

/// # Examples
///
/// ```
/// use icu::locid::subtags_region as region;
/// use icu::locid::LanguageIdentifier;
///
/// let region = region!("US");
/// let li = LanguageIdentifier::from(Some(region));
///
/// assert_eq!(li.region.unwrap(), region);
/// assert_eq!(li.to_string(), "und-US");
/// ```
impl From<Option<subtags::Region>> for LanguageIdentifier {
    fn from(region: Option<subtags::Region>) -> Self {
        Self {
            region,
            ..Default::default()
        }
    }
}

/// Convert from an LSR tuple to a [`LanguageIdentifier`].
///
/// # Examples
///
/// ```
/// use icu::locid::LanguageIdentifier;
/// use icu::locid::{subtags_language as language, subtags_region as region, subtags_script as script};
///
/// let lang = language!("en");
/// let script = script!("Latn");
/// let region = region!("US");
/// let li = LanguageIdentifier::from((lang, Some(script), Some(region)));
///
/// assert_eq!(li.language, lang);
/// assert_eq!(li.script.unwrap(), script);
/// assert_eq!(li.region.unwrap(), region);
/// assert_eq!(li.variants.len(), 0);
/// assert_eq!(li.to_string(), "en-Latn-US");
/// ```
impl
    From<(
        subtags::Language,
        Option<subtags::Script>,
        Option<subtags::Region>,
    )> for LanguageIdentifier
{
    fn from(
        lsr: (
            subtags::Language,
            Option<subtags::Script>,
            Option<subtags::Region>,
        ),
    ) -> Self {
        Self {
            language: lsr.0,
            script: lsr.1,
            region: lsr.2,
            ..Default::default()
        }
    }
}

/// Convert from a [`LanguageIdentifier`] to an LSR tuple.
///
/// # Examples
///
/// ```
/// use icu::locid::LanguageIdentifier;
/// use icu::locid::{subtags_language as language, subtags_region as region, subtags_script as script, langid};
///
/// let lid = langid!("en-Latn-US");
/// let (lang, script, region) = (&lid).into();
///
/// assert_eq!(lang, language!("en"));
/// assert_eq!(script, Some(script!("Latn")));
/// assert_eq!(region, Some(region!("US")));
/// ```
impl From<&LanguageIdentifier>
    for (
        subtags::Language,
        Option<subtags::Script>,
        Option<subtags::Region>,
    )
{
    fn from(langid: &LanguageIdentifier) -> Self {
        (langid.language, langid.script, langid.region)
    }
}
