use crate::structs;
use std::any::TypeId;
use std::fmt;
use tinystr::TinyStr16;

/// A top-level collection of related data keys.
#[non_exhaustive]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Category {
    Decimal,
    Plurals,
    PrivateUse(TinyStr16),
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Decimal => f.write_str("decimal")?,
            Category::Plurals => f.write_str("plurals")?,
            Category::PrivateUse(id) => {
                f.write_str("x-")?;
                f.write_str(id)?;
            }
        }
        Ok(())
    }
}

/// A category, subcategory, and version, used for requesting data from a DataProvider.
///
/// All of the fields in a DataKey should be resolved at build time.
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
        // TODO: Evaluate the code size of this implementation
        write!(
            f,
            "{}/{}/v{}",
            self.category, self.sub_category, self.version
        )
    }
}

impl DataKey {
    pub fn get_type_id(&self) -> Option<TypeId> {
        match self.category {
            Category::Decimal => structs::decimal::get_type_id(self),
            Category::Plurals => structs::plurals::get_type_id(self),
            Category::PrivateUse(_) => None,
        }
    }
}
