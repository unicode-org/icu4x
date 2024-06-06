// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{DataError, DataErrorKind};
use core::cmp::Ordering;
use core::default::Default;
use core::fmt;
use core::fmt::Debug;
use core::hash::Hash;
use core::str::FromStr;
use icu_locale_core::extensions::unicode as unicode_ext;
use icu_locale_core::subtags::{Language, Region, Script, Variants};
use icu_locale_core::{LanguageIdentifier, Locale};
use writeable::{LengthHint, Writeable};

use alloc::string::String;
use core::ops::Deref;
use icu_locale_core::extensions::private::Subtag;
use tinystr::TinyAsciiStr;

#[cfg(doc)]
use icu_locale_core::subtags::Variant;

/// The request type passed into all data provider implementations.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DataRequest<'a> {
    /// The locale for which to load data.
    ///
    /// If locale fallback is enabled, the resulting data may be from a different locale
    /// than the one requested here.
    pub locale: &'a DataLocale,
    /// Key-specific request attributes
    pub marker_attributes: &'a DataMarkerAttributes,
    /// Metadata that may affect the behavior of the data provider.
    pub metadata: DataRequestMetadata,
}

impl fmt::Display for DataRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.locale, f)
    }
}

/// Metadata for data requests. This is currently empty, but it may be extended with options
/// for tuning locale fallback, buffer layout, and so forth.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct DataRequestMetadata {
    /// Silent requests do not log errors. This can be used for exploratory querying, such as fallbacks.
    pub silent: bool,
}

#[doc(hidden)] // TODO: replace the `-x-` encoding by something more flexible
impl DataRequest<'_> {
    pub fn legacy_cmp(&self, bytes: &[u8]) -> Ordering {
        struct LegacyComparator<'a>(&'a DataLocale, &'a DataMarkerAttributes);
        impl writeable::Writeable for LegacyComparator<'_> {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                self.0.write_to(sink)?;
                if !self.1.is_empty() {
                    "-x-".write_to(sink)?;
                    self.1.write_to(sink)?;
                }
                Ok(())
            }
        }
        LegacyComparator(self.locale, self.marker_attributes).writeable_cmp_bytes(bytes)
    }

    pub fn legacy_encode(&self) -> String {
        self.locale.write_to_string().into_owned()
            + if self.marker_attributes.is_empty() {
                ""
            } else {
                "-x-"
            }
            + self.marker_attributes
    }

    pub fn legacy_decode(encoded: &str) -> Option<(DataLocale, DataMarkerAttributes)> {
        let mut iter = encoded.splitn(2, "-x-");
        #[allow(clippy::unwrap_used)] // at least one element
        let l = iter.next().unwrap().parse().ok()?;
        let a = iter.next().unwrap_or("").parse().ok()?;
        Some((l, a))
    }
}

/// A locale type optimized for use in fallbacking and the ICU4X data pipeline.
///
/// [`DataLocale`] contains less functionality than [`Locale`] but more than
/// [`LanguageIdentifier`] for better size and performance while still meeting
/// the needs of the ICU4X data pipeline.
///
/// # Examples
///
/// Convert a [`Locale`] to a [`DataLocale`] and back:
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::DataLocale;
///
/// let locale = locale!("en-u-ca-buddhist");
/// let data_locale = DataLocale::from(locale);
/// let locale = data_locale.into_locale();
///
/// assert_eq!(locale, locale!("en-u-ca-buddhist"));
/// ```
///
/// You can alternatively create a [`DataLocale`] from a borrowed [`Locale`], which is more
/// efficient than cloning the [`Locale`], but less efficient than converting an owned
/// [`Locale`]:
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::DataLocale;
///
/// let locale1 = locale!("en-u-ca-buddhist");
/// let data_locale = DataLocale::from(&locale1);
/// let locale2 = data_locale.into_locale();
///
/// assert_eq!(locale1, locale2);
/// ```
///
/// If you are sure that you have no Unicode keywords, start with [`LanguageIdentifier`]:
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::DataLocale;
///
/// let langid = langid!("es-CA-valencia");
/// let data_locale = DataLocale::from(langid);
/// let langid = data_locale.get_langid();
///
/// assert_eq!(langid, langid!("es-CA-valencia"));
/// ```
///
/// [`DataLocale`] only supports `-u` keywords, to reflect the current state of CLDR data
/// lookup and fallback. This may change in the future.
///
/// ```
/// use icu_locale_core::{locale, Locale};
/// use icu_provider::DataLocale;
///
/// let locale = "hi-t-en-h0-hybrid-u-attr-ca-buddhist"
///     .parse::<Locale>()
///     .unwrap();
/// let data_locale = DataLocale::from(locale);
///
/// assert_eq!(data_locale.into_locale(), locale!("hi-u-ca-buddhist"));
/// ```
#[derive(PartialEq, Clone, Default, Eq, Hash)]
pub struct DataLocale {
    langid: LanguageIdentifier,
    keywords: unicode_ext::Keywords,
}

