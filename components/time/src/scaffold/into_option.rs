// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    zone::{TimeZoneVariant, UtcOffset, ZoneNameTimestamp},
    Hour, Minute, Nanosecond, Second, TimeZone,
};
use icu_calendar::{types::*, AnyCalendarKind};

/// Converts Self to an `Option<T>`, either `Some(T)` if able or `None`
pub trait IntoOption<T> {
    /// Return `self` as an `Option<T>`
    fn into_option(self) -> Option<T>;
}

impl<T> IntoOption<T> for Option<T> {
    #[inline]
    fn into_option(self) -> Self {
        self
    }
}

impl<T> IntoOption<T> for () {
    #[inline]
    fn into_option(self) -> Option<T> {
        None
    }
}

impl IntoOption<Self> for YearInfo {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for MonthInfo {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for DayOfMonth {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for Weekday {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for DayOfYear {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for RataDie {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for AnyCalendarKind {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for Hour {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for Minute {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for Second {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for Nanosecond {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for TimeZone {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for UtcOffset {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for TimeZoneVariant {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}

impl IntoOption<Self> for ZoneNameTimestamp {
    #[inline]
    fn into_option(self) -> Option<Self> {
        Some(self)
    }
}
