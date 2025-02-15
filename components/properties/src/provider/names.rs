// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Property names-related data for this component
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use icu_locale_core::subtags::Script;
use icu_provider::prelude::*;

use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ule::NichedOption;
use zerovec::{VarZeroVec, ZeroMap, ZeroVec};

/// A set of characters and strings which share a particular property value.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(
    marker(BidiClassNameToValueV2, "propnames/from/bc@2", singleton),
    marker(
        CanonicalCombiningClassNameToValueV2,
        "propnames/from/ccc@2",
        singleton
    ),
    marker(EastAsianWidthNameToValueV2, "propnames/from/ea@2", singleton),
    marker(GeneralCategoryMaskNameToValueV2, "propnames/from/gcm@2", singleton),
    marker(GeneralCategoryNameToValueV2, "propnames/from/gc@2", singleton),
    marker(GraphemeClusterBreakNameToValueV2, "propnames/from/GCB@2", singleton),
    marker(HangulSyllableTypeNameToValueV2, "propnames/from/hst@2", singleton),
    marker(IndicSyllabicCategoryNameToValueV2, "propnames/from/InSC@2", singleton),
    marker(JoiningTypeNameToValueV2, "propnames/from/jt@2", singleton),
    marker(LineBreakNameToValueV2, "propnames/from/lb@2", singleton),
    marker(ScriptNameToValueV2, "propnames/from/sc@2", singleton),
    marker(SentenceBreakNameToValueV2, "propnames/from/SB@2", singleton),
    marker(WordBreakNameToValueV2, "propnames/from/WB@2", singleton)
)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PropertyValueNameToEnumMap<'data> {
    /// A map from names to their value discriminant
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
}

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(
    marker(
        CanonicalCombiningClassValueToLongNameV1,
        "propnames/to/long/sparse/ccc@1",
        singleton
    ),
    marker(
        CanonicalCombiningClassValueToShortNameV1,
        "propnames/to/short/sparse/ccc@1",
        singleton
    )
)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyEnumToValueNameSparseMap<'data> {
    /// A map from the value discriminant to the names
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap<'data, u16, str>,
}

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(
    marker(BidiClassValueToLongNameV1, "propnames/to/long/linear/bc@1", singleton),
    marker(
        BidiClassValueToShortNameV1,
        "propnames/to/short/linear/bc@1",
        singleton
    ),
    marker(
        EastAsianWidthValueToLongNameV1,
        "propnames/to/long/linear/ea@1",
        singleton
    ),
    marker(
        EastAsianWidthValueToShortNameV1,
        "propnames/to/short/linear/ea@1",
        singleton
    ),
    marker(
        GeneralCategoryValueToLongNameV1,
        "propnames/to/long/linear/gc@1",
        singleton
    ),
    marker(
        GeneralCategoryValueToShortNameV1,
        "propnames/to/short/linear/gc@1",
        singleton
    ),
    marker(
        GraphemeClusterBreakValueToLongNameV1,
        "propnames/to/long/linear/GCB@1",
        singleton
    ),
    marker(
        GraphemeClusterBreakValueToShortNameV1,
        "propnames/to/short/linear/GCB@1",
        singleton
    ),
    marker(
        HangulSyllableTypeValueToLongNameV1,
        "propnames/to/long/linear/hst@1",
        singleton
    ),
    marker(
        HangulSyllableTypeValueToShortNameV1,
        "propnames/to/short/linear/hst@1",
        singleton
    ),
    marker(
        IndicSyllabicCategoryValueToLongNameV1,
        "propnames/to/long/linear/InSC@1",
        singleton
    ),
    marker(
        IndicSyllabicCategoryValueToShortNameV1,
        "propnames/to/short/linear/InSC@1",
        singleton
    ),
    marker(
        JoiningTypeValueToLongNameV1,
        "propnames/to/long/linear/jt@1",
        singleton
    ),
    marker(
        JoiningTypeValueToShortNameV1,
        "propnames/to/short/linear/jt@1",
        singleton
    ),
    marker(LineBreakValueToLongNameV1, "propnames/to/long/linear/lb@1", singleton),
    marker(
        LineBreakValueToShortNameV1,
        "propnames/to/short/linear/lb@1",
        singleton
    ),
    marker(ScriptValueToLongNameV1, "propnames/to/long/linear/sc@1", singleton),
    marker(
        SentenceBreakValueToLongNameV1,
        "propnames/to/long/linear/SB@1",
        singleton
    ),
    marker(
        SentenceBreakValueToShortNameV1,
        "propnames/to/short/linear/SB@1",
        singleton
    ),
    marker(WordBreakValueToLongNameV1, "propnames/to/long/linear/WB@1", singleton),
    marker(
        WordBreakValueToShortNameV1,
        "propnames/to/short/linear/WB@1",
        singleton
    )
)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyEnumToValueNameLinearMap<'data> {
    /// A map from the value discriminant (the index) to the names, for mostly
    /// contiguous data. Empty strings count as missing.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: VarZeroVec<'data, str>,
}

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq)]
#[icu_provider::data_struct(marker(
    ScriptValueToShortNameV1,
    "propnames/to/short/linear4/sc@1",
    singleton
))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyScriptToIcuScriptMap<'data> {
    /// A map from the value discriminant (the index) to the names, for mostly
    /// contiguous data. Empty strings count as missing.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroVec<'data, NichedOption<Script, 4>>,
}
