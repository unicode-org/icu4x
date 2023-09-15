// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider structs for the IANA time-zone database.

pub mod serde;
pub mod ule;

use crate::TimeZoneBcp47Id;
use icu_provider::{yoke, zerofrom};
use zerovec::vecs::{Index32, VarZeroVec};
use zerovec::{ZeroMap, ZeroMap2d, ZeroSlice};

/// A record of the local time that contains the GMT offset in seconds and a [`bool`]
/// that is true if this time is daylight savings time (DST) or standard time (STD).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, yoke::Yokeable)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
pub struct LocalTimeRecordV1 {
    /// The GMT offset in seconds.
    pub offset: i32,
    /// Whether this record is DST or STD.
    pub is_dst: bool,
}

/// A mapping of historic daylight savings time transitions for TZIDs.
/// Each [`TimeZoneBcp47Id`] maps to many [`LocalTimeRecordV1`] objects,
/// and each [`LocalTimeRecordV1`] maps to many [`i64`] values that represent
/// the timestamp in seconds when the historic transition occurred.
///
/// This multi-layerd structure is too large to use [`Index32`] in a single [`ZeroMap2d`],
/// so to get around having to use a larger 64-bit index, we split the map into two data members
/// that each use a 32-bit index. The final values of `historic_transitions_indices` are indices
/// corresponding to the location of the relevant slice of timestamps in `historic_transitions`.
#[icu_provider::data_struct(marker(
    TimeZoneHistoricTransitionsV1Marker,
    "tzif/historic_transitions@1",
    singleton
))]
#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(::serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneHistoricTransitionsV1<'data> {
    /// A map from TZID, to local time record, to an index that corresponds to a location in `historic_transitions`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub historic_transitions_indices: ZeroMap2d<'data, TimeZoneBcp47Id, LocalTimeRecordV1, usize>,
    /// A vector of slices of timestamps that each correspond to an index value in `historic_transitions_indices`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub historic_transitions: VarZeroVec<'data, ZeroSlice<i64>, Index32>,
}

/// A struct for defining a DST transition day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "datagen",
    derive(::serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub enum TransitionDayV1 {
    /// The day of the year, ignoring Feb. 29 on leap years.
    ///
    /// Ranges from [1, 365]
    NoLeap(u16),

    /// The day of the year, accounting for Feb. 29 on leap years.
    ///
    /// Ranges from [0, 365]
    WithLeap(u16),

    /// The month, week, day value.
    ///
    /// `M` ranges from [1, 12].
    ///
    /// `w` ranges from [1, 5].
    ///
    /// `d` ranges from [0, 6].
    Mwd(u8, u8, u8),
}

/// A time-zone variant transition date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
pub struct TransitionDateV1 {
    /// The day on which the transition occurrs.
    pub day_of_year: TransitionDayV1,
    /// The time of day in seconds in which the transition occurrs.
    /// Value may range from `-167:59:59` to `167:59:59` (h:m:s) in seconds.
    pub time_of_day: i32,
}

/// A rule for daylight savings time transitions that can be used to calculate
/// transitions for future datetimes, or used in the event that there are no
/// historic transitions on record for a past datetime.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(::serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[zerovec::make_ule(TimeZoneTransitionRuleULE)]
pub struct TimeZoneTransitionRuleV1 {
    /// The GMT offset for the STD variant in seconds.
    pub std_offset: i32,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    /// The GMT offset for the DST variant in seconds.
    pub dst_offset: Option<i32>,
    /// The transition date when DST starts.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dst_start: Option<TransitionDateV1>,
    /// The transition date when DST ends.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dst_end: Option<TransitionDateV1>,
}

/// A map from TZID to its transition rules.
/// See [`TimeZoneTransitionRuleV1`] for more info.
#[icu_provider::data_struct(marker(
    TimeZoneTransitionRulesV1Marker,
    "tzif/transition_rules@1",
    singleton
))]
#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "datagen",
    derive(::serde::Serialize, databake::Bake),
    databake(path = icu_timezone::provider::tzif),
)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct TimeZoneTransitionRulesV1<'data> {
    /// The map from TZID to its transition rules.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub transition_rules: ZeroMap<'data, TimeZoneBcp47Id, TimeZoneTransitionRuleV1>,
}
