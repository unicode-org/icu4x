// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_calendar::provider::EraStartDate;
use icu_datetime::provider::calendar::*;
use icu_locid::{
    extensions::unicode::{key, value},
    Locale,
};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

mod patterns;
mod skeletons;
mod symbols;
pub mod week_data;

lazy_static! {
    // BCP-47 value -> CLDR identifier
    static ref SUPPORTED_CALS: HashMap<icu_locid::extensions::unicode::Value, &'static str> = [
        (value!("gregory"), "gregorian"),
        (value!("buddhist"), "buddhist"),
        (value!("japanese"), "japanese"),
        (value!("japanext"), "japanese"),
        (value!("coptic"), "coptic"),
        (value!("indian"), "indian"),
        (value!("ethiopic"), "ethiopic"),
    ]
    .into_iter()
    .collect();
}

macro_rules! impl_data_provider {
    ($marker:ident, $expr:expr, calendared = $calendared:expr) => {
        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                if $calendared == "locale" && req.locale.is_empty() {
                    return Err(DataErrorKind::NeedsLocale.into_error());
                }

                let langid = req.locale.get_langid();
                let calendar = if $calendared == "locale" {
                    req.locale
                        .get_unicode_ext(&key!("ca"))
                        .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?
                } else if $calendared == "false" {
                    value!("gregory")
                } else {
                    value!($calendared)
                };

                let cldr_cal = SUPPORTED_CALS
                    .get(&calendar)
                    .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;

                let resource: &cldr_serde::ca::Resource = self
                    .source
                    .cldr()?
                    .dates(cldr_cal)
                    .read_and_parse(&langid, &format!("ca-{}.json", cldr_cal))?;

                let mut data = resource
                    .main
                    .0
                    .get(&langid)
                    .expect("CLDR file contains the expected language")
                    .dates
                    .calendars
                    .get(*cldr_cal)
                    .expect("CLDR file contains the expected calendar")
                    .clone();

                // CLDR treats ethiopian and ethioaa as separate calendars; however we treat them as a single resource key that
                // supports symbols for both era patterns based on the settings on the date. Load in ethioaa data as well when dealing with
                // ethiopian.
                if calendar == value!("ethiopic") {
                    let ethioaa: &cldr_serde::ca::Resource = self
                        .source
                        .cldr()?
                        .dates("ethiopic")
                        .read_and_parse(&langid, "ca-ethiopic-amete-alem.json")?;

                    let ethioaa_data = ethioaa
                        .main
                        .0
                        .get(&langid)
                        .expect("CLDR ca-ethiopic-amete-alem.json contains the expected language")
                        .dates
                        .calendars
                        .get("ethiopic-amete-alem")
                        .expect("CLDR ca-ethiopic-amete-alem.json contains the expected calendar")
                        .clone();

                    let mundi_name = ethioaa_data
                        .eras
                        .names
                        .get("0")
                        .expect("ethiopic-amete-alem calendar must have 0 era");
                    let mundi_abbr = ethioaa_data
                        .eras
                        .abbr
                        .get("0")
                        .expect("ethiopic-amete-alem calendar must have 0 era");
                    let mundi_narrow = ethioaa_data
                        .eras
                        .narrow
                        .get("0")
                        .expect("ethiopic-amete-alem calendar must have 0 era");

                    data.eras.names.insert("2".to_string(), mundi_name.clone());
                    data.eras.abbr.insert("2".to_string(), mundi_abbr.clone());
                    data.eras
                        .narrow
                        .insert("2".to_string(), mundi_narrow.clone());
                }

                if calendar == value!("japanese") || calendar == value!("japanext") {
                    // Filter out non-modern eras
                    if calendar != value!("japanext") {
                        let era_dates: &cldr_serde::japanese::Resource = self
                            .source
                            .cldr()?
                            .core()
                            .read_and_parse("supplemental/calendarData.json")?;
                        let mut set = HashSet::<String>::new();
                        for (era_index, date) in
                            era_dates.supplemental.calendar_data.japanese.eras.iter()
                        {
                            let start_date =
                                EraStartDate::from_str(if let Some(start_date) = date.start.as_ref() { start_date } else { continue }).map_err(|_| {
                                    DataError::custom(
                                        "calendarData.json contains unparseable data for a japanese era",
                                    )
                                    .with_display_context(&format!("era index {}", era_index))
                                })?;

                            if start_date.year >= 1868 {
                                set.insert(era_index.into());
                            }
                        }

                        data.eras.names.retain(|e, _| set.contains(e));
                        data.eras.abbr.retain(|e, _| set.contains(e));
                        data.eras.narrow.retain(|e, _| set.contains(e));
                    }

                    // Splice in gregorian data for pre-meiji
                    let greg_resource: &cldr_serde::ca::Resource = self
                        .source
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(&langid, "ca-gregorian.json")?;

                    let greg = greg_resource
                        .main
                        .0
                        .get(&langid)
                        .expect("CLDR file contains the expected language")
                        .dates
                        .calendars
                        .get("gregorian")
                        .expect("CLDR file contains a gregorian calendar")
                        .clone();

                    data.eras.names.insert(
                        "-2".into(),
                        greg.eras
                            .names
                            .get("0")
                            .expect("Gregorian calendar must have data for BC")
                            .into(),
                    );
                    data.eras.names.insert(
                        "-1".into(),
                        greg.eras
                            .names
                            .get("1")
                            .expect("Gregorian calendar must have data for AD")
                            .into(),
                    );
                    data.eras.abbr.insert(
                        "-2".into(),
                        greg.eras
                            .abbr
                            .get("0")
                            .expect("Gregorian calendar must have data for BC")
                            .into(),
                    );
                    data.eras.abbr.insert(
                        "-1".into(),
                        greg.eras
                            .abbr
                            .get("1")
                            .expect("Gregorian calendar must have data for AD")
                            .into(),
                    );
                    data.eras.narrow.insert(
                        "-2".into(),
                        greg.eras
                            .narrow
                            .get("0")
                            .expect("Gregorian calendar must have data for BC")
                            .into(),
                    );
                    data.eras.narrow.insert(
                        "-1".into(),
                        greg.eras
                            .narrow
                            .get("1")
                            .expect("Gregorian calendar must have data for AD")
                            .into(),
                    );
                }

                Ok(DataResponse {
                    metadata: Default::default(),
                    #[allow(clippy::redundant_closure_call)]
                    payload: Some(DataPayload::from_owned(($expr)(
                        &data,
                        &calendar.to_string(),
                    ))),
                })
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                let mut r = Vec::new();
                if $calendared == "locale" {
                    for (cal_value, cldr_cal) in &*SUPPORTED_CALS {
                        r.extend(
                            self.source
                                .cldr()?
                                .dates(cldr_cal)
                                .list_langs()?
                                .map(|lid| {
                                    let mut locale: Locale = lid.into();
                                    locale
                                        .extensions
                                        .unicode
                                        .keywords
                                        .set(key!("ca"), cal_value.clone());
                                    DataLocale::from(locale)
                                }),
                        );
                    }
                } else {
                    let calendar = if $calendared == "false" {
                        value!("gregory")
                    } else {
                        value!($calendared)
                    };
                    let cldr_cal = SUPPORTED_CALS
                        .get(&calendar)
                        .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;
                    r.extend(
                        self.source
                            .cldr()?
                            .dates(cldr_cal)
                            .list_langs()?
                            .map(|lid| {
                                let locale: Locale = lid.into();
                                DataLocale::from(locale)
                            }),
                    );
                }

                // TODO(#3212): Remove
                if $marker::KEY == TimeLengthsV1Marker::KEY {
                    r.retain(|l| l.get_langid() != icu_locid::langid!("byn") && l.get_langid() != icu_locid::langid!("ssy"));
                }

                Ok(self.filter_data_locales(r))
            }
        }
    };
}

