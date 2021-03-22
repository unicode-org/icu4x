// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const CARDINAL_V1: ResourceKey = resource_key!(plurals, "cardinal", 1);
    pub const ORDINAL_V1: ResourceKey = resource_key!(plurals, "ordinal", 1);
}

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PluralRuleStringsV1<'s> {
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub zero: Option<Cow<'s, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub one: Option<Cow<'s, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub two: Option<Cow<'s, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub few: Option<Cow<'s, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub many: Option<Cow<'s, str>>,
}
