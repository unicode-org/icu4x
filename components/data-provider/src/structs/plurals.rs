// Plural types

use crate::data_key::Category;
use crate::data_key::DataKey;
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::borrow::Cow;

pub(crate) fn get_type_id(data_key: &DataKey) -> Option<TypeId> {
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

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PluralRuleStringsV1 {
    pub zero: Option<Cow<'static, str>>,
    pub one: Option<Cow<'static, str>>,
    pub two: Option<Cow<'static, str>>,
    pub few: Option<Cow<'static, str>>,
    pub many: Option<Cow<'static, str>>,
}
