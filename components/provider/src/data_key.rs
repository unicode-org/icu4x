// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;
use tinystr::{TinyStr16, TinyStr4};

// Re-export tinystr16 for crate macro data_key!()
pub(crate) use tinystr::tinystr16;

/// A top-level collection of related data keys.
#[non_exhaustive]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataCategory {
    Decimal,
    Plurals,
    Dates,
    PrivateUse(TinyStr4),
}

impl DataCategory {
    /// Gets or builds a string form of this `DataCategory`.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Self::Decimal => Cow::Borrowed("decimal"),
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

impl fmt::Display for DataCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// A category, subcategory, and version, used for requesting data from a `DataProvider`.
///
/// The fields in a `DataKey` should generally be known at compile time.
///
/// Use `icu_data_key!` as a shortcut to create data keys in code.
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct DataKey {
    pub category: DataCategory,
    pub sub_category: TinyStr16,
    pub version: u32,
}

/// Internal-only: Shortcut to construct a const data key.
macro_rules! data_key {
    (decimal, $sub_category:literal, $version:tt) => {
        data_key!($crate::DataCategory::Decimal, $sub_category, $version)
    };
    (plurals, $sub_category:literal, $version:tt) => {
        data_key!($crate::DataCategory::Plurals, $sub_category, $version)
    };
    (dates, $sub_category:literal, $version:tt) => {
        data_key!($crate::DataCategory::Dates, $sub_category, $version)
    };
    ($category:expr, $sub_category:literal, $version:tt) => {
        $crate::DataKey {
            category: $category,
            sub_category: $crate::data_key::tinystr16!($sub_category),
            version: $version,
        }
    };
}

impl fmt::Debug for DataKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DataKey{")?;
        fmt::Display::fmt(self, f)?;
        f.write_char('}')?;
        Ok(())
    }
}

impl fmt::Display for DataKey {
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

impl DataKey {
    /// Gets the standard path components of this `DataKey`. These components should be used when
    /// persisting the `DataKey` on the filesystem or in structured data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::structs;
    ///
    /// let data_key = structs::plurals::key::CARDINAL_V1;
    /// let components = data_key.get_components();
    ///
    /// assert_eq!(
    ///     ["plurals", "cardinal@1"],
    ///     components.iter().collect::<Vec<&str>>()[..]
    /// );
    /// ```
    pub fn get_components(&self) -> DataKeyComponents {
        self.into()
    }
}

/// The standard components of a `DataKey` path.
pub struct DataKeyComponents {
    components: [Cow<'static, str>; 2],
}

impl DataKeyComponents {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.components.iter().map(|cow| cow.borrow())
    }
}

impl From<&DataKey> for DataKeyComponents {
    fn from(data_key: &DataKey) -> Self {
        Self {
            components: [
                data_key.category.as_str(),
                // TODO: Evalute the size penalty of this format!
                Cow::Owned(format!(
                    "{}@{}",
                    data_key.sub_category.as_str(),
                    data_key.version
                )),
            ],
        }
    }
}

#[test]
fn test_to_string() {
    use tinystr::tinystr4;
    struct TestCase {
        pub data_key: DataKey,
        pub expected: &'static str,
    }
    let cases = [
        TestCase {
            data_key: data_key!(plurals, "cardinal", 1),
            expected: "plurals/cardinal@1",
        },
        TestCase {
            data_key: DataKey {
                category: DataCategory::PrivateUse(tinystr4!("priv")),
                sub_category: tinystr16!("cardinal"),
                version: 1,
            },
            expected: "x-priv/cardinal@1",
        },
        TestCase {
            data_key: data_key!(plurals, "maxlengthsubcatg", 1),
            expected: "plurals/maxlengthsubcatg@1",
        },
        TestCase {
            data_key: data_key!(plurals, "cardinal", 2147483647),
            expected: "plurals/cardinal@2147483647",
        },
    ];
    for cas in cases.iter() {
        assert_eq!(cas.expected, cas.data_key.to_string());
        assert_eq!(
            cas.expected,
            cas.data_key
                .get_components()
                .iter()
                .collect::<Vec<&str>>()
                .join("/")
        );
    }
}
