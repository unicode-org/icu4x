// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// Plural types
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[cfg(feature = "invariant")]
use crate::prelude::*;

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    if data_key.category != DataCategory::Plurals {
        return None;
    }
    // TODO(#212): Match on TinyStr instead of &str
    match (data_key.sub_category.as_str(), data_key.version) {
        ("cardinal", 1) => make_inv_response::<PluralRuleStringsV1>(),
        ("ordinal", 1) => make_inv_response::<PluralRuleStringsV1>(),
        _ => None,
    }
}

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "invariant", derive(Default))]
pub struct PluralRuleStringsV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub few: Option<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub many: Option<Cow<'static, str>>,
}
