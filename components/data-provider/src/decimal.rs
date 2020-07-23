// Decimal types

use std::prelude::v1::*;

use serde::{Deserialize, Serialize};
use smallstr::SmallString;
use std::any::TypeId;

// TODO: Automatically delegate from subcategory to type object

pub(crate) fn get_type_id(data_key: &crate::DataKey) -> Option<TypeId> {
    if data_key.category != crate::Category::Decimal {
        return None;
    }
    match data_key.sub_category.as_str() {
        "symbols" => match data_key.version {
            1 => Some(TypeId::of::<SymbolsV1>()),
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct SymbolsV1 {
    pub zero_digit: char,
    pub decimal_separator: SmallString<[u8; 8]>,
    pub grouping_separator: SmallString<[u8; 8]>,
}
