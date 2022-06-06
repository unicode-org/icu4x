// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::subtags::{Language, Region, Script};

use icu_provider::prelude::*;

use zerovec::ZeroMap;
use zerovec::ZeroMap2d;

#[icu_provider::data_struct(LocaleFallbackDataV1Marker = "fallback/locale@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, crabbake::Bakeable),
    crabbake(path = icu_provider_adapters::fallback::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LocaleFallbackDataV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub l2s: ZeroMap<'data, [u8; 3], Script>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub lr2s: ZeroMap2d<'data, [u8; 3], [u8; 3], Script>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub l2r: ZeroMap<'data, [u8; 3], Region>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ls2r: ZeroMap2d<'data, [u8; 3], [u8; 4], Region>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parents: ZeroMap<'data, [u8], (Language, Option<Script>, Option<Region>)>,
}
