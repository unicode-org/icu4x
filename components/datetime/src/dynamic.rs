// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use crate::options::preferences::HourCycle;
use crate::{fieldset, NeoSkeletonLength};
use crate::neo_skeleton::{Alignment, FractionalSecondDigits, NeoTimeZoneStyle};
use crate::scaffold::DateTimeMarkers;
use icu_provider::prelude::*;
use crate::raw::neo::RawNeoOptions;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateFieldSet {
    /// The day of the month, as in
    /// “on the 1st”.
    D(fieldset::D),
    /// The month and day of the month, as in
    /// “January 1st”.
    MD(fieldset::MD),
    /// The year, month, and day of the month, as in
    /// “January 1st, 2000”.
    YMD(fieldset::YMD),
    /// The day of the month and day of the week, as in
    /// “Saturday 1st”.
    DE(fieldset::DE),
    /// The month, day of the month, and day of the week, as in
    /// “Saturday, January 1st”.
    MDE(fieldset::MDE),
    /// The year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000”.
    YMDE(fieldset::YMDE),
    /// The day of the week alone, as in
    /// “Saturday”.
    E(fieldset::E),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CalendarPeriodFieldSet {
    /// A standalone month, as in
    /// “January”.
    M(fieldset::M),
    /// A month and year, as in
    /// “January 2000”.
    YM(fieldset::YM),
    /// A year, as in
    /// “2000”.
    Y(fieldset::Y),
    // TODO: Add support for week-of-year
    // /// The year and week of the year, as in
    // /// “52nd week of 1999”.
    // YW(fieldset::YW),
    // TODO(#501): Consider adding support for Quarter and YearQuarter.
}

/// Field set for a standalone time of day.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeFieldSet {
    /// An hour (12-hour or 24-hour chosen by locale), as in
    /// "4 pm" or "16h"
    H(fieldset::H),
    /// An hour and minute (12-hour or 24-hour chosen by locale), as in
    /// "4:03 pm" or "16:03"
    HM(fieldset::HM),
    /// An hour, minute, and second (12-hour or 24-hour chosen by locale), as in
    /// "4:03:51 pm" or "16:03:51"
    HMS(fieldset::HMS),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TimeZoneStyleWithLength {
    pub style: NeoTimeZoneStyle,
    pub length: NeoSkeletonLength,
}

/// Field set for a time when used in combination with a date.
///
/// This is separate from [`TimeFieldSet`] in order to avoid duplication of options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeOfDateFieldSet {
    /// An hour (12-hour or 24-hour chosen by locale), as in
    /// "4 pm" or "16h"
    #[non_exhaustive]
    H {},
    /// An hour and minute (12-hour or 24-hour chosen by locale), as in
    /// "4:03 pm" or "16:03"
    #[non_exhaustive]
    HM {},
    /// An hour, minute, and second (12-hour or 24-hour chosen by locale), as in
    /// "4:03:51 pm" or "16:03:51"
    #[non_exhaustive]
    HMS {
        fractional_second_digits: Option<FractionalSecondDigits>
    }
}

