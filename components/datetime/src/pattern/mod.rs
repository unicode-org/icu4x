// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod common;
mod error;
pub mod hour_cycle;
mod item;
pub mod reference;
pub mod runtime;

use crate::fields;
pub use error::PatternError;
pub use hour_cycle::CoarseHourCycle;
use icu_provider::prelude::*;
pub use item::{GenericPatternItem, PatternItem};

/// The granularity of time represented in a pattern item.
/// Ordered from least granular to most granular for comparison.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, yoke::Yokeable, zerofrom::ZeroFrom,
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::pattern),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum TimeGranularity {
    None,
    Hours,
    Minutes,
    Seconds,
    Nanoseconds,
}

impl Default for TimeGranularity {
    fn default() -> Self {
        Self::None
    }
}

impl TimeGranularity {
    /// Returns [`true`] if the most granular time being displayed will align with
    /// the top of the hour, otherwise returns [`false`].
    /// e.g. `12:00:00` is at the top of the hour for any display granularity.
    /// e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed.
    pub fn is_top_of_hour(self, minute: u8, second: u8, nanosecond: u32) -> bool {
        match self {
            Self::None | Self::Hours => true,
            Self::Minutes => minute == 0,
            Self::Seconds => minute + second == 0,
            Self::Nanoseconds => minute as u32 + second as u32 + nanosecond == 0,
        }
    }
}

impl From<&PatternItem> for TimeGranularity {
    /// Retrieves the granularity of time represented by a [`PatternItem`].
    /// If the [`PatternItem`] is not time-related, returns [`None`].
    fn from(item: &PatternItem) -> Self {
        match item {
            PatternItem::Field(field) => match field.symbol {
                fields::FieldSymbol::Hour(_) => Self::Hours,
                fields::FieldSymbol::Minute => Self::Minutes,
                fields::FieldSymbol::Second(s) => match s {
                    fields::Second::FractionalSecond => Self::Nanoseconds,
                    fields::Second::Millisecond | fields::Second::Second => Self::Seconds,
                },
                _ => Self::None,
            },
            _ => Self::None,
        }
    }
}
