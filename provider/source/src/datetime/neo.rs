// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DatagenCalendar;
use crate::cldr_serde::ca;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::pattern;

use icu::datetime::provider::neo::marker_attrs::GlueType;
use icu::datetime::provider::neo::marker_attrs::{self, Context, Length, PatternLength};
use icu::datetime::provider::neo::*;
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
use writeable::Writeable;
use zerovec::VarZeroCow;

/// Most keys don't have short symbols (except weekdays)
///
/// We may further investigate and kick out standalone for some keys
const NORMAL_KEY_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::ABBR,
    marker_attrs::NARROW,
    marker_attrs::WIDE,
    marker_attrs::ABBR_STANDALONE,
    marker_attrs::NARROW_STANDALONE,
    marker_attrs::WIDE_STANDALONE,
];

/// Lengths for month data (NORMAL_KEY_LENGTHS + numeric)
const NUMERIC_MONTHS_KEY_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::ABBR,
    marker_attrs::NARROW,
    marker_attrs::WIDE,
    marker_attrs::ABBR_STANDALONE,
    marker_attrs::NARROW_STANDALONE,
    marker_attrs::WIDE_STANDALONE,
    marker_attrs::NUMERIC,
];

/// Lengths for year data (does not do standalone formatting)
const YEARS_KEY_LENGTHS: &[&DataMarkerAttributes] =
    &[marker_attrs::ABBR, marker_attrs::NARROW, marker_attrs::WIDE];

/// All possible non-numeric lengths
const FULL_KEY_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::ABBR,
    marker_attrs::NARROW,
    marker_attrs::WIDE,
    marker_attrs::SHORT,
    marker_attrs::ABBR_STANDALONE,
    marker_attrs::NARROW_STANDALONE,
    marker_attrs::WIDE_STANDALONE,
    marker_attrs::SHORT_STANDALONE,
];

/// Lengths for glue patterns
const GLUE_PATTERN_KEY_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::PATTERN_LONG_DT,
    marker_attrs::PATTERN_MEDIUM_DT,
    marker_attrs::PATTERN_SHORT_DT,
    marker_attrs::PATTERN_LONG_DZ,
    marker_attrs::PATTERN_MEDIUM_DZ,
    marker_attrs::PATTERN_SHORT_DZ,
    marker_attrs::PATTERN_LONG_TZ,
    marker_attrs::PATTERN_MEDIUM_TZ,
    marker_attrs::PATTERN_SHORT_TZ,
    marker_attrs::PATTERN_LONG_DTZ,
    marker_attrs::PATTERN_MEDIUM_DTZ,
    marker_attrs::PATTERN_SHORT_DTZ,
];

impl SourceDataProvider {
    fn load_calendar_dates(
        &self,
        locale: &DataLocale,
        calendar: DatagenCalendar,
    ) -> Result<&ca::Dates, DataError> {
        let resource: &ca::Resource = self
            .cldr()?
            .dates(calendar.cldr_name())
            .read_and_parse(locale, &format!("ca-{}.json", calendar.cldr_name()))?;

        let data = resource
            .main
            .value
            .dates
            .calendars
            .get(calendar.cldr_name())
            .expect("CLDR file contains the expected calendar");
        Ok(data)
    }

    fn load_neo_key<M: DataMarker>(
        &self,
        req: DataRequest,
        calendar: DatagenCalendar,
        conversion: impl FnOnce(DataIdentifierBorrowed, &ca::Dates) -> Result<M::DataStruct, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;

        let data = self.load_calendar_dates(req.id.locale, calendar)?;
        let data = conversion(req.id, data)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }

