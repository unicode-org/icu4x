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
use alloc::borrow::Cow;
use core::marker::PhantomData;
use icu_locid::Locale;
use icu_provider::prelude::*;

type Result<T> = core::result::Result<T, DateTimeFormatError>;

fn patterns_data_payload<D>(
    data_provider: &D,
    locale: &Locale,
) -> Result<DataPayload<DatePatternsV1Marker>>
where
    D: DataProvider<DatePatternsV1Marker>,
{
    let data = data_provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: provider::key::DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: Some("gregorian".into()),
                    langid: Some(locale.clone().into()),
                },
            },
        })?
        .take_payload()?;
    Ok(data)
}

fn skeleton_data_payload<D>(
    data_provider: &D,
    locale: &Locale,
) -> Result<DataPayload<DateSkeletonPatternsV1Marker>>
where
    D: DataProvider<DateSkeletonPatternsV1Marker>,
{
    let data = data_provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: provider::key::DATE_SKELETON_PATTERNS_V1,
                options: ResourceOptions {
                    variant: Some("gregorian".into()),
                    langid: Some(locale.clone().into()),
                },
            },
        })?
        .take_payload()?;
    Ok(data)
}

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

pub struct PatternSelector<D>(PhantomData<D>);

impl<D> PatternSelector<D>
where
    D: DataProvider<DatePatternsV1Marker> + DataProvider<DateSkeletonPatternsV1Marker>,
{
    pub(crate) fn for_options(
        data_provider: &D,
        locale: &Locale,
        options: &DateTimeFormatOptions,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        match options {
            DateTimeFormatOptions::Length(bag) => {
                Self::pattern_for_length_bag(data_provider, locale, bag)
            }
            DateTimeFormatOptions::Components(bag) => {
                Self::patterns_for_components_bag(data_provider, locale, bag)
            }
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::Length` bag.
    fn pattern_for_length_bag(
        data_provider: &D,
        locale: &Locale,
        length: &length::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        match (length.date, length.time) {
            (None, None) => Ok(DataPayload::from_owned(PatternPluralsV1(
                PatternPlurals::default(),
            ))),
            (None, Some(time_length)) => Self::pattern_for_time_length(
                data_provider,
                locale,
                time_length,
                length.preferences,
            ),
            (Some(date_length), None) => {
                Self::pattern_for_date_length(data_provider, locale, date_length)
            }
            (Some(date_length), Some(time_length)) => Self::pattern_for_datetime_length(
                data_provider,
                locale,
                date_length,
                time_length,
                length.preferences,
            ),
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
    fn pattern_for_date_length(
        data_provider: &D,
        locale: &Locale,
        length: length::Date,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = patterns_data_payload(data_provider, locale)?;
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
        data_provider: &D,
        locale: &Locale,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = patterns_data_payload(data_provider, locale)?;
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
        data_provider: &D,
        locale: &Locale,
        date_length: length::Date,
        time_length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let patterns_data = patterns_data_payload(data_provider, locale)?;
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
        data_provider: &D,
        locale: &Locale,
        components: &components::Bag,
    ) -> Result<DataPayload<PatternPluralsFromPatternsV1Marker>> {
        let skeletons_data = skeleton_data_payload(data_provider, locale)?;
        let patterns_data = patterns_data_payload(data_provider, locale)?;
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
        .unwrap_or_default();
        Ok(DataPayload::from_owned(PatternPluralsV1(
            patterns.into_owned(),
        )))
    }
}

pub trait DateTimeSymbols {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str>;
}

impl DateTimeSymbols for provider::calendar::DateSymbolsV1 {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str> {
        let widths = match weekday {
            fields::Weekday::Format => &self.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => widths
                            .short
                            .as_ref()
                            .or_else(|| widths.abbreviated.as_ref()),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[(day as usize) % 7];
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
        &symbols.0[(day as usize) % 7]
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str> {
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
                        return &symbols.0[num];
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
        &symbols.0[num]
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        }
    }
}
