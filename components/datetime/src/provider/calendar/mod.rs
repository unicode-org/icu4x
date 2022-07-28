// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod skeletons;
mod symbols;

use crate::pattern;
use icu_provider::prelude::*;
use icu_provider::{yoke, zerofrom};
pub use skeletons::*;
pub use symbols::*;

/// Pattern data for dates.
#[icu_provider::data_struct(marker(
    DatePatternsV1Marker,
    "datetime/datelengths@1",
    extension_key = "ca"
))]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::calendar),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DatePatternsV1<'data> {
    /// Date pattern data, broken down by pattern length.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub date: patterns::LengthPatternsV1<'data>,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub length_combinations: patterns::GenericLengthPatternsV1<'data>,
}

/// Pattern data for times.
#[icu_provider::data_struct(marker(
    TimePatternsV1Marker,
    "datetime/timelengths@1",
    extension_key = "ca"
))]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::calendar),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimePatternsV1<'data> {
    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub time_h11_h12: patterns::LengthPatternsV1<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub time_h23_h24: patterns::LengthPatternsV1<'data>,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: pattern::CoarseHourCycle,
}

/// Data structs for date / time patterns that store data corresponding to pattern lengths
/// and/or plural forms.
pub mod patterns {
    use super::*;
    use crate::pattern::runtime::{self, GenericPattern, PatternPlurals};
    use icu_provider::{yoke, zerofrom};

    /// Data struct for date/time patterns broken down by pattern length.
    #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(
        feature = "datagen",
        derive(serde::Serialize, databake::Bake),
        databake(path = icu_datetime::provider::calendar::patterns),
    )]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct LengthPatternsV1<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub full: runtime::Pattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub long: runtime::Pattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub medium: runtime::Pattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub short: runtime::Pattern<'data>,
    }

    /// Data struct for date/time patterns based on plural category, broken down by pattern length.
    #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(
        feature = "datagen",
        derive(serde::Serialize, databake::Bake),
        databake(path = icu_datetime::provider::calendar::patterns),
    )]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct LengthPatternPluralsV1<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub full: PatternPlurals<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub long: PatternPlurals<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub medium: PatternPlurals<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub short: PatternPlurals<'data>,
    }

    /// Data struct for generic date/time patterns, broken down by pattern length.
    #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
    #[cfg_attr(
        feature = "datagen",
        derive(serde::Serialize, databake::Bake),
        databake(path = icu_datetime::provider::calendar::patterns),
    )]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct GenericLengthPatternsV1<'data> {
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub full: GenericPattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub long: GenericPattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub medium: GenericPattern<'data>,
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub short: GenericPattern<'data>,
    }

    /// 
    #[icu_provider::data_struct]
    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct PatternPluralsV1<'data>(
        #[cfg_attr(feature = "serde", serde(borrow))] pub PatternPlurals<'data>,
    );

    impl<'data> From<PatternPlurals<'data>> for PatternPluralsV1<'data> {
        fn from(pattern: PatternPlurals<'data>) -> Self {
            Self(pattern)
        }
    }

    /// Helper struct used to allow for projection of `DataPayload<DatePatternsV1>` to
    /// `DataPayload<PatternPluralsV1>`.
    pub(crate) struct PatternPluralsFromPatternsV1Marker;

    impl DataMarker for PatternPluralsFromPatternsV1Marker {
        type Yokeable = PatternPluralsV1<'static>;
    }

    #[icu_provider::data_struct]
    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize))]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct GenericPatternV1<'data>(
        #[cfg_attr(feature = "serde", serde(borrow))] pub GenericPattern<'data>,
    );

    /// Helper struct used to allow for projection of `DataPayload<DatePatternsV1>` to
    /// `DataPayload<GenericLengthPatternsV1>`.
    pub(crate) struct GenericPatternV1Marker;

    impl DataMarker for GenericPatternV1Marker {
        type Yokeable = GenericPatternV1<'static>;
    }

    impl<'data> From<GenericPattern<'data>> for GenericPatternV1<'data> {
        fn from(pattern: GenericPattern<'data>) -> Self {
            Self(pattern)
        }
    }
}