impl_data_provider!(
    GregorianDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "gregory"
);
impl_data_provider!(
    BuddhistDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "buddhist"
);
impl_data_provider!(
    JapaneseDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "japanese"
);
impl_data_provider!(
    JapaneseExtendedDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "japanext"
);
impl_data_provider!(
    CopticDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "coptic"
);
impl_data_provider!(
    IndianDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "indian"
);
impl_data_provider!(
    EthiopianDateSymbolsV1Marker,
    symbols::convert_dates,
    calendared = "ethiopic"
);
impl_data_provider!(
    TimeSymbolsV1Marker,
    |dates, _| { symbols::convert_times(dates) },
    calendared = "false"
);
impl_data_provider!(
    DateSkeletonPatternsV1Marker,
    |dates, _| { DateSkeletonPatternsV1::from(dates) },
    calendared = "locale"
);
impl_data_provider!(
    GregorianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "gregory"
);
impl_data_provider!(
    BuddhistDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "buddhist"
);
impl_data_provider!(
    JapaneseDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "japanese"
);
impl_data_provider!(
    JapaneseExtendedDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "japanext"
);
impl_data_provider!(
    CopticDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "coptic"
);
impl_data_provider!(
    IndianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "indian"
);
impl_data_provider!(
    EthiopianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    calendared = "ethiopic"
);
impl_data_provider!(
    TimeLengthsV1Marker,
    |dates, _| TimeLengthsV1::from(dates),
    calendared = "false"
);

