// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Resource paths and related types.

use crate::error::Error;
use icu_locid::LanguageIdentifier;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::default::Default;
use std::fmt;
use std::fmt::Write;
use tinystr::{TinyStr16, TinyStr4};

/// A top-level collection of related resource keys.
#[non_exhaustive]
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum ResourceCategory {
    Aliases,
    DatePatterns,
    DateSymbols,
    Decimal,
    Icu4x,
    LikelySubtags,
    Plurals,
    TimeZones,
    Uniset,
    PrivateUse(TinyStr4),
}

impl ResourceCategory {
    /// Gets or builds a string form of this [`ResourceCategory`].
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Aliases => Cow::Borrowed("aliases"),
            Self::DatePatterns => Cow::Borrowed("date_patterns"),
            Self::DateSymbols => Cow::Borrowed("date_symbols"),
            Self::Decimal => Cow::Borrowed("decimal"),
            Self::Icu4x => Cow::Borrowed("icu4x"),
            Self::LikelySubtags => Cow::Borrowed("likelysubtags"),
            Self::Plurals => Cow::Borrowed("plurals"),
            Self::TimeZones => Cow::Borrowed("time_zones"),
            Self::Uniset => Cow::Borrowed("uniset"),
            Self::PrivateUse(id) => {
                let mut result = String::from("x-");
                result.push_str(id.as_str());
                Cow::Owned(result)
            }
        }
    }
}

impl fmt::Display for ResourceCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl writeable::Writeable for ResourceCategory {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        sink.write_str(&self.as_str())
    }

    fn write_len(&self) -> writeable::LengthHint {
        writeable::LengthHint::Exact(self.as_str().len())
    }
}

/// A category, subcategory, and version, used for requesting data from a
/// [`DataProvider`](crate::DataProvider).
///
/// The fields in a [`ResourceKey`] should generally be known at compile time.
///
/// Use [`resource_key!`] as a shortcut to create resource keys in code.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct ResourceKey {
    pub category: ResourceCategory,
    pub sub_category: TinyStr16,
    pub version: u16,
}

/// Shortcut to construct a const resource identifier.
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
///
/// // Create a private-use ResourceKey
/// const MY_PRIVATE_USE_KEY: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
/// assert_eq!("x-foo/bar@1", format!("{}", MY_PRIVATE_USE_KEY));
/// ```
#[macro_export]
macro_rules! resource_key {
    (aliases, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Aliases, $sub_category, $version)
    };
    (date_patterns, $sub_category:literal, $version:tt) => {
        $crate::resource_key!(
            $crate::ResourceCategory::DatePatterns,
            $sub_category,
            $version
        )
    };
    (date_symbols, $sub_category:literal, $version:tt) => {
        $crate::resource_key!(
            $crate::ResourceCategory::DateSymbols,
            $sub_category,
            $version
        )
    };
    (icu4x, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Icu4x, $sub_category, $version)
    };
    (likelysubtags, $sub_category:literal, $version:tt) => {
        $crate::resource_key!(
            $crate::ResourceCategory::LikelySubtags,
            $sub_category,
            $version
        )
    };
    (plurals, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Plurals, $sub_category, $version)
    };
    (time_zones, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::TimeZones, $sub_category, $version)
    };
    (uniset, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Uniset, $sub_category, $version)
    };
    (decimal, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Decimal, $sub_category, $version)
    };
    (x, $pu:literal, $sub_category:literal, $version:tt) => {
        $crate::resource_key!(
            $crate::ResourceCategory::PrivateUse($crate::internal::tinystr4!($pu)),
            $sub_category,
            $version
        )
    };
    ($category:expr, $sub_category:literal, $version:tt) => {
        $crate::ResourceKey {
            category: $category,
            sub_category: $crate::internal::tinystr16!($sub_category),
            version: $version,
        }
    };
}

impl fmt::Debug for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ResourceKey{")?;
        fmt::Display::fmt(self, f)?;
        f.write_char('}')?;
        Ok(())
    }
}

impl fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for ResourceKey {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        sink.write_str(&self.category.as_str())?;
        sink.write_char('/')?;
        sink.write_str(self.sub_category.as_str())?;
        sink.write_char('@')?;
        write!(sink, "{}", self.version)?;
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        writeable::LengthHint::Exact(2)
            + self.category.as_str().len()
            + self.sub_category.len()
            + if self.version < 10 {
                1
            } else if self.version < 100 {
                2
            } else if self.version < 1000 {
                3
            } else if self.version < 10000 {
                4
            } else {
                5
            }
    }
}

