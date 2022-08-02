// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DateTimeFormatterError;
use crate::fields;
use crate::input;
use crate::options::{components, length, preferences, DateTimeFormatterOptions};
use crate::pattern::{hour_cycle, runtime::PatternPlurals};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsV1;
use crate::provider::calendar::{
    patterns::GenericPatternV1Marker, patterns::PatternPluralsFromPatternsV1Marker,
    DateLengthsV1Marker, DateSkeletonPatternsV1Marker, TimeLengthsV1Marker,
};
use crate::provider::calendar::{DatePatternsV1, TimeLengthsV1};
use crate::skeleton;
use icu_calendar::types::{Era, MonthCode};
use icu_provider::prelude::*;

type Result<T> = core::result::Result<T, DateTimeFormatterError>;

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

fn time_patterns_data_payload<D>(
    data_provider: &D,
    locale: &DataLocale,
) -> Result<DataPayload<TimeLengthsV1Marker>>
where
    D: DataProvider<TimeLengthsV1Marker> + ?Sized,
{
    let data = data_provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(data)
}

fn pattern_for_date_length_inner(data: DatePatternsV1, length: length::Date) -> PatternPlurals {
    let pattern = match length {
        length::Date::Full => data.date.full,
        length::Date::Long => data.date.long,
        length::Date::Medium => data.date.medium,
        length::Date::Short => data.date.short,
    };
    PatternPlurals::from(pattern)
}

/// Determine the appropriate `Pattern` for a given `options::length::Time` bag.
/// If a preference for an hour cycle is set, it will look look up a pattern in the time_h11_12 or
/// time_h23_h24 provider data, and then manually modify the symbol in the pattern if needed.
pub(crate) fn pattern_for_time_length<'a, D>(
    data_provider: &'a D,
    locale: &'a DataLocale,
    length: length::Time,
    preferences: Option<preferences::Bag>,
) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>>
where
    D: DataProvider<TimeLengthsV1Marker> + ?Sized,
{
    let patterns_data = time_patterns_data_payload(data_provider, locale)?;
    Ok(patterns_data.map_project(|data, _| {
        let pattern = pattern_for_time_length_inner(data, length, &preferences).clone();
        pattern.into()
    }))
}

fn date_patterns_data_payload<D>(
    data_provider: &D,
    locale: &DataLocale,
) -> Result<DataPayload<DateLengthsV1Marker>>
where
    D: DataProvider<DateLengthsV1Marker> + ?Sized,
{
    let data = data_provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(data)
}

/// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
pub(crate) fn pattern_for_date_length<D>(
    data_provider: &D,
    locale: &DataLocale,
    length: length::Date,
) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>>
where
    D: DataProvider<DateLengthsV1Marker> + ?Sized,
{
    let patterns_data = date_patterns_data_payload(data_provider, locale)?;
    Ok(patterns_data.map_project(|data, _| {
        let pattern = pattern_for_date_length_inner(data, length);
        pattern.into()
    }))
}

/// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
pub(crate) fn generic_pattern_for_date_length<D>(
    data_provider: &D,
    locale: &DataLocale,
    length: length::Date,
) -> Result<DataPayload<GenericPatternV1Marker>>
where
    D: DataProvider<DateLengthsV1Marker> + ?Sized,
{
    let patterns_data = date_patterns_data_payload(data_provider, locale)?;
    Ok(patterns_data.map_project(|data, _| {
        let pattern = match length {
            length::Date::Full => data.length_combinations.full,
            length::Date::Long => data.length_combinations.long,
            length::Date::Medium => data.length_combinations.medium,
            length::Date::Short => data.length_combinations.short,
        };

        pattern.into()
    }))
}

pub struct PatternSelector<'a, D: ?Sized> {
    data_provider: &'a D,
    locale: &'a DataLocale,
}

