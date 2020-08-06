// Plural types

use crate::data_key::Category;
use crate::data_key::DataKey;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::borrow::Cow;

/// Gets the expected TypeID given a data key in this module's category.
pub fn get_type_id(data_key: &DataKey) -> Option<TypeId> {
    if data_key.category != Category::Plurals {
        return None;
    }
    match data_key.sub_category.as_str() {
        "cardinal" => match data_key.version {
            1 => Some(TypeId::of::<PluralRuleStringsV1>()),
            _ => None,
        },
        "ordinal" => match data_key.version {
            1 => Some(TypeId::of::<PluralRuleStringsV1>()),
            _ => None,
        },
        _ => None,
    }
}

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub fn get_invariant(
    data_key: &crate::data_key::DataKey,
) -> Option<crate::data_provider::Response<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != Category::Plurals {
        return None;
    }
    match data_key.sub_category.as_str() {
        "cardinal" => match data_key.version {
            1 => Some(make_inv_response(PluralRuleStringsV1::default())),
            _ => None,
        },
        "ordinal" => match data_key.version {
            1 => Some(make_inv_response(PluralRuleStringsV1::default())),
            _ => None,
        },
        _ => None,
    }
}

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PluralRuleStringsV1 {
    pub zero: Option<Cow<'static, str>>,
    pub one: Option<Cow<'static, str>>,
    pub two: Option<Cow<'static, str>>,
    pub few: Option<Cow<'static, str>>,
    pub many: Option<Cow<'static, str>>,
}

#[cfg(feature = "invariant")]
impl Default for PluralRuleStringsV1 {
    fn default() -> Self {
        Self {
            zero: None,
            one: None,
            two: None,
            few: None,
            many: None,
        }
    }
}
