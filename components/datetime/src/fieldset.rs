// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! All available field sets for datetime formatting.

use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz, *},
    scaffold::*,
    DateTimeCombo,
};
use icu_calendar::{
    types::{
        DayOfMonth, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond, YearInfo,
    },
    AnyCalendarKind, Date, Iso, Time,
};
use icu_provider::marker::NeverMarker;
use icu_timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};

/// Maps the token `yes` to the given ident
macro_rules! yes_to {
    ($any:ident, yes) => {
        $any
    };
    () => {
        unreachable!() // prevent bugs
    };
}

/// Generates the options argument passed into the docs test constructor
macro_rules! length_option_helper {
    ($type:ty, $length:ident) => {
        concat!(
            stringify!($type),
            "::with_length(NeoSkeletonLength::",
            stringify!($length),
            ")"
        )
    };
}

macro_rules! impl_marker_with_options {
    (
        $(#[$attr:meta])*
        $type:ident,
        $(sample_length: $sample_length:ident,)?
        $(alignment: $alignment_yes:ident,)?
        $(year_style: $yearstyle_yes:ident,)?
        $(fractional_second_digits: $fractionalsecondigits_yes:ident,)?
    ) => {
        $(#[$attr])*
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct $type {
            $(
                /// The desired length of the formatted string.
                ///
                /// See: [`NeoSkeletonLength`]
                pub length: datetime_marker_helper!(@option/length, $sample_length),
            )?
            $(
                /// Whether fields should be aligned for a column-like layout.
                ///
                /// See: [`Alignment`]
                pub alignment: datetime_marker_helper!(@option/alignment, $alignment_yes),
            )?
            $(
                /// When to display the era field in the formatted string.
                ///
                /// See: [`YearStyle`]
                pub year_style: datetime_marker_helper!(@option/yearstyle, $yearstyle_yes),
            )?
            $(
                /// How many fractional seconds to display.
                ///
                /// See: [`FractionalSecondDigits`]
                pub fractional_second_digits: datetime_marker_helper!(@option/fractionalsecondigits, $fractionalsecondigits_yes),
            )?
        }
        impl $type {
            #[doc = concat!("Creates a ", stringify!($type), " skeleton with the given formatting length.")]
            pub const fn with_length(length: NeoSkeletonLength) -> Self {
                Self {
                    length,
                    $(
                        alignment: yes_to!(None, $alignment_yes),
                    )?
                    $(
                        year_style: yes_to!(None, $yearstyle_yes),
                    )?
                    $(
                        fractional_second_digits: yes_to!(None, $fractionalsecondigits_yes),
                    )?
                }
            }
        }
        impl_get_field!($type, never);
        impl_get_field!($type, length, yes);
        $(
            impl_get_field!($type, alignment, $alignment_yes);
            impl $type {
                /// Sets the alignment option.
                pub const fn with_alignment(mut self, alignment: Alignment) -> Self {
                    self.alignment = Some(alignment);
                    self
                }
            }
        )?
        $(
            impl_get_field!($type, year_style, $yearstyle_yes);
            impl $type {
                /// Sets the year style option.
                pub const fn with_year_style(mut self, year_style: YearStyle) -> Self {
                    self.year_style = Some(year_style);
                    self
                }
            }
        )?
        $(
            impl_get_field!($type, fractional_second_digits, $fractionalsecondigits_yes);
            impl $type {
                /// Sets the fractional second digits option.
                pub const fn with_fractional_second_digits(mut self, digits: FractionalSecondDigits) -> Self {
                    self.fractional_second_digits = Some(digits);
                    self
                }
            }
        )?
    };
}

/// Internal helper macro used by [`impl_date_marker`] and [`impl_calendar_period_marker`]
macro_rules! impl_date_or_calendar_period_marker {
    (
        // The name of the type being created.
        $type:ident,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether years can occur.
        $(years = $years_yes:ident,)?
        // Whether months can occur.
        $(months = $months_yes:ident,)?
        // Whether weekdays can occur.
        $(weekdays = $weekdays_yes:ident,)?
        // Whether the input should contain years.
        $(input_year = $year_yes:ident,)?
        // Whether the input should contain months.
        $(input_month = $month_yes:ident,)?
        // Whether the input should contain the day of the month.
        $(input_day_of_month = $day_of_month_yes:ident,)?
        // Whether the input should contain the day of the week.
        $(input_day_of_week = $day_of_week_yes:ident,)?
        // Whether the input should contain the day of the year.
        $(input_day_of_year = $day_of_year_yes:ident,)?
        // Whether the input should declare its calendar kind.
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        // Whether the alignment option should be available.
        // According to UTS 35, it should be available with years, months, and days.
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// # Examples
            ///
            /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::Date;
            /// use icu::datetime::DateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use writeable::assert_try_writeable_eq;
            #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_iso(2024, 5, 17).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.convert_and_format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            ///
            /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::Date;
            /// use icu::calendar::Gregorian;
            /// use icu::datetime::FixedCalendarDateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use writeable::assert_try_writeable_eq;
            ///
            #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Date::try_new_gregorian(2024, 5, 17).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $type,
            sample_length: $sample_length,
            $(alignment: $option_alignment_yes,)?
            $(year_style: $year_yes,)?
        );
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year, $($years_yes)?);
            type MonthNames = datetime_marker_helper!(@names/month, $($months_yes)?);
            type WeekdayNames = datetime_marker_helper!(@names/weekday, $($weekdays_yes)?);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
        }
        impl DateInputMarkers for $type {
            type YearInput = datetime_marker_helper!(@input/year, $($year_yes)?);
            type MonthInput = datetime_marker_helper!(@input/month, $($month_yes)?);
            type DayOfMonthInput = datetime_marker_helper!(@input/day_of_month, $($day_of_month_yes)?);
            type DayOfWeekInput = datetime_marker_helper!(@input/day_of_week, $($day_of_week_yes)?);
            type AnyCalendarKindInput = datetime_marker_helper!(@input/any_calendar_kind, $($any_calendar_kind_yes)?);
        }
        impl<C: CldrCalendar> TypedDateDataMarkers<C> for $type {
            type DateSkeletonPatternsV1Marker = datetime_marker_helper!(@dates/typed, yes);
            type YearNamesV1Marker = datetime_marker_helper!(@years/typed, $($years_yes)?);
            type MonthNamesV1Marker = datetime_marker_helper!(@months/typed, $($months_yes)?);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateDataMarkers for $type {
            type Skel = datetime_marker_helper!(@calmarkers, yes);
            type Year = datetime_marker_helper!(@calmarkers, $($years_yes)?);
            type Month = datetime_marker_helper!(@calmarkers, $($months_yes)?);
            type WeekdayNamesV1Marker = datetime_marker_helper!(@weekdays, $($weekdays_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = Self;
            type T = NeoNeverMarker;
            type Z = NeoNeverMarker;
            type LengthOption = datetime_marker_helper!(@option/length, $sample_length);
            type AlignmentOption = datetime_marker_helper!(@option/alignment, $($months_yes)?);
            type YearStyleOption = datetime_marker_helper!(@option/yearstyle, $($year_yes)?);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
    };
}

/// Implements a field set of date fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// See [`impl_date_marker`].
macro_rules! impl_date_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        $(years = $years_yes:ident,)?
        $(months = $months_yes:ident,)?
        $(dates = $dates_yes:ident,)?
        $(weekdays = $weekdays_yes:ident,)?
        $(input_year = $year_yes:ident,)?
        $(input_month = $month_yes:ident,)?
        $(input_day_of_month = $day_of_month_yes:ident,)?
        $(input_day_of_week = $day_of_week_yes:ident,)?
        $(input_day_of_year = $day_of_year_yes:ident,)?
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_date_or_calendar_period_marker!(
            $type,
            description = $description,
            sample_length = $sample_length,
            sample = $sample,
            $(years = $years_yes,)?
            $(months = $months_yes,)?
            $(dates = $dates_yes,)?
            $(weekdays = $weekdays_yes,)?
            $(input_year = $year_yes,)?
            $(input_month = $month_yes,)?
            $(input_day_of_month = $day_of_month_yes,)?
            $(input_day_of_week = $day_of_week_yes,)?
            $(input_day_of_year = $day_of_year_yes,)?
            $(input_any_calendar_kind = $any_calendar_kind_yes,)?
            $(option_alignment = $option_alignment_yes,)?
        );
        impl HasConstDateComponents for $type {
            const COMPONENTS: NeoDateComponents = $components;
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Date($components);
        }
    };
}

/// Implements a field set of calendar period fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// See [`impl_date_marker`].
macro_rules! impl_calendar_period_marker {
    (
        $type:ident,
        $components:expr,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        $(years = $years_yes:ident,)?
        $(months = $months_yes:ident,)?
        $(dates = $dates_yes:ident,)?
        $(input_year = $year_yes:ident,)?
        $(input_month = $month_yes:ident,)?
        $(input_any_calendar_kind = $any_calendar_kind_yes:ident,)?
        $(option_alignment = $option_alignment_yes:ident,)?
    ) => {
        impl_date_or_calendar_period_marker!(
            $type,
            description = $description,
            sample_length = $sample_length,
            sample = $sample,
            $(years = $years_yes,)?
            $(months = $months_yes,)?
            $(dates = $dates_yes,)?
            $(input_year = $year_yes,)?
            $(input_month = $month_yes,)?
            $(input_any_calendar_kind = $any_calendar_kind_yes,)?
            $(option_alignment = $option_alignment_yes,)?
        );
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::CalendarPeriod($components);
        }
    };
}