// Manual impls needed since `derive(Copy)` inserts
// a `D: Copy` bound
impl<D: ?Sized> Copy for PatternSelector<'_, D> {}
impl<D: ?Sized> Clone for PatternSelector<'_, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> PatternSelector<'_, D>
where
    D: DataProvider<DateLengthsV1Marker>
        + DataProvider<TimeLengthsV1Marker>
        + DataProvider<DateSkeletonPatternsV1Marker>
        + ?Sized,
{
    pub(crate) fn for_options<'a>(
        data_provider: &'a D,
        locale: &'a DataLocale,
        options: &DateTimeFormatterOptions,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let selector = PatternSelector {
            data_provider,
            locale,
        };
        match options {
            DateTimeFormatterOptions::Length(bag) => selector.pattern_for_length_bag(bag),
            DateTimeFormatterOptions::Components(bag) => selector.patterns_for_components_bag(bag),
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::Length` bag.
    fn pattern_for_length_bag(
        self,
        length: &length::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        match (length.date, length.time) {
            (None, None) => Ok(DataPayload::from_owned(PatternPluralsV1(
                PatternPlurals::default(),
            ))),
            (None, Some(time_length)) => pattern_for_time_length(
                self.data_provider,
                self.locale,
                time_length,
                length.preferences,
            ),
            (Some(date_length), None) => {
                pattern_for_date_length(self.data_provider, self.locale, date_length)
            }
            (Some(date_length), Some(time_length)) => {
                self.pattern_for_datetime_length(date_length, time_length, length.preferences)
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
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let time_patterns_data = time_patterns_data_payload(self.data_provider, self.locale)?;

        let date_patterns_data = date_patterns_data_payload(self.data_provider, self.locale)?;
        date_patterns_data.try_map_project(|data, _| {
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
            Ok(PatternPlurals::from(pattern.combined(date, time)?).into())
        })
    }

    /// Determine the appropriate `PatternPlurals` for a given `options::components::Bag`.
    fn patterns_for_components_bag(
        self,
        components: &components::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let skeletons_data = self.skeleton_data_payload()?;
        let patterns_data = date_patterns_data_payload(self.data_provider, self.locale)?;
        // Not all skeletons are currently supported.
        let requested_fields = components.to_vec_fields();
        let patterns = match skeleton::create_best_pattern_for_fields(
            skeletons_data.get(),
            &patterns_data.get().length_combinations,
            &requested_fields,
            components,
            false, // Prefer the requested fields over the matched pattern.
        ) {
            skeleton::BestSkeleton::AllFieldsMatch(pattern)
            | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => Some(pattern),
            skeleton::BestSkeleton::NoMatch => None,
        }
        .ok_or(DateTimeFormatterError::UnsupportedOptions)?;
        Ok(DataPayload::from_owned(PatternPluralsV1(
            patterns.into_owned(),
        )))
    }

    fn skeleton_data_payload(self) -> Result<DataPayload<DateSkeletonPatternsV1Marker>> {
        let data = self
            .data_provider
            .load(DataRequest {
                locale: self.locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(data)
    }
}

pub trait DateSymbols {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<&str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str>;
    fn get_symbol_for_era<'a>(&'a self, length: fields::FieldLength, era_code: &'a Era) -> &str;
}

impl<'data> DateSymbols for provider::calendar::DateSymbolsV1<'data> {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str> {
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
                            .ok_or(DateTimeFormatterError::MissingWeekdaySymbol(idx));
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
            .ok_or(DateTimeFormatterError::MissingWeekdaySymbol(idx))
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<&str> {
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
                        return symbols
                            .get(code)
                            .ok_or(DateTimeFormatterError::MissingMonthSymbol(code));
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, code);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, code);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        symbols
            .get(code)
            .ok_or(DateTimeFormatterError::MissingMonthSymbol(code))
    }

    /// Get the era symbol
    ///
    /// This will fall back to the era code directly, if, for example,
    /// a japanext datetime is formatted with a `DateTimeFormat<Japanese>`
    fn get_symbol_for_era<'a>(&'a self, length: fields::FieldLength, era_code: &'a Era) -> &str {
        let symbols = match length {
            fields::FieldLength::Wide => &self.eras.names,
            fields::FieldLength::Narrow => &self.eras.narrow,
            _ => &self.eras.abbr,
        };
        symbols.get(&era_code.0).unwrap_or(&era_code.0)
    }
}

pub trait TimeSymbols {
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str>;
}

impl<'data> TimeSymbols for provider::calendar::TimeSymbolsV1<'data> {
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str> {
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
