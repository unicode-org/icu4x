// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for managing data needed by [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter).

use crate::fields;
use crate::input;
use crate::options::{length, preferences, DateTimeFormatterOptions};
use crate::pattern::PatternError;
use crate::pattern::{hour_cycle, runtime::PatternPlurals};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsV1;
use crate::provider::calendar::{months, DateLengthsV1, TimeLengthsV1};
use crate::provider::calendar::{
    patterns::GenericPatternV1Marker, patterns::PatternPluralsFromPatternsV1Marker,
    ErasedDateLengthsV1Marker, TimeLengthsV1Marker,
};
#[cfg(feature = "experimental")]
use crate::provider::neo::SimpleSubstitutionPattern;
use crate::time_zone::TimeZoneDataPayloadsBorrowed;
#[cfg(feature = "experimental")]
use crate::{fields::Field, options::components, provider::calendar::DateSkeletonPatternsV1Marker};
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;
use icu_locale_core::extensions::unicode::Value;
use icu_provider::prelude::*;

pub(crate) enum GetSymbolForMonthError {
    Missing,
    #[cfg(feature = "experimental")]
    MissingNames(Field),
}
pub(crate) enum GetSymbolForWeekdayError {
    Missing,
    #[cfg(feature = "experimental")]
    MissingNames(Field),
}

pub(crate) enum GetSymbolForEraError {
    Missing,
    #[cfg(feature = "experimental")]
    MissingNames(Field),
}

pub(crate) enum GetSymbolForDayPeriodError {
    #[cfg(feature = "experimental")]
    MissingNames(Field),
}

#[cfg(feature = "experimental")]
pub(crate) enum UnsupportedOptionsOrDataError {
    UnsupportedOptions,
    Data(DataError),
}

#[cfg(feature = "experimental")]
pub(crate) enum UnsupportedOptionsOrDataOrPatternError {
    UnsupportedOptions,
    Data(DataError),
    Pattern(PatternError),
}

fn pattern_for_time_length_inner<'data>(
    data: TimeLengthsV1<'data>,
    length: length::Time,
    preferences: &Option<preferences::Bag>,
) -> PatternPlurals<'data> {
    // Determine the coarse hour cycle patterns to use from either the preference bag,
    // or the preferred hour cycle for the locale.
    let time = if let Some(preferences::Bag {
        hour_cycle: Some(hour_cycle_pref),
    }) = preferences
    {
        match hour_cycle_pref {
            preferences::HourCycle::H11 | preferences::HourCycle::H12 => data.time_h11_h12,
            preferences::HourCycle::H23 | preferences::HourCycle::H24 => data.time_h23_h24,
        }
    } else {
        match data.preferred_hour_cycle {
            crate::pattern::CoarseHourCycle::H11H12 => data.time_h11_h12,
            crate::pattern::CoarseHourCycle::H23H24 => data.time_h23_h24,
        }
    };

    let mut pattern = match length {
        length::Time::Full => time.full,
        length::Time::Long => time.long,
        length::Time::Medium => time.medium,
        length::Time::Short => time.short,
    };

    hour_cycle::naively_apply_preferences(&mut pattern, preferences);
    PatternPlurals::from(pattern)
}

fn pattern_for_date_length_inner(data: DateLengthsV1, length: length::Date) -> PatternPlurals {
    let pattern = match length {
        length::Date::Full => data.date.full,
        length::Date::Long => data.date.long,
        length::Date::Medium => data.date.medium,
        length::Date::Short => data.date.short,
    };
    PatternPlurals::from(pattern)
}

/// Determine the appropriate `Pattern` for a given `options::length::Time` bag.
/// If a preference for an hour cycle is set, it will look look up a pattern in the `time_h11_12` or
/// `time_h23_h24` provider data, and then manually modify the symbol in the pattern if needed.
pub(crate) fn pattern_for_time_length<'a, D>(
    data_provider: &'a D,
    locale: &'a DataLocale,
    length: length::Time,
    preferences: Option<preferences::Bag>,
) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>, DataError>
where
    D: DataProvider<TimeLengthsV1Marker> + ?Sized,
{
    Ok(data_provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        })?
        .payload
        .map_project(|data, _| pattern_for_time_length_inner(data, length, &preferences).into()))
}

/// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
pub(crate) fn pattern_for_date_length(
    length: length::Date,
    patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
) -> DataPayload<PatternPluralsFromPatternsV1Marker> {
    patterns_data.map_project(|data, _| pattern_for_date_length_inner(data, length).into())
}

/// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
pub(crate) fn generic_pattern_for_date_length(
    length: length::Date,
    patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
) -> DataPayload<GenericPatternV1Marker> {
    patterns_data.map_project(|data, _| {
        match length {
            length::Date::Full => data.length_combinations.full,
            length::Date::Long => data.length_combinations.long,
            length::Date::Medium => data.length_combinations.medium,
            length::Date::Short => data.length_combinations.short,
        }
        .into()
    })
}

/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone)]
pub struct PatternSelector<'a, D: ?Sized> {
    data_provider: &'a D,
    date_patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
    locale: &'a DataLocale,
    #[allow(dead_code)] // non-experimental mode
    cal_val: Option<&'a Value>,
}

pub(crate) enum PatternForLengthError {
    Pattern(PatternError),
    Data(DataError),
}

impl<D> PatternSelector<'_, D>
where
    D: DataProvider<TimeLengthsV1Marker> + ?Sized,
{
    pub(crate) fn for_options<'a>(
        data_provider: &'a D,
        date_patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
        locale: &'a DataLocale,
        options: &DateTimeFormatterOptions,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>, PatternForLengthError> {
        let selector = PatternSelector {
            data_provider,
            date_patterns_data,
            locale,
            cal_val: None,
        };
        match options {
            DateTimeFormatterOptions::Length(bag) => selector
                .pattern_for_length_bag(bag, Some(preferences::Bag::from_data_locale(locale))),
            #[cfg(any(feature = "datagen", feature = "experimental"))]
            #[allow(clippy::panic)] // explicit panic in experimental mode
            _ => panic!("Non-experimental constructor cannot handle experimental options"),
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::Length` bag.
    fn pattern_for_length_bag(
        self,
        length: &length::Bag,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>, PatternForLengthError> {
        match (length.date, length.time) {
            (None, None) => Ok(DataPayload::from_owned(PatternPluralsV1(
                PatternPlurals::default(),
            ))),
            (None, Some(time_length)) => {
                pattern_for_time_length(self.data_provider, self.locale, time_length, preferences)
                    .map_err(PatternForLengthError::Data)
            }
            (Some(date_length), None) => Ok(pattern_for_date_length(
                date_length,
                self.date_patterns_data,
            )),
            (Some(date_length), Some(time_length)) => {
                self.pattern_for_datetime_length(date_length, time_length, preferences)
            }
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` and
    /// `options::length::Time` bag.
    fn pattern_for_datetime_length(
        self,
        date_length: length::Date,
        time_length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>, PatternForLengthError> {
        let time_patterns_data = self
            .data_provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(self.locale),
                ..Default::default()
            })
            .map_err(PatternForLengthError::Data)?
            .payload;

        self.date_patterns_data.try_map_project(|data, _| {
            // TODO (#1131) - We may be able to remove the clone here.
            let date = pattern_for_date_length_inner(data.clone(), date_length)
                .expect_pattern("Lengths are single patterns");

            let pattern = match date_length {
                length::Date::Full => data.length_combinations.full,
                length::Date::Long => data.length_combinations.long,
                length::Date::Medium => data.length_combinations.medium,
                length::Date::Short => data.length_combinations.short,
            };

            // TODO (#1131) - We may be able to remove the clone here.
            let time = pattern_for_time_length_inner(
                time_patterns_data.get().clone(),
                time_length,
                &preferences,
            )
            .expect_pattern("Lengths are single patterns");
            Ok(PatternPlurals::from(
                pattern
                    .combined(date, time)
                    .map_err(PatternForLengthError::Pattern)?,
            )
            .into())
        })
    }
}

#[cfg(feature = "experimental")]
impl<D> PatternSelector<'_, D>
where
    D: DataProvider<TimeLengthsV1Marker> + DataProvider<DateSkeletonPatternsV1Marker> + ?Sized,
{
    pub(crate) fn for_options_experimental<'a>(
        data_provider: &'a D,
        date_patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
        locale: &'a DataLocale,
        cal_val: &'a Value,
        options: &DateTimeFormatterOptions,
    ) -> Result<
        DataPayload<PatternPluralsFromPatternsV1Marker>,
        UnsupportedOptionsOrDataOrPatternError,
    > {
        let selector = PatternSelector {
            data_provider,
            date_patterns_data,
            locale,
            cal_val: Some(cal_val),
        };
        match options {
            DateTimeFormatterOptions::Length(bag) => selector
                .pattern_for_length_bag(bag, Some(preferences::Bag::from_data_locale(locale)))
                .map_err(|e| match e {
                    PatternForLengthError::Data(e) => {
                        UnsupportedOptionsOrDataOrPatternError::Data(e)
                    }
                    PatternForLengthError::Pattern(e) => {
                        UnsupportedOptionsOrDataOrPatternError::Pattern(e)
                    }
                }),
            DateTimeFormatterOptions::Components(bag) => selector
                .patterns_for_components_bag(bag)
                .map_err(|e| match e {
                    UnsupportedOptionsOrDataError::Data(e) => {
                        UnsupportedOptionsOrDataOrPatternError::Data(e)
                    }
                    UnsupportedOptionsOrDataError::UnsupportedOptions => {
                        UnsupportedOptionsOrDataOrPatternError::UnsupportedOptions
                    }
                }),
        }
    }

    /// Determine the appropriate `PatternPlurals` for a given `options::components::Bag`.
    fn patterns_for_components_bag(
        self,
        components: &components::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>, UnsupportedOptionsOrDataError>
    {
        use crate::skeleton;
        let skeletons_data = self
            .skeleton_data_payload()
            .map_err(UnsupportedOptionsOrDataError::Data)?;
        // Not all skeletons are currently supported.
        // TODO(#594) - This should default should be the locale default, which is
        // region-based (h12 for US, h23 for GB, etc). This is in CLDR, but we need
        // to load it as well as think about the best architecture for where that
        // data loading code should reside.
        let requested_fields = components.to_vec_fields(preferences::HourCycle::H23);
        let patterns = match skeleton::create_best_pattern_for_fields(
            skeletons_data.get(),
            &self.date_patterns_data.get().length_combinations,
            &requested_fields,
            components,
            false, // Prefer the requested fields over the matched pattern.
        ) {
            skeleton::BestSkeleton::AllFieldsMatch(pattern)
            | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => Ok(pattern),
            skeleton::BestSkeleton::NoMatch => {
                Err(UnsupportedOptionsOrDataError::UnsupportedOptions)
            }
        }?;
        Ok(DataPayload::from_owned(PatternPluralsV1(
            patterns.into_owned(),
        )))
    }

    fn skeleton_data_payload(
        &self,
    ) -> Result<DataPayload<DateSkeletonPatternsV1Marker>, DataError> {
        #![allow(clippy::expect_used)] // experimental
        use icu_locale_core::{extensions::unicode::value, subtags::subtag};
        let cal_val = self.cal_val.expect("should be present for components bag");
        // Skeleton data for ethioaa is stored under ethiopic
        let cal = if cal_val == &value!("ethioaa") {
            "ethiopic"
        } else if cal_val == &value!("islamicc")
            || cal_val.get_subtag(0) == Some(&subtag!("islamic"))
        {
            // All islamic calendars store skeleton data under islamic, not their individual extension keys
            "islamic"
        } else {
            cal_val.as_single_subtag().expect("single").as_str()
        };

        self.data_provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic(cal),
                    self.locale,
                ),
                ..Default::default()
            })
            .map(|r| r.payload)
    }
}

