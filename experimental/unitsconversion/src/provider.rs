// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//
// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::{yoke, zerofrom};
use zerovec::{VarZeroVec, ZeroMap};
use Default;

#[icu_provider::data_struct(marker(UnitsConstantsV1Marker, "units/constants@1", singleton))]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct UnitsConstantsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub constants_map: ZeroMap<'data, str, u16>,

    // TODO(#3882): Use a more efficient representation for the values with numerators and denominators.
    // Also, the constant types.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub constants_values: VarZeroVec<'data, str>,
}
