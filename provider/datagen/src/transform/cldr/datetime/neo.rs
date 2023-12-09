// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::supported_cals;
use crate::transform::cldr::cldr_serde::ca;
use crate::DatagenProvider;
use icu_datetime::pattern::{self, CoarseHourCycle};

use icu_datetime::provider::calendar::{patterns::GenericLengthPatternsV1, DateSkeletonPatternsV1};
use icu_datetime::provider::neo::aux::{self, Context, Length, PatternLength};
use icu_datetime::provider::neo::*;
use icu_locid::{
    extensions::private::Subtag,
    extensions::unicode::{value, Value},
    LanguageIdentifier, Locale,
};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use tinystr::TinyAsciiStr;
use zerovec::ule::UnvalidatedStr;

/// Most keys don't have short symbols (except weekdays)
///
/// We may further investigate and kick out standalone for some keys
const NORMAL_KEY_LENGTHS: &[Subtag] = &[
    aux::ABBR,
    aux::NARROW,
    aux::WIDE,
    aux::ABBR_STANDALONE,
    aux::NARROW_STANDALONE,
    aux::WIDE_STANDALONE,
];

/// Lengths for month data (NORMAL_KEY_LENGTHS + numeric)
const NUMERIC_MONTHS_KEY_LENGTHS: &[Subtag] = &[
    aux::ABBR,
    aux::NARROW,
    aux::WIDE,
    aux::ABBR_STANDALONE,
    aux::NARROW_STANDALONE,
    aux::WIDE_STANDALONE,
    aux::NUMERIC,
];

/// Lengths for year data (does not do standalone formatting)
const YEARS_KEY_LENGTHS: &[Subtag] = &[aux::ABBR, aux::NARROW, aux::WIDE];

/// All possible non-numeric lengths
const FULL_KEY_LENGTHS: &[Subtag] = &[
    aux::ABBR,
    aux::NARROW,
    aux::WIDE,
    aux::SHORT,
    aux::ABBR_STANDALONE,
    aux::NARROW_STANDALONE,
    aux::WIDE_STANDALONE,
    aux::SHORT_STANDALONE,
];

/// Lengths for normal patterns (not counting hour cycle stuff)
const NORMAL_PATTERN_KEY_LENGTHS: &[Subtag] = &[
    aux::PATTERN_FULL,
    aux::PATTERN_LONG,
    aux::PATTERN_MEDIUM,
    aux::PATTERN_SHORT,
];

impl DatagenProvider {
    fn load_calendar_dates(
        &self,
        langid: &LanguageIdentifier,
        calendar: &Value,
    ) -> Result<&ca::Dates, DataError> {
        let cldr_cal = supported_cals()
            .get(calendar)
            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;

        let resource: &ca::Resource = self
            .cldr()?
            .dates(cldr_cal)
            .read_and_parse(langid, &format!("ca-{}.json", cldr_cal))?;

        let data = resource
            .main
            .value
            .dates
            .calendars
            .get(*cldr_cal)
            .expect("CLDR file contains the expected calendar");
        Ok(data)
    }

