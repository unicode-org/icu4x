// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date;
use crate::error::DateTimeFormatError;
use crate::fields;
use crate::options::{components, length, preferences, DateTimeFormatOptions};
use crate::pattern::{hour_cycle, runtime::PatternPlurals};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsV1;
use crate::provider::calendar::DatePatternsV1;
use crate::provider::calendar::{
    patterns::PatternPluralsFromPatternsV1Marker, DatePatternsV1Marker,
    DateSkeletonPatternsV1Marker,
};
use crate::skeleton;
use icu_calendar::types::Era;
use icu_locid::Locale;
use icu_provider::prelude::*;

type Result<T> = core::result::Result<T, DateTimeFormatError>;

fn pattern_for_time_length_inner<'data>(
    data: DatePatternsV1<'data>,
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

fn pattern_for_date_length_inner(data: DatePatternsV1, length: length::Date) -> PatternPlurals {
    let pattern = match length {
        length::Date::Full => data.date.full,
        length::Date::Long => data.date.long,
        length::Date::Medium => data.date.medium,
        length::Date::Short => data.date.short,
    };
    PatternPlurals::from(pattern)
}

pub struct PatternSelector<'a, D> {
    data_provider: &'a D,
    locale: &'a Locale,
    calendar: &'static str,
}

// Manual impls needed since `derive(Copy)` inserts
// a `D: Copy` bound
impl<D> Copy for PatternSelector<'_, D> {}
impl<D> Clone for PatternSelector<'_, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> PatternSelector<'_, D>
where
    D: ResourceProvider<DatePatternsV1Marker> + ResourceProvider<DateSkeletonPatternsV1Marker>,
{
    pub(crate) fn for_options<'a>(
        data_provider: &'a D,
        locale: &'a Locale,
        options: &DateTimeFormatOptions,
        calendar: &'static str,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let selector = PatternSelector {
            data_provider,
            locale,
            calendar,
        };
        match options {
            DateTimeFormatOptions::Length(bag) => selector.pattern_for_length_bag(bag),
            DateTimeFormatOptions::Components(bag) => selector.patterns_for_components_bag(bag),
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
            (None, Some(time_length)) => {
                self.pattern_for_time_length(time_length, length.preferences)
            }
            (Some(date_length), None) => self.pattern_for_date_length(date_length),
            (Some(date_length), Some(time_length)) => {
                self.pattern_for_datetime_length(date_length, time_length, length.preferences)
            }
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
    fn pattern_for_date_length(
        self,
        length: length::Date,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = self.patterns_data_payload()?;
        Ok(
            patterns_data.map_project_with_capture(length, |data, length, _| {
                let pattern = pattern_for_date_length_inner(data, length);
                pattern.into()
            }),
        )
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Time` bag.
    /// If a preference for an hour cycle is set, it will look look up a pattern in the time_h11_12 or
    /// time_h23_h24 provider data, and then manually modify the symbol in the pattern if needed.
    fn pattern_for_time_length(
        self,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = self.patterns_data_payload()?;
        Ok(patterns_data.map_project_with_capture(
            (length, preferences),
            |data, (length, preferences), _| {
                let pattern = pattern_for_time_length_inner(data, length, &preferences).clone();
                pattern.into()
            },
        ))
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` and
    /// `options::length::Time` bag.
    fn pattern_for_datetime_length(
        self,
        date_length: length::Date,
        time_length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = self.patterns_data_payload()?;
        patterns_data.try_map_project_with_capture(
            (date_length, time_length, preferences),
            |data, (date_length, time_length, preferences), _| {
                let date = pattern_for_date_length_inner(data.clone(), date_length)
                    .expect_pattern("Lengths are single patterns");
                let time = pattern_for_time_length_inner(data.clone(), time_length, &preferences)
                    .expect_pattern("Lengths are single patterns");

                let pattern = match date_length {
                    length::Date::Full => data.length_combinations.full,
                    length::Date::Long => data.length_combinations.long,
                    length::Date::Medium => data.length_combinations.medium,
                    length::Date::Short => data.length_combinations.short,
                };
                let pattern = pattern.combined(date, time)?;
                Ok(PatternPlurals::from(pattern).into())
            },
        )
    }

    /// Determine the appropriate `PatternPlurals` for a given `options::components::Bag`.
    fn patterns_for_components_bag(
        self,
        components: &components::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let skeletons_data = self.skeleton_data_payload()?;
        let patterns_data = self.patterns_data_payload()?;
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
        .ok_or(DateTimeFormatError::UnsupportedOptions)?;
        Ok(DataPayload::from_owned(PatternPluralsV1(
            patterns.into_owned(),
        )))
    }
    fn patterns_data_payload(self) -> Result<DataPayload<DatePatternsV1Marker>> {
        let data = self
            .data_provider
            .load_resource(&DataRequest {
                options: ResourceOptions::temp_with_unicode_ext(
                    self.locale.id.clone(),
                    "ca",
                    self.calendar,
                ),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(data)
    }

    fn skeleton_data_payload(self) -> Result<DataPayload<DateSkeletonPatternsV1Marker>> {
        let data = self
            .data_provider
            .load_resource(&DataRequest {
                options: ResourceOptions::temp_with_unicode_ext(
                    self.locale.id.clone(),
                    "ca",
                    self.calendar,
                ),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(data)
    }
}

pub trait DateTimeSymbols {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> Result<&str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> Result<&str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str>;
    fn get_symbol_for_era(&self, length: fields::FieldLength, era_code: Era) -> Result<&str>;
}

impl<'data> DateTimeSymbols for provider::calendar::DateSymbolsV1<'data> {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
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
                            .ok_or(DateTimeFormatError::MissingWeekdaySymbol(idx));
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
            .ok_or(DateTimeFormatError::MissingWeekdaySymbol(idx))
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> Result<&str> {
        // TODO(#493): Support symbols for non-Gregorian calendars.
        debug_assert!(num < 12);
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
                            .0
                            .get(num)
                            .map(|x| &**x)
                            .ok_or(DateTimeFormatError::MissingMonthSymbol(num));
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, num);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, num);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        symbols
            .0
            .get(num)
            .map(|x| &**x)
            .ok_or(DateTimeFormatError::MissingMonthSymbol(num))
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
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

    fn get_symbol_for_era(&self, length: fields::FieldLength, era_code: Era) -> Result<&str> {
        let symbols = match length {
            fields::FieldLength::Wide => &self.eras.names,
            fields::FieldLength::Narrow => &self.eras.narrow,
            _ => &self.eras.abbr,
        };
        symbols
            .get(&era_code.0)
            .ok_or(DateTimeFormatError::MissingEraSymbol(era_code.0))
    }
}
