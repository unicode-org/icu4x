// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
use crate::neo_serde::*;
use crate::neo_skeleton::NeoTimeZoneStyle;
use crate::options::preferences::HourCycle;
use crate::raw::neo::RawNeoOptions;
use crate::{fieldset, NeoSkeletonLength};
use icu_provider::prelude::*;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeFieldSet {
    /// A time of day.
    T(fieldset::T),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct TimeZoneStyleWithLength {
    pub style: NeoTimeZoneStyle,
    pub length: NeoSkeletonLength,
}

impl TimeZoneStyleWithLength {
    pub fn from_style_and_length(style: NeoTimeZoneStyle, length: NeoSkeletonLength) -> Self {
        Self { style, length }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateAndTimeFieldSet {
    /// The day of the month with time of day, as in
    /// “on the 1st at 10:31 AM”.
    DT(fieldset::DT),
    /// The month and day of the month with time of day, as in
    /// “January 1st at 10:31 AM”.
    MDT(fieldset::MDT),
    /// The year, month, and day of the month with time of day, as in
    /// “January 1st, 2000 at 10:31 AM”.
    YMDT(fieldset::YMDT),
    /// The day of the month and day of the week with time of day, as in
    /// “Saturday 1st at 10:31 AM”.
    DET(fieldset::DET),
    /// The month, day of the month, and day of the week with time of day, as in
    /// “Saturday, January 1st at 10:31 AM”.
    MDET(fieldset::MDET),
    /// The year, month, day of the month, and day of the week with time of day, as in
    /// “Saturday, January 1st, 2000 at 10:31 AM”.
    YMDET(fieldset::YMDET),
    /// The day of the week alone with time of day, as in
    /// “Saturday at 10:31 AM”.
    ET(fieldset::ET),
}

/// An enum supporting date, calendar period, time, and date+time field sets
/// and options. Time zones are not supported with this enum.
///
/// This enum is useful when formatting a type that does not contain a
/// time zone or to avoid storing time zone data.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompositeDateTimeFieldSet {
    /// Field set for a date.
    Date(DateFieldSet),
    /// Field set for a calendar period.
    CalendarPeriod(CalendarPeriodFieldSet),
    /// Field set for a time.
    Time(TimeFieldSet),
    /// Field set for a date and a time together.
    DateTime(DateAndTimeFieldSet),
}

impl CompositeDateTimeFieldSet {
    /// If the [`CompositeFieldSet`] does not contain a time zone,
    /// returns the corresponding [`CompositeDateTimeFieldSet`].
    pub fn try_from_composite_field_set(field_set: CompositeFieldSet) -> Option<Self> {
        match field_set {
            CompositeFieldSet::Date(v) => Some(Self::Date(v)),
            CompositeFieldSet::CalendarPeriod(v) => Some(Self::CalendarPeriod(v)),
            CompositeFieldSet::Time(v) => Some(Self::Time(v)),
            CompositeFieldSet::Zone(_) => None,
            CompositeFieldSet::DateTime(v) => Some(Self::DateTime(v)),
            CompositeFieldSet::DateZone(_, _) => None,
            CompositeFieldSet::TimeZone(_, _) => None,
            CompositeFieldSet::DateTimeZone(_, _) => None,
        }
    }

    /// Returns the [`CompositeFieldSet`] corresponding to this
    /// [`CompositeDateTimeFieldSet`].
    pub fn to_composite_field_set(self) -> CompositeFieldSet {
        match self {
            Self::Date(v) => CompositeFieldSet::Date(v),
            Self::CalendarPeriod(v) => CompositeFieldSet::CalendarPeriod(v),
            Self::Time(v) => CompositeFieldSet::Time(v),
            Self::DateTime(v) => CompositeFieldSet::DateTime(v),
        }
    }
}

/// An enum supporting all possible field sets and options.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SemanticSkeletonSerde", into = "SemanticSkeletonSerde")
)]
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

impl CompositeFieldSet {
    pub(crate) fn to_raw_options(self) -> RawNeoOptions {
        match self {
            CompositeFieldSet::Date(field_set) => field_set.to_raw_options(),
            CompositeFieldSet::CalendarPeriod(field_set) => field_set.to_raw_options(),
            CompositeFieldSet::Time(field_set) => field_set.to_raw_options(),
            CompositeFieldSet::Zone(field_set) => field_set.to_raw_options(),
            CompositeFieldSet::DateTime(field_set) => field_set.to_raw_options(),
            CompositeFieldSet::DateZone(field_set, _) => field_set.to_raw_options(),
            CompositeFieldSet::TimeZone(field_set, _) => field_set.to_raw_options(),
            CompositeFieldSet::DateTimeZone(field_set, _) => field_set.to_raw_options(),
        }
    }
}

