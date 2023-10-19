// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::supported_cals;
use crate::transform::cldr::cldr_serde::ca::{self, Context, Length};

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

impl crate::DatagenProvider {
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
            .read_and_parse(&langid, &format!("ca-{}.json", cldr_cal))?;

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
        conversion: impl FnOnce(&ca::Dates, &Value, Context, Length) -> Result<M::Yokeable, DataError>,
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

        let data = conversion(&data, &calendar, context, length)?;

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
            keylengths.into_iter().map(move |length| {
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

    let eras = match length {
        Length::Abbr => &data.eras.abbr,
        Length::Narrow => &data.eras.narrow,
        Length::Wide => &data.eras.names,
        Length::Short => unreachable!("Years do not have short symbols!"),
    };

    // Tostring can be removed when we delete symbols.rs, or we can perhaps refactor it to use Value
    let calendar_str = calendar.to_string();
    let map = super::symbols::get_era_code_map(&calendar_str);
    let mut out_eras: BTreeMap<TinyAsciiStr<16>, &str> = BTreeMap::new();

    for (cldr, code) in map {
        if let Some(name) = eras.get(&*cldr) {
            out_eras.insert(code, &**name);
        }
    }
    // Todo: Cyclic years (#3761)
    Ok(YearSymbolsV1::Eras(
        out_eras
            .iter()
            .map(|(k, v)| (UnvalidatedStr::from_str(&k), &**v))
            .collect(),
    ))
}

fn months_convert(
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
        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.load_neo_key::<$marker>(req, value!($calendar), $convert)
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
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
