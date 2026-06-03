// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::DatagenCalendar;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde::ca;
use icu::datetime::provider::pattern;

use icu::datetime::provider::names::*;
use icu::datetime::provider::semantic_skeletons::marker_attrs::GlueType;
use icu::datetime::provider::semantic_skeletons::marker_attrs::{
    self, Context, Length, PatternLength,
};
use icu::datetime::provider::semantic_skeletons::{DatetimePatternsGlueV1, GluePattern};
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
use zerovec::ule::vartuple::VarTuple;

/// Lengths for day period data
const DAY_PERIOD_MARKER_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::ABBR,
    marker_attrs::NARROW,
    marker_attrs::WIDE,
    marker_attrs::ABBR_STANDALONE,
    marker_attrs::NARROW_STANDALONE,
    marker_attrs::WIDE_STANDALONE,
];

/// Lengths for month data
const MONTHS_MARKER_LENGTHS: &[&DataMarkerAttributes] = &[
    marker_attrs::ABBR,
    marker_attrs::NARROW,
    marker_attrs::WIDE,
    marker_attrs::ABBR_STANDALONE,
    marker_attrs::NARROW_STANDALONE,
    marker_attrs::WIDE_STANDALONE,
    marker_attrs::NUMERIC,
];

/// Lengths for year data (does not do standalone formatting)
const YEARS_MARKER_LENGTHS: &[&DataMarkerAttributes] =
    &[marker_attrs::ABBR, marker_attrs::NARROW, marker_attrs::WIDE];