    fn load_neo_key<M: KeyedDataMarker>(
        &self,
        req: DataRequest,
        calendar: &Value,
        conversion: impl FnOnce(
            &LanguageIdentifier,
            &ca::Dates,
            Subtag,
        ) -> Result<M::Yokeable, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProvider<M>,
    {
        self.check_req::<M>(req)?;
        let langid = req.locale.get_langid();
        let private = &req
            .locale
            .get_aux()
            .expect("Symbols data provider called without aux subtag");
        let data = self.load_calendar_dates(&langid, calendar)?;

        let mut aux_iter = private.iter();
        let aux = aux_iter
            .next()
            .expect("Symbols data provider called with empty aux subtag");
        assert!(
            aux_iter.next().is_none(),
            "Symbols data provider called with too many aux subtags"
        );

        let data = conversion(&langid, data, aux)?;

        #[allow(clippy::redundant_closure_call)]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }

    fn load_neo_symbols_key<M: KeyedDataMarker>(
        &self,
        req: DataRequest,
        calendar: Value,
        conversion: impl FnOnce(
            &DatagenProvider,
            &LanguageIdentifier,
            &ca::Dates,
            &Value,
            Context,
            Length,
        ) -> Result<M::Yokeable, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProvider<M>,
    {
        self.load_neo_key(req, &calendar, |langid, data, aux| {
            let Some((context, length)) = aux::symbol_subtag_info(aux) else {
                panic!("Found unexpected auxiliary subtag {}", aux)
            };
            conversion(self, langid, data, &calendar, context, length)
        })
    }

    fn load_neo_patterns_key<M: KeyedDataMarker>(
        &self,
        req: DataRequest,
        calendar: Value,
        conversion: impl FnOnce(
            &ca::Dates,
            PatternLength,
            Option<CoarseHourCycle>,
        ) -> Result<M::Yokeable, DataError>,
    ) -> Result<DataResponse<M>, DataError>
    where
        Self: IterableDataProvider<M>,
    {
        self.load_neo_key(req, &calendar, |_langid, data, aux| {
            let Some((length, hc)) = aux::pattern_subtag_info(aux) else {
                panic!("Found unexpected auxiliary subtag {}", aux)
            };
            conversion(data, length, hc)
        })
    }

    fn supported_locales_neo(
        &self,
        calendar: Value,
        keylengths: &'static [Subtag],
    ) -> Result<Vec<DataLocale>, DataError> {
        let mut r = Vec::new();

        let cldr_cal = supported_cals()
            .get(&calendar)
            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;
        r.extend(self.cldr()?.dates(cldr_cal).list_langs()?.flat_map(|lid| {
            keylengths.iter().map(move |length| {
                let locale: Locale = lid.clone().into();

                let mut locale = DataLocale::from(locale);

                locale.set_aux((*length).into());
                locale
            })
        }));

        Ok(r)
    }
}

fn weekday_convert(
    _datagen: &DatagenProvider,
    _langid: &LanguageIdentifier,
    data: &ca::Dates,
    _calendar: &Value,
    context: Context,
    length: Length,
) -> Result<LinearNamesV1<'static>, DataError> {
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

    Ok(LinearNamesV1 {
        symbols: (&days).into(),
    })
}

fn dayperiods_convert(
    _datagen: &DatagenProvider,
    _langid: &LanguageIdentifier,
    data: &ca::Dates,
    _calendar: &Value,
    context: Context,
    length: Length,
) -> Result<LinearNamesV1<'static>, DataError> {
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

    Ok(LinearNamesV1 {
        symbols: (&periods).into(),
    })
}

fn eras_convert(
    datagen: &DatagenProvider,
    langid: &LanguageIdentifier,
    eras: &ca::Eras,
    calendar: &Value,
    length: Length,
) -> Result<YearNamesV1<'static>, DataError> {
    let eras = eras.load(length);
    // Tostring can be removed when we delete symbols.rs, or we can perhaps refactor it to use Value
    let calendar_str = calendar.to_string();
    let map = super::symbols::get_era_code_map(&calendar_str);
    let mut out_eras: BTreeMap<TinyAsciiStr<16>, &str> = BTreeMap::new();

    // CLDR treats ethiopian and ethioaa as separate calendars; however we treat them as a single resource key that
    // supports symbols for both era patterns based on the settings on the date. Load in ethioaa data as well when dealing with
    // ethiopian.
    let extra_ethiopic = if *calendar == value!("ethiopic") {
        let ethioaa: &ca::Resource = datagen
            .cldr()?
            .dates("ethiopic")
            .read_and_parse(langid, "ca-ethiopic-amete-alem.json")?;

        let ethioaa_data = ethioaa
            .main
            .value
            .dates
            .calendars
            .get("ethiopic-amete-alem")
            .expect("CLDR ca-ethiopic-amete-alem.json contains the expected calendar");

        Some(
            ethioaa_data
                .eras
                .as_ref()
                .expect("ethiopic-amete-alem must have eras")
                .load(length)
                .get("0")
                .expect("ethiopic-amete-alem calendar must have 0 era"),
        )
    } else {
        None
    };