    fn load_neo_symbols_marker<M: DataMarker>(
        &self,
        req: DataRequest,
        calendar: DatagenCalendar,
        conversion: impl FnOnce(
            &SourceDataProvider,
            &DataLocale,
            &ca::Dates,
            DatagenCalendar,
            Context,
            Length,
        ) -> Result<M::DataStruct, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProviderCached<M>,
    {
        self.load_neo_key(req, calendar, |id, data| {
            let Some((context, length)) = marker_attrs::name_marker_attr_info(id.marker_attributes)
            else {
                panic!(
                    "Found unexpected marker attributes {}",
                    id.marker_attributes.as_str()
                )
            };
            conversion(self, id.locale, data, calendar, context, length)
        })
    }

    fn load_neo_patterns_marker<M: DataMarker>(
        &self,
        req: DataRequest,
        calendar: DatagenCalendar,
        conversion: impl FnOnce(&ca::Dates, PatternLength, GlueType) -> Result<M::DataStruct, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProviderCached<M>,
    {
        self.load_neo_key(req, calendar, |id, data| {
            let Some((length, glue_type)) =
                marker_attrs::pattern_marker_attr_info_for_glue(id.marker_attributes)
            else {
                panic!(
                    "Found unexpected marker attributes {}",
                    id.marker_attributes.as_str()
                )
            };
            conversion(data, length, glue_type)
        })
    }

    fn iter_ids_neo(
        &self,
        calendar: DatagenCalendar,
        keylengths: &'static [&DataMarkerAttributes],
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .dates(calendar.cldr_name())
            .list_locales()?
            .flat_map(|locale| {
                keylengths
                    .iter()
                    .map(move |&length| DataIdentifierCow::from_borrowed_and_owned(length, locale))
            })
            .collect())
    }
}

fn weekday_convert(
    _datagen: &SourceDataProvider,
    _locale: &DataLocale,
    data: &ca::Dates,
    _calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<LinearNames<'static>, DataError> {
    let day_symbols = data.days.get_symbols(context, length);

    let days = [
        &*day_symbols.sun,
        &*day_symbols.mon,
        &*day_symbols.tue,
        &*day_symbols.wed,
        &*day_symbols.thu,
        &*day_symbols.fri,
        &*day_symbols.sat,
    ];

    Ok(LinearNames {
        names: (&days).into(),
    })
}

fn dayperiods_convert(
    _datagen: &SourceDataProvider,
    _locale: &DataLocale,
    data: &ca::Dates,
    _calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<LinearNames<'static>, DataError> {
    let day_periods = data.day_periods.get_symbols(context, length);

    let mut periods = vec![&*day_periods.am, &*day_periods.pm];

    if let Some(ref noon) = day_periods.noon {
        periods.push(noon);
    } else if day_periods.midnight.is_some() {
        periods.push(""); // blank entry to separate midnight
    };

    if let Some(ref midnight) = day_periods.midnight {
        periods.push(midnight)
    }

    Ok(LinearNames {
        names: (&periods).into(),
    })
}

fn eras_convert(
    datagen: &SourceDataProvider,
    locale: &DataLocale,
    eras: &ca::Eras,
    calendar: DatagenCalendar,
    length: Length,
) -> Result<YearNames<'static>, DataError> {
    let eras = eras.load(length);
    let all_eras = &datagen.all_eras()?[&calendar];
    if matches!(
        calendar,
        DatagenCalendar::JapaneseModern | DatagenCalendar::JapaneseExtended
    ) {
        let greg_eras = datagen
            .cldr()?
            .dates(DatagenCalendar::Gregorian.cldr_name())
            .read_and_parse::<ca::Resource>(locale, "ca-gregorian.json")?
            .main
            .value
            .dates
            .calendars
            .get(DatagenCalendar::Gregorian.cldr_name())
            .expect("CLDR gregorian.json contains the expected calendar")
            .eras
            .as_ref()
            .expect("gregorian must have eras")
            .load(length);

        let mut out_eras = BTreeMap::new();

        for &(cldr, ref data) in all_eras {
            if cldr == 0 {
                out_eras.insert(
                    data.code.as_deref().unwrap(),
                    greg_eras
                        .get("0")
                        .expect("gregorian calendar must have 0 era")
                        .as_str(),
                );
            } else if cldr == 1 {
                out_eras.insert(
                    data.code.as_deref().unwrap(),
                    greg_eras
                        .get("1")
                        .expect("gregorian calendar must have 1 era")
                        .as_str(),
                );
            } else {
                // https://unicode-org.atlassian.net/browse/CLDR-18388 for why we need to do -2
                if let Some(name) = eras.get(&(cldr - 2).to_string()) {
                    out_eras.insert(data.code.as_deref().unwrap(), name);
                } else {
                    panic!("Unknown japanese era number {cldr}");
                }
            }
        }
        let keys: Vec<&PotentialUtf8> = out_eras
            .keys()
            .map(|k| PotentialUtf8::from_str(k))
            .collect();
        let values: Vec<&str> = out_eras.values().copied().collect();
        let kv = (keys, values);
        let cow = VarZeroCow::from_encodeable(&kv);
        Ok(YearNames::VariableEras(cow))
    } else {
        let max_era_index = all_eras.iter().flat_map(|(_, e)| e.icu4x_era_index).max();
        let mut out_eras: Vec<&str> =
            vec![""; max_era_index.map(|n| n + 1).unwrap_or_default() as usize];
        for &(cldr, ref era) in all_eras.iter() {
            if let Some(name) = eras.get(&cldr.to_string()) {
                if let Some(icu4x_hardcoded_index) = era.icu4x_era_index {
                    out_eras[icu4x_hardcoded_index as usize] = &**name;
                }
            } else {
                panic!("Did not find era data for era index {cldr} for {calendar:?} and {locale}");
            }
        }

        Ok(YearNames::FixedEras((&out_eras).into()))
    }
}
fn years_convert(
    datagen: &SourceDataProvider,
    locale: &DataLocale,
    data: &ca::Dates,
    calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<YearNames<'static>, DataError> {
    assert_eq!(
        context,
        Context::Format,
        "Eras and cyclic years do not participate in standalone formatting"
    );

    if let Some(ref eras) = data.eras {
        eras_convert(datagen, locale, eras, calendar, length)
    } else if let Some(years) = data
        .cyclic_name_sets
        .as_ref()
        .and_then(|c| c.years.as_ref())
    {
        let years = years.get_symbols(context, length);

        let years: Vec<_> = years.iter().enumerate().map(|(index, (key, value))| {
            if *key as usize != index + 1 {
                panic!("Calendar {calendar:?} in locale {locale} missing cyclic year name for index {index}");
            }
            &**value
        }).collect();
        Ok(YearNames::Cyclic((&years).into()))
    } else {
        panic!(
            "Calendar {calendar:?} in locale {locale} has neither eras nor cyclicNameSets for years"
        )
    }
}

