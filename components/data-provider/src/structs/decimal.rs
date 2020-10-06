// Decimal types
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use smallstr::SmallString;

pub const KNOWN_KEYS: [DataKey; 1] = [
    icu_data_key!(decimal: symbols@1)
];

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != DataCategory::Decimal {
        return None;
    }
    // TODO(#212): Match on TinyStr instead of &str
    match (data_key.sub_category.as_str(), data_key.version) {
        ("symbols", 1) => make_inv_response::<SymbolsV1>(),
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