/// Internal enum to represent the kinds of month symbols for interpolation
pub(crate) enum MonthPlaceholderValue<'a> {
    PlainString(&'a str),
    StringNeedingLeapPrefix(&'a str),
    #[cfg(feature = "experimental")]
    Numeric,
    #[cfg(feature = "experimental")]
    NumericPattern(&'a SimpleSubstitutionPattern<'a>),
}

pub(crate) trait DateSymbols<'data> {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetSymbolForMonthError>;

    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetSymbolForWeekdayError>;

    /// Gets the era symbol, or `None` if data is loaded but symbol isn't found.
    ///
    /// `None` should fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    fn get_symbol_for_era<'a>(
        &'a self,
        length: fields::FieldLength,
        era_code: &'a Era,
    ) -> Result<&str, GetSymbolForEraError>;
}

impl<'data> provider::calendar::DateSymbolsV1<'data> {
    fn get_symbols_map_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
    ) -> Result<&months::SymbolsV1<'data>, core::convert::Infallible> {
        let widths = match month {
            fields::Month::Format => &self.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return Ok(symbols);
                    } else {
                        return self.get_symbols_map_for_month(fields::Month::Format, length);
                    }
                } else {
                    return self.get_symbols_map_for_month(fields::Month::Format, length);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        Ok(symbols)
    }
}