    let modern_japanese_eras = if *calendar == value!("japanese") {
        Some(datagen.cldr()?.modern_japanese_eras()?)
    } else {
        None
    };

    let extra_japanese = if *calendar == value!("japanese") || *calendar == value!("japanext") {
        let greg: &ca::Resource = datagen
            .cldr()?
            .dates("gregorian")
            .read_and_parse(langid, "ca-gregorian.json")?;

        let greg_data = greg
            .main
            .value
            .dates
            .calendars
            .get("gregorian")
            .expect("CLDR gregorian.json contains the expected calendar");

        let eras = greg_data
            .eras
            .as_ref()
            .expect("gregorian must have eras")
            .load(length);
        Some((
            eras.get("0").expect("gregorian calendar must have 0 era"),
            eras.get("1").expect("gregorian calendar must have 1 era"),
        ))
    } else {
        None
    };

    for (cldr, code) in map {
        if let Some(name) = eras.get(cldr) {
            if let Some(modern_japanese_eras) = modern_japanese_eras {
                if !modern_japanese_eras.contains(cldr) {
                    continue;
                }
            }
            out_eras.insert(code, &**name);
        } else if let Some(extra_ethiopic) = extra_ethiopic {
            if cldr == "2" {
                out_eras.insert(code, extra_ethiopic);
            } else {
                panic!("Unknown ethiopic era number {cldr}");
            }
        } else if let Some(extra_japanese) = extra_japanese {
            if cldr == "-1" {
                // AD era
                out_eras.insert(code, extra_japanese.0);
            } else if cldr == "-2" {
                // BC era
                out_eras.insert(code, extra_japanese.1);
            } else {
                panic!("Unknown japanese era number {cldr}");
            }
        } else {
            panic!("Did not find era data for era {code} (#{cldr}) for {calendar} and {langid}");
        }
    }

    Ok(YearNamesV1::Eras(
        out_eras
            .iter()
            .map(|(k, v)| (UnvalidatedStr::from_str(k), &**v))
            .collect(),
    ))
}
fn years_convert(
    datagen: &DatagenProvider,
    langid: &LanguageIdentifier,
    data: &ca::Dates,
    calendar: &Value,
    context: Context,
    length: Length,
) -> Result<YearNamesV1<'static>, DataError> {
    assert_eq!(
        context,
        Context::Format,
        "Eras and cyclic years do not participate in standalone formatting"
    );

    if let Some(ref eras) = data.eras {
        eras_convert(datagen, langid, eras, calendar, length)
    } else if let Some(years) = data
        .cyclic_name_sets
        .as_ref()
        .and_then(|c| c.years.as_ref())
    {
        let years = years.get_symbols(context, length);

        let years: Vec<_> = years.iter().enumerate().map(|(index, (key, value))| {
            if *key as usize != index + 1 {
                panic!("Calendar {calendar} in locale {langid} missing cyclic year name for index {index}");
            }
            &**value
        }).collect();
        Ok(YearNamesV1::Cyclic((&years).into()))
    } else {
        panic!(
            "Calendar {calendar} in locale {langid} has neither eras nor cyclicNameSets for years"
        )
    }
}

/// Returns the number of regular months in a calendar, as well as whether it is
/// has leap months
fn calendar_months(cal: &Value) -> (usize, bool) {
    if *cal == value!("hebrew") || *cal == value!("chinese") || *cal == value!("dangi") {
        (24, true)
    } else if *cal == value!("coptic") || *cal == value!("ethiopic") {
        (13, false)
    } else if *cal == value!("gregory")
        || *cal == value!("buddhist")
        || *cal == value!("japanese")
        || *cal == value!("japanext")
        || *cal == value!("indian")
        || *cal == value!("persian")
        || *cal == value!("roc")
        || *cal == value!("islamic")
        || *cal == value!("islamicc")
        || *cal == value!("umalqura")
        || *cal == value!("tbla")
    {
        (12, false)
    } else {
        panic!("Month map unknown for {cal}")
    }
}