/// Implements a field set of time fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_time_marker {
    (
        // The name of the type being created.
        $type:ident,
        // An expression for the field set.
        $components:expr,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether day periods can occur.
        $(dayperiods = $dayperiods_yes:ident,)?
        // Whether the input should include hours.
        $(input_hour = $hour_yes:ident,)?
        // Whether the input should contain minutes.
        $(input_minute = $minute_yes:ident,)?
        // Whether the input should contain seconds.
        $(input_second = $second_yes:ident,)?
        // Whether the input should contain fractional seconds.
        $(input_nanosecond = $nanosecond_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// # Examples
            ///
            /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::DateTime;
            /// use icu::datetime::DateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use writeable::assert_try_writeable_eq;
            ///
            #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = DateTime::try_new_iso(2024, 5, 17, 15, 47, 50).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.convert_and_format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            ///
            /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::Time;
            /// use icu::calendar::Gregorian;
            /// use icu::datetime::FixedCalendarDateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use writeable::assert_try_writeable_eq;
            ///
            #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            /// let dt = Time::try_new(15, 47, 50, 0).unwrap();
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.format(&dt),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $type,
            sample_length: $sample_length,
            alignment: yes,
            $(fractional_second_digits: $nanosecond_yes,)?
        );
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod, $($dayperiods_yes)?);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials,);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations,);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long,);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short,);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long,);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short,);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods,);
        }
        impl HasConstTimeComponents for $type {
            const COMPONENTS: NeoTimeComponents = $components;
        }
        impl TimeMarkers for $type {
            type DayPeriodNamesV1Marker = datetime_marker_helper!(@dayperiods, $($dayperiods_yes)?);
            type TimeSkeletonPatternsV1Marker = datetime_marker_helper!(@times, yes);
            type HourInput = datetime_marker_helper!(@input/hour, $($hour_yes)?);
            type MinuteInput = datetime_marker_helper!(@input/minute, $($minute_yes)?);
            type SecondInput = datetime_marker_helper!(@input/second, $($second_yes)?);
            type NanoSecondInput = datetime_marker_helper!(@input/nanosecond, $($nanosecond_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = NeoNeverMarker;
            type T = Self;
            type Z = NeoNeverMarker;
            type LengthOption = datetime_marker_helper!(@option/length, $sample_length);
            type AlignmentOption = datetime_marker_helper!(@option/alignment, yes);
            type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits, $($nanosecond_yes)?);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Time($components);
        }
    };
}

