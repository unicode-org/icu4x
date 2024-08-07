// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::subtags::{Language, Region, Script};
use icu_provider::prelude::*;
use zerovec::ule::UnvalidatedStr;
use zerovec::ZeroMap;

/// Locale fallback rules derived from CLDR parent locales data.
#[icu_provider::data_struct(marker(
    LocaleFallbackParentsV1Marker,
    "fallback/parents@1",
    singleton
))]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_locale::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LocaleFallbackParentsV1<'data> {
    /// Map from language identifier to language identifier, indicating that the language on the
    /// left should inherit from the language on the right.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parents: ZeroMap<'data, UnvalidatedStr, (Language, Option<Script>, Option<Region>)>,
}