fn months_convert(
    _datagen: &DatagenProvider,
    langid: &LanguageIdentifier,
    data: &ca::Dates,
    calendar: &Value,
    context: Context,
    length: Length,
) -> Result<MonthNamesV1<'static>, DataError> {
    if length == Length::Numeric {
        assert_eq!(
            context,
            Context::Format,
            "numeric months only found for Context::Format"
        );
        let Some(ref patterns) = data.month_patterns else {
            panic!("No month_patterns found but numeric months were requested for {calendar} with {langid}");
        };
        let pattern = patterns.get_symbols(context, length);
        let leap = &pattern.leap;
        let Some(index) = leap.find("{0}") else {
            panic!("Calendar {calendar} for {langid} has leap pattern {leap}, which does not contain {{0}} placeholder");
        };
        let string = leap.replace("{0}", "");
        let pattern = SimpleSubstitutionPattern {
            pattern: string.into(),
            subst_index: index,
        };
        return Ok(MonthNamesV1::LeapNumeric(pattern));
    }

    let months = data.months.get_symbols(context, length);

    let (month_count, has_leap) = calendar_months(calendar);
    let mut symbols = vec![Cow::Borrowed(""); month_count];

    if *calendar == value!("hebrew") {
        for (k, v) in months.0.iter() {
            // CLDR's numbering for hebrew has Adar I as 6, Adar as 7, and Adar II as 7-yeartype-leap
            //
            // So we need to handle the 6 and 7 cases as leap months, and everything after 6 needs to
            // be offset by 1
            let index = if k == "7-yeartype-leap" {
                7 + 11
            } else if k == "6" {
                6 + 11
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
        Ok(MonthNamesV1::LeapLinear((&symbols).into()))
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
                panic!("Calendar {calendar} does not have data for month {i}; found data for {symbols:?}");
            }
        }

        if has_leap {
            // This branch is only for chinese-like calendars with N regular months and N potential leap months
            // rather than hebrew-like where there's one or two special leap months
            debug_assert!(
                *calendar != value!("hebrew"),
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
                let replaced = leap.replace("{0}", &symbols[i]);
                symbols[nonleap + i] = replaced.into();
            }
            Ok(MonthNamesV1::LeapLinear((&symbols).into()))
        } else {
            Ok(MonthNamesV1::Linear((&symbols).into()))
        }
    }
}

/// Given a lengthpattern, apply any numeric overrides it may have to `pattern`
fn apply_numeric_overrides(lp: &ca::LengthPattern, pattern: &mut pattern::runtime::Pattern) {
    use icu_datetime::fields::{self, FieldLength, FieldNumericOverrides::*, FieldSymbol};
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
                    field.length != FieldLength::TwoDigit
                        || symbol_to_replace != Some(field.symbol),
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

fn datepattern_convert(
    data: &ca::Dates,
    length: PatternLength,
    _hc: Option<CoarseHourCycle>,
) -> Result<DatePatternV1<'static>, DataError> {
    let length_pattern = data.date_formats.get_pattern(length);

    let mut pattern = length_pattern
        .get_pattern()
        .parse()
        .expect("failed to parse pattern");

    apply_numeric_overrides(length_pattern, &mut pattern);
    Ok(DatePatternV1 { pattern })
}

fn datetimepattern_convert(
    data: &ca::Dates,
    length: PatternLength,
    _hc: Option<CoarseHourCycle>,
) -> Result<DateTimePatternV1<'static>, DataError> {
    let pattern = data.datetime_formats.get_pattern(length);

    let pattern = pattern
        .get_pattern()
        .parse()
        .expect("failed to parse pattern");
    Ok(DateTimePatternV1 { pattern })
}

