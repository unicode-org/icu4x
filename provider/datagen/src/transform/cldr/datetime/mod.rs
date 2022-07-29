// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_calendar::provider::EraStartDate;
use icu_datetime::provider::calendar::*;
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
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
    ($(($marker:ident, $expr:expr, calendared = $calendared:expr)),+) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(
                    &self,
                    req: DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    if $calendared && req.locale.is_empty() {
                        return Err(DataErrorKind::NeedsLocale.into_error());
                    }

                    let langid = req.locale.get_langid();
                    let calendar = if $calendared {
                        req
                            .locale
                            .get_unicode_ext(&key!("ca"))
                            .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?
                    } else {
                        value!("gregory")
                    };

                    let cldr_cal = SUPPORTED_CALS
                        .get(&calendar)
                        .ok_or_else(|| DataErrorKind::MissingLocale.into_error())?;

                    let resource: &cldr_serde::ca::Resource = self
                        .source
                        .cldr()?
                        .dates(cldr_cal).read_and_parse(&langid, &format!("ca-{}.json", cldr_cal))?;

                    let mut data =
                        resource
                            .main
                            .0
                            .get(&langid)
                            .expect("CLDR file contains the expected language")
                            .dates
                            .calendars
                            .get(*cldr_cal)
                            .expect("CLDR file contains the expected calendar")
                            .clone();

                    // CLDR treats ethiopic and ethioaa as separate calendars; however we treat them as a single resource key that
                    // supports symbols for both era patterns based on the settings on the date. Load in ethioaa data as well when dealing with
                    // ethiopic.
                    if calendar == value!("ethiopic") {
                        let ethioaa: &cldr_serde::ca::Resource = self.source.cldr()?.dates("ethiopic").read_and_parse(&langid, "ca-ethiopic-amete-alem.json")?;

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

                        let mundi_name = ethioaa_data.eras.names.get("0").expect("ethiopic-amete-alem calendar must have 0 era");
                        let mundi_abbr = ethioaa_data.eras.abbr.get("0").expect("ethiopic-amete-alem calendar must have 0 era");
                        let mundi_narrow = ethioaa_data.eras.narrow.get("0").expect("ethiopic-amete-alem calendar must have 0 era");

                        data.eras.names.insert("2".to_string(), mundi_name.clone());
                        data.eras.abbr.insert("2".to_string(), mundi_abbr.clone());
                        data.eras.narrow.insert("2".to_string(), mundi_narrow.clone());
                    }

                    if calendar == value!("japanese") {
                        let era_dates: &cldr_serde::japanese::Resource = self
                            .source
                            .cldr()?
                            .core()
                            .read_and_parse("supplemental/calendarData.json")?;
                        let mut set = HashSet::<String>::new();
                        for (era_index, date) in era_dates.supplemental.calendar_data.japanese.eras.iter() {
                                let start_date = EraStartDate::from_str(&date.start).map_err(|_| {
                                    DataError::custom("calendarData.json contains unparseable data for a japanese era")
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
                    if calendar == value!("japanese") || calendar == value!("japanext") {

                        // Splice in gregorian data for pre-meiji
                        let greg_resource: &cldr_serde::ca::Resource = self
                            .source
                            .cldr()?
                            .dates("gregorian").read_and_parse(&langid, "ca-gregorian.json")?;

                        let greg =
                            greg_resource
                                .main
                                .0
                                .get(&langid)
                                .expect("CLDR file contains the expected language")
                                .dates
                                .calendars
                                .get("gregorian")
                                .expect("CLDR file contains a gregorian calendar")
                                .clone();

                        // The "eras" map is largely keyed by a number, but instead of trying to
                        // come up with a number that the japanese era map would not use, we just use a regular string
                        data.eras.names.insert("bce".into(), greg.eras.names.get("0").expect("Gregorian calendar must have data for BC").into());
                        data.eras.names.insert("ce".into(), greg.eras.names.get("1").expect("Gregorian calendar must have data for AD").into());
                        data.eras.abbr.insert("bce".into(), greg.eras.abbr.get("0").expect("Gregorian calendar must have data for BC").into());
                        data.eras.abbr.insert("ce".into(), greg.eras.abbr.get("1").expect("Gregorian calendar must have data for AD").into());
                        data.eras.narrow.insert("bce".into(), greg.eras.narrow.get("0").expect("Gregorian calendar must have data for BC").into());
                        data.eras.narrow.insert("ce".into(), greg.eras.narrow.get("1").expect("Gregorian calendar must have data for AD").into());
                    }

                    Ok(DataResponse {
                        metadata: Default::default(),
                        #[allow(clippy::redundant_closure_call)]
                        payload: Some(DataPayload::from_owned(($expr)(&data, &calendar.to_string()))),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                    let mut r = Vec::new();
                    if $calendared {
                        for (cal_value, cldr_cal) in &*SUPPORTED_CALS {
                            r.extend(self
                                        .source
                                        .cldr()?
                                        .dates(cldr_cal).list_langs()?
                                .map(|lid| {
                                    let mut locale: Locale = lid.into();
                                    locale
                                        .extensions
                                        .unicode
                                        .keywords
                                        .set(key!("ca"), cal_value.clone());
                                    DataLocale::from(locale)
                                }));
                        }
                    } else {
                        r.extend(self
                                    .source
                                    .cldr()?
                                    .dates("gregorian").list_langs()?
                                .map(|lid| {
                                    let locale: Locale = lid.into();
                                    DataLocale::from(locale)
                                }));
                    }

                    Ok(r)
                }
            }
        )+
    };
}

impl_data_provider!(
    (
        DateSymbolsV1Marker,
        symbols::convert_dates,
        calendared = true
    ),
    (
        TimeSymbolsV1Marker,
        |dates, _| { symbols::convert_times(dates) },
        calendared = false
    ),
    (
        DateSkeletonPatternsV1Marker,
        |dates, _| { DateSkeletonPatternsV1::from(dates) },
        calendared = true
    ),
    (
        DatePatternsV1Marker,
        |dates, _| DatePatternsV1::from(dates),
        calendared = true
    ),
    (
        TimePatternsV1Marker,
        |dates, _| TimePatternsV1::from(dates),
        calendared = false
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use icu_datetime::pattern::runtime::{Pattern, PluralPattern};
    use icu_plurals::PluralCategory;

    #[test]
    fn test_basic_patterns() {
        let provider = crate::DatagenProvider::for_test();

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DatePatternsV1Marker> = provider
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

        let locale: Locale = "haw-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DatePatternsV1Marker> = provider
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

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
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

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
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
