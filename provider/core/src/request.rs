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
use icu_locale_core::extensions::unicode::Value;
use icu_locale_core::{LanguageIdentifier, Locale};
use writeable::Writeable;

use alloc::string::String;
use core::ops::Deref;
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
    pub langid: &'a LanguageIdentifier,
    /// Key-specific request attributes
    pub key_attributes: &'a DataKeyAttributes,
    /// Metadata that may affect the behavior of the data provider.
    pub metadata: DataRequestMetadata,
}

impl fmt::Display for DataRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.langid, f)
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

// TODO: replace the `-x-` encoding by something more flexible
#[doc(hidden)]
impl DataRequest<'_> {
    pub fn legacy_cmp(&self, bytes: &[u8]) -> Ordering {
        struct LegacyComparator<'a>(&'a LanguageIdentifier, &'a DataKeyAttributes);
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
        LegacyComparator(self.langid, self.key_attributes).writeable_cmp_bytes(bytes)
    }

    pub fn legacy_encode(&self) -> String {
        self.langid.write_to_string().into_owned()
            + if self.key_attributes.is_empty() {
                ""
            } else {
                "-x-"
            }
            + self.key_attributes
    }

    pub fn legacy_decode(encoded: &str) -> Option<(LanguageIdentifier, DataKeyAttributes)> {
        let mut iter = encoded.splitn(2, "-x-");
        #[allow(clippy::unwrap_used)] // at least one element
        let l = iter.next().unwrap().parse().ok()?;
        let a = iter.next().unwrap_or("").parse().ok()?;
        Some((l, a))
    }
}

/// TODO: Replace by preferences everywhere
#[derive(PartialEq, Clone, Default, Eq, Hash, Debug)]
#[allow(clippy::exhaustive_structs)] // to be killed
pub struct DataLocale {
    /// Locale.id
    pub id: LanguageIdentifier,
    /// Locale.extensions.unicode.keywords
    pub keywords: unicode_ext::Keywords,
}

impl From<Locale> for DataLocale {
    fn from(locale: Locale) -> Self {
        Self {
            id: locale.id,
            keywords: locale.extensions.unicode.keywords,
        }
    }
}

impl From<LanguageIdentifier> for DataLocale {
    fn from(id: LanguageIdentifier) -> Self {
        Self {
            id,
            keywords: unicode_ext::Keywords::new(),
        }
    }
}

/// TODO
#[derive(Clone, Default)]
pub struct DataKeyAttributes {
    value: DataKeyAttributesInner,
}

#[derive(Clone, Default)]
enum DataKeyAttributesInner {
    #[default]
    Empty,
    Boxed(alloc::boxed::Box<str>),
    Stack(TinyAsciiStr<23>),
    // NOTE: In the future, a `Static` variant could be added to allow `data_locale!("...")`
    // Static(&'static str),
}

impl<'a> Default for &'a DataKeyAttributes {
    fn default() -> Self {
        static DEFAULT: DataKeyAttributes = DataKeyAttributes {
            value: DataKeyAttributesInner::Empty,
        };
        &DEFAULT
    }
}

impl Deref for DataKeyAttributes {
    type Target = str;
    #[inline]
    fn deref(&self) -> &Self::Target {
        match &self.value {
            DataKeyAttributesInner::Empty => "",
            DataKeyAttributesInner::Boxed(s) => s.deref(),
            DataKeyAttributesInner::Stack(s) => s.as_str(),
        }
    }
}

impl PartialEq for DataKeyAttributes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for DataKeyAttributes {}

impl PartialOrd for DataKeyAttributes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataKeyAttributes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl Debug for DataKeyAttributes {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl Hash for DataKeyAttributes {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.deref().hash(state)
    }
}

impl FromStr for DataKeyAttributes {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Self {
                value: DataKeyAttributesInner::Empty,
            })
        } else if let Ok(tiny) = s.parse() {
            Ok(Self {
                value: DataKeyAttributesInner::Stack(tiny),
            })
        } else {
            Ok(Self {
                value: DataKeyAttributesInner::Boxed(s.into()),
            })
        }
    }
}

impl DataKeyAttributes {
    /// Creates a [`DataKeyAttributes`] from an iterator of individual values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinystr::tinystr;
    /// use icu_provider::prelude::*;
    ///
    /// // Single value:
    /// let a = DataKeyAttributes::try_from_iter([tinystr!(8, "abc")]).unwrap();
    /// let b = "abc".parse::<DataKeyAttributes>().unwrap();
    /// assert_eq!(a, b);
    ///
    /// // Multiple values:
    /// let a = DataKeyAttributes::try_from_iter([tinystr!(8, "abc"), tinystr!(8, "defg")])
    ///     .unwrap();
    /// let b = "abc-defg".parse::<DataKeyAttributes>().unwrap();
    /// assert_eq!(a, b);
    /// ```
    ///
    /// The iterator can't be empty:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// assert!(DataKeyAttributes::try_from_iter([]).is_err());
    /// ```
    pub fn try_from_iter(
        iter: impl IntoIterator<Item = TinyAsciiStr<8>>,
    ) -> Result<Self, DataError> {
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
                value: DataKeyAttributesInner::Stack(builder.parse().unwrap()),
            })
        } else {
            Ok(Self {
                value: DataKeyAttributesInner::Boxed(builder.into()),
            })
        }
    }

    /// Creates a [`DataKeyAttributes`] from a single subtag.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_core::extensions::private::subtag;
    /// use icu_provider::prelude::*;
    ///
    /// // Single auxiliary key:
    /// let a = DataKeyAttributes::from_tinystr("abc".parse().unwrap());
    /// let b = "abc".parse::<DataKeyAttributes>().unwrap();
    /// assert_eq!(a, b);
    /// ```
    pub const fn from_tinystr(input: TinyAsciiStr<8>) -> Self {
        Self {
            value: DataKeyAttributesInner::Stack(input.resize()),
        }
    }

    /// TODO
    pub fn from_unicode_value(value: &Value) -> Self {
        if let Some(&tiny) = value.as_single_subtag() {
            Self::from_tinystr(tiny)
        } else {
            #[allow(clippy::unwrap_used)] // valid subtags
            Self::try_from_iter(value.as_tinystr_slice().iter().copied()).unwrap()
        }
    }

    /// TODO
    pub fn single(&self) -> Option<TinyAsciiStr<8>> {
        if self.is_empty() {
            return None;
        }
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
