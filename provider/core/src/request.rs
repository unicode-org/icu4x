// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;
use core::default::Default;
use core::fmt;
use core::fmt::Debug;
use icu_locid::extensions::unicode as unicode_ext;
use icu_locid::ordering::SubtagOrderingResult;
use icu_locid::subtags::{Language, Region, Script, Variants};
use icu_locid::{LanguageIdentifier, Locale};
use writeable::{LengthHint, Writeable};

#[cfg(doc)]
use icu_locid::subtags::Variant;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DataRequest {
    pub options: DataOptions,
    pub metadata: DataRequestMetadata,
}

impl fmt::Display for DataRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.options, f)
    }
}

impl AsMut<DataOptions> for DataRequest {
    fn as_mut(&mut self) -> &mut DataOptions {
        &mut self.options
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub struct DataRequestMetadata;

/// A variant and language identifier, used for requesting data from a data provider.
///
/// The fields in a [`DataOptions`] are not generally known until runtime.
#[derive(PartialEq, Clone, Default, Eq, Hash)]
pub struct DataOptions {
    langid: LanguageIdentifier,
    keywords: unicode_ext::Keywords,
}

impl fmt::Debug for DataOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataOptions{{{}}}", self)
    }
}

impl fmt::Display for DataOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl AsMut<DataOptions> for DataOptions {
    fn as_mut(&mut self) -> &mut DataOptions {
        self
    }
}

impl Writeable for DataOptions {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.langid.write_to(sink)?;
        if !self.keywords.is_empty() {
            sink.write_str("-u-")?;
            self.keywords.write_to(sink)?;
        }
        Ok(())
    }

    fn write_len(&self) -> LengthHint {
        self.langid.write_len()
            + if !self.keywords.is_empty() {
                self.keywords.write_len() + 3
            } else {
                LengthHint::exact(0)
            }
    }
}

impl From<LanguageIdentifier> for DataOptions {
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid,
            keywords: unicode_ext::Keywords::new(),
        }
    }
}

impl From<Locale> for DataOptions {
    fn from(locale: Locale) -> Self {
        Self {
            langid: locale.id,
            keywords: locale.extensions.unicode.keywords,
        }
    }
}

impl From<&Locale> for DataOptions {
    fn from(locale: &Locale) -> Self {
        Self {
            langid: locale.id.clone(),
            keywords: locale.extensions.unicode.keywords.clone(),
        }
    }
}

impl DataOptions {
    /// Compare this [`DataOptions`] with BCP-47 bytes.
    ///
    /// The return value is equivalent to what would happen if you first converted this
    /// [`DataOptions`] to a BCP-47 string and then performed a byte comparison.
    ///
    /// This function is case-sensitive and results in a *total order*, so it is appropriate for
    /// binary search. The only argument producing [`Ordering::Equal`] is `self.to_string()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::DataOptions;
    /// use icu_locid::Locale;
    /// use std::cmp::Ordering;
    ///
    /// let bcp47_strings: &[&str] = &[
    ///     "ca-ES",
    ///     "ca-ES-u-ca-buddhist",
    ///     "ca-ES-valencia",
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
    ///     assert!(a.cmp(b) == Ordering::Less);
    ///     let a_loc: DataOptions = a.parse::<Locale>().unwrap().into();
    ///     assert_eq!(a, a_loc.to_string());
    ///     assert!(a_loc.strict_cmp(a.as_bytes()) == Ordering::Equal, "{} == {}", a, a);
    ///     assert!(a_loc.strict_cmp(b.as_bytes()) == Ordering::Less, "{} < {}", a, b);
    ///     let b_loc: DataOptions = b.parse::<Locale>().unwrap().into();
    ///     assert_eq!(b, b_loc.to_string());
    ///     assert!(b_loc.strict_cmp(b.as_bytes()) == Ordering::Equal, "{} == {}", b, b);
    ///     assert!(b_loc.strict_cmp(a.as_bytes()) == Ordering::Greater, "{} > {}", b, a);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        let subtags = other.split(|b| *b == b'-');
        let mut subtag_result = self.langid.strict_cmp_iter(subtags);
        if self.has_unicode_ext() {
            let mut subtags = match subtag_result {
                SubtagOrderingResult::Subtags(s) => s,
                SubtagOrderingResult::Ordering(o) => return o,
            };
            match subtags.next() {
                Some(b"u") => (),
                Some(s) => return s.cmp(b"u").reverse(),
                None => return Ordering::Greater,
            }
            subtag_result = self.keywords.strict_cmp_iter(subtags);
        }
        subtag_result.end()
    }
}

impl DataOptions {
    /// Returns whether this [`DataOptions`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Returns whether the [`LanguageIdentifier`] associated with this request is `und`.
    ///
    /// Note that this only checks the language identifier; extension keywords may also be set.
    /// To check the entire `DataOptions`, use [`DataOptions::is_empty()`].
    pub fn is_langid_und(&self) -> bool {
        self.langid == LanguageIdentifier::UND
    }

    /// Gets the [`LanguageIdentifier`] for this [`DataOptions`].
    ///
    /// This may allocate memory if there are variant subtags. If you need only the language,
    /// script, and/or region subtag, use the specific getters for those subtags:
    ///
    /// - [`DataOptions::language()`]
    /// - [`DataOptions::script()`]
    /// - [`DataOptions::region()`]
    ///
    /// If you have ownership over the `DataOptions`, use [`DataOptions::into_locale()`]
    /// and then access the `id` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::langid;
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: DataKey = icu_provider::data_key!("foo/bar@1");
    ///
    /// let req_no_langid = DataRequest {
    ///     options: DataOptions::default(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// let req_with_langid = DataRequest {
    ///     options: langid!("ar-EG").into(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// assert_eq!(req_no_langid.options.get_langid(), langid!("und"));
    /// assert_eq!(req_with_langid.options.get_langid(), langid!("ar-EG"));
    /// ```
    pub fn get_langid(&self) -> LanguageIdentifier {
        self.langid.clone()
    }

