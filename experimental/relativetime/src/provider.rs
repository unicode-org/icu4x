// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom, DataError};
use zerovec::ZeroMap;

/// Relative time pattern V1 data struct.
#[icu_provider::data_struct(
    LongSecondRelativeTimeFormatDataV1Marker = "relativetime/long/second@1",
    ShortSecondRelativeTimeFormatDataV1Marker = "relativetime/short/second@1",
    NarrowSecondRelativeTimeFormatDataV1Marker = "relativetime/narrow/second@1",
    LongMinuteRelativeTimeFormatDataV1Marker = "relativetime/long/minute@1",
    ShortMinuteRelativeTimeFormatDataV1Marker = "relativetime/short/minute@1",
    NarrowMinuteRelativeTimeFormatDataV1Marker = "relativetime/narrow/minute@1",
    LongHourRelativeTimeFormatDataV1Marker = "relativetime/long/hour@1",
    ShortHourRelativeTimeFormatDataV1Marker = "relativetime/short/hour@1",
    NarrowHourRelativeTimeFormatDataV1Marker = "relativetime/narrow/hour@1",
    LongDayRelativeTimeFormatDataV1Marker = "relativetime/long/day@1",
    ShortDayRelativeTimeFormatDataV1Marker = "relativetime/short/day@1",
    NarrowDayRelativeTimeFormatDataV1Marker = "relativetime/narrow/day@1",
    LongWeekRelativeTimeFormatDataV1Marker = "relativetime/long/week@1",
    ShortWeekRelativeTimeFormatDataV1Marker = "relativetime/short/week@1",
    NarrowWeekRelativeTimeFormatDataV1Marker = "relativetime/narrow/week@1",
    LongMonthRelativeTimeFormatDataV1Marker = "relativetime/long/month@1",
    ShortMonthRelativeTimeFormatDataV1Marker = "relativetime/short/month@1",
    NarrowMonthRelativeTimeFormatDataV1Marker = "relativetime/narrow/month@1",
    LongQuarterRelativeTimeFormatDataV1Marker = "relativetime/long/quarter@1",
    ShortQuarterRelativeTimeFormatDataV1Marker = "relativetime/short/quarter@1",
    NarrowQuarterRelativeTimeFormatDataV1Marker = "relativetime/narrow/quarter@1",
    LongYearRelativeTimeFormatDataV1Marker = "relativetime/long/year@1",
    ShortYearRelativeTimeFormatDataV1Marker = "relativetime/short/year@1",
    NarrowYearRelativeTimeFormatDataV1Marker = "relativetime/narrow/year@1"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePatternDataV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The display name of the pattern
    pub display_name: Cow<'data, str>,
    /// Mapping for relative time fields.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub relatives: ZeroMap<'data, i8, str>,
    /// Plural rules mapping for past.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past: PluralRulesCategoryMapping<'data>,
    /// Plural rules mapping for future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future: PluralRulesCategoryMapping<'data>,
}

/// Plural rules category mapping.
#[derive(Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct PluralRulesCategoryMapping<'data> {
    /// Mapping for PluralCategory::Zero or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<SingularSubPattern<'data>>,
    /// Mapping for PluralCategory::One or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<SingularSubPattern<'data>>,
    /// Mapping for PluralCategory::Two or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<SingularSubPattern<'data>>,
    /// Mapping for PluralCategory::Few or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<SingularSubPattern<'data>>,
    /// Mapping for PluralCategory::Many or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<SingularSubPattern<'data>>,
    /// Mapping for PluralCategory::Other or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub other: Option<SingularSubPattern<'data>>,
}

/// Singular substitution pattern string.
#[derive(Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_relativetime::provider)
)]
pub struct SingularSubPattern<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The underlying pattern
    pub pattern: Cow<'data, str>,
    /// Optional index which is absent if the pattern does not have substitution
    pub index: Option<u8>,
}

impl<'data> SingularSubPattern<'data> {
    /// Construct a singular sub pattern from a pattern
    pub fn try_from_str(value: &str) -> Result<Self, DataError> {
        let index = value.find("{0}").map(|index| index as u8);
        Ok(Self {
            pattern: Cow::Owned(value.to_string()),
            index,
        })
    }
}