impl ResourceKey {
    /// Gets the standard path components of this [`ResourceKey`]. These components should be used when
    /// persisting the [`ResourceKey`] on the filesystem or in structured data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let resc_key = icu_plurals::provider::key::CARDINAL_V1;
    /// let components = resc_key.get_components();
    ///
    /// assert_eq!(
    ///     ["plurals", "cardinal@1"],
    ///     components.iter().collect::<Vec<&str>>()[..]
    /// );
    /// ```
    pub fn get_components(&self) -> ResourceKeyComponents {
        self.into()
    }

    /// Returns [`Ok`] if this data key matches the argument, or the appropriate error.
    ///
    /// Convenience method for data providers that support a single [`ResourceKey`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
    /// const FOO_BAZ: ResourceKey = icu_provider::resource_key!(x, "foo", "baz", 1);
    /// const BAR_BAZ: ResourceKey = icu_provider::resource_key!(x, "bar", "baz", 1);
    ///
    /// assert!(matches!(FOO_BAR.match_key(FOO_BAR), Ok(())));
    /// assert!(matches!(FOO_BAR.match_key(FOO_BAZ), Err(DataError::UnsupportedResourceKey(_))));
    /// assert!(matches!(FOO_BAR.match_key(BAR_BAZ), Err(DataError::UnsupportedResourceKey(_))));
    /// ```
    pub fn match_key(&self, key: Self) -> Result<(), Error> {
        if *self == key {
            Ok(())
        } else {
            Err(Error::UnsupportedResourceKey(*self))
        }
    }
}

/// The standard components of a [`ResourceKey`] path.
pub struct ResourceKeyComponents {
    components: [Cow<'static, str>; 2],
}

impl ResourceKeyComponents {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|cow| cow.borrow())
    }
}

impl From<&ResourceKey> for ResourceKeyComponents {
    fn from(resc_key: &ResourceKey) -> Self {
        Self {
            components: [
                resc_key.category.as_str(),
                // TODO: Evalute the size penalty of this format!
                Cow::Owned(format!(
                    "{}@{}",
                    resc_key.sub_category.as_str(),
                    resc_key.version
                )),
            ],
        }
    }
}

/// A variant and language identifier, used for requesting data from a
/// [`DataProvider`](crate::DataProvider).
///
/// The fields in a [`ResourceOptions`] are not generally known until runtime.
#[derive(PartialEq, Clone)]
pub struct ResourceOptions {
    // TODO: Consider making multiple variant fields.
    pub variant: Option<Cow<'static, str>>,
    pub langid: Option<LanguageIdentifier>,
}

impl fmt::Debug for ResourceOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourceOptions{{{}}}", self)
    }
}

impl fmt::Display for ResourceOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for ResourceOptions {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        let mut initial = true;
        for component in self.get_components().iter() {
            if initial {
                initial = false;
            } else {
                sink.write_char('/')?;
            }
            sink.write_str(component)?;
        }
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = 0;
        let mut initial = true;
        for component in self.get_components().iter() {
            if initial {
                initial = false;
            } else {
                result += 1;
            }
            result += component.len();
        }
        writeable::LengthHint::Exact(result)
    }
}

impl Default for ResourceOptions {
    fn default() -> Self {
        Self {
            variant: None,
            langid: None,
        }
    }
}

impl From<LanguageIdentifier> for ResourceOptions {
    /// Create a ResourceOptions with the given language identifier and an empty variant field.
    fn from(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            variant: None,
        }
    }
}

impl ResourceOptions {
    /// Gets the standard path components of this [`ResourceOptions`]. These components should be used when
    /// persisting the [`ResourceOptions`] on the filesystem or in structured data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use icu_provider::prelude::*;
    /// use icu_locid_macros::langid;
    ///
    /// let resc_options = ResourceOptions {
    ///     variant: Some(Cow::Borrowed("GBP")),
    ///     langid: Some(langid!("pt_BR")),
    /// };
    /// let components = resc_options.get_components();
    ///
    /// assert_eq!(
    ///     ["GBP", "pt-BR"],
    ///     components.iter().collect::<Vec<&str>>()[..]
    /// );
    /// ```
    pub fn get_components(&self) -> ResourceOptionsComponents {
        self.into()
    }