fn timepattern_convert(
    data: &ca::Dates,
    length: PatternLength,
    hc: Option<CoarseHourCycle>,
) -> Result<TimePatternV1<'static>, DataError> {
    let pattern = data.time_formats.get_pattern(length);

    let pattern_str = pattern.get_pattern();
    let pattern = pattern_str.parse().expect("failed to parse pattern");
    // We only get an hc if we're generating the non-default hc, so we know we must replace it in the pattern
    let pattern: pattern::runtime::Pattern = if let Some(hc) = hc {
        let length_combinations_v1 = GenericLengthPatternsV1::from(&data.datetime_formats);
        let skeletons_v1 = DateSkeletonPatternsV1::from(data);
        let pattern =
            hc.apply_on_pattern(&length_combinations_v1, &skeletons_v1, pattern_str, pattern);
        let pattern = pattern
            .as_ref()
            .expect("Failed to apply a coarse hour cycle to a full pattern.");
        pattern.into()
    } else {
        (&pattern).into()
    };
    Ok(TimePatternV1 { pattern })
}

/// Looks at the hour cycle in `time_pattern`, and return the subtag related to the *opposite* hour cycle; i.e. the
/// non-default one
fn nondefault_subtag(
    time_pattern: &ca::LengthPattern,
    subtag12: Subtag,
    subtag24: Subtag,
) -> Subtag {
    let pattern = time_pattern.get_pattern();

    let pattern = pattern
        .parse()
        .expect("Failed to create a Pattern from bytes.");

    let hc = CoarseHourCycle::determine(&pattern)
        .expect("Could not find preferred hour cycle in locale");
    match hc {
        // this must invert the hour cycle since we're getting the nondefault one
        CoarseHourCycle::H11H12 => subtag24,
        CoarseHourCycle::H23H24 => subtag12,
    }
}

// Time patterns have a manual implementation since they have custom supported_locales logic below
impl DataProvider<TimePatternV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<TimePatternV1Marker>, DataError> {
        self.load_neo_patterns_key::<TimePatternV1Marker>(
            req,
            value!("gregory"),
            timepattern_convert,
        )
    }
}

// Potential future optimization: if we split out aux subtags from supported_locales into a separate
// method that eagerly generates all aux subtags (even if unused) and expects load() to figure out if the aux
// subtag actually should be produced (by returning a special error), then this code is no longer necessary
// and we can use a union of the H12/H24 key lengths arrays, instead checking for preferred hc
// in timepattern_convert
impl IterableDataProvider<TimePatternV1Marker> for DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        let calendar = value!("gregory");
        let mut r = Vec::new();

        let cldr_cal = supported_cals()
            .get(&calendar)
            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;
        r.extend(self.cldr()?.dates(cldr_cal).list_langs()?.flat_map(|lid| {
            let data = self
                .load_calendar_dates(&lid, &calendar)
                .expect("list_langs returned a language that couldn't be loaded");
            let tp = &data.time_formats;

            let keylengths = [
                aux::PATTERN_FULL,
                aux::PATTERN_LONG,
                aux::PATTERN_MEDIUM,
                aux::PATTERN_SHORT,
                nondefault_subtag(&tp.full, aux::PATTERN_FULL12, aux::PATTERN_FULL24),
                nondefault_subtag(&tp.long, aux::PATTERN_LONG12, aux::PATTERN_LONG24),
                nondefault_subtag(&tp.medium, aux::PATTERN_MEDIUM12, aux::PATTERN_MEDIUM24),
                nondefault_subtag(&tp.short, aux::PATTERN_SHORT12, aux::PATTERN_SHORT24),
            ];
            keylengths.into_iter().map(move |length| {
                let locale: Locale = lid.clone().into();

                let mut locale = DataLocale::from(locale);

                locale.set_aux(length.into());
                locale
            })
        }));

        Ok(r)
    }
}