/// Returns the number of regular months in a calendar, as well as whether it is
/// has leap months
fn calendar_months(cal: DatagenCalendar) -> (usize, bool) {
    match cal {
        DatagenCalendar::Hebrew | DatagenCalendar::Chinese | DatagenCalendar::Dangi => (24, true),
        DatagenCalendar::Coptic | DatagenCalendar::Ethiopic => (13, false),
        DatagenCalendar::Gregorian
        | DatagenCalendar::Buddhist
        | DatagenCalendar::JapaneseModern
        | DatagenCalendar::JapaneseExtended
        | DatagenCalendar::Indian
        | DatagenCalendar::Persian
        | DatagenCalendar::Hijri
        | DatagenCalendar::Roc => (12, false),
    }
}

fn months_convert(
    _datagen: &SourceDataProvider,
    locale: &DataLocale,
    data: &ca::Dates,
    calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<MonthNames<'static>, DataError> {
    if length == Length::Numeric {
        assert_eq!(
            context,
            Context::Format,
            "numeric months only found for Context::Format"
        );
        let Some(ref patterns) = data.month_patterns else {
            panic!("No month_patterns found but numeric months were requested for {calendar:?} with {locale}");
        };
        let pattern = patterns.get_symbols(context, length);
        return Ok(MonthNames::LeapNumeric(Cow::Owned(
            pattern.leap.0.to_owned(),
        )));
    }

    let months = data.months.get_symbols(context, length);

    let (month_count, has_leap) = calendar_months(calendar);
    let mut symbols = vec![Cow::Borrowed(""); month_count];

    if calendar == DatagenCalendar::Hebrew {
        for (k, v) in months.0.iter() {
            // CLDR's numbering for hebrew has Adar I as 6, Adar as 7, and Adar II as 7-yeartype-leap
            //
            // So we need to handle the 6 and 7 cases as leap months, and everything after 6 needs to
            // be offset by 1
            let index = if k == "7-yeartype-leap" {
                // Adar II => M06L:
                // * 12 months in a year
                // * M06 is the 6th month
                // * -1 because the data is zero-indexed
                12 + 6 - 1
            } else if k == "6" {
                // Adar I => M05L:
                // * 12 months in a year
                // * M05 is the 5th month
                // * -1 because the data is zero-indexed
                12 + 5 - 1
            } else {
                let mut index: usize = k
                    .parse()
                    .expect("CLDR month indices must parse as numbers!");

                if index > 5 {
                    index -= 1;
                }
                if index == 0 {
                    panic!("CLDR month indices cannot be zero");
                }

                index - 1
            };

            symbols[index] = (&**v).into();
        }
        Ok(MonthNames::LeapLinear((&symbols).into()))
    } else {
        for (k, v) in months.0.iter() {
            let index: usize = k
                .parse()
                .expect("CLDR month indices must parse as numbers!");
            if index == 0 {
                panic!("CLDR month indices cannot be zero");
            }

            symbols[index - 1] = v.into();
        }

        let nonleap = if has_leap {
            month_count / 2
        } else {
            month_count
        };

        for (i, val) in symbols.iter().take(nonleap).enumerate() {
            if val.is_empty() {
                panic!("Calendar {calendar:?} does not have data for month {i}; found data for {symbols:?}");
            }
        }

        if has_leap {
            // This branch is only for chinese-like calendars with N regular months and N potential leap months
            // rather than hebrew-like where there's one or two special leap months
            debug_assert!(
                calendar != DatagenCalendar::Hebrew,
                "Hebrew calendar should have been handled in the branch above"
            );
            let patterns = data
                .month_patterns
                .as_ref()
                .expect("Calendar with leap months must have monthPatterns");
            let leap = &patterns.get_symbols(context, length).leap;

            for i in 0..nonleap {
                if symbols[i].is_empty() {
                    continue;
                }
                let replaced = leap
                    .0
                    .interpolate([&symbols[i]])
                    .write_to_string()
                    .into_owned();
                symbols[nonleap + i] = replaced.into();
            }
            Ok(MonthNames::LeapLinear((&symbols).into()))
        } else {
            Ok(MonthNames::Linear((&symbols).into()))
        }
    }
}