    /// Returns whether this [`ResourceOptions`] has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// The standard components of a [`ResourceOptions`] path.
pub struct ResourceOptionsComponents {
    components: [Option<Cow<'static, str>>; 2],
}

impl ResourceOptionsComponents {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.components
            .iter()
            .filter_map(|option| option.as_ref().map(|cow| cow.borrow()))
    }
}

impl From<&ResourceOptions> for ResourceOptionsComponents {
    fn from(resc_options: &ResourceOptions) -> Self {
        Self {
            components: [
                resc_options.variant.as_ref().cloned(),
                resc_options
                    .langid
                    .as_ref()
                    .map(|s| Cow::Owned(s.to_string())),
            ],
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ResourcePath {
    pub key: ResourceKey,
    pub options: ResourceOptions,
}

impl fmt::Debug for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ResourcePath{{{}}}", self)
    }
}

impl fmt::Display for ResourcePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeable::Writeable::write_to(self, f)
    }
}

impl writeable::Writeable for ResourcePath {
    fn write_to<W: std::fmt::Write + ?Sized>(&self, sink: &mut W) -> std::fmt::Result {
        writeable::Writeable::write_to(&self.key, sink)?;
        if !self.options.is_empty() {
            sink.write_char('/')?;
            writeable::Writeable::write_to(&self.options, sink)?;
        }
        Ok(())
    }

    fn write_len(&self) -> writeable::LengthHint {
        let mut result = writeable::Writeable::write_len(&self.key);
        if !self.options.is_empty() {
            result += writeable::Writeable::write_len(&self.options) + 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinystr::tinystr4;

    struct KeyTestCase {
        pub resc_key: ResourceKey,
        pub expected: &'static str,
    }

    fn get_key_test_cases() -> [KeyTestCase; 4] {
        [
            KeyTestCase {
                resc_key: resource_key!(plurals, "cardinal", 1),
                expected: "plurals/cardinal@1",
            },
            KeyTestCase {
                resc_key: ResourceKey {
                    category: ResourceCategory::PrivateUse(tinystr4!("priv")),
                    sub_category: tinystr::tinystr16!("cardinal"),
                    version: 1,
                },
                expected: "x-priv/cardinal@1",
            },
            KeyTestCase {
                resc_key: resource_key!(plurals, "maxlengthsubcatg", 1),
                expected: "plurals/maxlengthsubcatg@1",
            },
            KeyTestCase {
                resc_key: resource_key!(plurals, "cardinal", 65535),
                expected: "plurals/cardinal@65535",
            },
        ]
    }

    #[test]
    fn test_options_to_string() {
        for cas in get_key_test_cases().iter() {
            assert_eq!(cas.expected, cas.resc_key.to_string());
            writeable::assert_writeable_eq!(cas.expected, &cas.resc_key);
            assert_eq!(
                cas.expected,
                cas.resc_key
                    .get_components()
                    .iter()
                    .collect::<Vec<&str>>()
                    .join("/")
            );
        }
    }

    struct OptionsTestCase {
        pub resc_options: ResourceOptions,
        pub expected: &'static str,
    }

    fn get_options_test_cases() -> [OptionsTestCase; 3] {
        use icu_locid_macros::langid;
        [
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: None,
                    langid: Some(LanguageIdentifier::und()),
                },
                expected: "und",
            },
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(LanguageIdentifier::und()),
                },
                expected: "GBP/und",
            },
            OptionsTestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(langid!("en-ZA")),
                },
                expected: "GBP/en-ZA",
            },
        ]
    }

    #[test]
    fn test_key_to_string() {
        for cas in get_options_test_cases().iter() {
            assert_eq!(cas.expected, cas.resc_options.to_string());
            writeable::assert_writeable_eq!(cas.expected, &cas.resc_options);
            assert_eq!(
                cas.expected,
                cas.resc_options
                    .get_components()
                    .iter()
                    .collect::<Vec<&str>>()
                    .join("/")
            );
        }
    }

    #[test]
    fn test_resource_path_to_string() {
        for key_cas in get_key_test_cases().iter() {
            for options_cas in get_options_test_cases().iter() {
                let expected = if options_cas.resc_options.is_empty() {
                    key_cas.expected.to_string()
                } else {
                    format!("{}/{}", key_cas.expected, options_cas.expected)
                };
                let resource_path = ResourcePath {
                    key: key_cas.resc_key,
                    // Note: once https://github.com/rust-lang/rust/pull/80470 is accepted,
                    // we won't have to clone here.
                    options: options_cas.resc_options.clone(),
                };
                assert_eq!(expected, resource_path.to_string());
                writeable::assert_writeable_eq!(expected, &resource_path);
            }
        }
    }
}