    /// Overrides the entire [`LanguageIdentifier`] portion of this [`DataOptions`].
    #[inline]
    pub fn set_langid(&mut self, lid: LanguageIdentifier) {
        self.langid = lid;
    }

    /// Converts this [`DataOptions`] into a [`Locale`].
    ///
    /// See also [`DataOptions::get_langid()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{langid, subtags_language as language, subtags_region as region, Locale};
    /// use icu_provider::prelude::*;
    ///
    /// let locale: Locale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    /// let options: DataOptions = locale.into();
    ///
    /// assert_eq!(options.to_string(), "it-IT-u-ca-coptic");
    /// assert_eq!(options.get_langid(), langid!("it-IT"));
    /// assert_eq!(options.language(), language!("it"));
    /// assert_eq!(options.script(), None);
    /// assert_eq!(options.region(), Some(region!("IT")));
    ///
    /// let locale = options.into_locale();
    /// assert_eq!(locale.to_string(), "it-IT-u-ca-coptic");
    /// ```
    pub fn into_locale(self) -> Locale {
        let mut loc = Locale {
            id: self.langid,
            ..Default::default()
        };
        loc.extensions.unicode.keywords = self.keywords;
        loc
    }

    /// Returns the [`Language`] for this [`DataOptions`].
    #[inline]
    pub fn language(&self) -> Language {
        self.langid.language
    }

    /// Returns the [`Language`] for this [`DataOptions`].
    #[inline]
    pub fn set_language(&mut self, language: Language) {
        self.langid.language = language;
    }

    /// Returns the [`Script`] for this [`DataOptions`].
    #[inline]
    pub fn script(&self) -> Option<Script> {
        self.langid.script
    }

    /// Sets the [`Script`] for this [`DataOptions`].
    #[inline]
    pub fn set_script(&mut self, script: Option<Script>) {
        self.langid.script = script;
    }

    /// Returns the [`Region`] for this [`DataOptions`].
    #[inline]
    pub fn region(&self) -> Option<Region> {
        self.langid.region
    }

    /// Sets the [`Region`] for this [`DataOptions`].
    #[inline]
    pub fn set_region(&mut self, region: Option<Region>) {
        self.langid.region = region;
    }

    /// Returns whether there are any [`Variant`] subtags in this [`DataOptions`].
    #[inline]
    pub fn has_variants(&self) -> bool {
        !self.langid.variants.is_empty()
    }

    #[inline]
    pub fn set_variants(&mut self, variants: Variants) {
        self.langid.variants = variants;
    }

    /// Removes all [`Variant`] subtags in this [`DataOptions`].
    #[inline]
    pub fn clear_variants(&mut self) -> Variants {
        self.langid.variants.clear()
    }

    /// Gets the value of the specified Unicode extension keyword for this [`DataOptions`].
    #[inline]
    pub fn get_unicode_ext(&self, key: &unicode_ext::Key) -> Option<unicode_ext::Value> {
        self.keywords.get(key).cloned()
    }

    /// Returns whether there are any Unicode extension keywords in this [`DataOptions`].
    #[inline]
    pub fn has_unicode_ext(&self) -> bool {
        !self.keywords.is_empty()
    }

    /// Returns whether a specific Unicode extension keyword is present in this [`DataOptions`].
    #[inline]
    pub fn contains_unicode_ext(&self, key: &unicode_ext::Key) -> bool {
        self.keywords.contains_key(key)
    }

    /// Returns whether this [`DataOptions`] contains a Unicode extension keyword
    /// with the specified key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
    /// use icu_provider::prelude::*;
    ///
    /// let locale: Locale = "it-IT-u-ca-coptic".parse().expect("Valid BCP-47");
    /// let options: DataOptions = locale.into();
    ///
    /// assert_eq!(options.get_unicode_ext(&key!("hc")), None);
    /// assert_eq!(
    ///     options.get_unicode_ext(&key!("ca")),
    ///     Some(value!("coptic"))
    /// );
    /// assert!(options.matches_unicode_ext(&key!("ca"), &value!("coptic"),));
    /// ```
    #[inline]
    pub fn matches_unicode_ext(&self, key: &unicode_ext::Key, value: &unicode_ext::Value) -> bool {
        self.keywords.get(key) == Some(value)
    }

    /// Sets the value for a specific Unicode extension keyword on this [`DataOptions`].
    #[inline]
    pub fn set_unicode_ext(
        &mut self,
        key: unicode_ext::Key,
        value: unicode_ext::Value,
    ) -> Option<unicode_ext::Value> {
        self.keywords.set(key, value)
    }

    /// Removes a specific Unicode extension keyword from this [`DataOptions`], returning
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

#[test]
fn test_options_to_string() {
    struct OptionsTestCase {
        pub options: DataOptions,
        pub expected: &'static str,
    }

    for cas in [
        OptionsTestCase {
            options: Locale::UND.into(),
            expected: "und",
        },
        OptionsTestCase {
            options: "und-u-cu-gbp".parse::<Locale>().unwrap().into(),
            expected: "und-u-cu-gbp",
        },
        OptionsTestCase {
            options: "en-ZA-u-cu-gbp".parse::<Locale>().unwrap().into(),
            expected: "en-ZA-u-cu-gbp",
        },
    ] {
        assert_eq!(cas.expected, cas.options.to_string());
        writeable::assert_writeable_eq!(&cas.options, cas.expected);
    }
}
