// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod error;
mod hour_cycle;
mod item;
pub mod reference;

use crate::fields;
pub use error::PatternError;
pub use hour_cycle::CoarseHourCycle;
pub use item::{GenericPatternItem, PatternItem};

/// The granularity of time represented in a pattern item.
/// Ordered from least granular to most granular for comparsion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub(crate) enum TimeGranularity {
    None,
    Hours,
    Minutes,
    Seconds,
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
    pub fn is_top_of_hour(self, minute: u8, second: u8) -> bool {
        match self {
            Self::None | Self::Hours => true,
            Self::Minutes => minute == 0,
            Self::Seconds => minute + second == 0,
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
                fields::FieldSymbol::Second(_) => Self::Seconds,
                _ => Self::None,
            },
            _ => Self::None,
        }
    }
}
