// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::rules::runtime::ast::Rule;
use icu_provider::DataMarker;
use icu_provider::{yoke, zerofrom};

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>
#[icu_provider::data_struct(
    CardinalV1Marker = "plurals/cardinal@1",
    OrdinalV1Marker = "plurals/ordinal@1"
)]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, crabbake::Bakeable),
    crabbake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(missing_docs)] // TODO(#1029) - Add missing docs.
pub struct PluralRulesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<Rule<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<Rule<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<Rule<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<Rule<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<Rule<'data>>,
}

pub(crate) struct ErasedPluralRulesV1Marker;

impl DataMarker for ErasedPluralRulesV1Marker {
    type Yokeable = PluralRulesV1<'static>;
}
