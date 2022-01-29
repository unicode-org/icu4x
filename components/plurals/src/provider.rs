// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::rules::runtime::ast::Rule;
use icu_provider::prelude::*;
use icu_provider::yoke::{self, *};

#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub mod key {
    use icu_provider::{resource_key, ResourceKey};
    pub const CARDINAL_V1: ResourceKey = resource_key!("plurals/cardinal@1");
    pub const ORDINAL_V1: ResourceKey = resource_key!("plurals/ordinal@1");
}

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>
#[icu_provider::data_struct(
    CardinalV1Marker = "plurals/cardinal@1",
    OrdinalV1Marker = "plurals/ordinal@1",
)]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct PluralRulesV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub zero: Option<Rule<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub one: Option<Rule<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub two: Option<Rule<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub few: Option<Rule<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub many: Option<Rule<'data>>,
}