impl<'a> Default for &'a DataLocale {
    fn default() -> Self {
        static DEFAULT: DataLocale = DataLocale {
            langid: LanguageIdentifier::UND,
            keywords: unicode_ext::Keywords::new(),
        };
        &DEFAULT
    }
}

impl fmt::Debug for DataLocale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataLocale{{{self}}}")
    }
}

impl Writeable for DataLocale {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.langid.write_to(sink)?;
        if !self.keywords.is_empty() {
            sink.write_str("-u-")?;
            self.keywords.write_to(sink)?;
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        let mut length_hint = self.langid.writeable_length_hint();
        if !self.keywords.is_empty() {
            length_hint += self.keywords.writeable_length_hint() + 3;
        }
        length_hint
    }

    fn write_to_string(&self) -> alloc::borrow::Cow<str> {
        if self.keywords.is_empty() {
            return self.langid.write_to_string();
        }
        let mut string =
            alloc::string::String::with_capacity(self.writeable_length_hint().capacity());
        let _ = self.write_to(&mut string);
        alloc::borrow::Cow::Owned(string)
    }
}

writeable::impl_display_with_writeable!(DataLocale);

impl From<LanguageIdentifier> for DataLocale {
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            keywords: unicode_ext::Keywords::new(),
        }
    }
}

impl From<Locale> for DataLocale {
    fn from(locale: Locale) -> Self {
        Self {
            langid: locale.id,
            keywords: locale.extensions.unicode.keywords,
        }
    }
}

impl From<&LanguageIdentifier> for DataLocale {
    fn from(langid: &LanguageIdentifier) -> Self {
        Self {
            langid: langid.clone(),
            keywords: unicode_ext::Keywords::new(),
        }
    }
}

impl From<&Locale> for DataLocale {
    fn from(locale: &Locale) -> Self {
        Self {
            langid: locale.id.clone(),
            keywords: locale.extensions.unicode.keywords.clone(),
        }
    }
}

impl FromStr for DataLocale {
    type Err = DataError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locale = Locale::from_str(s).map_err(|e| {
            DataErrorKind::KeyLocaleSyntax
                .into_error()
                .with_display_context(s)
                .with_display_context(&e)
        })?;
        Ok(DataLocale::from(locale))
    }
}