/// Implements a field set of time zone fields.
///
/// Several arguments to this macro are required, and the rest are optional.
/// The optional arguments should be written as `key = yes,` if that parameter
/// should be included.
///
/// Documentation for each option is shown inline below.
macro_rules! impl_zone_marker {
    (
        $(#[$attr:meta])*
        // The name of the type being created.
        $type:ident,
        // An expression for the field set.
        $components:expr,
        // A plain language description of the field set for documentation.
        description = $description:literal,
        // Length of the sample string below.
        sample_length = $sample_length:ident,
        // A sample string. A docs test will be generated!
        sample = $sample:literal,
        // Whether zone-essentials should be loaded.
        $(zone_essentials = $zone_essentials_yes:ident,)?
        // Whether locations formats can occur.
        $(zone_locations = $zone_locations_yes:ident,)?
        // Whether generic long formats can occur.
        $(zone_generic_long = $zone_generic_long_yes:ident,)?
        // Whether generic short formats can occur.
        $(zone_generic_short = $zone_generic_short_yes:ident,)?
        // Whether specific long formats can occur.
        $(zone_specific_long = $zone_specific_long_yes:ident,)?
        // Whether specific short formats can occur.
        $(zone_specific_short = $zone_specific_short_yes:ident,)?
        // Whether metazone periods are needed
        $(metazone_periods = $metazone_periods_yes:ident,)?
        // Whether to require the TimeZoneBcp47Id
        $(input_tzid = $tzid_input_yes:ident,)?
        // Whether to require the ZoneVariant
        $(input_variant = $variant_input_yes:ident,)?
        // Whether to require the Local Time
        $(input_localtime = $localtime_input_yes:ident,)?
    ) => {
        impl_marker_with_options!(
            #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
            ///
            /// # Examples
            ///
            /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::{Date, Time};
            /// use icu::timezone::{TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant};
            /// use icu::datetime::DateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use tinystr::tinystr;
            /// use writeable::assert_try_writeable_eq;
            ///
            #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            ///
            /// // Time zone info for America/Chicago in the summer
            /// let zone = TimeZoneInfo::from_id_and_offset(
            ///     TimeZoneBcp47Id(tinystr!(8, "uschi")),
            ///     UtcOffset::from_eighths_of_hour(-5 * 8),
            /// )
            /// .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()))
            /// .with_zone_variant(ZoneVariant::daylight());
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.convert_and_format(&zone),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            ///
            /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
            ///
            /// ```
            /// use icu::calendar::{Date, Time};
            /// use icu::timezone::{CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant};
            /// use icu::calendar::Gregorian;
            /// use icu::datetime::FixedCalendarDateTimeFormatter;
            #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
            /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
            /// use icu::locale::locale;
            /// use tinystr::tinystr;
            /// use writeable::assert_try_writeable_eq;
            ///
            #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
            ///     &locale!("en").into(),
            #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
            /// )
            /// .unwrap();
            ///
            /// // Time zone info for America/Chicago in the summer
            /// let zone = TimeZoneInfo::from_id_and_offset(
            ///     TimeZoneBcp47Id(tinystr!(8, "uschi")),
            ///     UtcOffset::from_eighths_of_hour(-5 * 8),
            /// )
            /// .at_time((Date::try_new_iso(2022, 8, 29).unwrap(), Time::midnight()))
            /// .with_zone_variant(ZoneVariant::daylight());
            ///
            /// assert_try_writeable_eq!(
            ///     fmt.format(&zone),
            #[doc = concat!("    \"", $sample, "\"")]
            /// );
            /// ```
            $(#[$attr])*
            $type,
            sample_length: $sample_length,
        );
        impl UnstableSealed for $type {}
        impl DateTimeNamesMarker for $type {
            type YearNames = datetime_marker_helper!(@names/year,);
            type MonthNames = datetime_marker_helper!(@names/month,);
            type WeekdayNames = datetime_marker_helper!(@names/weekday,);
            type DayPeriodNames = datetime_marker_helper!(@names/dayperiod,);
            type ZoneEssentials = datetime_marker_helper!(@names/zone/essentials, $($zone_essentials_yes)?);
            type ZoneLocations = datetime_marker_helper!(@names/zone/locations, $($zone_locations_yes)?);
            type ZoneGenericLong = datetime_marker_helper!(@names/zone/generic_long, $($zone_generic_long_yes)?);
            type ZoneGenericShort = datetime_marker_helper!(@names/zone/generic_short, $($zone_generic_short_yes)?);
            type ZoneSpecificLong = datetime_marker_helper!(@names/zone/specific_long, $($zone_specific_long_yes)?);
            type ZoneSpecificShort = datetime_marker_helper!(@names/zone/specific_short, $($zone_specific_short_yes)?);
            type MetazoneLookup = datetime_marker_helper!(@names/zone/metazone_periods, $($metazone_periods_yes)?);
        }
        impl HasConstZoneComponent for $type {
            const COMPONENT: NeoTimeZoneStyle = $components;
        }
        impl ZoneMarkers for $type {
            type TimeZoneIdInput = datetime_marker_helper!(@input/timezone/id, $($tzid_input_yes)?);
            type TimeZoneOffsetInput = datetime_marker_helper!(@input/timezone/offset, yes);
            type TimeZoneVariantInput = datetime_marker_helper!(@input/timezone/variant, $($variant_input_yes)?);
            type TimeZoneLocalTimeInput = datetime_marker_helper!(@input/timezone/local_time, $($localtime_input_yes)?);
            type EssentialsV1Marker = datetime_marker_helper!(@data/zone/essentials, $($zone_essentials_yes)?);
            type LocationsV1Marker = datetime_marker_helper!(@data/zone/locations, $($zone_locations_yes)?);
            type GenericLongV1Marker = datetime_marker_helper!(@data/zone/generic_long, $($zone_generic_long_yes)?);
            type GenericShortV1Marker = datetime_marker_helper!(@data/zone/generic_short, $($zone_generic_short_yes)?);
            type SpecificLongV1Marker = datetime_marker_helper!(@data/zone/specific_long, $($zone_specific_long_yes)?);
            type SpecificShortV1Marker = datetime_marker_helper!(@data/zone/specific_short, $($zone_specific_short_yes)?);
            type MetazonePeriodV1Marker = datetime_marker_helper!(@data/zone/metazone_periods, $($metazone_periods_yes)?);
        }
        impl DateTimeMarkers for $type {
            type D = NeoNeverMarker;
            type T = NeoNeverMarker;
            type Z = Self;
            type LengthOption = datetime_marker_helper!(@option/length, yes);
            type AlignmentOption = datetime_marker_helper!(@option/alignment,);
            type YearStyleOption = datetime_marker_helper!(@option/yearstyle,);
            type FractionalSecondDigitsOption = datetime_marker_helper!(@option/fractionalsecondigits,);
            type GluePatternV1Marker = datetime_marker_helper!(@glue,);
        }
        impl HasConstComponents for $type {
            const COMPONENTS: NeoComponents = NeoComponents::Zone($components);
        }
    };
}

macro_rules! impl_datetime_marker {
    (
        $type:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        date = $date:path,
        time = $time:path,
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
        ///
        /// # Examples
        ///
        /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
        ///
        /// ```
        /// use icu::calendar::DateTime;
        /// use icu::datetime::DateTimeFormatter;
        #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_iso(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&dt),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        ///
        /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
        ///
        /// ```
        /// use icu::calendar::DateTime;
        /// use icu::calendar::Gregorian;
        /// use icu::datetime::FixedCalendarDateTimeFormatter;
        #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en").into(),
        #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
        /// )
        /// .unwrap();
        /// let dt = DateTime::try_new_gregorian(2024, 5, 17, 15, 47, 50).unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&dt),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        pub type $type = DateTimeCombo<$date, $time, NeoNeverMarker>;
    }
}

macro_rules! impl_zoneddatetime_marker {
    (
        $type:ident,
        description = $description:literal,
        sample_length = $sample_length:ident,
        sample = $sample:literal,
        date = $date:path,
        time = $time:path,
        zone = $zone:path,
    ) => {
        #[doc = concat!("**“", $sample, "**” ⇒ ", $description)]
        ///
        /// # Examples
        ///
        /// In [`DateTimeFormatter`](crate::neo::DateTimeFormatter):
        ///
        /// ```
        /// use icu::calendar::{Date, Time};
        /// use icu::timezone::{TimeZoneInfo, CustomZonedDateTime};
        /// use icu::datetime::DateTimeFormatter;
        #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = DateTimeFormatter::<", stringify!($type), ">::try_new(")]
        ///     &locale!("en-GB").into(),
        #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// let mut dtz = CustomZonedDateTime::try_from_str("2024-05-17T15:47:50+01:00[Europe/London]").unwrap();
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.convert_and_format(&dtz),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        ///
        /// In [`FixedCalendarDateTimeFormatter`](crate::neo::FixedCalendarDateTimeFormatter):
        ///
        /// ```
        /// use icu::calendar::{Date, Time};
        /// use icu::timezone::{TimeZoneInfo, CustomZonedDateTime};
        /// use icu::calendar::Gregorian;
        /// use icu::datetime::FixedCalendarDateTimeFormatter;
        #[doc = concat!("use icu::datetime::fieldset::", stringify!($type), ";")]
        /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
        /// use icu::locale::locale;
        /// use writeable::assert_try_writeable_eq;
        ///
        #[doc = concat!("let fmt = FixedCalendarDateTimeFormatter::<Gregorian, ", stringify!($type), ">::try_new(")]
        ///     &locale!("en-GB").into(),
        #[doc = concat!("    ", length_option_helper!($type, $sample_length), ",")]
        /// )
        /// .unwrap();
        ///
        /// let mut dtz = CustomZonedDateTime::try_from_str("2024-05-17T15:47:50+01:00[Europe/London]")
        ///     .unwrap()
        ///     .to_calendar(Gregorian);
        ///
        /// assert_try_writeable_eq!(
        ///     fmt.format(&dtz),
        #[doc = concat!("    \"", $sample, "\"")]
        /// );
        /// ```
        pub type $type = DateTimeCombo<$date, $time, $zone>;
    }
}

impl_date_marker!(
    NeoYearMonthDayMarker,
    NeoDateComponents::YearMonthDay,
    description = "year, month, and day (year might be abbreviated)",
    sample_length = Short,
    sample = "5/17/24",
    years = yes,
    months = yes,
    weekdays = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    NeoMonthDayMarker,
    NeoDateComponents::MonthDay,
    description = "month and day",
    sample_length = Medium,
    sample = "May 17",
    months = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_date_marker!(
    NeoAutoDateMarker,
    NeoDateComponents::Auto,
    description = "locale-dependent date fields",
    sample_length = Medium,
    sample = "May 17, 2024",
    years = yes,
    months = yes,
    weekdays = yes,
    input_year = yes,
    input_month = yes,
    input_day_of_month = yes,
    input_day_of_week = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_time_marker!(
    NeoHourMinuteMarker,
    NeoTimeComponents::HourMinute,
    description = "hour and minute (locale-dependent hour cycle)",
    sample_length = Medium,
    sample = "3:47 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
);

impl_time_marker!(
    NeoHourMinuteSecondMarker,
    NeoTimeComponents::HourMinuteSecond,
    description = "hour, minute, and second (locale-dependent hour cycle)",
    sample_length = Medium,
    sample = "3:47:50 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_nanosecond = yes,
);

impl_time_marker!(
    NeoAutoTimeMarker,
    NeoTimeComponents::Auto,
    description = "locale-dependent time fields",
    sample_length = Medium,
    sample = "3:47:50 PM",
    dayperiods = yes,
    input_hour = yes,
    input_minute = yes,
    input_second = yes,
    input_nanosecond = yes,
);

// TODO: Make NeoAutoZoneMarker, derived from time length patterns

impl_datetime_marker!(
    NeoAutoDateTimeMarker,
    description = "locale-dependent date and time fields",
    sample_length = Medium,
    sample = "May 17, 2024, 3:47:50 PM",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_datetime_marker!(
    NeoYearMonthDayHourMinuteMarker,
    description = "locale-dependent date and time fields",
    sample_length = Medium,
    sample = "May 17, 2024, 3:47 PM",
    date = NeoYearMonthDayMarker,
    time = NeoHourMinuteMarker,
);

impl_calendar_period_marker!(
    NeoYearMonthMarker,
    NeoCalendarPeriodComponents::YearMonth,
    description = "year and month (era elided when possible)",
    sample_length = Medium,
    sample = "May 2024",
    years = yes,
    months = yes,
    input_year = yes,
    input_month = yes,
    input_any_calendar_kind = yes,
    option_alignment = yes,
);

impl_zone_marker!(
    /// When a display name is unavailable, falls back to the offset format:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneSpecificMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///     &locale!("en").into(),
    ///     NeoTimeZoneSpecificMarker::with_length(NeoSkeletonLength::Short),
    /// )
    /// .unwrap();
    ///
    /// // Time zone info for America/Sao_Paulo in the winter
    /// let zone = TimeZoneInfo::from_id_and_offset(
    ///     TimeZoneBcp47Id(tinystr!(8, "brsao")),
    ///     UtcOffset::from_eighths_of_hour(-3 * 8),
    /// )
    /// .at_time((Date::try_new_iso(2022, 1, 29).unwrap(), Time::midnight()))
    /// .with_zone_variant(ZoneVariant::standard());
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.format(&zone),
    ///     "GMT-3"
    /// );
    /// ```
    ///
    /// Only a full time zone info can be formatted with this style.
    /// For example, AtTime cannot be formatted.
    ///
    /// ```compile_fail
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneSpecificMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let datetime = DateTime::try_new_gregorian(2024, 10, 18, 0, 0, 0).unwrap();
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    /// let time_zone_basic = utc_offset.with_id(TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// let time_zone_at_time = time_zone_basic.at_time((datetime.date.to_iso(), datetime.time));
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneSpecificMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// // error[E0271]: type mismatch resolving `<AtTime as TimeZoneModel>::ZoneVariant == ZoneVariant`
    /// // note: required by a bound in `FixedCalendarDateTimeFormatter::<C, FSet>::format`
    /// formatter.format(&time_zone_at_time);
    /// ```
    NeoTimeZoneSpecificMarker,
    NeoTimeZoneStyle::Specific,
    description = "specific time zone, or raw offset if unavailable",
    sample_length = Long,
    sample = "Central Daylight Time",
    zone_essentials = yes,
    zone_specific_long = yes,
    zone_specific_short = yes,
    metazone_periods = yes,
    input_tzid = yes,
    input_variant = yes,
    input_localtime = yes,
);

impl_zone_marker!(
    /// This marker only loads data for the short length. Useful when combined with other fields:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{TimeZoneInfo, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldset::NeoMonthDayMarker;
    /// use icu::datetime::fieldset::NeoHourMinuteMarker;
    /// use icu::datetime::fieldset::NeoTimeZoneSpecificShortMarker;
    /// use icu::datetime::DateTimeCombo;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// type MyDateTimeZoneSet = DateTimeCombo<
    ///     NeoMonthDayMarker,
    ///     NeoHourMinuteMarker,
    ///     NeoTimeZoneSpecificShortMarker,
    /// >;
    ///
    /// let fmt = DateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     MyDateTimeZoneSet::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap();
    ///
    /// let dtz = CustomZonedDateTime::try_from_str("2024-09-17T15:47:50-05:00[America/Chicago]").unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dtz),
    ///     "September 17, 3:47 PM CDT"
    /// );
    /// ```
    ///
    /// Don't use long length if it is the only field:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneSpecificShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::LoadError;
    /// use icu::locale::locale;
    ///
    /// let result = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///     &locale!("en").into(),
    ///     NeoTimeZoneSpecificShortMarker::with_length(NeoSkeletonLength::Long),
    /// );
    ///
    /// assert!(matches!(result, Err(LoadError::TypeTooNarrow(_))));
    /// ```
    ///
    /// Only a full time zone info can be formatted with this style.
    /// For example, AtTime cannot be formatted.
    ///
    /// ```compile_fail
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneSpecificShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let datetime = DateTime::try_new_gregorian(2024, 10, 18, 0, 0, 0).unwrap();
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    /// let time_zone_basic = utc_offset.with_id(TimeZoneBcp47Id(tinystr!(8, "uschi")));
    /// let time_zone_at_time = time_zone_basic.at_time((datetime.date.to_iso(), datetime.time));
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneSpecificShortMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// // error[E0271]: type mismatch resolving `<AtTime as TimeZoneModel>::ZoneVariant == ZoneVariant`
    /// // note: required by a bound in `FixedCalendarDateTimeFormatter::<C, FSet>::format`
    /// formatter.format(&time_zone_at_time);
    /// ```
    NeoTimeZoneSpecificShortMarker,
    NeoTimeZoneStyle::Specific,
    description = "specific time zone (only short), or raw offset if unavailable",
    sample_length = Short,
    sample = "CDT",
    zone_essentials = yes,
    zone_specific_short = yes,
    metazone_periods = yes,
    input_tzid = yes,
    input_variant = yes,
    input_localtime = yes,
);

impl_zone_marker!(
    /// All shapes of time zones can be formatted with this style.
    ///
    /// ```
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneOffsetMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant, CustomZonedDateTime};
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let datetime = DateTime::try_new_gregorian(2024, 10, 18, 0, 0, 0).unwrap();
    ///
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    ///
    /// let zdt_utc_offset = CustomZonedDateTime {
    ///     date: datetime.date,
    ///     time: datetime.time,
    ///     zone: utc_offset
    /// };
    ///
    /// let time_zone_basic = utc_offset.with_id(TimeZoneBcp47Id(tinystr!(8, "uschi")));
    ///
    /// let zdt_time_zone_basic = CustomZonedDateTime {
    ///     date: datetime.date,
    ///     time: datetime.time,
    ///     zone: time_zone_basic
    /// };
    ///
    /// let time_zone_at_time = time_zone_basic.at_time((datetime.date.to_iso(), datetime.time));
    ///
    /// let zdt_time_zone_at_time = CustomZonedDateTime {
    ///     date: datetime.date,
    ///     time: datetime.time,
    ///     zone: time_zone_at_time
    /// };
    ///
    /// let time_zone_full = time_zone_at_time.with_zone_variant(ZoneVariant::standard());
    ///
    /// let zdt_time_zone_full = CustomZonedDateTime {
    ///     date: datetime.date,
    ///     time: datetime.time,
    ///     zone: time_zone_full
    /// };
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneOffsetMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&utc_offset),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&zdt_utc_offset),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&time_zone_basic),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&zdt_time_zone_basic),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&time_zone_at_time),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&zdt_time_zone_at_time),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&time_zone_full),
    ///     "GMT-6"
    /// );
    ///
    /// assert_try_writeable_eq!(
    ///     formatter.format(&zdt_time_zone_full),
    ///     "GMT-6"
    /// );
    /// ```
    NeoTimeZoneOffsetMarker,
    NeoTimeZoneStyle::Offset,
    description = "UTC offset time zone",
    sample_length = Medium,
    sample = "GMT-5",
    zone_essentials = yes,
);