/// Lengths for weekday data
const WEEKDAY_MARKER_LENGTHS: &[&DataMarkerAttributes] = &[
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
const GLUE_PATTERN_MARKER_LENGTHS: &[&DataMarkerAttributes] = &[
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
    fn load_datetime_marker<M: DataMarker>(
        &self,
        req: DataRequest,
        calendar: DatagenCalendar,
        conversion: impl FnOnce(DataIdentifierBorrowed, &ca::Dates) -> Result<M::DataStruct, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProviderCached<M>,
    {
        self.check_req::<M>(req)?;

        let data = self.get_dates_resource(req.id.locale, Some(calendar))?;
        let data = conversion(req.id, data)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }

    fn load_datetime_symbols_marker<M: DataMarker>(
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
        self.load_datetime_marker(req, calendar, |id, data| {
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

    fn load_datetime_patterns_marker<M: DataMarker>(
        &self,
        req: DataRequest,
        calendar: DatagenCalendar,
        conversion: impl FnOnce(&ca::Dates, PatternLength, GlueType) -> Result<M::DataStruct, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProviderCached<M>,
    {
        self.load_datetime_marker(req, calendar, |id, data| {
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

    fn iter_datetime_ids(
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

#[allow(clippy::unnecessary_wraps)] // signature required by macro
fn weekday_convert(
    _datagen: &SourceDataProvider,
    _locale: &DataLocale,
    data: &ca::Dates,
    _calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<WeekdayNames<'static>, DataError> {
    let day_symbols = data.days.get_symbols(context, length);

    use icu::calendar::types::Weekday::*;
    Ok(WeekdayNames::new(
        [
            (Sunday, &*day_symbols.sun),
            (Monday, &*day_symbols.mon),
            (Tuesday, &*day_symbols.tue),
            (Wednesday, &*day_symbols.wed),
            (Thursday, &*day_symbols.thu),
            (Friday, &*day_symbols.fri),
            (Saturday, &*day_symbols.sat),
        ]
        .into_iter(),
    ))
}

/// Checks if the locale needs flexible day periods.
/// A locale needs them if any of its supported calendars has standard time formats
/// or available formats containing 'B' (flexible day periods) for non-B skeletons.
pub(super) fn needs_flexible_day_periods(
    datagen: &SourceDataProvider,
    locale: &DataLocale,
) -> bool {
    const ALL_CALENDARS: &[DatagenCalendar] = &[
        DatagenCalendar::Buddhist,
        DatagenCalendar::Chinese,
        DatagenCalendar::Coptic,
        DatagenCalendar::Dangi,
        DatagenCalendar::Ethiopic,
        DatagenCalendar::Gregorian,
        DatagenCalendar::Hebrew,
        DatagenCalendar::Indian,
        DatagenCalendar::Hijri,
        DatagenCalendar::Japanese,
        DatagenCalendar::Persian,
        DatagenCalendar::Roc,
    ];

    fn time_format_has_b(formats: &ca::LengthPatterns) -> bool {
        formats.full.get_pattern().contains('B')
            || formats.long.get_pattern().contains('B')
            || formats.medium.get_pattern().contains('B')
            || formats.short.get_pattern().contains('B')
    }

    for &calendar in ALL_CALENDARS {
        if let Ok(data) = datagen.get_dates_resource(locale, Some(calendar)) {
            if time_format_has_b(&data.time_formats) {
                return true;
            }
            if time_format_has_b(&data.time_skeletons) {
                return true;
            }
            for (skeleton, pattern) in &data.datetime_formats.available_formats.0 {
                if pattern.contains('B') && !skeleton.contains('B') {
                    return true;
                }
            }
        }
    }
    false
}

#[allow(clippy::unnecessary_wraps)] // signature required by macro
fn dayperiods_convert(
    datagen: &SourceDataProvider,
    locale: &DataLocale,
    data: &ca::Dates,
    _calendar: DatagenCalendar,
    context: Context,
    length: Length,
) -> Result<DayPeriodNames<'static>, DataError> {
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

    let rules_encoded;

    if needs_flexible_day_periods(datagen, locale) {
        let rules = datagen
            .cldr()?
            .core()
            .read_and_parse::<crate::cldr_serde::day_periods::Resource>(
                "supplemental/dayPeriods.json",
            )?
            .supplemental
            .day_period_rule_set
            .0
            // Day period rules are stored on a language level, i.e. zh and zh-Hant share rules
            .get(locale.language.as_str())
            .expect("day period rules should exist");

        let (rules, mut names) =
            super::day_periods::compute_day_periods(rules, &day_periods.flexible, *locale)?;

        periods.resize(4, "");
        rules_encoded = format!("{}{}", rules.encode_to_string(), names.next().unwrap());
        periods.push(&rules_encoded);
        periods.extend(names);
    }

    Ok(DayPeriodNames {
        names: (&periods).into(),
    })
}

fn eras_collect<'a>(
    provider: &'a SourceDataProvider,
    locale: &DataLocale,
    eras: &'a ca::Eras,
    calendar: DatagenCalendar,
    length: Length,
) -> Result<BTreeMap<(&'a str, usize), &'a str>, DataError> {
    let (inherit, ref all_eras) = provider.all_eras()?[&calendar];

    let mut out = BTreeMap::new();

    for &(cldr, ref era) in all_eras {
        out.insert(
            (era.code.as_str(), era.icu4x_era_index.unwrap() as usize),
            &*eras.load(length)[&cldr.to_string()],
        );
    }

    if let Some(inherit) = inherit {
        out.extend(eras_collect(
            provider,
            locale,
            provider
                .get_dates_resource(locale, Some(inherit))?
                .eras
                .as_ref()
                .unwrap(),
            inherit,
            length,
        )?);
    }

    Ok(out)
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
        let eras = eras_collect(datagen, locale, eras, calendar, length)?;

        let max_icu4x_era_index = eras
            .keys()
            .map(|(_, idx)| idx + 1)
            .max()
            .unwrap_or_default();

        let mut out_eras = vec![""; max_icu4x_era_index];
        for ((_, idx), era) in eras {
            out_eras[idx] = era;
        }
        Ok(YearNames::FixedEras((&out_eras).into()))
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

#[allow(clippy::unnecessary_wraps)] // signature required by macro
fn months_convert(
    _datagen: &SourceDataProvider,
    _locale: &DataLocale,
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
        if calendar == DatagenCalendar::Hebrew {
            return Ok(MonthNames::LeapNumericWithBase(
                (&[
                    // M05L should be 6a
                    VarTuple {
                        sized: 1,
                        variable: &SinglePlaceholderPattern::try_from_str(
                            "{0}a",
                            Default::default(),
                        )
                        .unwrap(),
                    },
                    // M06 should be 6b after M05L
                    VarTuple {
                        sized: 0,
                        variable: &SinglePlaceholderPattern::try_from_str(
                            "{0}b",
                            Default::default(),
                        )
                        .unwrap(),
                    },
                ])
                    .into(),
            ));
        }
        let Some(ref patterns) = data.month_patterns else {
            return Ok(MonthNames::Numeric);
        };
        let pattern = patterns.get_symbols(context, length);
        return Ok(MonthNames::LeapNumeric(Cow::Owned(
            pattern.leap.0.to_owned(),
        )));
    }

    let months = &data.months.get_symbols(context, length).0;

    if calendar == DatagenCalendar::Hebrew
        && length == Length::Narrow
        && months["10"].starts_with(&months["1"])
        && months["11"].starts_with(&months["1"])
        && months["12"].starts_with(&months["1"])
        && months["13"].starts_with(&months["1"])
    {
        // CLDR currently has these locales that have data for Hebrew narrow months:
        // * und: uses digits "6", "7", "7"
        // * ast: uses digits, "6", "7", "7b"
        // * bn: uses digits, "৬", "৭", "৭"
        // * fa: uses words, "آ", "و", "و" (<- RTL)
        // * ff-Adlm: uses digits, "𞥖", "𞥗", "𞥗" (<- RTL)
        // * fi: uses letters, "A", "A", "A"
        // * he: uses words, "א״א", "אד׳" ,"א״ב" (<- RTL)
        // * ml: uses letters, "അ I", "അ.", "അ II",
        // * mr: uses digits, "६", "७", "७",
        // where the names are for Adar I, Adar, Adar II in that order.

        // Unlike CLDR, ICU4X does not consider Adar/Adar II to have number 7 (and subsequent
        // months to be shifted), so we have to special-case the locales that use digits.
        // We detect this by checking whether the names for month["1n"] starts with the name
        // for month["1"]. This branch will therefore be taken for und, ast, bn, ff-Adlm, and mr.

        // The CLDR 48 data, for e.g. fa, can be inspected at
        // https://github.com/unicode-org/cldr-json/blob/48.0.0/cldr-json/cldr-cal-hebrew-full/main/fa/ca-hebrew.json#L28-L43

        Ok(MonthNames::LeapPattern(
            (&[
                months["1"].as_str(),
                months["2"].as_str(),
                months["3"].as_str(),
                months["4"].as_str(),
                months["5"].as_str(),
                months["6"].as_str(),
                months["7"].as_str(),
                months["8"].as_str(),
                months["9"].as_str(),
                months["10"].as_str(),
                months["11"].as_str(),
                months["12"].as_str(),
                // For lack of a better solution, we call Adar I and Adar II "a" and "b" instead.
                &SinglePlaceholderPattern::try_from_str(
                    &format!("{}a", &months["6"]),
                    Default::default(),
                )
                .unwrap()
                .store,
                &SinglePlaceholderPattern::try_from_str(
                    &format!("{}b", &months["6"]),
                    Default::default(),
                )
                .unwrap()
                .store,
            ])
                .into(),
        ))
    } else if calendar == DatagenCalendar::Hebrew {
        let shevat = &months["5"];
        let adar_i = &months["6"];
        let adar = &months["7"];
        let adar_ii = &months["7-yeartype-leap"];
        // Adar I is the only leap month, so we can hardcode it as the leap pattern. The placeholder
        // is the normal fifth month (Shevat), we can try reducing the data size by using it (but it
        // should not actually match).
        let leap_pattern = SinglePlaceholderPattern::try_from_str(
            &adar_i.replace(shevat, "{0}"),
            Default::default(),
        )
        .unwrap();
        // Adar II is the only leap-base month, so we can hardcode it as the leap-base pattern. The
        // placeholder is the normal sixth month (Adar), we can reduce the data size by using it.
        let leap_base_pattern = SinglePlaceholderPattern::try_from_str(
            &adar_ii.replace(adar, "{0}"),
            Default::default(),
        )
        .unwrap();

        let symbols = [
            months["1"].as_str(),
            months["2"].as_str(),
            months["3"].as_str(),
            months["4"].as_str(),
            months["5"].as_str(),
            months["7"].as_str(),
            months["8"].as_str(),
            months["9"].as_str(),
            months["10"].as_str(),
            months["11"].as_str(),
            months["12"].as_str(),
            months["13"].as_str(),
            &leap_pattern.store,
            &leap_base_pattern.store,
        ];
        Ok(MonthNames::LeapPattern((&symbols).into()))
    } else {
        let months = months
            .iter()
            .map(|(k, v)| {
                let index: usize = k
                    .parse()
                    .expect("CLDR month indices must parse as numbers!");
                if index == 0 {
                    panic!("CLDR month indices cannot be zero");
                }
                (index, v.as_str())
            })
            .collect::<BTreeMap<_, _>>();

        if *months.last_key_value().unwrap().0 != months.len() {
            panic!("Calendar {calendar:?} does not have data for all months: {months:?}");
        }

        let mut symbols = months.into_values().collect::<Vec<_>>();

        if let Some(patterns) = data.month_patterns.as_ref() {
            symbols.push(&patterns.get_symbols(context, length).leap.0.store);
            // Leap bases format as normal months for non-Hebrew
            symbols.push(&SinglePlaceholderPattern::PASS_THROUGH.store);
            Ok(MonthNames::LeapPattern((&symbols).into()))
        } else {
            Ok(MonthNames::Linear((&symbols).into()))
        }
    }
}

/// Given a lengthpattern, apply any numeric overrides it may have to `pattern`
pub(crate) fn apply_numeric_overrides(
    lp: &ca::LengthPattern,
    pattern: &mut pattern::runtime::Pattern,
) {
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
            // We currently only support overrides for these fields
            // and in CLDR overrides are only found in dateFormats and dateSkeletons
            // So we should not be applying them to e.g. time fields
            if !matches!(field.symbol, FieldSymbol::Year(..)| FieldSymbol::Month(..) | FieldSymbol::Day(..)) {
                return;
            }

            // only replace numeric items
            if (*field).get_length_type() != fields::TextOrNumeric::Numeric {
                return;
            }
            if field.length == FieldLength::Two {
                if symbol_to_replace.is_none() {
                    eprintln!("WARN: Skipping non-targeted numeric override on a two-digit field {:?} because it implies fixed width.", field.symbol);
                    return;
                } else {
                    eprintln!("WARN: Applying targeted numeric override to a two-digit field {:?}. This may not respect fixed width!", field.symbol);
                }
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

#[allow(clippy::unnecessary_wraps)] // signature required by macro
fn datetimepattern_convert(
    data: &ca::Dates,
    length: PatternLength,
    glue_type: GlueType,
) -> Result<GluePattern<'static>, DataError> {
    let append_tz = &data.datetime_formats.append_items.timezone;

    // Note: We default to atTime here (See https://github.com/unicode-org/conformance/issues/469)
    let at_time = data
        .datetime_formats_at_time
        .get_pattern(length)
        .get_pattern();

    let pattern = match glue_type {
        GlueType::DateTime => at_time.parse(),
        GlueType::DateZone => append_tz.interpolate_to_string(["{1}", "{2}"]).parse(),
        GlueType::TimeZone => append_tz.interpolate_to_string(["{0}", "{2}"]).parse(),
        GlueType::DateTimeZone => at_time
            .replace("{0}", &append_tz.interpolate_to_string(["{0}", "{2}"]))
            .parse(),
    }
    .expect("failed to parse pattern");
    Ok(GluePattern { pattern })
}

macro_rules! impl_symbols_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_datetime_symbols_marker::<$marker>(req, $calendar, $convert)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.iter_datetime_ids($calendar, $lengths)
            }
        }
    };
}

macro_rules! impl_pattern_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_datetime_patterns_marker::<$marker>(req, $calendar, $convert)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                self.iter_datetime_ids($calendar, $lengths)
            }
        }
    };
}

// Weekdays
impl_symbols_datagen!(
    WeekdayNamesV1,
    DatagenCalendar::Gregorian,
    WEEKDAY_MARKER_LENGTHS,
    weekday_convert
);

// Dayperiods
impl_symbols_datagen!(
    DayPeriodNamesV1,
    DatagenCalendar::Gregorian,
    DAY_PERIOD_MARKER_LENGTHS,
    dayperiods_convert
);

// Years
impl_symbols_datagen!(
    DatetimeNamesYearBuddhistV1,
    DatagenCalendar::Buddhist,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearChineseV1,
    DatagenCalendar::Chinese,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearCopticV1,
    DatagenCalendar::Coptic,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearDangiV1,
    DatagenCalendar::Dangi,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearEthiopianV1,
    DatagenCalendar::Ethiopic,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearGregorianV1,
    DatagenCalendar::Gregorian,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearHebrewV1,
    DatagenCalendar::Hebrew,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearIndianV1,
    DatagenCalendar::Indian,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearHijriV1,
    DatagenCalendar::Hijri,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearJapaneseV1,
    DatagenCalendar::Japanese,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearPersianV1,
    DatagenCalendar::Persian,
    YEARS_MARKER_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DatetimeNamesYearRocV1,
    DatagenCalendar::Roc,
    YEARS_MARKER_LENGTHS,
    years_convert
);

// Months
impl_symbols_datagen!(
    DatetimeNamesMonthBuddhistV1,
    DatagenCalendar::Buddhist,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthChineseV1,
    DatagenCalendar::Chinese,
    MONTHS_MARKER_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthCopticV1,
    DatagenCalendar::Coptic,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthDangiV1,
    DatagenCalendar::Dangi,
    MONTHS_MARKER_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthEthiopianV1,
    DatagenCalendar::Ethiopic,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthGregorianV1,
    DatagenCalendar::Gregorian,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthHebrewV1,
    DatagenCalendar::Hebrew,
    MONTHS_MARKER_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthIndianV1,
    DatagenCalendar::Indian,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthHijriV1,
    DatagenCalendar::Hijri,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthJapaneseV1,
    DatagenCalendar::Japanese,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthPersianV1,
    DatagenCalendar::Persian,
    MONTHS_MARKER_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DatetimeNamesMonthRocV1,
    DatagenCalendar::Roc,
    MONTHS_MARKER_LENGTHS,
    months_convert
);

// Datetime patterns
// TODO: This is modeled with glue patterns that are the same across calendar
// systems, but CLDR has some instances where the glue patterns differ, such
// as in French (Gregorian has a comma but other calendars do not).
impl_pattern_datagen!(
    DatetimePatternsGlueV1,
    DatagenCalendar::Gregorian,
    GLUE_PATTERN_MARKER_LENGTHS,
    datetimepattern_convert
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::langid;
    #[test]
    fn test_basic_symbols() {
        let provider = SourceDataProvider::new_testing();
        let dl: DataLocale = langid!("cs").into();
        let data = provider
            .get_dates_resource(&dl, Some(DatagenCalendar::Gregorian))
            .unwrap();

        // let cs_dates = convert_dates(data, DatagenCalendar::Gregorian);

        let months_wide = months_convert(
            &provider,
            &dl,
            data,
            DatagenCalendar::Gregorian,
            Context::Format,
            Length::Wide,
        )
        .unwrap();
        let MonthNames::Linear(months) = months_wide else {
            panic!("Must be linear for Gregorian");
        };

        assert_eq!("srpna", &months[7]);

        let wd_short = weekday_convert(
            &provider,
            &dl,
            data,
            DatagenCalendar::Gregorian,
            Context::Format,
            Length::Short,
        )
        .unwrap();

        assert_eq!(
            "po",
            wd_short.get(icu::calendar::types::Weekday::Monday).unwrap()
        );
    }
}
