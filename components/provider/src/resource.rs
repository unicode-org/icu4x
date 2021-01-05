// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use icu_locid::LanguageIdentifier;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::default::Default;
use std::fmt;
use std::fmt::Write;
use tinystr::{TinyStr16, TinyStr4};

/// Re-export tinystr4 for crate macro resource_key!()
pub use tinystr::tinystr4;

/// Re-export tinystr16 for crate macro resource_key!()
pub use tinystr::tinystr16;

/// A top-level collection of related resource keys.
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ResourceCategory {
    Icu4x,
    Plurals,
    Dates,
    PrivateUse(TinyStr4),
}

impl ResourceCategory {
    /// Gets or builds a string form of this `ResourceCategory`.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Icu4x => Cow::Borrowed("icu4x"),
            Self::Plurals => Cow::Borrowed("plurals"),
            Self::Dates => Cow::Borrowed("dates"),
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

/// A category, subcategory, and version, used for requesting data from a `DataProvider`.
///
/// The fields in a `ResourceKey` should generally be known at compile time.
///
/// Use `resource_key!` as a shortcut to create resource keys in code.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ResourceKey {
    pub category: ResourceCategory,
    pub sub_category: TinyStr16,
    pub version: u32,
}

/// Shortcut to construct a const resource identifier.
///
/// # Example
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
    (icu4x, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Icu4x, $sub_category, $version)
    };
    (decimal, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Decimal, $sub_category, $version)
    };
    (plurals, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Plurals, $sub_category, $version)
    };
    (dates, $sub_category:literal, $version:tt) => {
        $crate::resource_key!($crate::ResourceCategory::Dates, $sub_category, $version)
    };
    (x, $pu:literal, $sub_category:literal, $version:tt) => {
        $crate::resource_key!(
            $crate::ResourceCategory::PrivateUse($crate::resource::tinystr4!($pu)),
            $sub_category,
            $version
        )
    };
    ($category:expr, $sub_category:literal, $version:tt) => {
        $crate::ResourceKey {
            category: $category,
            sub_category: $crate::resource::tinystr16!($sub_category),
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
        write!(
            f,
            "{}/{}@{}",
            &self.category.as_str(),
            self.sub_category,
            self.version
        )
    }
}

impl ResourceKey {
    /// Gets the standard path components of this `ResourceKey`. These components should be used when
    /// persisting the `ResourceKey` on the filesystem or in structured data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::structs;
    ///
    /// let resc_key = structs::plurals::key::CARDINAL_V1;
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

    /// Returns Ok if this DataKey matches the argument, or the appropriate error.
    ///
    /// Convenience method for data providers that support a single ResourceKey.
    ///
    /// # Example
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
    /// assert!(matches!(FOO_BAR.match_key(BAR_BAZ), Err(DataError::UnsupportedCategory(_))));
    /// ```
    pub fn match_key(&self, key: ResourceKey) -> Result<(), Error> {
        if self.category != key.category {
            Err(Error::UnsupportedCategory(self.category))
        } else if *self != key {
            Err(Error::UnsupportedResourceKey(*self))
        } else {
            Ok(())
        }
    }
}

/// The standard components of a `ResourceKey` path.
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

/// A variant and language identifier, used for requesting data from a `DataProvider`.
///
/// The fields in a `ResourceOptions` are not generally known until runtime.
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
        let components = self.get_components();
        let mut it = components.iter();
        if let Some(s) = it.next() {
            f.write_str(s)?;
            for s in it {
                f.write_char('/')?;
                f.write_str(s)?;
            }
        }
        Ok(())
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

impl ResourceOptions {
    /// Gets the standard path components of this `ResourceOptions`. These components should be used when
    /// persisting the `ResourceOptions` on the filesystem or in structured data.
    ///
    /// # Example
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

    /// Returns whether this `ResourceOptions` has all empty fields (no components).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// The standard components of a ResourceOptions path.
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

#[derive(Clone, Debug, PartialEq)]
pub struct ResourcePath {
    pub key: ResourceKey,
    pub options: ResourceOptions,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinystr::tinystr4;

    #[test]
    fn test_entry_to_string() {
        struct TestCase {
            pub resc_key: ResourceKey,
            pub expected: &'static str,
        }
        let cases = [
            TestCase {
                resc_key: resource_key!(plurals, "cardinal", 1),
                expected: "plurals/cardinal@1",
            },
            TestCase {
                resc_key: ResourceKey {
                    category: ResourceCategory::PrivateUse(tinystr4!("priv")),
                    sub_category: tinystr16!("cardinal"),
                    version: 1,
                },
                expected: "x-priv/cardinal@1",
            },
            TestCase {
                resc_key: resource_key!(plurals, "maxlengthsubcatg", 1),
                expected: "plurals/maxlengthsubcatg@1",
            },
            TestCase {
                resc_key: resource_key!(plurals, "cardinal", 2147483647),
                expected: "plurals/cardinal@2147483647",
            },
        ];
        for cas in cases.iter() {
            assert_eq!(cas.expected, cas.resc_key.to_string());
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

    #[test]
    fn test_key_to_string() {
        use icu_locid_macros::langid;
        struct TestCase {
            pub resc_options: ResourceOptions,
            pub expected: &'static str,
        }
        let cases = [
            TestCase {
                resc_options: ResourceOptions {
                    variant: None,
                    langid: Some(LanguageIdentifier::default()),
                },
                expected: "und",
            },
            TestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(LanguageIdentifier::default()),
                },
                expected: "GBP/und",
            },
            TestCase {
                resc_options: ResourceOptions {
                    variant: Some(Cow::Borrowed("GBP")),
                    langid: Some(langid!("en-ZA")),
                },
                expected: "GBP/en-ZA",
            },
        ];
        for cas in cases.iter() {
            assert_eq!(cas.expected, cas.resc_options.to_string());
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
}
