// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::supported_cals;
use crate::transform::cldr::cldr_serde::ca::{self, Context, Length, PatternLength};
use crate::DatagenProvider;
use icu_datetime::pattern;

use icu_datetime::provider::neo::*;
use icu_locid::{
    extensions::private::{subtag, Subtag},
    extensions::unicode::{value, Value},
    LanguageIdentifier, Locale,
};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use tinystr::{tinystr, TinyAsciiStr, UnvalidatedTinyAsciiStr};
use zerovec::ule::UnvalidatedStr;

const ABBR: Subtag = subtag!("3");
const NARROW: Subtag = subtag!("4");
const WIDE: Subtag = subtag!("5");
const SHORT: Subtag = subtag!("6");
const ABBR_STANDALONE: Subtag = subtag!("3s");
const NARROW_STANDALONE: Subtag = subtag!("4s");
const WIDE_STANDALONE: Subtag = subtag!("5s");
const SHORT_STANDALONE: Subtag = subtag!("6s");

const PATTERN_FULL: Subtag = subtag!("f");
const PATTERN_LONG: Subtag = subtag!("l");
const PATTERN_MEDIUM: Subtag = subtag!("m");
const PATTERN_SHORT: Subtag = subtag!("s");

const PATTERN_FULL12: Subtag = subtag!("f12");
const PATTERN_LONG12: Subtag = subtag!("l12");
const PATTERN_MEDIUM12: Subtag = subtag!("m12");
const PATTERN_SHORT12: Subtag = subtag!("s12");

const PATTERN_FULL24: Subtag = subtag!("f24");
const PATTERN_LONG24: Subtag = subtag!("l24");
const PATTERN_MEDIUM24: Subtag = subtag!("m24");
const PATTERN_SHORT24: Subtag = subtag!("s24");

fn aux_subtag_info(subtag: Subtag) -> (Context, Length) {
    use {Context::*, Length::*};
    match subtag {
        ABBR => (Format, Abbr),
        NARROW => (Format, Narrow),
        WIDE => (Format, Wide),
        SHORT => (Format, Short),
        ABBR_STANDALONE => (Standalone, Abbr),
        NARROW_STANDALONE => (Standalone, Narrow),
        WIDE_STANDALONE => (Standalone, Wide),
        SHORT_STANDALONE => (Standalone, Short),
        _ => panic!("Found unexpected auxiliary subtag {}", subtag),
    }
}

fn aux_pattern_subtag_info(subtag: Subtag) -> (PatternLength, Option<pattern::CoarseHourCycle>) {
    use {pattern::CoarseHourCycle::*, PatternLength::*};
    match subtag {
        PATTERN_FULL => (Full, None),
        PATTERN_LONG => (Long, None),
        PATTERN_MEDIUM => (Medium, None),
        PATTERN_SHORT => (Short, None),

        PATTERN_FULL12 => (Full, Some(H11H12)),
        PATTERN_LONG12 => (Long, Some(H11H12)),
        PATTERN_MEDIUM12 => (Medium, Some(H11H12)),
        PATTERN_SHORT12 => (Short, Some(H11H12)),

        PATTERN_FULL24 => (Full, Some(H23H24)),
        PATTERN_LONG24 => (Long, Some(H23H24)),
        PATTERN_MEDIUM24 => (Medium, Some(H23H24)),
        PATTERN_SHORT24 => (Short, Some(H23H24)),
        _ => panic!("Found unexpected auxiliary subtag {}", subtag),
    }
}

/// Most keys don't have short symbols (except weekdays)
///
/// We may further investigate and kick out standalone for some keys
const NORMAL_KEY_LENGTHS: &[Subtag] = &[
    ABBR,
    NARROW,
    WIDE,
    ABBR_STANDALONE,
    NARROW_STANDALONE,
    WIDE_STANDALONE,
];

const YEARS_KEY_LENGTHS: &[Subtag] = &[ABBR, NARROW, WIDE];
/// All possible lengths
const FULL_KEY_LENGTHS: &[Subtag] = &[
    ABBR,
    NARROW,
    WIDE,
    SHORT,
    ABBR_STANDALONE,
    NARROW_STANDALONE,
    WIDE_STANDALONE,
    SHORT_STANDALONE,
];

const NORMAL_PATTERN_KEY_LENGTHS: &[Subtag] =
    &[PATTERN_FULL, PATTERN_LONG, PATTERN_MEDIUM, PATTERN_SHORT];

const H12_PATTERN_KEY_LENGTHS: &[Subtag] = &[
    PATTERN_FULL,
    PATTERN_LONG,
    PATTERN_MEDIUM,
    PATTERN_SHORT,
    PATTERN_FULL12,
    PATTERN_LONG12,
    PATTERN_MEDIUM12,
    PATTERN_SHORT12,
];

const H24_PATTERN_KEY_LENGTHS: &[Subtag] = &[
    PATTERN_FULL,
    PATTERN_LONG,
    PATTERN_MEDIUM,
    PATTERN_SHORT,
    PATTERN_FULL24,
    PATTERN_LONG24,
    PATTERN_MEDIUM24,
    PATTERN_SHORT24,
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
        self.check_req::<M>(req)?;
        let langid = req.locale.get_langid();
        let private = &req
            .locale
            .get_aux()
            .expect("Symbols data provider called without aux subtag");
        let data = self.load_calendar_dates(&langid, &calendar)?;

        let mut aux_iter = private.iter();
        let aux = aux_iter
            .next()
            .expect("Symbols data provider called with empty aux subtag");
        assert!(
            aux_iter.next().is_none(),
            "Symbols data provider called with too many aux subtags"
        );
        let (context, length) = aux_subtag_info(aux);

        let data = conversion(self, &langid, data, &calendar, context, length)?;

        #[allow(clippy::redundant_closure_call)]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
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
) -> Result<LinearSymbolsV1<'static>, DataError> {
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

    Ok(LinearSymbolsV1 {
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
) -> Result<LinearSymbolsV1<'static>, DataError> {
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

    Ok(LinearSymbolsV1 {
        symbols: (&periods).into(),
    })
}

