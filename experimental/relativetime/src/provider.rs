// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::borrow::Cow;
use icu_provider::{yoke, zerofrom};
use zerovec::ZeroMap;

/// Relative time patterns data struct.
#[icu_provider::data_struct(
    StandardRelativeTimeV1Marker = "relativetime/standard@1",
    NarrowRelativeTimeV1Marker = "relativetime/narrow@1",
    ShortRelativeTimeV1Marker = "relativetime/short@1"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePatternsV1<'data>(
    #[cfg_attr(feature = "serde", serde(borrow))]
    // Contains patterns in order second, minute, hour, day, week, month, quarter, year
    pub [RelativeTimePattern<'data>; 8],
);

impl<'data> RelativeTimePatternsV1<'data> {
    /// Create a new RelativeTimePatternsV1
    pub fn new(patterns: [RelativeTimePattern<'data>; 8]) -> Self {
        Self(patterns)
    }
}

/// A single relative time pattern.
#[derive(Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_relativetime::provider)
)]
#[yoke(prove_covariance_manually)]
pub struct RelativeTimePattern<'data> {
    /// The display name of the pattern
    pub display_name: Option<Cow<'data, str>>,
    /// Mapping for relative time fields.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub relatives: ZeroMap<'data, i8, str>,
    /*
    // Plural rules mapping for past.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_zero: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_one: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_two: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_few: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_many: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub past_other: Option<SingularSubPattern<'data>>,
    // Plural rules mapping for future.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_zero: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_one: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_two: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_few: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_many: Option<SingularSubPattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub future_other: Option<SingularSubPattern<'data>>,
    */
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
    pattern: Cow<'data, str>,
    index: u8,
}
