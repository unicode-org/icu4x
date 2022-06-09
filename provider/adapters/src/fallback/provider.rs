// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::subtags::{Language, Region, Script};

use icu_provider::prelude::*;

use zerovec::ZeroMap;
use zerovec::ZeroMap2d;

// We use raw TinyAsciiStrs for map keys, as we then don't have to
// validate them as subtags on deserialization. Map lookup can be
// done even if they are not valid tags (an invalid key will just
// become inaccessible).
type UnvalidatedLanguage = TinyAsciiStr<3>;
type UnvalidatedScript = TinyAsciiStr<4>;
type UnvalidatedRegion = TinyAsciiStr<3>;

/// Fallback rules for a particular language.
#[icu_provider::data_struct(LocaleFallbackRulesV1Marker = "fallback/locale@1")]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, crabbake::Bakeable),
    crabbake(path = icu_provider_adapters::fallback::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct LocaleFallbackRulesV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub l2s: ZeroMap<'data, UnvalidatedLanguage, Script>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub lr2s: ZeroMap2d<'data, UnvalidatedLanguage, UnvalidatedRegion, Script>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub l2r: ZeroMap<'data, UnvalidatedLanguage, Region>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub ls2r: ZeroMap2d<'data, UnvalidatedLanguage, UnvalidatedScript, Region>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parents: ZeroMap<'data, [u8], (Language, Option<Script>, Option<Region>)>,
}
