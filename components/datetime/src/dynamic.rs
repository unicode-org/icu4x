// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "serde")]
use crate::neo_serde::*;
use crate::options::preferences::HourCycle;
use crate::raw::neo::RawNeoOptions;
use crate::scaffold::GetField;
use crate::{fields, fieldset, NeoSkeletonLength};
use icu_provider::prelude::*;

/// An enumeration over all possible date field sets.
///
/// 📏 Caution: This enumeration links more data than the
/// individual field set structs!
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

/// An enumeration over all possible calendar period field sets.
///
/// 📏 Caution: This enumeration links more data than the
/// individual field set structs!
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

/// An enumeration over all possible time field sets.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeFieldSet {
    /// A time of day.
    T(fieldset::T),
}

/// An enumeration over all possible zone field sets.
///
/// 📏 Caution: This enumeration links more data than the
/// individual field set structs!
///
/// Note: [`fieldset::Zs`] and [`fieldset::Vs`] are not included in this enum
/// because they are data size optimizations only.
///
/// # Time Zone Data Size
///
/// Time zone names contribute a lot of data size. For resource-constrained
/// environments, the following formats require the least amount of data:
///
/// - [`fieldset::Zs`]
/// - [`fieldset::O`]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZoneFieldSet {
    /// The specific non-location format, as in
    /// “Pacific Daylight Time”.
    Z(fieldset::Z),
    /// The offset format, as in
    /// “GMT−8”.
    O(fieldset::O),
    /// The generic non-location format, as in
    /// “Pacific Time”.
    V(fieldset::V),
    /// The location format, as in
    /// “Los Angeles time”.
    L(fieldset::L),
}

/// An enumeration over all possible zone styles.
///
/// This is similar to [`ZoneFieldSet`], except the fields are not
/// self-contained semantic skeletons: they do not contain the length.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ZoneStyle {
    /// The specific non-location format, as in
    /// “Pacific Daylight Time”.
    Z,
    /// The offset format, as in
    /// “GMT−8”.
    O,
    /// The generic non-location format, as in
    /// “Pacific Time”.
    V,
    /// The location format, as in
    /// “Los Angeles time”.
    L,
}

/// An enumeration over all possible date+time composite field sets.
///
/// 📏 Caution: This enumeration links more data than the
/// individual field set structs!
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

impl GetField<CompositeFieldSet> for CompositeDateTimeFieldSet {
    fn get_field(&self) -> CompositeFieldSet {
        self.to_composite_field_set()
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
    Zone(ZoneFieldSet),
    /// Field set for a date and a time together.
    DateTime(DateAndTimeFieldSet),
    /// Field set for a date and a time zone together.
    DateZone(DateFieldSet, ZoneStyle),
    /// Field set for a time and a time zone together.
    TimeZone(TimeFieldSet, ZoneStyle),
    /// Field set for a date, a time, and a time zone together.
    DateTimeZone(DateAndTimeFieldSet, ZoneStyle),
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
            pub const ALL_DATA_MARKER_ATTRIBUTES: &'static [&'static DataMarkerAttributes] = &[
                $(
                    Self::$attr_var,
                )+
            ];
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
    (@composite, $type:path, $variant:ident) => {
        impl $type {
            #[inline]
            pub(crate) fn to_enum(self) -> $type {
                self
            }
        }
        impl GetField<CompositeFieldSet> for $type {
            #[inline]
            fn get_field(&self) -> CompositeFieldSet {
                CompositeFieldSet::$variant(self.to_enum())
            }
        }
    };
    (@date, $type:path, [$(($variant:ident, $attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl_attrs! { @attrs, $type, [$(($attr_var, $str_var, $value)),+,] }
        impl_attrs! { @id_str, $type, [$(($variant, $attr_var)),+,] }
        impl_attrs! { @to_raw_options, $type, [$($variant),+,] }
        impl_attrs! { @composite, $type, Date }
    };
    (@calendar_period, $type:path, [$(($variant:ident, $attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl_attrs! { @attrs, $type, [$(($attr_var, $str_var, $value)),+,] }
        impl_attrs! { @to_raw_options, $type, [$($variant),+,] }
        impl_attrs! { @composite, $type, CalendarPeriod }
        impl_attrs! { @id_str, $type, [$(($variant, $attr_var)),+,] }
    };
    (@time, $type:path, [$(($attr_var:ident, $str_var:ident, $value:literal)),+,]) => {
        impl_attrs! { @attrs, $type, [$(($attr_var, $str_var, $value)),+,] }
        impl_attrs! { @to_raw_options, $type, [T,] }
        impl_attrs! { @composite, $type, Time }
    };
    (@zone, $type:path, [$($variant:ident),+,]) => {
        impl_attrs! { @composite, $type, Zone }
        impl $type {
            pub(crate) fn to_field(self) -> (fields::TimeZone, fields::FieldLength) {
                match self {
                    $(
                        Self::$variant(variant) => variant.to_field(),
                    )+
                }
            }
        }
    };
    (@datetime, $type:path, [$(($d_variant:ident, $variant:ident)),+,]) => {
        impl_attrs! { @to_raw_options, $type, [$($variant),+,] }
        impl_attrs! { @composite, $type, DateTime }
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
            #[cfg(feature = "serde")]
            pub(crate) fn from_date_field_set_with_raw_options(date_field_set: DateFieldSet, options: RawNeoOptions) -> Self {
                match date_field_set {
                    $(
                        DateFieldSet::$d_variant(_) => Self::$variant(fieldset::$variant::from_raw_options(options)),
                    )+
                }
            }
        }
    };
}

impl_attrs! {
    @date,
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
    @calendar_period,
    CalendarPeriodFieldSet,
    [
        (M, ATTR_M, STR_M, "m0"),
        (YM, ATTR_YM, STR_YM, "ym0"),
        (Y, ATTR_Y, STR_Y, "y"),
    ]
}

impl_attrs! {
    @time,
    TimeFieldSet,
    [
        (ATTR_T, STR_T, "j"),
        (ATTR_T12, STR_T12, "h"),
        (ATTR_T24, STR_T24, "h0"),
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

impl_attrs! {
    @zone,
    ZoneFieldSet,
    [
        Z,
        O,
        V,
        L,
    ]
}

impl ZoneFieldSet {
    pub(crate) fn from_time_zone_style_and_length(
        style: ZoneStyle,
        length: NeoSkeletonLength,
    ) -> Self {
        match style {
            ZoneStyle::Z => Self::Z(fieldset::Z::with_length(length)),
            ZoneStyle::O => Self::O(fieldset::O::with_length(length)),
            ZoneStyle::V => Self::V(fieldset::V::with_length(length)),
            ZoneStyle::L => Self::L(fieldset::L::with_length(length)),
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
    @datetime,
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
