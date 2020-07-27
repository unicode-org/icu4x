use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;
use std::path::PathBuf;
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
#[derive(PartialEq, Copy, Clone, Debug)]
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
        Category::PrivateUse(s) => icu_data_key!(Category::PrivateUse(s), foo, 1),
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
    /// Append standard path components for this data key to a PathBuf.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use icu_data_provider::prelude::*;
    ///
    /// let data_key = icu_data_key!(plurals: cardinal@1);
    /// let mut path_buf = PathBuf::new();
    /// data_key.append_path_to(&mut path_buf);
    ///
    /// let components: Vec<&str> = path_buf.iter().map(|c| c.to_str().unwrap()).collect();
    ///
    /// assert_eq!(["plurals", "cardinal@1"], &components[..]);
    /// ```
    pub fn append_path_to(&self, path_buf: &mut PathBuf) {
        let category_cow = self.category.as_str();
        let category_str: &str = category_cow.borrow();
        path_buf.push(category_str);
        path_buf.push(format!("{}@{}", self.sub_category.as_str(), self.version));
    }
}
