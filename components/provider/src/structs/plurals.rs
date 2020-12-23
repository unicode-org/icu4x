// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// Plural types
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod key {
    use crate::resource::ResourceKey;
    pub const CARDINAL_V1: ResourceKey = resource_key!(plurals, "cardinal", 1);
    pub const ORDINAL_V1: ResourceKey = resource_key!(plurals, "ordinal", 1);
}

/// Gets a locale-invariant default struct given a resource key in this module's category.
#[cfg(feature = "invariant")]
pub fn get_invariant<'d>(
    resc_key: &ResourceKey,
    receiver: &mut dyn DataReceiver<'d, 'static>,
) -> Result<(), DataError> {
    match *resc_key {
        key::CARDINAL_V1 => receiver.receive_invariant::<PluralRuleStringsV1>(),
        key::ORDINAL_V1 => receiver.receive_invariant::<PluralRuleStringsV1>(),
        _ => Err(DataError::UnsupportedResourceKey(*resc_key)),
    }
}

/// Gets a boxed DataReceiver capable of receiving a resource key in this module's category.
pub fn get_receiver<'d>(resc_key: &ResourceKey) -> Option<Box<dyn DataReceiver<'d, 'static> + 'd>> {
    match *resc_key {
        key::CARDINAL_V1 => Some(DataReceiverForType::<PluralRuleStringsV1>::new_boxed()),
        key::ORDINAL_V1 => Some(DataReceiverForType::<PluralRuleStringsV1>::new_boxed()),
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
    #[cfg_attr(
        not(feature = "serialize_none"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub zero: Option<Cow<'static, str>>,
    #[cfg_attr(
        not(feature = "serialize_none"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub one: Option<Cow<'static, str>>,
    #[cfg_attr(
        not(feature = "serialize_none"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub two: Option<Cow<'static, str>>,
    #[cfg_attr(
        not(feature = "serialize_none"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub few: Option<Cow<'static, str>>,
    #[cfg_attr(
        not(feature = "serialize_none"),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub many: Option<Cow<'static, str>>,
}