impl_zone_marker!(
    /// When a display name is unavailable, falls back to the location format:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{CustomZonedDateTime, TimeZoneBcp47Id, TimeZoneInfo, UtcOffset, ZoneVariant};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneGenericMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let fmt = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///     &locale!("en").into(),
    ///     NeoTimeZoneGenericMarker::with_length(NeoSkeletonLength::Short),
    /// )
    /// .unwrap();
    ///
    /// // Time zone info for America/Sao_Paulo in the winter
    /// let zone = TimeZoneInfo::from_id_and_offset(
    ///     TimeZoneBcp47Id(tinystr!(8, "brsao")),
    ///     UtcOffset::from_eighths_of_hour(-3 * 8),
    /// )
    /// .at_time((Date::try_new_iso(2022, 1, 29).unwrap(), Time::midnight()))
    /// .with_zone_variant(ZoneVariant::standard());
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.format(&zone),
    ///     "Sao Paulo Time"
    /// );
    /// ```
    ///
    /// A time zone requires a reference time to be formatted with this style.
    ///
    /// ```compile_fail
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneGenericMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::{TimeZoneBcp47Id, UtcOffset};
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    /// let time_zone_basic = utc_offset.with_id(TimeZoneBcp47Id(tinystr!(8, "uschi")));
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneGenericMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::LocalTime == (Date<Iso>, Time)`
    /// // note: required by a bound in `FixedCalendarDateTimeFormatter::<C, FSet>::format`
    /// formatter.format(&time_zone_basic);
    /// ```
    NeoTimeZoneGenericMarker,
    NeoTimeZoneStyle::Generic,
    description = "generic time zone, or location if unavailable",
    sample_length = Long,
    sample = "Central Time",
    zone_essentials = yes,
    zone_locations = yes,
    zone_generic_long = yes,
    zone_generic_short = yes,
    metazone_periods = yes,
    input_tzid = yes,
    input_localtime = yes,
);

