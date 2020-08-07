// Decimal types
use serde::{Deserialize, Serialize};
use smallstr::SmallString;

#[cfg(feature = "invariant")]
use crate::prelude::*;

// TODO: Automatically delegate from subcategory to type object

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != DataCategory::Decimal {
        return None;
    }
    match data_key.sub_category.as_str() {
        "symbols" => match data_key.version {
            1 => Some(make_inv_response(SymbolsV1::default())),
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

#[cfg(feature = "invariant")]
impl Default for SymbolsV1 {
    fn default() -> Self {
        Self {
            zero_digit: '0',
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
        }
    }
}