/// Given a lengthpattern, apply any numeric overrides it may have to `pattern`
#[allow(dead_code)] // TODO: Implement numeric overrides in neo patterns
fn apply_numeric_overrides(lp: &ca::LengthPattern, pattern: &mut pattern::runtime::Pattern) {
    use icu::datetime::provider::fields::{
        self, FieldLength, FieldNumericOverrides::*, FieldSymbol,
    };
    let ca::LengthPattern::WithNumberingSystems {
        ref numbering_systems,
        ..
    } = *lp
    else {
        // no numeric override
        return;
    };
    // symbol_to_replace is None when we need to replace *all* symbols
    let (numeric, symbol_to_replace) = match &**numbering_systems {
        "hanidec" => (Hanidec, None),
        "hebr" => (Hebr, None),
        "d=hanidays" => (Hanidays, Some(FieldSymbol::Day(fields::Day::DayOfMonth))),
        "M=romanlow" => (Romanlow, Some(FieldSymbol::Month(fields::Month::Format))),
        "y=jpanyear" => (Jpnyear, Some(FieldSymbol::Year(fields::Year::Calendar))),
        _ => panic!("Found unexpected numeric override {numbering_systems}"),
    };

    pattern.items.for_each_mut(|item| {
        if let pattern::PatternItem::Field(ref mut field) = *item {
            // only replace numeric items
            if field.length != FieldLength::One {
                assert!(
                    field.length != FieldLength::Two || symbol_to_replace != Some(field.symbol),
                    "We don't know what to do when there is a non-targeted numeric override \
                         on a two-digit numeric field"
                );
                return;
            }
            // if we need to replace a specific symbol, filter
            // out everyone else
            if let Some(symbol) = symbol_to_replace {
                if symbol != field.symbol {
                    return;
                }
            }

            field.length = FieldLength::NumericOverride(numeric);
        }
    })
}