impl_zone_marker!(
    /// This marker only loads data for the short length. Useful when combined with other fields:
    ///
    /// ```
    /// use icu::calendar::{Date, Time};
    /// use icu::timezone::{TimeZoneInfo, CustomZonedDateTime};
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldset::NeoMonthDayMarker;
    /// use icu::datetime::fieldset::NeoHourMinuteMarker;
    /// use icu::datetime::fieldset::NeoTimeZoneGenericShortMarker;
    /// use icu::datetime::DateTimeCombo;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::locale::locale;
    /// use tinystr::tinystr;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// type MyDateTimeZoneSet = DateTimeCombo<
    ///     NeoMonthDayMarker,
    ///     NeoHourMinuteMarker,
    ///     NeoTimeZoneGenericShortMarker,
    /// >;
    ///
    /// let fmt = DateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     MyDateTimeZoneSet::with_length(NeoSkeletonLength::Long),
    /// )
    /// .unwrap();
    ///
    /// let dtz = CustomZonedDateTime::try_from_str("2024-09-17T15:47:50-05:00[America/Chicago]").unwrap();
    ///
    /// assert_try_writeable_eq!(
    ///     fmt.convert_and_format(&dtz),
    ///     "September 17, 3:47 PM CT"
    /// );
    /// ```
    ///
    /// Don't use long length if it is the only field:
    ///
    /// ```
    /// use icu::calendar::Gregorian;
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneGenericShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::datetime::LoadError;
    /// use icu::locale::locale;
    ///
    /// let result = FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
    ///     &locale!("en").into(),
    ///     NeoTimeZoneGenericShortMarker::with_length(NeoSkeletonLength::Long),
    /// );
    ///
    /// assert!(matches!(result, Err(LoadError::TypeTooNarrow(_))));
    /// ```
    ///
    /// A time zone requires a reference time to be formatted with this style.
    ///
    /// ```compile_fail
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneGenericShortMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::{TimeZoneBcp47Id, UtcOffset};
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    /// let time_zone_basic = utc_offset.with_id(TimeZoneBcp47Id(tinystr!(8, "uschi")));
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneGenericShortMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// // error[E0271]: type mismatch resolving `<Base as TimeZoneModel>::LocalTime == (Date<Iso>, Time)`
    /// // note: required by a bound in `FixedCalendarDateTimeFormatter::<C, FSet>::format`
    /// formatter.format(&time_zone_basic);
    /// ```
    NeoTimeZoneGenericShortMarker,
    NeoTimeZoneStyle::Generic,
    description = "generic time zone (only short), or location if unavailable",
    sample_length = Short,
    sample = "CT",
    zone_essentials = yes,
    zone_locations = yes,
    zone_generic_short = yes,
    metazone_periods = yes,
    input_tzid = yes,
    input_localtime = yes,
);

