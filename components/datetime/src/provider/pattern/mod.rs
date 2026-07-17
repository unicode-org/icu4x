// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Structured datetime pattern types for datagen and the data provider.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>

mod common;
mod error;
mod hour_cycle;
mod item;
pub mod reference;
pub mod runtime;

use crate::provider::fields;
pub use error::PatternError;
pub use hour_cycle::CoarseHourCycle;
#[cfg(feature = "datagen")]
pub(crate) use hour_cycle::naively_apply_hour_cycle;
use icu_provider::prelude::*;
pub use item::{GenericPatternItem, PatternItem};

/// The granularity of time represented in a [`Pattern`](runtime::Pattern).
/// Ordered from least granular to most granular for comparison.
#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, yoke::Yokeable, zerofrom::ZeroFrom,
)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::pattern))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum TimeGranularity {
    /// No time is in the pattern, or hours are 23/24-hour style.
    #[default]
    Hours23OrNone = 0,
    /// Smallest time unit = 12-hour style hours.
    Hours12 = 1,
    /// Smallest time unit = minutes.
    Minutes = 2,
    /// Smallest time unit = seconds.
    Seconds = 3,
    /// Smallest time unit = nanoseconds.
    Nanoseconds = 4,
}

impl TimeGranularity {
    /// Deprecated alias for [`TimeGranularity::Hours12`].
    #[deprecated(note = "use Hours12 or Hours23OrNone")]
    #[allow(non_upper_case_globals)]
    pub const Hours: TimeGranularity = TimeGranularity::Hours12;
    /// Deprecated alias for [`TimeGranularity::Hours23OrNone`].
    #[deprecated(note = "use Hours23OrNone")]
    #[allow(non_upper_case_globals)]
    pub const None: TimeGranularity = TimeGranularity::Hours23OrNone;

    /// Returns [`true`] if the most granular time being displayed will align with
    /// the top of the hour, otherwise returns [`false`].
    /// e.g. `12:00:00` is at the top of the hour for any display granularity.
    /// e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed.
    pub fn is_top_of_hour(self, minute: u8, second: u8, subsecond: u32) -> bool {
        match self {
            Self::Hours23OrNone | Self::Hours12 => true,
            Self::Minutes => minute == 0,
            Self::Seconds => minute == 0 && second == 0,
            Self::Nanoseconds => minute == 0 && second == 0 && subsecond == 0,
        }
    }

    /// Returns [`true`] if [`TimePrecision::MinuteOptional`](crate::options::TimePrecision::MinuteOptional)
    /// should retain zero minutes (as in 24-hour hour cycles), or [`false`] if zero minutes
    /// should be omitted (as in 12-hour hour cycles).
    #[inline]
    pub(crate) fn prefer_keep_minutes(self) -> bool {
        debug_assert!(
            matches!(self, Self::Hours23OrNone | Self::Hours12),
            "prefer_keep_minutes should only be called on hour-only or date-only patterns, got {:?}",
            self
        );
        matches!(self, Self::Hours23OrNone)
    }

    #[inline]
    pub(crate) fn from_ordinal(ordinal: u8) -> TimeGranularity {
        use TimeGranularity::*;
        match ordinal {
            0 => Hours23OrNone,
            1 => Hours12,
            2 => Minutes,
            3 => Seconds,
            4 => Nanoseconds,
            _ => Hours23OrNone,
        }
    }

    #[inline]
    pub(crate) const fn ordinal(self) -> u8 {
        use TimeGranularity::*;
        match self {
            Hours23OrNone => 0,
            Hours12 => 1,
            Minutes => 2,
            Seconds => 3,
            Nanoseconds => 4,
        }
    }
}

impl From<PatternItem> for TimeGranularity {
    /// Retrieves the granularity of time represented by a [`PatternItem`].
    /// If the [`PatternItem`] is not time-related, returns [`TimeGranularity::Hours23OrNone`].
    fn from(item: PatternItem) -> Self {
        match item {
            PatternItem::Field(field) => match field.symbol {
                fields::FieldSymbol::Hour(fields::Hour::H11 | fields::Hour::H12) => Self::Hours12,
                fields::FieldSymbol::Hour(fields::Hour::H23) => Self::Hours23OrNone,
                fields::FieldSymbol::Minute => Self::Minutes,
                fields::FieldSymbol::Second(_) => Self::Seconds,
                fields::FieldSymbol::DecimalSecond(_) => Self::Nanoseconds,
                _ => Self::Hours23OrNone,
            },
            _ => Self::Hours23OrNone,
        }
    }
}
