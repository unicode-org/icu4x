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
    pub zero: Option<Cow<'s, str>>,
    pub one: Option<Cow<'s, str>>,
    pub two: Option<Cow<'s, str>>,
    pub few: Option<Cow<'s, str>>,
    pub many: Option<Cow<'s, str>>,
}

mod convert {
    use crate::{PluralRuleType, PluralRulesError};
    use icu_locid::LanguageIdentifier;
    use icu_provider::prelude::*;
    use std::borrow::Cow;

    impl<'s> super::PluralRuleStringsV1<'s> {
        pub fn new_from_provider<D: DataProvider<'s, super::PluralRuleStringsV1<'s>> + ?Sized>(
            langid: LanguageIdentifier,
            data_provider: &D,
            type_: PluralRuleType,
        ) -> Result<Cow<'s, Self>, PluralRulesError> {
            let key = match type_ {
                PluralRuleType::Cardinal => super::key::CARDINAL_V1,
                PluralRuleType::Ordinal => super::key::ORDINAL_V1,
            };
            Ok(data_provider
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(langid),
                        },
                    },
                })?
                .payload
                .take()?)
        }
    }
}
