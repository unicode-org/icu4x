// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::relativetime::provider::PluralPatterns;
use icu_pattern::SinglePlaceholder;
use icu_provider::prelude::*;

#[icu_provider::data_struct(marker(
    UnitsDisplayNameV1Marker,
    "units/displaynames@1",
    attributes_domain = "units"
))]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::units))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsDisplayNameV1<'data> {
    // TODO: use `MeasureUnit` for the units key instead of strings.
    /// Contains the long width patterns for the units.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: PluralPatterns<'data, SinglePlaceholder>,
}
