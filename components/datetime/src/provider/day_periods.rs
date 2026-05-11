// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for day periods rules.

use icu_provider::prelude::*;
use zerovec::ZeroVec;

/// Rule for Design 1.
/// Bitset: which period (3 bits = 8 periods), rule start (5 bits = 24 hours + leftovers), rule end (5 bits)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[zerovec::make_ule(DayPeriodRule1ULE)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DayPeriodRule1(pub u16);

/// Day period rule design 1.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRulesV1Design1<'data> {
    /// The entries.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub entries: ZeroVec<'data, DayPeriodRule1>,
}

icu_provider::data_struct!(
    DayPeriodRulesV1Design1<'_>,
    #[cfg(feature = "datagen")]
);

/// Rule for Design 2.
/// Bitset: rule start (5 bits = 24 hours + leftovers), rule delta (3 bits = 16hrs)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[zerovec::make_ule(DayPeriodRule2ULE)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DayPeriodRule2(pub u8);

/// Day period rule design 2.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRulesV1Design2<'data> {
    /// Bitmask: which rules are present.
    pub presence: u8,
    /// The entries.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub entries: ZeroVec<'data, DayPeriodRule2>,
}

icu_provider::data_struct!(
    DayPeriodRulesV1Design2<'_>,
    #[cfg(feature = "datagen")]
);

/// Day period rule design 3.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRulesV1Design3<'data> {
    /// Bitmask: which rules are present.
    pub presence: u8,
    /// The start time of the first rule.
    pub start_time: u8,
    /// The delta of each rule.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub diffs: ZeroVec<'data, u8>,
}

icu_provider::data_struct!(
    DayPeriodRulesV1Design3<'_>,
    #[cfg(feature = "datagen")]
);

// Define markers

icu_provider::data_marker!(
    /// `DayPeriodRulesDesign1V1` marker
    DayPeriodRulesDesign1V1,
    DayPeriodRulesV1Design1<'static>,
);

icu_provider::data_marker!(
    /// `DayPeriodRulesDesign2V1` marker
    DayPeriodRulesDesign2V1,
    DayPeriodRulesV1Design2<'static>,
);

icu_provider::data_marker!(
    /// `DayPeriodRulesDesign3V1` marker
    DayPeriodRulesDesign3V1,
    DayPeriodRulesV1Design3<'static>,
);

/// Day period rule design 4.
#[derive(Debug, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRulesV1Design4 {
    /// The bitmap, 24 hours * 3 bits = 72 bits.
    /// We use 9 bytes = 72 bits.
    pub bitmap: [u8; 9],
}

icu_provider::data_struct!(
    DayPeriodRulesV1Design4,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DayPeriodRulesDesign4V1` marker
    DayPeriodRulesDesign4V1,
    DayPeriodRulesV1Design4,
);

/// Day period rule design 5.
#[derive(Debug, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRulesV1Design5(pub u32);

icu_provider::data_struct!(
    DayPeriodRulesV1Design5,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DayPeriodRulesDesign5V1` marker
    DayPeriodRulesDesign5V1,
    DayPeriodRulesV1Design5,
);
