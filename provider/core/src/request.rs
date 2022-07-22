// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;
use core::default::Default;
use core::fmt;
use core::fmt::Debug;
use icu_locid::extensions::unicode as unicode_ext;
use icu_locid::ordering::SubtagOrderingResult;
use icu_locid::{LanguageIdentifier, Locale};
use writeable::{LengthHint, Writeable};

#[cfg(doc)]
use icu_locid::subtags::Variant;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct DataRequest<'a> {
    pub locale: DataLocale<'a>,
    pub metadata: DataRequestMetadata,
}

impl fmt::Display for DataRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.locale, f)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct DataRequestMetadata;

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub struct DataLocale<'a> {
    langid: &'a LanguageIdentifier,
    keywords: &'a unicode_ext::Keywords,
}

impl<'a> Default for DataLocale<'a> {
    fn default() -> Self {
        // We need static homes for the default values.
        static LANGID: LanguageIdentifier = LanguageIdentifier::UND;
        static KEYWORDS: unicode_ext::Keywords = unicode_ext::Keywords::new();
        Self {
            langid: &LANGID,
            keywords: &KEYWORDS,
        }
    }
}

impl fmt::Debug for DataLocale<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataLocale{{{}}}", self)
    }
}

impl fmt::Display for DataLocale<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl Writeable for DataLocale<'_> {
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

impl<'a> From<&'a LanguageIdentifier> for DataLocale<'a> {
    fn from(langid: &'a LanguageIdentifier) -> Self {
        Self {
            langid,
            ..Default::default()
        }
    }
}

impl<'a> From<&'a Locale> for DataLocale<'a> {
    fn from(locale: &'a Locale) -> Self {
        Self {
            langid: &locale.id,
            keywords: &locale.extensions.unicode.keywords,
        }
    }
}

impl Into<Locale> for DataLocale<'_> {
    /// Expensive
    fn into(self) -> Locale {
        let mut locale = Locale::from(self.langid.clone());
        locale.extensions.unicode.keywords = self.keywords.clone();
        locale
    }
}

impl DataLocale<'_> {
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
    ///     let a_loc = a.parse::<Locale>().unwrap();
    ///     assert_eq!(a, DataLocale::from(&a_loc).to_string());
    ///     assert!(DataLocale::from(&a_loc).strict_cmp(a.as_bytes()) == Ordering::Equal, "{} == {}", a, a);
    ///     assert!(DataLocale::from(&a_loc).strict_cmp(b.as_bytes()) == Ordering::Less, "{} < {}", a, b);
    ///     let b_loc = b.parse::<Locale>().unwrap();
    ///     assert_eq!(b, DataLocale::from(&b_loc).to_string());
    ///     assert!(DataLocale::from(&b_loc).strict_cmp(b.as_bytes()) == Ordering::Equal, "{} == {}", b, b);
    ///     assert!(DataLocale::from(&b_loc).strict_cmp(a.as_bytes()) == Ordering::Greater, "{} > {}", b, a);
    /// }
    /// ```
    pub fn strict_cmp(&self, other: &[u8]) -> Ordering {
        let subtags = other.split(|b| *b == b'-');
        let mut subtag_result = self.langid.strict_cmp_iter(subtags);
        if !self.keywords.is_empty() {
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

    /// Returns whether this [`DataLocale`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Gets the [`LanguageIdentifier`] for this [`DataLocale`].
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
    ///     locale: Default::default(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// let lang_id = langid!("ar-EG");
    /// let req_with_langid = DataRequest {
    ///     locale: (&lang_id).into(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// assert_eq!(req_no_langid.locale.get_langid(), &langid!("und"));
    /// assert_eq!(req_with_langid.locale.get_langid(), &lang_id);
    /// ```
    pub fn get_langid(&self) -> &LanguageIdentifier {
        &self.langid
    }

    pub fn get_unicode_keyword(&self, key: unicode_ext::Key) -> Option<&unicode_ext::Value> {
        self.keywords.get(&key)
    }
}

#[test]
fn test_data_locale_to_string() {
    struct TestCase {
        pub locale: Locale,
        pub expected: &'static str,
    }

    for cas in [
        TestCase {
            locale: Locale::UND,
            expected: "und",
        },
        TestCase {
            locale: "und-u-cu-gbp".parse::<Locale>().unwrap(),
            expected: "und-u-cu-gbp",
        },
        TestCase {
            locale: "en-ZA-u-cu-gbp".parse::<Locale>().unwrap(),
            expected: "en-ZA-u-cu-gbp",
        },
    ] {
        assert_eq!(cas.expected, DataLocale::from(&cas.locale).to_string());
        writeable::assert_writeable_eq!(&cas.locale, cas.expected);
    }
}