macro_rules! impl_symbols_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_symbols_key::<$marker>(req, value!($calendar), $convert)
            }
        }

        impl IterableDataProvider<$marker> for DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                self.supported_locales_neo(value!($calendar), $lengths)
            }
        }
    };
}

macro_rules! impl_pattern_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_patterns_key::<$marker>(req, value!($calendar), $convert)
            }
        }

        impl IterableDataProvider<$marker> for DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                self.supported_locales_neo(value!($calendar), $lengths)
            }
        }
    };
}

// Weekdays
impl_symbols_datagen!(
    WeekdayNamesV1Marker,
    "gregory",
    FULL_KEY_LENGTHS,
    weekday_convert
);

// Dayperiods
impl_symbols_datagen!(
    DayPeriodNamesV1Marker,
    "gregory",
    NORMAL_KEY_LENGTHS,
    dayperiods_convert
);

// Years
impl_symbols_datagen!(
    BuddhistYearNamesV1Marker,
    "buddhist",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    ChineseYearNamesV1Marker,
    "chinese",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    CopticYearNamesV1Marker,
    "coptic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DangiYearNamesV1Marker,
    "dangi",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    EthiopianYearNamesV1Marker,
    "ethiopic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    GregorianYearNamesV1Marker,
    "gregory",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    HebrewYearNamesV1Marker,
    "hebrew",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    IndianYearNamesV1Marker,
    "indian",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    IslamicYearNamesV1Marker,
    "islamic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    JapaneseYearNamesV1Marker,
    "japanese",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    JapaneseExtendedYearNamesV1Marker,
    "japanext",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    PersianYearNamesV1Marker,
    "persian",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    RocYearNamesV1Marker,
    "roc",
    YEARS_KEY_LENGTHS,
    years_convert
);

// Months
impl_symbols_datagen!(
    BuddhistMonthNamesV1Marker,
    "buddhist",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    ChineseMonthNamesV1Marker,
    "chinese",
    NUMERIC_MONTHS_KEY_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    CopticMonthNamesV1Marker,
    "coptic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DangiMonthNamesV1Marker,
    "dangi",
    NUMERIC_MONTHS_KEY_LENGTHS, // has leap month patterns
    months_convert
);
impl_symbols_datagen!(
    EthiopianMonthNamesV1Marker,
    "ethiopic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    GregorianMonthNamesV1Marker,
    "gregory",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    HebrewMonthNamesV1Marker,
    "hebrew",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    IndianMonthNamesV1Marker,
    "indian",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    IslamicMonthNamesV1Marker,
    "islamic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    JapaneseMonthNamesV1Marker,
    "japanese",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    JapaneseExtendedMonthNamesV1Marker,
    "japanext",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    PersianMonthNamesV1Marker,
    "persian",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    RocMonthNamesV1Marker,
    "roc",
    NORMAL_KEY_LENGTHS,
    months_convert
);

// Datetime patterns
impl_pattern_datagen!(
    DateTimePatternV1Marker,
    "gregory",
    NORMAL_PATTERN_KEY_LENGTHS,
    datetimepattern_convert
);

// Date patterns
impl_pattern_datagen!(
    BuddhistDatePatternV1Marker,
    "buddhist",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    ChineseDatePatternV1Marker,
    "chinese",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    CopticDatePatternV1Marker,
    "coptic",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    DangiDatePatternV1Marker,
    "dangi",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    EthiopianDatePatternV1Marker,
    "ethiopic",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    GregorianDatePatternV1Marker,
    "gregory",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    HebrewDatePatternV1Marker,
    "hebrew",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    IndianDatePatternV1Marker,
    "indian",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    IslamicDatePatternV1Marker,
    "islamic",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    JapaneseDatePatternV1Marker,
    "japanese",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    JapaneseExtendedDatePatternV1Marker,
    "japanext",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    PersianDatePatternV1Marker,
    "persian",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
impl_pattern_datagen!(
    RocDatePatternV1Marker,
    "roc",
    NORMAL_PATTERN_KEY_LENGTHS,
    datepattern_convert
);