macro_rules! impl_attrs {
    (@attrs, $type:path, [$(($attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl $type {
            $(
                const $attr_var: &'static DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic($value);
            )+
            // $(
            //     const $str_var: &'static str = $value;
            // )+
            /// All attributes associated with this enum.
            pub const ALL_DATA_MARKER_ATTRIBUTES: &[&DataMarkerAttributes] = &[
                $(
                    Self::$attr_var,
                )+
            ];
        }
    };
    (@to_raw_options, $type:path, [$($variant:ident),+,]) => {
        impl $type {
            pub(crate) fn to_raw_options(self) -> RawNeoOptions {
                match self {
                    $(
                        Self::$variant(variant) => variant.to_raw_options(),
                    )+
                }
            }
        }
    };
    (@datetime_fns, $type:path, [$(($d_variant:ident, $variant:ident)),+,]) => {
        impl_attrs! { @to_raw_options, $type, [$($variant),+,] }
        impl $type {
            pub(crate) fn to_date_field_set(self) -> DateFieldSet {
                match self {
                    $(
                        Self::$variant(variant) => DateFieldSet::$d_variant(variant.to_date_field_set()),
                    )+
                }
            }
            pub(crate) fn to_time_field_set(self) -> TimeFieldSet {
                let (length, time_precision, alignment) = match self {
                    $(
                        Self::$variant(variant) => (variant.length, variant.time_precision, variant.alignment),
                    )+
                };
                TimeFieldSet::T(fieldset::T {
                    length,
                    time_precision,
                    alignment,
                })
            }
            pub(crate) fn from_date_field_set_with_raw_options(date_field_set: DateFieldSet, options: RawNeoOptions) -> Self {
                match date_field_set {
                    $(
                        DateFieldSet::$d_variant(_) => Self::$variant(fieldset::$variant::from_raw_options(options)),
                    )+
                }
            }
        }
    };
    (@id_str, $type:path, [$(($variant:ident, $attr_var:ident)),+,]) => {
        impl $type {
            /// Returns a stable string identifying this set of fields.
            pub(crate) const fn id_str(self) -> &'static DataMarkerAttributes {
                match self {
                    $(
                        Self::$variant(_) => Self::$attr_var,
                    )+
                }
            }
        }
    };
    ($type:path, [$(($variant:ident, $attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl_attrs! { @attrs, $type, [$(($attr_var, $str_var, $value)),+,] }
        impl_attrs! { @to_raw_options, $type, [$($variant),+,] }
        impl_attrs! { @id_str, $type, [$(($variant, $attr_var)),+,] }
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
    @attrs,
    TimeFieldSet,
    [
        (ATTR_T, STR_T, "j"),
        (ATTR_T12, STR_T12, "h"),
        (ATTR_T24, STR_T24, "h0"),
    ]
}

impl_attrs! {
    @to_raw_options,
    TimeFieldSet,
    [
        T,
    ]
}

impl TimeFieldSet {
    pub(crate) const fn id_str_for_hour_cycle(
        self,
        hour_cycle: Option<HourCycle>,
    ) -> &'static DataMarkerAttributes {
        use HourCycle::*;
        match hour_cycle {
            None => Self::ATTR_T,
            Some(H11 | H12) => Self::ATTR_T12,
            Some(H23 | H24) => Self::ATTR_T24,
        }
    }
}

impl TimeZoneStyleWithLength {
    pub(crate) fn to_raw_options(self) -> RawNeoOptions {
        RawNeoOptions {
            length: self.length,
            alignment: None,
            year_style: None,
            time_precision: None,
        }
    }
}

impl_attrs! {
    @attrs,
    DateAndTimeFieldSet,
    [
        (ATTR_ET, STR_ET, "ej"),
    ]
}

impl_attrs! {
    @datetime_fns,
    DateAndTimeFieldSet,
    [
        (D, DT),
        (MD, MDT),
        (YMD, YMDT),
        (DE, DET),
        (MDE, MDET),
        (YMDE, YMDET),
        (E, ET),
    ]
}

impl DateAndTimeFieldSet {
    pub(crate) const fn id_str(self) -> Option<&'static DataMarkerAttributes> {
        match self {
            DateAndTimeFieldSet::ET(_) => Some(Self::ATTR_ET),
            _ => None,
        }
    }
}