impl TimeOfDateFieldSet {
    /// Construct a [`TimeOfDateFieldSet::H`].
    pub const fn h() -> Self {
        Self::H {}
    }
    /// Construct a [`TimeOfDateFieldSet::HM`].
    pub const fn hm() -> Self {
        Self::HM {}
    }
    /// Construct a [`TimeOfDateFieldSet::HMS`].
    pub const fn hms() -> Self {
        Self::HMS {
            fractional_second_digits: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateAndTimeFieldSet {
    /// The day of the month with time of day, as in
    /// “on the 1st at 10:31 AM”.
    #[non_exhaustive]
    D {
        date: fieldset::D,
        time: TimeOfDateFieldSet,
    },
    /// The month and day of the month with time of day, as in
    /// “January 1st at 10:31 AM”.
    #[non_exhaustive]
    MD {
        date: fieldset::MD,
        time: TimeOfDateFieldSet,
    },
    /// The year, month, and day of the month with time of day, as in
    /// “January 1st, 2000 at 10:31 AM”.
    #[non_exhaustive]
    YMD {
        date: fieldset::YMD,
        time: TimeOfDateFieldSet,
    },
    /// The day of the month and day of the week with time of day, as in
    /// “Saturday 1st at 10:31 AM”.
    #[non_exhaustive]
    DE {
        date: fieldset::DE,
        time: TimeOfDateFieldSet,
    },
    /// The month, day of the month, and day of the week with time of day, as in
    /// “Saturday, January 1st at 10:31 AM”.
    #[non_exhaustive]
    MDE {
        date: fieldset::MDE,
        time: TimeOfDateFieldSet,
    },
    /// The year, month, day of the month, and day of the week with time of day, as in
    /// “Saturday, January 1st, 2000 at 10:31 AM”.
    #[non_exhaustive]
    YMDE {
        date: fieldset::YMDE,
        time: TimeOfDateFieldSet,
    },
    /// The day of the week alone with time of day, as in
    /// “Saturday at 10:31 AM”.
    #[non_exhaustive]
    E {
        date: fieldset::E,
        time: TimeOfDateFieldSet,
        alignment: Option<Alignment>,
    }
}

impl DateAndTimeFieldSet {
    /// Construct a [`DateAndTimeFieldSet::D`].
    pub const fn d(date: fieldset::D, time: TimeOfDateFieldSet) -> Self {
        Self::D {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::MD`].
    pub const fn md(date: fieldset::MD, time: TimeOfDateFieldSet) -> Self {
        Self::MD {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::YMD`].
    pub const fn ymd(date: fieldset::YMD, time: TimeOfDateFieldSet) -> Self {
        Self::YMD {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::DE`].
    pub const fn de(date: fieldset::DE, time: TimeOfDateFieldSet) -> Self {
        Self::DE {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::MDE`].
    pub const fn mde(date: fieldset::MDE, time: TimeOfDateFieldSet) -> Self {
        Self::MDE {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::YMDE`].
    pub const fn ymde(date: fieldset::YMDE, time: TimeOfDateFieldSet) -> Self {
        Self::YMDE {
            date,
            time,
        }
    }
    /// Construct a [`DateAndTimeFieldSet::E`].
    pub const fn e(date: fieldset::E, time: TimeOfDateFieldSet) -> Self {
        Self::E {
            date,
            time,
            alignment: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompositeFieldSet {
    /// Field set for a date.
    Date(DateFieldSet),
    /// Field set for a calendar period.
    CalendarPeriod(CalendarPeriodFieldSet),
    /// Field set for a time.
    Time(TimeFieldSet),
    /// Field set for a time zone.
    Zone(TimeZoneStyleWithLength),
    /// Field set for a date and a time together.
    DateTime(DateAndTimeFieldSet),
    /// Field set for a date and a time zone together.
    DateZone(DateFieldSet, NeoTimeZoneStyle),
    /// Field set for a time and a time zone together.
    TimeZone(TimeFieldSet, NeoTimeZoneStyle),
    /// Field set for a date, a time, and a time zone together.
    DateTimeZone(DateAndTimeFieldSet, NeoTimeZoneStyle),
}

macro_rules! impl_attrs {
    (@skip_fns, $type:path, [$(($attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl $type {
            $(
                const $attr_var: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic($value);
            )+
            $(
                const $str_var: &'static str = $value;
            )+
            /// All attributes associated with this enum.
            pub const ALL_DATA_MARKER_ATTRIBUTES: &[&DataMarkerAttributes] = &[
                $(
                    Self::$attr_var,
                )+
            ];
        }
    };
    ($type:path, [$(($variant:ident, $attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl_attrs! { @skip_fns, $type, [$(($attr_var, $str_var, $value)),+,] }
        impl $type {
            /// Returns a stable string identifying this set of fields.
            pub(crate) const fn id_str(self) -> &'static DataMarkerAttributes {
                match self {
                    $(
                        Self::$variant(_) => Self::$attr_var,
                    )+
                }
            }
            pub(crate) fn to_raw_options(self) -> RawNeoOptions {
                match self {
                    $(
                        Self::$variant(variant) => variant.to_raw_options(),
                    )+
                }
            }
        }
    };
}

impl_attrs! {
    DateFieldSet,
    [
        (D, ATTR_D, STR_D, "d"),
        (MD, ATTR_MD, STR_MD, "m0d"),
        (YMD, ATTR_YMD, STR_YMD, "ym0d"),
        (DE, ATTR_DE, STR_DE, "de"),
        (MDE, ATTR_MDE, STR_MDE, "m0de"),
        (YMDE, ATTR_YMDE, STR_YMDE, "ym0de"),
        (E, ATTR_E, STR_E, "e"),
    ]
}

impl_attrs! {
    CalendarPeriodFieldSet,
    [
        (M, ATTR_M, STR_M, "m0"),
        (YM, ATTR_YM, STR_YM, "ym0"),
        (Y, ATTR_Y, STR_Y, "y"),
    ]
}

impl_attrs! {
    @skip_fns,
    TimeFieldSet,
    [
        (ATTR_H, STR_H, "j"),
        (ATTR_HM, STR_HM, "jm"),
        (ATTR_HMS, STR_HMS, "jms"),
        (ATTR_H12, STR_H12, "h"),
        (ATTR_H12M, STR_H12M, "hm"),
        (ATTR_H12MS, STR_H12MS, "hms"),
        (ATTR_H24, STR_H24, "h0"),
        (ATTR_H24M, STR_H24M, "h0m"),
        (ATTR_H24MS, STR_H24MS, "h0ms"),
    ]
}

impl TimeFieldSet {
    pub(crate) const fn id_str_for_hour_cycle(self, hour_cycle: Option<HourCycle>) -> &'static DataMarkerAttributes {
        use HourCycle::*;
        match (self, hour_cycle) {
            (TimeFieldSet::H(_), None) => Self::ATTR_H,
            (TimeFieldSet::H(_), Some(H11 | H12)) => Self::ATTR_H12,
            (TimeFieldSet::H(_), Some(H23 | H24)) => Self::ATTR_H24,
            (TimeFieldSet::HM(_), None) => Self::ATTR_HM,
            (TimeFieldSet::HM(_), Some(H11 | H12)) => Self::ATTR_H12M,
            (TimeFieldSet::HM(_), Some(H23 | H24)) => Self::ATTR_H24M,
            (TimeFieldSet::HMS(_), None) => Self::ATTR_HMS,
            (TimeFieldSet::HMS(_), Some(H11 | H12)) => Self::ATTR_H12MS,
            (TimeFieldSet::HMS(_), Some(H23 | H24)) => Self::ATTR_H24MS,
        }
    }
    pub(crate) fn to_raw_options(self) -> RawNeoOptions {
        match self {
            TimeFieldSet::H(variant) => variant.to_raw_options(),
            TimeFieldSet::HM(variant) => variant.to_raw_options(),
            TimeFieldSet::HMS(variant) => variant.to_raw_options(),
        }
    }
}

impl TimeZoneStyleWithLength {
    pub(crate) fn to_raw_options(self) -> RawNeoOptions {
        RawNeoOptions {
            length: self.length,
            alignment: None,
            year_style: None,
            fractional_second_digits: None,
        }
    }
}

impl TimeOfDateFieldSet {
    pub(crate) fn set_on_raw_options(self, options: &mut RawNeoOptions) {
        match self {
            TimeOfDateFieldSet::H {  } => (),
            TimeOfDateFieldSet::HM {  } => (),
            TimeOfDateFieldSet::HMS { fractional_second_digits } => {
                options.fractional_second_digits = fractional_second_digits;
            },
        }
    }
    pub(crate) fn to_time_field_set_with_length_and_alignment(self, length: NeoSkeletonLength, alignment: Option<Alignment>) -> TimeFieldSet {
        match self {
            TimeOfDateFieldSet::H {  } => TimeFieldSet::H(fieldset::H {
                length,
                alignment,
            }),
            TimeOfDateFieldSet::HM {  } => TimeFieldSet::HM(fieldset::HM {
                length,
                alignment,
            }),
            TimeOfDateFieldSet::HMS { fractional_second_digits } => TimeFieldSet::HMS(fieldset::HMS {
                length,
                alignment,
                fractional_second_digits,
            }),
        }
    }
}

impl_attrs! {
    @skip_fns,
    DateAndTimeFieldSet,
    [
        (ATTR_EHM, STR_EHM, "ejm"),
        (ATTR_EHMS, STR_EHMS, "ejms"),
    ]
}

impl DateAndTimeFieldSet {
    pub(crate) const fn id_str(self) -> Option<&'static DataMarkerAttributes> {
        match self {
            DateAndTimeFieldSet::E { time: TimeOfDateFieldSet::HM { .. }, ..} => Some(Self::ATTR_EHM),
            DateAndTimeFieldSet::E { time: TimeOfDateFieldSet::HMS { .. }, ..} => Some(Self::ATTR_EHMS),
            _ => None,
        }
    }
    pub(crate) fn to_raw_options(self) -> RawNeoOptions {
        match self {
            DateAndTimeFieldSet::D { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::MD { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::YMD { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::DE { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::MDE { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::YMDE { date, time } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options
            },
            DateAndTimeFieldSet::E { date, time, alignment } => {
                let mut options = date.to_raw_options();
                time.set_on_raw_options(&mut options);
                options.alignment = alignment;
                options
            },
        }
    }
    pub(crate) fn to_date_field_set(self) -> DateFieldSet {
        match self {
            DateAndTimeFieldSet::D { date, .. } => DateFieldSet::D(date),
            DateAndTimeFieldSet::MD { date, .. } => DateFieldSet::MD(date),
            DateAndTimeFieldSet::YMD { date, .. } => DateFieldSet::YMD(date),
            DateAndTimeFieldSet::DE { date, .. } => DateFieldSet::DE(date),
            DateAndTimeFieldSet::MDE { date, .. } => DateFieldSet::MDE(date),
            DateAndTimeFieldSet::YMDE { date, .. } => DateFieldSet::YMDE(date),
            DateAndTimeFieldSet::E { date, .. } => DateFieldSet::E(date),
        }
    }
    pub(crate) fn to_time_field_set(self) -> TimeFieldSet {
        let (time, length, alignment) = match self {
            DateAndTimeFieldSet::D { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::MD { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::YMD { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::DE { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::MDE { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::YMDE { date, time } => (time, date.length, date.alignment),
            DateAndTimeFieldSet::E { date, time, alignment } => (time, date.length, alignment),
        };
        time.to_time_field_set_with_length_and_alignment(length, alignment)
    }
}
