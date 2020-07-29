use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;
use tinystr::TinyStr16;

/// A top-level collection of related data keys.
#[non_exhaustive]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Category {
    Decimal,
    Plurals,
    PrivateUse(TinyStr16),
}

impl Category {
    /// Gets or builds a string form of this Category.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Category::Decimal => Cow::Borrowed("decimal"),
            Category::Plurals => Cow::Borrowed("plurals"),
            Category::PrivateUse(id) => {
                let mut result = String::from("x-");
                result.push_str(id.as_str());
                Cow::Owned(result)
            }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

/// A category, subcategory, and version, used for requesting data from a DataProvider.
///
/// The fields in a DataKey should generally be known at compile time.
///
/// Use `icu_data_key!` as a shortcut to create data keys in code.
#[derive(PartialEq, Copy, Clone)]
pub struct DataKey {
    pub category: Category,
    pub sub_category: TinyStr16,
    pub version: u32,
}

/// Shortcut to construct a data key from a URI-like syntax.
///
/// # Examples
///
/// ```
/// use icu_data_provider::icu_data_key;
///
/// // Data key to request version 1 of cardinal plural rules
/// let data_key = icu_data_key!(plurals: cardinal@1);
/// ```
#[macro_export]
macro_rules! icu_data_key {
    (decimal: $sub_category:tt @ $version:tt) => {
        icu_data_key!($crate::data_key::Category::Decimal, $sub_category, $version)
    };
    (plurals: $sub_category:tt @ $version:tt) => {
        icu_data_key!($crate::data_key::Category::Plurals, $sub_category, $version)
    };
    (x-$private_use:tt: $sub_category:tt @ $version:tt) => {
        icu_data_key!(
            $crate::data_key::Category::PrivateUse(stringify!($private_use).parse().unwrap()),
            $sub_category,
            $version
        )
    };
    ($category:expr, $sub_category:tt, $version:tt) => {
        $crate::data_key::DataKey {
            category: $category,
            // TODO: Parse to TinyStr at compile time
            sub_category: stringify!($sub_category).parse().unwrap(),
            version: $version,
        }
    };
}

#[cfg(test)]
fn test_data_key_macro(category: Category) {
    let data_key_1 = match category {
        Category::Decimal => icu_data_key!(decimal: foo@1),
        Category::Plurals => icu_data_key!(plurals: foo@1),
        Category::PrivateUse(_) => icu_data_key!(x-private: foo@1),
    };
    let data_key_2 = DataKey {
        category,
        sub_category: "foo".parse().unwrap(),
        version: 1,
    };
    assert_eq!(data_key_1, data_key_2);
}

#[test]
fn test_all_data_key_macros() {
    test_data_key_macro(Category::Decimal);
    test_data_key_macro(Category::Plurals);
    test_data_key_macro(Category::PrivateUse("private".parse().unwrap()));
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
    /// Gets the standard path components of this DataKey. These components should be used when
    /// persisting the DataKey on the filesystem or in structured data.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_data_provider::prelude::*;
    ///
    /// let data_key = icu_data_key!(plurals: cardinal@1);
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

/// The standard components of a DataKey path.
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
        DataKeyComponents {
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
    struct TestCase {
        pub data_key: DataKey,
        pub expected: &'static str,
    }
    let cases = [
        TestCase {
            data_key: icu_data_key!(plurals: cardinal@1),
            expected: "plurals/cardinal@1",
        },
        TestCase {
            data_key: icu_data_key!(x-private: cardinal@1),
            expected: "x-private/cardinal@1",
        },
        TestCase {
            data_key: icu_data_key!(x-maxlengthprivate: cardinal@1),
            expected: "x-maxlengthprivate/cardinal@1",
        },
        TestCase {
            data_key: icu_data_key!(plurals: maxlengthsubcatg@1),
            expected: "plurals/maxlengthsubcatg@1",
        },
        TestCase {
            data_key: icu_data_key!(plurals: cardinal@2147483647),
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