impl DataLocale {
    /// Compare this [`DataLocale`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`DataLocale`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "ca",
    ///     "ca-ES",
    ///     "ca-ES-u-ca-buddhist",
    ///     "ca-ES-valencia",
    ///     "cat",
    ///     "pl-Latn-PL",
    ///     "und",
    ///     "und-fonipa",
    ///     "und-u-ca-hebrew",
    ///     "und-u-ca-japanese",
    ///     "zh",
    /// ];
    ///
    /// for ab in bcp47_strings.windows(2) {
    ///     let a = ab[0];
    ///     let b = ab[1];
    ///     assert_eq!(a.cmp(b), Ordering::Less, "strings: {} < {}", a, b);
    ///     let a_loc: DataLocale = a.parse().unwrap();
    ///     assert_eq!(
    ///         a_loc.strict_cmp(a.as_bytes()),
    ///         Ordering::Equal,
    ///         "strict_cmp: {} == {}",
    ///         a_loc,
    ///         a
    ///     );
    ///     assert_eq!(
    ///         a_loc.strict_cmp(b.as_bytes()),
    ///         Ordering::Less,
    ///         "strict_cmp: {} < {}",
    ///         a_loc,
    ///         b
    ///     );
    ///     let b_loc: DataLocale = b.parse().unwrap();
    ///     assert_eq!(
    ///         b_loc.strict_cmp(b.as_bytes()),
    ///         Ordering::Equal,
    ///         "strict_cmp: {} == {}",
    ///         b_loc,
    ///         b
    ///     );
    ///     assert_eq!(
    ///         b_loc.strict_cmp(a.as_bytes()),
    ///         Ordering::Greater,
    ///         "strict_cmp: {} > {}",
    ///         b_loc,
    ///         a
    ///     );
    /// }
    /// ```
    ///
    /// Comparison against invalid strings:
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// let invalid_strings: &[&str] = &[
    ///     // Less than "ca-ES"
    ///     "CA",
    ///     "ar-x-gbp-FOO",
    ///     // Greater than "ca-AR"
    ///     "ca_ES",
    ///     "ca-ES-x-gbp-FOO",
    /// ];
    ///
    /// let data_locale = "ca-ES".parse::<DataLocale>().unwrap();
    ///
    /// for s in invalid_strings.iter() {
    ///     let expected_ordering = "ca-AR".cmp(s);
    ///     let actual_ordering = data_locale.strict_cmp(s.as_bytes());
    ///     assert_eq!(expected_ordering, actual_ordering, "{}", s);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        self.writeable_cmp_bytes(other)
    }
}