impl_zone_marker!(
    /// A time zone requires a time zone ID to be formatted with this style.
    /// For example, a raw [`UtcOffset`] cannot be used here.
    ///
    /// ```compile_fail
    /// use icu::calendar::{DateTime, Iso};
    /// use icu::datetime::FixedCalendarDateTimeFormatter;
    /// use icu::datetime::fieldset::NeoTimeZoneLocationMarker;
    /// use icu::datetime::neo_skeleton::NeoSkeletonLength;
    /// use icu::timezone::UtcOffset;
    /// use tinystr::tinystr;
    /// use icu::locale::locale;
    /// use writeable::assert_try_writeable_eq;
    ///
    /// let utc_offset = UtcOffset::from_eighths_of_hour(-6 * 8);
    ///
    /// let formatter = FixedCalendarDateTimeFormatter::try_new(
    ///     &locale!("en-US").into(),
    ///     NeoTimeZoneLocationMarker::with_length(NeoSkeletonLength::Medium),
    /// )
    /// .unwrap();
    ///
    /// // error[E0277]: the trait bound `UtcOffset: AllInputMarkers<NeoTimeZoneLocationMarker>` is not satisfied
    /// // note: required by a bound in `FixedCalendarDateTimeFormatter::<C, FSet>::format`
    /// formatter.format(&utc_offset);
    /// ```
    NeoTimeZoneLocationMarker,
    NeoTimeZoneStyle::Location,
    description = "location time zone",
    sample_length = Long,
    sample = "Chicago Time",
    zone_essentials = yes,
    zone_locations = yes,
    input_tzid = yes,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_zoneddatetime_marker!(
    NeoYearMonthDayHourMinuteSecondTimeZoneGenericShortMarker,
    description = "locale-dependent date and time fields with a time zone",
    sample_length = Medium,
    sample = "17 May 2024, 15:47:50 GMT",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
    zone = NeoTimeZoneGenericMarker,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_zoneddatetime_marker!(
    NeoYearMonthDayHourMinuteSecondTimeZoneSpecificShortMarker,
    description = "locale-dependent date and time fields with a time zone",
    sample_length = Medium,
    sample = "17 May 2024, 15:47:50 BST",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
    zone = NeoTimeZoneSpecificMarker,
);

// TODO: Type aliases like this are excessive; make a curated set
impl_zoneddatetime_marker!(
    NeoYearMonthDayHourMinuteSecondTimeZoneOffsetMarker,
    description = "locale-dependent date and time fields with a time zone",
    sample_length = Medium,
    sample = "17 May 2024, 15:47:50 GMT+1",
    date = NeoAutoDateMarker,
    time = NeoAutoTimeMarker,
    zone = NeoTimeZoneOffsetMarker,
);
