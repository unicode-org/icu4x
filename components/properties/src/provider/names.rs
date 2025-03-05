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

icu_provider::data_marker!(
    /// `PropertyNameParseBidiClassV2`
    PropertyNameParseBidiClassV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseCanonicalCombiningClassV2`
    PropertyNameParseCanonicalCombiningClassV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseEastAsianWidthV2`
    PropertyNameParseEastAsianWidthV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseGeneralCategoryMaskV2`
    PropertyNameParseGeneralCategoryMaskV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseGeneralCategoryV2`
    PropertyNameParseGeneralCategoryV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseGraphemeClusterBreakV2`
    PropertyNameParseGraphemeClusterBreakV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseHangulSyllableTypeV2`
    PropertyNameParseHangulSyllableTypeV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseIndicSyllabicCategoryV2`
    PropertyNameParseIndicSyllabicCategoryV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseJoiningTypeV2`
    PropertyNameParseJoiningTypeV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseLineBreakV2`
    PropertyNameParseLineBreakV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseScriptV2`
    PropertyNameParseScriptV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseSentenceBreakV2`
    PropertyNameParseSentenceBreakV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameParseWordBreakV2`
    PropertyNameParseWordBreakV2,
    PropertyValueNameToEnumMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongBidiClassV1`
    PropertyNameLongBidiClassV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortBidiClassV1`
    PropertyNameShortBidiClassV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongEastAsianWidthV1`
    PropertyNameLongEastAsianWidthV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortEastAsianWidthV1`
    PropertyNameShortEastAsianWidthV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongGeneralCategoryV1`
    PropertyNameLongGeneralCategoryV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortGeneralCategoryV1`
    PropertyNameShortGeneralCategoryV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongGraphemeClusterBreakV1`
    PropertyNameLongGraphemeClusterBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortGraphemeClusterBreakV1`
    PropertyNameShortGraphemeClusterBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongHangulSyllableTypeV1`
    PropertyNameLongHangulSyllableTypeV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortHangulSyllableTypeV1`
    PropertyNameShortHangulSyllableTypeV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongIndicSyllabicCategoryV1`
    PropertyNameLongIndicSyllabicCategoryV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortIndicSyllabicCategoryV1`
    PropertyNameShortIndicSyllabicCategoryV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongJoiningTypeV1`
    PropertyNameLongJoiningTypeV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortJoiningTypeV1`
    PropertyNameShortJoiningTypeV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongLineBreakV1`
    PropertyNameLongLineBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortLineBreakV1`
    PropertyNameShortLineBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongScriptV1`
    PropertyNameLongScriptV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongSentenceBreakV1`
    PropertyNameLongSentenceBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortSentenceBreakV1`
    PropertyNameShortSentenceBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongWordBreakV1`
    PropertyNameLongWordBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameShortWordBreakV1`
    PropertyNameShortWordBreakV1,
    PropertyEnumToValueNameLinearMap<'static>,
    is_singleton = true
);
icu_provider::data_marker!(
    /// `PropertyNameLongCanonicalCombiningClassV1`
    PropertyNameLongCanonicalCombiningClassV1,
    PropertyEnumToValueNameSparseMap<'static>,
    is_singleton = true,
);
icu_provider::data_marker!(
    /// `PropertyNameShortCanonicalCombiningClassV1`
    PropertyNameShortCanonicalCombiningClassV1,
    PropertyEnumToValueNameSparseMap<'static>,
    is_singleton = true,
);
icu_provider::data_marker!(
    /// `PropertyNameShortScriptV1`
    PropertyNameShortScriptV1,
    PropertyScriptToIcuScriptMap<'static>,
    is_singleton = true,
);

/// A set of characters and strings which share a particular property value.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PropertyValueNameToEnumMap<'data> {
    /// A map from names to their value discriminant
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
}

icu_provider::data_struct!(
    PropertyValueNameToEnumMap<'_>,
    #[cfg(feature = "datagen")]
);

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyEnumToValueNameSparseMap<'data> {
    /// A map from the value discriminant to the names
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap<'data, u16, str>,
}

icu_provider::data_struct!(
    PropertyEnumToValueNameSparseMap<'_>,
    #[cfg(feature = "datagen")]
);

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
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

icu_provider::data_struct!(
    PropertyEnumToValueNameLinearMap<'_>,
    #[cfg(feature = "datagen")]
);

/// A mapping of property values to their names. A single instance of this map will only cover
/// either long or short names, determined whilst loading data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
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

icu_provider::data_struct!(
    PropertyScriptToIcuScriptMap<'_>,
    #[cfg(feature = "datagen")]
);