impl DataLocale {
    /// Returns whether this [`DataLocale`] has all empty fields (no components).
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_und()`]
    /// - [`DataLocale::is_langid_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_empty());
    /// assert!(!"und-u-ca-buddhist"
    ///     .parse::<DataLocale>()
    ///     .unwrap()
    ///     .is_empty());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self == <&DataLocale>::default()
    }

    /// Returns an ordering suitable for use in [`BTreeSet`].
    ///
    /// The ordering may or may not be equivalent to string ordering, and it
    /// may or may not be stable across ICU4X releases.
    ///
    /// [`BTreeSet`]: alloc::collections::BTreeSet
    pub fn total_cmp(&self, other: &Self) -> Ordering {
        self.langid
            .total_cmp(&other.langid)
            .then_with(|| self.keywords.cmp(&other.keywords))
    }

    /// Returns whether this [`DataLocale`] is `und` in the locale and extensions portion.
    ///
    /// This ignores auxiliary keys.
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_empty()`]
    /// - [`DataLocale::is_langid_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_und());
    /// assert!(!"und-u-ca-buddhist".parse::<DataLocale>().unwrap().is_und());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_und());
    /// ```
    pub fn is_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND && self.keywords.is_empty()
    }

    /// Returns whether the [`LanguageIdentifier`] associated with this request is `und`.
    ///
    /// This ignores extension keywords and auxiliary keys.
    ///
    /// See also:
    ///
    /// - [`DataLocale::is_empty()`]
    /// - [`DataLocale::is_und()`]
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataLocale;
    ///
    /// assert!("und".parse::<DataLocale>().unwrap().is_langid_und());
    /// assert!("und-u-ca-buddhist"
    ///     .parse::<DataLocale>()
    ///     .unwrap()
    ///     .is_langid_und());
    /// assert!(!"ca-ES".parse::<DataLocale>().unwrap().is_langid_und());
    /// ```
    pub fn is_langid_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND
    }

    /// Gets the [`LanguageIdentifier`] for this [`DataLocale`].
    ///
    /// This may allocate memory if there are variant subtags. If you need only the language,
    /// script, and/or region subtag, use the specific getters for those subtags:
    ///
    /// - [`DataLocale::language()`]
    /// - [`DataLocale::script()`]
    /// - [`DataLocale::region()`]
    ///
    /// If you have ownership over the `DataLocale`, use [`DataLocale::into_locale()`]
    /// and then access the `id` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::langid;
    /// use icu_provider::prelude::*;
    ///
    /// let req_no_langid = DataRequest {
    ///     locale: &Default::default(),
    ///     ..Default::default()
    /// };
    ///
    /// let req_with_langid = DataRequest {
    ///     locale: &langid!("ar-EG").into(),
    ///     ..Default::default()
    /// };
    ///
    /// assert_eq!(req_no_langid.locale.get_langid(), langid!("und"));
    /// assert_eq!(req_with_langid.locale.get_langid(), langid!("ar-EG"));
    /// ```
    pub fn get_langid(&self) -> LanguageIdentifier {
        self.langid.clone()
    }

    /// Overrides the entire [`LanguageIdentifier`] portion of this [`DataLocale`].
    #[inline]
    pub fn set_langid(&mut self, lid: LanguageIdentifier) {
        self.langid = lid;
    }

    /// Converts this [`DataLocale`] into a [`Locale`].
    ///
    /// See also [`DataLocale::get_langid()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::{
    ///     langid, locale,
    ///     subtags::{language, region},
    /// };
    /// use icu_provider::prelude::*;
    ///
    /// let locale: DataLocale = locale!("it-IT-u-ca-coptic").into();
    ///
    /// assert_eq!(locale.get_langid(), langid!("it-IT"));
    /// assert_eq!(locale.language(), language!("it"));
    /// assert_eq!(locale.script(), None);
    /// assert_eq!(locale.region(), Some(region!("IT")));
    ///
    /// let locale = locale.into_locale();
    /// assert_eq!(locale, locale!("it-IT-u-ca-coptic"));
    /// ```
    pub fn into_locale(self) -> Locale {
        let mut loc = Locale {
            id: self.langid,
            ..Default::default()
        };
        loc.extensions.unicode.keywords = self.keywords;
        loc
    }

    /// Returns the [`Language`] for this [`DataLocale`].
    #[inline]
    pub fn language(&self) -> Language {
        self.langid.language
    }

    /// Returns the [`Language`] for this [`DataLocale`].
    #[inline]
    pub fn set_language(&mut self, language: Language) {
        self.langid.language = language;
    }

    /// Returns the [`Script`] for this [`DataLocale`].
    #[inline]
    pub fn script(&self) -> Option<Script> {
        self.langid.script
    }

    /// Sets the [`Script`] for this [`DataLocale`].
    #[inline]
    pub fn set_script(&mut self, script: Option<Script>) {
        self.langid.script = script;
    }

    /// Returns the [`Region`] for this [`DataLocale`].
    #[inline]
    pub fn region(&self) -> Option<Region> {
        self.langid.region
    }

    /// Sets the [`Region`] for this [`DataLocale`].
    #[inline]
    pub fn set_region(&mut self, region: Option<Region>) {
        self.langid.region = region;
    }

    /// Returns whether there are any [`Variant`] subtags in this [`DataLocale`].
    #[inline]
    pub fn has_variants(&self) -> bool {
        !self.langid.variants.is_empty()
    }

    /// Sets all [`Variants`] on this [`DataLocale`], overwriting any that were there previously.
    #[inline]
    pub fn set_variants(&mut self, variants: Variants) {
        self.langid.variants = variants;
    }

    /// Removes all [`Variant`] subtags in this [`DataLocale`].
    #[inline]
    pub fn clear_variants(&mut self) -> Variants {
        self.langid.variants.clear()
    }

    /// Gets the value of the specified Unicode extension keyword for this [`DataLocale`].
    #[inline]
    pub fn get_unicode_ext(&self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.get(key).cloned()
    }

    /// Returns whether there are any Unicode extension keywords in this [`DataLocale`].
    #[inline]
    pub fn has_unicode_ext(&self) -> bool {
        !self.keywords.is_empty()
    }

    /// Returns whether a specific Unicode extension keyword is present in this [`DataLocale`].
    #[inline]
    pub fn contains_unicode_ext(&self, key: &unicode_ext::Key) -> bool {
        self.keywords.contains_key(key)
    }

    /// Returns whether this [`DataLocale`] contains a Unicode extension keyword
    /// with the specified key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::extensions::unicode::{key, value};
    /// use icu_provider::prelude::*;
    ///
    /// let locale: DataLocale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    ///
    /// assert_eq!(locale.get_unicode_ext(&key!("hc")), None);
    /// assert_eq!(locale.get_unicode_ext(&key!("ca")), Some(value!("coptic")));
    /// assert!(locale.matches_unicode_ext(&key!("ca"), &value!("coptic"),));
    /// ```
    #[inline]
    pub fn matches_unicode_ext(&self, key: &unicode_ext::Key, value: &unicode_ext::Value) -> bool {
        self.keywords.get(key) == Some(value)
    }

    /// Sets the value for a specific Unicode extension keyword on this [`DataLocale`].
    #[inline]
    pub fn set_unicode_ext(
        &mut self,
        key: unicode_ext::Key,
        value: unicode_ext::Value,
    ) -> Option<unicode_ext::Value> {
        self.keywords.set(key, value)
    }

    /// Removes a specific Unicode extension keyword from this [`DataLocale`], returning
    /// the value if it was present.
    #[inline]
    pub fn remove_unicode_ext(&mut self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.remove(key)
    }

    /// Retains a subset of keywords as specified by the predicate function.
    #[inline]
    pub fn retain_unicode_ext<F>(&mut self, predicate: F)
    where
        F: FnMut(&unicode_ext::Key) -> bool,
    {
        self.keywords.retain_by_key(predicate)
    }
}