fn datetimepattern_convert(
    data: &ca::Dates,
    length: PatternLength,
    glue_type: GlueType,
) -> Result<GluePattern<'static>, DataError> {
    let mut pattern_anchor = None;
    let pattern = match glue_type {
        GlueType::DateTime => data.datetime_formats.get_pattern(length).get_pattern(),
        GlueType::DateZone => {
            // TODO: Use proper appendItem here
            "{1} {2}"
        }
        GlueType::TimeZone => {
            // TODO: Use proper appendItem here
            "{0} {2}"
        }
        GlueType::DateTimeZone => {
            let pattern = pattern_anchor.insert(
                data.datetime_formats
                    .get_pattern(length)
                    .get_pattern()
                    .to_string(),
            );
            // TODO: Use proper appendItem here
            pattern.push_str(" {2}");
            pattern
        }
    };

    let pattern = pattern.parse().expect("failed to parse pattern");
    Ok(GluePattern { pattern })
}

macro_rules! impl_symbols_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_symbols_marker::<$marker>(req, $calendar, $convert)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.iter_ids_neo($calendar, $lengths)
            }
        }
    };
}

macro_rules! impl_pattern_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_patterns_marker::<$marker>(req, $calendar, $convert)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.iter_ids_neo($calendar, $lengths)
            }
        }
    };
}

// Weekdays
impl_symbols_datagen!(
    WeekdayNamesV1,
    DatagenCalendar::Gregorian,
    FULL_KEY_LENGTHS,
    weekday_convert
);

// Dayperiods
impl_symbols_datagen!(
    DayPeriodNamesV1,
    DatagenCalendar::Gregorian,
    NORMAL_KEY_LENGTHS,
    dayperiods_convert
);

// Years
impl_symbols_datagen!(
    DatetimeNamesYearBuddhistV1,
    DatagenCalendar::Buddhist,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearChineseV1,
    DatagenCalendar::Chinese,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearCopticV1,
    DatagenCalendar::Coptic,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearDangiV1,
    DatagenCalendar::Dangi,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearEthiopianV1,
    DatagenCalendar::Ethiopic,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearGregorianV1,
    DatagenCalendar::Gregorian,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearHebrewV1,
    DatagenCalendar::Hebrew,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearIndianV1,
    DatagenCalendar::Indian,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearHijriV1,
    DatagenCalendar::Hijri,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearJapaneseV1,
    DatagenCalendar::JapaneseModern,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearJapanextV1,
    DatagenCalendar::JapaneseExtended,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearPersianV1,
    DatagenCalendar::Persian,
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearRocV1,
    DatagenCalendar::Roc,
    YEARS_KEY_LENGTHS,
    years_convert
);

// Months
impl_symbols_datagen!(
    DatetimeNamesMonthBuddhistV1,
    DatagenCalendar::Buddhist,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthChineseV1,
    DatagenCalendar::Chinese,
    NUMERIC_MONTHS_KEY_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthCopticV1,
    DatagenCalendar::Coptic,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthDangiV1,
    DatagenCalendar::Dangi,
    NUMERIC_MONTHS_KEY_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthEthiopianV1,
    DatagenCalendar::Ethiopic,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthGregorianV1,
    DatagenCalendar::Gregorian,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthHebrewV1,
    DatagenCalendar::Hebrew,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthIndianV1,
    DatagenCalendar::Indian,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthHijriV1,
    DatagenCalendar::Hijri,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthJapaneseV1,
    DatagenCalendar::JapaneseModern,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthJapanextV1,
    DatagenCalendar::JapaneseExtended,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthPersianV1,
    DatagenCalendar::Persian,
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthRocV1,
    DatagenCalendar::Roc,
    NORMAL_KEY_LENGTHS,
    months_convert
);

// Datetime patterns
// TODO: This is modeled with glue patterns that are the same across calendar
// systems, but CLDR has some instances where the glue patterns differ, such
// as in French (Gregorian has a comma but other calendars do not).
impl_pattern_datagen!(
    DatetimePatternsGlueV1,
    DatagenCalendar::Gregorian,
    GLUE_PATTERN_KEY_LENGTHS,
    datetimepattern_convert
);