#[cfg(test)]
mod test {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn test_basic_patterns() {
        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = locale!("cs");
        let cs_dates: DataPayload<GregorianDateLengthsV1Marker> = provider
            .load(DataRequest {
                locale: &locale.into(),
                metadata: Default::default(),
            })
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");

        assert_eq!("d. M. y", cs_dates.get().date.medium.to_string());
    }

    #[test]
    fn test_with_numbering_system() {
        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = locale!("haw");
        let cs_dates: DataPayload<GregorianDateLengthsV1Marker> = provider
            .load(DataRequest {
                locale: &locale.into(),
                metadata: Default::default(),
            })
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");

        assert_eq!("d MMM y", cs_dates.get().date.medium.to_string());
        // TODO(#308): Support numbering system variations. We currently throw them away.
        assert_eq!("d/M/yy", cs_dates.get().date.short.to_string());
    }

    #[test]
    fn test_datetime_skeletons() {
        use icu_datetime::pattern::runtime::{Pattern, PluralPattern};
        use icu_plurals::PluralCategory;
        use std::convert::TryFrom;

        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = "fil-u-ca-gregory".parse().unwrap();
        let skeletons: DataPayload<DateSkeletonPatternsV1Marker> = provider
            .load(DataRequest {
                locale: &locale.into(),
                metadata: Default::default(),
            })
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");
        let skeletons = &skeletons.get().0;

        assert_eq!(
            Some(
                &"L".parse::<Pattern>()
                    .expect("Failed to create pattern")
                    .into()
            ),
            skeletons.get(&SkeletonV1::try_from("M").expect("Failed to create Skeleton"))
        );

        let mut expected = PluralPattern::new(
            "'linggo' w 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        )
        .expect("Failed to create PatternPlurals");
        expected.maybe_set_variant(
            PluralCategory::One,
            "'ika'-w 'linggo' 'ng' Y"
                .parse()
                .expect("Failed to create pattern"),
        );
        assert_eq!(
            Some(&expected.into()),
            skeletons.get(&SkeletonV1::try_from("yw").expect("Failed to create Skeleton"))
        );
    }

    #[test]
    fn test_basic_symbols() {
        use icu_calendar::types::MonthCode;
        use tinystr::tinystr;
        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = locale!("cs");
        let cs_dates: DataPayload<GregorianDateSymbolsV1Marker> = provider
            .load(DataRequest {
                locale: &locale.into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            "srpna",
            cs_dates
                .get()
                .months
                .format
                .wide
                .get(MonthCode(tinystr!(4, "M08")))
                .unwrap()
        );

        assert_eq!(
            "po",
            cs_dates.get().weekdays.format.short.as_ref().unwrap().0[1]
        );
    }

    #[test]
    fn unalias_contexts() {
        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = locale!("cs");
        let cs_dates: DataPayload<GregorianDateSymbolsV1Marker> = provider
            .load(DataRequest {
                locale: &locale.into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates.get().months.stand_alone.is_some());

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .abbreviated
            .is_none());
        assert!(cs_dates
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .short
            .is_none());
        assert!(cs_dates
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .narrow
            .is_none());
        assert!(cs_dates
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .wide
            .is_some());

        // Czech weekdays are unaliased because they completely overlap.
        assert!(cs_dates.get().weekdays.stand_alone.is_none());
    }
}