/// TODO
#[derive(Clone, Default)]
pub struct DataMarkerAttributes {
    value: DataMarkerAttributesInner,
}

#[derive(Clone, Default)]
enum DataMarkerAttributesInner {
    #[default]
    Empty,
    Boxed(alloc::boxed::Box<str>),
    Stack(TinyAsciiStr<23>),
    // NOTE: In the future, a `Static` variant could be added to allow `data_locale!("...")`
    // Static(&'static str),
}

impl<'a> Default for &'a DataMarkerAttributes {
    fn default() -> Self {
        static DEFAULT: DataMarkerAttributes = DataMarkerAttributes {
            value: DataMarkerAttributesInner::Empty,
        };
        &DEFAULT
    }
}

impl Deref for DataMarkerAttributes {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.value {
            DataMarkerAttributesInner::Empty => "",
            DataMarkerAttributesInner::Boxed(s) => s.deref(),
            DataMarkerAttributesInner::Stack(s) => s.as_str(),
        }
    }
}

impl PartialEq for DataMarkerAttributes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for DataMarkerAttributes {}

impl PartialOrd for DataMarkerAttributes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataMarkerAttributes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl Debug for DataMarkerAttributes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl Hash for DataMarkerAttributes {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl FromStr for DataMarkerAttributes {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self {
                value: DataMarkerAttributesInner::Empty,
            })
        } else if let Ok(tiny) = s.parse() {
            Ok(Self {
                value: DataMarkerAttributesInner::Stack(tiny),
            })
        } else {
            Ok(Self {
                value: DataMarkerAttributesInner::Boxed(s.into()),
            })
        }
    }
}

