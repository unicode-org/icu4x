// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::yoke::ZeroCopyFrom;
use std::borrow::Cow;

pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const CARDINAL_V1: ResourceKey = resource_key!(plurals, "cardinal", 1);
    pub const ORDINAL_V1: ResourceKey = resource_key!(plurals, "ordinal", 1);
}

pub mod resolver;

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
    pub zero: Option<Cow<'s, str>>,
    pub one: Option<Cow<'s, str>>,
    pub two: Option<Cow<'s, str>>,
    pub few: Option<Cow<'s, str>>,
    pub many: Option<Cow<'s, str>>,
}

icu_provider::unsafe_impl_data_marker_with_lifetime!(
    PluralRuleStringsV1<'s>,
    /// Marker type for [`PluralRuleStringsV1`]
    PluralRuleStringsV1Marker
);

impl<'s> ZeroCopyFrom<PluralRuleStringsV1<'s>> for PluralRuleStringsV1<'static> {
    fn zero_copy_from<'b>(this: &'b PluralRuleStringsV1<'s>) -> PluralRuleStringsV1<'b> {
        PluralRuleStringsV1 {
            zero: this.zero.as_ref().map(|cow| Cow::Borrowed(cow.as_ref())),
            one: this.one.as_ref().map(|cow| Cow::Borrowed(cow.as_ref())),
            two: this.two.as_ref().map(|cow| Cow::Borrowed(cow.as_ref())),
            few: this.few.as_ref().map(|cow| Cow::Borrowed(cow.as_ref())),
            many: this.many.as_ref().map(|cow| Cow::Borrowed(cow.as_ref())),
        }
    }
}