fn years_convert(
    datagen: &DatagenProvider,
    langid: &LanguageIdentifier,
    data: &ca::Dates,
    calendar: &Value,
    context: Context,
    length: Length,
) -> Result<YearSymbolsV1<'static>, DataError> {
    assert_eq!(
        context,
        Context::Format,
        "Eras do not participate in standalone formatting"
    );

    let eras = data.eras.load(length);

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

        let eras = greg_data.eras.load(length);
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

    // Todo: Cyclic years (#3761)
    Ok(YearSymbolsV1::Eras(
        out_eras
            .iter()
            .map(|(k, v)| (UnvalidatedStr::from_str(k), &**v))
            .collect(),
    ))
}

fn months_convert(
    _datagen: &DatagenProvider,
    _langid: &LanguageIdentifier,
    data: &ca::Dates,
    calendar: &Value,
    context: Context,
    length: Length,
) -> Result<MonthSymbolsV1<'static>, DataError> {
    let months = data.months.get_symbols(context, length);

    // Tostring can be removed when we delete symbols.rs, or we can perhaps refactor it to use Value
    let calendar_str = calendar.to_string();
    let (map, has_leap) = super::symbols::get_month_code_map(&calendar_str);

    if has_leap {
        let mut symbols: BTreeMap<UnvalidatedTinyAsciiStr<4>, &str> = BTreeMap::new();
        for (k, v) in months.0.iter() {
            let code = if k == "7-yeartype-leap" && *calendar == value!("hebrew") {
                tinystr!(4, "M06L")
            } else {
                let index: usize = k
                    .parse()
                    .expect("CLDR month indices must parse as numbers!");

                if index == 0 {
                    panic!("CLDR month indices cannot be zero");
                }
                *map.get(index - 1)
                    .expect("Found out of bounds month index for calendar")
            };

            symbols.insert(code.into(), &**v);
        }
        Ok(MonthSymbolsV1::Map(symbols.into_iter().collect()))
    } else {
        let mut symbols = vec![""; map.len()];

        for (k, v) in months.0.iter() {
            let index: usize = k
                .parse()
                .expect("CLDR month indices must parse as numbers!");
            if index == 0 {
                panic!("CLDR month indices cannot be zero");
            }

            symbols[index - 1] = &v;
        }
        for (i, val) in symbols.iter().enumerate() {
            if val.is_empty() {
                panic!("Calendar {calendar} does not have data for month {i}; found data for {symbols:?}");
            }
        }
        Ok(MonthSymbolsV1::Numeric((&symbols).into()))
    }
}

macro_rules! impl_symbols_datagen {
    ($marker:ident, $calendar:expr, $lengths:ident, $convert:expr) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_key::<$marker>(req, value!($calendar), $convert)
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
    WeekdaySymbolsV1Marker,
    "gregory",
    FULL_KEY_LENGTHS,
    weekday_convert
);

// Dayperiods
impl_symbols_datagen!(
    DayPeriodSymbolsV1Marker,
    "gregory",
    NORMAL_KEY_LENGTHS,
    dayperiods_convert
);

// Years
impl_symbols_datagen!(
    BuddhistYearSymbolsV1Marker,
    "buddhist",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    ChineseYearSymbolsV1Marker,
    "chinese",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    CopticYearSymbolsV1Marker,
    "coptic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    DangiYearSymbolsV1Marker,
    "dangi",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    EthiopianYearSymbolsV1Marker,
    "ethiopic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    GregorianYearSymbolsV1Marker,
    "gregory",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    HebrewYearSymbolsV1Marker,
    "hebrew",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    IndianYearSymbolsV1Marker,
    "indian",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    IslamicYearSymbolsV1Marker,
    "islamic",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    JapaneseYearSymbolsV1Marker,
    "japanese",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    JapaneseExtendedYearSymbolsV1Marker,
    "japanext",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    PersianYearSymbolsV1Marker,
    "persian",
    YEARS_KEY_LENGTHS,
    years_convert
);
impl_symbols_datagen!(
    RocYearSymbolsV1Marker,
    "roc",
    YEARS_KEY_LENGTHS,
    years_convert
);

// Months
impl_symbols_datagen!(
    BuddhistMonthSymbolsV1Marker,
    "buddhist",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    ChineseMonthSymbolsV1Marker,
    "chinese",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    CopticMonthSymbolsV1Marker,
    "coptic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    DangiMonthSymbolsV1Marker,
    "dangi",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    EthiopianMonthSymbolsV1Marker,
    "ethiopic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    GregorianMonthSymbolsV1Marker,
    "gregory",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    HebrewMonthSymbolsV1Marker,
    "hebrew",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    IndianMonthSymbolsV1Marker,
    "indian",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    IslamicMonthSymbolsV1Marker,
    "islamic",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    JapaneseMonthSymbolsV1Marker,
    "japanese",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    JapaneseExtendedMonthSymbolsV1Marker,
    "japanext",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    PersianMonthSymbolsV1Marker,
    "persian",
    NORMAL_KEY_LENGTHS,
    months_convert
);
impl_symbols_datagen!(
    RocMonthSymbolsV1Marker,
    "roc",
    NORMAL_KEY_LENGTHS,
    months_convert
);