impl DataMarkerAttributes {
    /// Creates a [`DataMarkerAttributes`] from an iterator of individual keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::extensions::private::subtag;
    /// use icu_provider::prelude::*;
    ///
    /// // Single auxiliary key:
    /// let a = DataMarkerAttributes::try_from_iter([subtag!("abc")]).unwrap();
    /// let b = "abc".parse::<DataMarkerAttributes>().unwrap();
    /// assert_eq!(a, b);
    ///
    /// // Multiple auxiliary keys:
    /// let a = DataMarkerAttributes::try_from_iter([subtag!("abc"), subtag!("defg")])
    ///     .unwrap();
    /// let b = "abc-defg".parse::<DataMarkerAttributes>().unwrap();
    /// assert_eq!(a, b);
    /// ```
    ///
    /// The iterator can't be empty:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// assert!(DataMarkerAttributes::try_from_iter([]).is_err());
    /// ```
    pub fn try_from_iter(iter: impl IntoIterator<Item = Subtag>) -> Result<Self, DataError> {
        // TODO: Avoid the allocation when possible
        let mut builder = String::new();
        for item in iter {
            if !builder.is_empty() {
                builder.push('-');
            }
            builder.push_str(item.as_str())
        }
        if builder.is_empty() {
            return Err(DataErrorKind::KeyLocaleSyntax.with_str_context("empty aux iterator"));
        }
        if builder.len() <= 23 {
            #[allow(clippy::unwrap_used)] // we just checked that the string is ascii
            Ok(Self {
                value: DataMarkerAttributesInner::Stack(builder.parse().unwrap()),
            })
        } else {
            Ok(Self {
                value: DataMarkerAttributesInner::Boxed(builder.into()),
            })
        }
    }

    /// Creates a [`DataMarkerAttributes`] from a single subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::extensions::private::subtag;
    /// use icu_provider::prelude::*;
    ///
    /// // Single auxiliary key:
    /// let a = DataMarkerAttributes::from_tinystr("abc".parse().unwrap());
    /// let b = "abc".parse::<DataMarkerAttributes>().unwrap();
    /// assert_eq!(a, b);
    /// ```
    pub const fn from_tinystr(input: TinyAsciiStr<8>) -> Self {
        Self {
            value: DataMarkerAttributesInner::Stack(input.resize()),
        }
    }

    /// TODO
    pub fn single(&self) -> Option<TinyAsciiStr<8>> {
        let mut iter = self.split('-').filter_map(|x| match x.parse() {
            Ok(x) => Some(x),
            Err(_) => {
                debug_assert!(false, "failed to convert to subtag: {x}");
                None
            }
        });
        let subtag = iter.next()?;
        if iter.next().is_some() {
            return None;
        }
        Some(subtag)
    }
}

#[test]
fn test_data_locale_to_string() {
    struct TestCase {
        pub locale: &'static str,
        pub expected: &'static str,
    }

    for cas in [
        TestCase {
            locale: "und",
            expected: "und",
        },
        TestCase {
            locale: "und-u-cu-gbp",
            expected: "und-u-cu-gbp",
        },
        TestCase {
            locale: "en-ZA-u-cu-gbp",
            expected: "en-ZA-u-cu-gbp",
        },
    ] {
        let locale = cas.locale.parse::<DataLocale>().unwrap();
        writeable::assert_writeable_eq!(locale, cas.expected);
    }
}

#[test]
fn test_data_locale_from_string() {
    #[derive(Debug)]
    struct TestCase {
        pub input: &'static str,
        pub success: bool,
    }

    for cas in [
        TestCase {
            input: "und",
            success: true,
        },
        TestCase {
            input: "und-u-cu-gbp",
            success: true,
        },
        TestCase {
            input: "en-ZA-u-cu-gbp",
            success: true,
        },
        TestCase {
            input: "en...",
            success: false,
        },
        TestCase {
            input: "en-ZA-u-nu-arab",
            success: true,
        },
    ] {
        let data_locale = match (DataLocale::from_str(cas.input), cas.success) {
            (Ok(l), true) => l,
            (Err(_), false) => {
                continue;
            }
            (Ok(_), false) => {
                panic!("DataLocale parsed but it was supposed to fail: {cas:?}");
            }
            (Err(_), true) => {
                panic!("DataLocale was supposed to parse but it failed: {cas:?}");
            }
        };
        writeable::assert_writeable_eq!(data_locale, cas.input);
    }
}