impl<'data> DateSymbols<'data> for provider::calendar::DateSymbolsV1<'data> {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, GetSymbolForWeekdayError> {
        let widths = match weekday {
            fields::Weekday::Format => &self.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => {
                            widths.short.as_ref().or(widths.abbreviated.as_ref())
                        }
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        let idx = (day as usize) % 7;
                        return symbols
                            .0
                            .get(idx)
                            .map(|x| &**x)
                            .ok_or(GetSymbolForWeekdayError::Missing);
                    } else {
                        return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                    }
                } else {
                    return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                }
            }
            fields::Weekday::Local => unimplemented!(),
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            fields::FieldLength::Six => widths.short.as_ref().unwrap_or(&widths.abbreviated),
            _ => &widths.abbreviated,
        };
        let idx = (day as usize) % 7;
        symbols
            .0
            .get(idx)
            .map(|x| &**x)
            .ok_or(GetSymbolForWeekdayError::Missing)
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, GetSymbolForMonthError> {
        let symbols_map = self.get_symbols_map_for_month(month, length).unwrap();
        let mut symbol_option = symbols_map.get(code);
        let mut fallback = false;
        if symbol_option.is_none() {
            if let Some(code) = code.get_normal_if_leap() {
                let symbols_map = self.get_symbols_map_for_month(month, length).unwrap();
                symbol_option = symbols_map.get(code);
                fallback = true;
            }
        }
        let symbol = symbol_option.ok_or(GetSymbolForMonthError::Missing)?;
        Ok(if fallback {
            MonthPlaceholderValue::StringNeedingLeapPrefix(symbol)
        } else {
            MonthPlaceholderValue::PlainString(symbol)
        })
    }

    fn get_symbol_for_era<'a>(
        &'a self,
        length: fields::FieldLength,
        era_code: &'a Era,
    ) -> Result<&str, GetSymbolForEraError> {
        let symbols = match length {
            fields::FieldLength::Wide => &self.eras.names,
            fields::FieldLength::Narrow => &self.eras.narrow,
            _ => &self.eras.abbr,
        };
        symbols
            .get(era_code.0.as_str().into())
            .ok_or(GetSymbolForEraError::Missing)
    }
}

pub(crate) trait TimeSymbols {
    /// Gets the day period symbol.
    ///
    /// Internally, 'noon' and 'midnight' should fall back to 'am' and 'pm'.
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetSymbolForDayPeriodError>;
}

impl<'data> TimeSymbols for provider::calendar::TimeSymbolsV1<'data> {
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, GetSymbolForDayPeriodError> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        Ok(match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        })
    }
}

pub(crate) trait ZoneSymbols<'data> {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data>;
}

impl<'data> ZoneSymbols<'data> for () {
    fn get_payloads(&self) -> TimeZoneDataPayloadsBorrowed<'data> {
        TimeZoneDataPayloadsBorrowed::empty()
    }
}
