// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::SourceDataProvider;
use icu::calendar::provider::*;
use icu::calendar::Date;
use icu::locale::locale;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;

pub(crate) const JAPANEXT_FILE: &str =
    include_str!("../../data/japanese-golden/calendar/CalendarJapaneseExtendedV1.json");

/// calendarData.json
impl SourceDataProvider {
    #[allow(clippy::type_complexity)]
    pub(crate) fn all_eras(
        &self,
    ) -> Result<&BTreeMap<DatagenCalendar, Vec<(usize, EraData)>>, DataError> {
        let cldr = self.cldr()?;
        cldr.calendar_eras
            .get_or_init(|| {
                // The Japanese era codes depend on the Latin romanizations of the eras, found
                // in the `en` locale. We load this data to construct era codes but
                // actual user code only needs to load the data for the locales it cares about.
                let japanese_names = &self
                    .cldr()?
                    .dates("japanese")
                    .read_and_parse::<cldr_serde::ca::Resource>(
                        &locale!("en").into(),
                        "ca-japanese.json",
                    )?
                    .main
                    .value
                    .dates
                    .calendars
                    .get("japanese")
                    .ok_or(DataError::custom(
                        "ca-japanese.json does not contain 'japanese' calendar",
                    ))?
                    .eras
                    .as_ref()
                    .expect("japanese must have eras")
                    .abbr;

                let era_dates_map = &cldr
                    .core()
                    .read_and_parse::<cldr_serde::eras::Resource>("supplemental/calendarData.json")?
                    .supplemental
                    .calendar_data;

                let era_dates_map = process_era_dates_map(era_dates_map.clone());

                Ok([
                    DatagenCalendar::Buddhist,
                    DatagenCalendar::JapaneseModern,
                    DatagenCalendar::JapaneseExtended,
                    DatagenCalendar::Coptic,
                    DatagenCalendar::Dangi,
                    DatagenCalendar::Chinese,
                    DatagenCalendar::Indian,
                    DatagenCalendar::Hijri,
                    DatagenCalendar::Persian,
                    DatagenCalendar::Hebrew,
                    DatagenCalendar::Ethiopic,
                    DatagenCalendar::EthiopicAmeteAlem,
                    DatagenCalendar::Roc,
                    DatagenCalendar::Gregorian,
                ]
                .into_iter()
                .map(|cal| {
                    let mut vec = if cal == DatagenCalendar::JapaneseExtended
                        || cal == DatagenCalendar::JapaneseModern
                    {
                        era_dates_map[DatagenCalendar::Gregorian.cldr_name()]
                            .clone()
                            .eras
                            .into_iter()
                            .filter_map(|(id, data)| {
                                data.code.as_ref()?;
                                Some((id.parse::<usize>().ok()?, data))
                            })
                            .chain(
                                era_dates_map[cal.cldr_name()]
                                    .clone()
                                    .eras
                                    .into_iter()
                                    .filter_map(|(key, mut data)| {
                                        let key = key.parse::<usize>().ok()?;
                                        if data.code.as_ref().is_none() {
                                            if cal == DatagenCalendar::JapaneseExtended {
                                                data.code =
                                                    Some(crate::calendar::eras::era_to_code(
                                                        japanese_names
                                                            .get(&(key - 2).to_string())?,
                                                        data.start?.year,
                                                    ));
                                            } else {
                                                None?;
                                            }
                                        }
                                        Some((key, data))
                                    }),
                            )
                            .collect::<Vec<_>>()
                    } else {
                        era_dates_map[cal.cldr_name()]
                            .clone()
                            .eras
                            .into_iter()
                            .filter_map(|(key, data)| {
                                data.code.as_ref()?;
                                Some((key.parse::<usize>().ok()?, data))
                            })
                            .collect::<Vec<_>>()
                    };
                    vec.sort_by_key(|&(k, _)| k);
                    (cal, vec)
                })
                .collect())
            })
            .as_ref()
            .map_err(|e| *e)
    }
}

/// Aplies some fixes to the data, as it is pretty buggy as of v47
fn process_era_dates_map(
    mut data: BTreeMap<String, cldr_serde::eras::CalendarData>,
) -> BTreeMap<String, cldr_serde::eras::CalendarData> {
    fn replace_julian_by_iso(d: &mut EraStartDate) {
        let date = Date::try_new_julian(d.year, d.month, d.day)
            .unwrap()
            .to_iso();
        *d = EraStartDate {
            year: date.year().extended_year,
            month: date.month().ordinal,
            day: date.day_of_month().0,
        };
    }

    data.get_mut("ethiopic")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .end = None;

    data.get_mut("ethiopic")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -5492,
        month: 7,
        day: 17,
    });

    data.get_mut("ethiopic")
        .unwrap()
        .eras
        .get_mut("1")
        .unwrap()
        .start
        .as_mut()
        .into_iter()
        .for_each(replace_julian_by_iso);

    let ethiopic_0 = data["ethiopic"].eras["0"].clone();
    data.get_mut("ethiopic-amete-alem")
        .unwrap()
        .eras
        .insert("0".into(), ethiopic_0);

    data.get_mut("chinese")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -2636,
        month: 1,
        day: 1,
    });

    data.get_mut("dangi")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -2331,
        month: 1,
        day: 1,
    });

    data.get_mut("hebrew")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: -3760,
        month: 9,
        day: 7,
    });

    data.get_mut("islamic-civil")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start
        .as_mut()
        .into_iter()
        .for_each(replace_julian_by_iso);

    data.get_mut("islamic-rgsa")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 7,
        day: 19,
    });

    data.get_mut("islamic-tbla")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start
        .as_mut()
        .into_iter()
        .for_each(replace_julian_by_iso);

    data.get_mut("islamic-umalqura")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = data
        .get_mut("islamic-civil")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start;

    data.get_mut("persian")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 622,
        month: 3,
        day: 21,
    });

    data.get_mut("indian")
        .unwrap()
        .eras
        .get_mut("0")
        .unwrap()
        .start = Some(EraStartDate {
        year: 79,
        month: 3,
        day: 22,
    });

    data.get_mut("japanese").unwrap().eras =
        core::mem::take(&mut data.get_mut("japanese").unwrap().eras)
            .into_iter()
            .map(|(idx, mut era)| {
                let idx = idx.parse::<usize>().unwrap();
                if let Some(start) = era.start.as_mut() {
                    if start.year <= 1868 {
                        // pre-Meiji eras seem to use Chinese/Japanese lunisolar months

                        // These were calculated using Temporal polyfill
                        let corrected_start_dates = [
                            (645, 7, 20),
                            (650, 3, 25),
                            (672, 2, 8),
                            (686, 8, 17),
                            (701, 5, 7),
                            (704, 6, 20),
                            (708, 2, 11),
                            (715, 10, 7),
                            (717, 12, 28),
                            (724, 3, 7),
                            (729, 9, 6),
                            (749, 5, 9),
                            (749, 7, 24),
                            (757, 9, 10),
                            (765, 2, 5),
                            (767, 9, 17),
                            (770, 10, 27),
                            (781, 2, 2),
                            (782, 9, 5),
                            (806, 6, 12),
                            (810, 10, 24),
                            (824, 2, 12),
                            (834, 2, 18),
                            (848, 7, 20),
                            (851, 6, 5),
                            (854, 12, 26),
                            (857, 3, 24),
                            (859, 5, 24),
                            (877, 5, 6),
                            (885, 3, 15),
                            (889, 6, 3),
                            (898, 5, 24),
                            (901, 8, 6),
                            (923, 5, 4),
                            (931, 5, 21),
                            (938, 6, 27),
                            (947, 5, 20),
                            (957, 11, 26),
                            (961, 3, 10),
                            (964, 7, 26),
                            (968, 9, 13),
                            (970, 5, 8),
                            (974, 1, 20),
                            (976, 8, 16),
                            (979, 1, 5),
                            (983, 5, 5),
                            (985, 5, 24),
                            (987, 5, 10),
                            (989, 9, 15),
                            (990, 12, 1),
                            (995, 3, 30),
                            (999, 2, 6),
                            (1004, 8, 14),
                            (1013, 1, 15),
                            (1017, 5, 27),
                            (1021, 2, 21),
                            (1024, 8, 25),
                            (1028, 8, 24),
                            (1037, 5, 14),
                            (1040, 11, 22),
                            (1044, 12, 21),
                            (1046, 5, 28),
                            (1053, 2, 8),
                            (1058, 9, 25),
                            (1065, 9, 10),
                            (1069, 5, 12),
                            (1074, 9, 22),
                            (1077, 12, 10),
                            (1081, 3, 28),
                            (1084, 3, 21),
                            (1087, 5, 17),
                            (1094, 12, 30),
                            (1097, 1, 8),
                            (1097, 12, 3),
                            (1099, 9, 21),
                            (1104, 3, 15),
                            (1106, 5, 20),
                            (1108, 9, 16),
                            (1110, 8, 7),
                            (1113, 8, 3),
                            (1118, 5, 2),
                            (1120, 5, 15),
                            (1124, 4, 25),
                            (1126, 2, 22),
                            (1131, 3, 7),
                            (1132, 8, 30),
                            (1135, 5, 18),
                            (1141, 8, 20),
                            (1142, 5, 31),
                            (1144, 4, 4),
                            (1145, 8, 19),
                            (1151, 2, 20),
                            (1154, 11, 12),
                            (1156, 5, 24),
                            (1159, 5, 16),
                            (1160, 2, 25),
                            (1161, 10, 1),
                            (1163, 5, 10),
                            (1165, 6, 21),
                            (1166, 9, 29),
                            (1169, 5, 13),
                            (1171, 6, 3),
                            (1175, 8, 23),
                            (1177, 9, 4),
                            (1181, 8, 3),
                            (1182, 7, 6),
                            (1184, 5, 4),
                            (1185, 9, 16),
                            (1190, 5, 23),
                            (1199, 5, 30),
                            (1201, 3, 25),
                            (1204, 3, 29),
                            (1206, 6, 12),
                            (1207, 11, 23),
                            (1211, 3, 31),
                            (1213, 12, 26),
                            (1219, 5, 4),
                            (1222, 6, 1),
                            (1224, 12, 8),
                            (1225, 6, 4),
                            (1227, 12, 26),
                            (1229, 4, 7),
                            (1232, 4, 30),
                            (1233, 6, 1),
                            (1234, 12, 4),
                            (1235, 10, 9),
                            (1238, 12, 7),
                            (1239, 3, 20),
                            (1240, 8, 12),
                            (1243, 3, 24),
                            (1247, 4, 12),
                            (1249, 4, 9),
                            (1256, 10, 31),
                            (1257, 4, 6),
                            (1259, 4, 26),
                            (1260, 5, 1),
                            (1261, 3, 29),
                            (1264, 4, 3),
                            (1275, 5, 28),
                            (1278, 3, 30),
                            (1288, 6, 5),
                            (1293, 9, 13),
                            (1299, 6, 1),
                            (1302, 12, 18),
                            (1303, 8, 26),
                            (1306, 12, 28),
                            (1308, 10, 31),
                            (1311, 5, 24),
                            (1312, 5, 5),
                            (1317, 2, 22),
                            (1319, 5, 26),
                            (1321, 3, 30),
                            (1325, 1, 2),
                            (1326, 6, 5),
                            (1329, 9, 30),
                            (1331, 9, 19),
                            (1334, 3, 13),
                            (1336, 3, 20),
                            (1340, 6, 1),
                            (1346, 12, 28),
                            (1370, 8, 23),
                            (1372, 5, 12),
                            (1375, 7, 4),
                            (1379, 4, 17),
                            (1381, 3, 14),
                            (1384, 5, 26),
                            (1387, 9, 12),
                            (1387, 9, 13),
                            (1389, 3, 14),
                            (1390, 4, 19),
                            (1394, 8, 9),
                            (1428, 5, 20),
                            (1429, 10, 11),
                            (1441, 3, 18),
                            (1444, 3, 3),
                            (1449, 8, 25),
                            (1452, 8, 19),
                            (1455, 8, 17),
                            (1457, 10, 25),
                            (1461, 1, 11),
                            (1466, 3, 23),
                            (1467, 4, 15),
                            (1469, 5, 18),
                            (1487, 8, 26),
                            (1489, 9, 24),
                            (1492, 8, 20),
                            (1501, 3, 28),
                            (1504, 3, 25),
                            (1521, 10, 3),
                            (1528, 9, 13),
                            (1532, 9, 8),
                            (1555, 11, 16),
                            (1558, 3, 27),
                            (1570, 6, 6),
                            (1573, 9, 4),
                            (1593, 1, 10),
                            (1596, 11, 16),
                            (1615, 8, 7),
                            (1624, 4, 17),
                            (1645, 1, 13),
                            (1648, 3, 8),
                            (1652, 10, 20),
                            (1655, 5, 18),
                            (1658, 8, 21),
                            (1661, 5, 23),
                            (1673, 10, 30),
                            (1681, 11, 8),
                            (1684, 4, 5),
                            (1688, 10, 23),
                            (1704, 4, 16),
                            (1711, 6, 10),
                            (1716, 7, 11),
                            (1736, 6, 7),
                            (1741, 4, 12),
                            (1744, 4, 3),
                            (1748, 8, 5),
                            (1751, 11, 14),
                            (1764, 6, 30),
                            (1772, 12, 10),
                            (1781, 4, 25),
                            (1789, 2, 19),
                            (1801, 3, 18),
                            (1804, 3, 22),
                            (1818, 5, 26),
                            (1830, 12, 24),
                            (1845, 1, 9),
                            (1848, 4, 1),
                            (1854, 12, 16),
                            (1860, 4, 8),
                            (1861, 3, 29),
                            (1864, 3, 27),
                            (1865, 5, 1),
                            (1868, 9, 23),
                        ];

                        let (y, m, d) = corrected_start_dates[idx];

                        let icu4x = Date::try_new_chinese_with_calendar(
                            // (-2636 - 1) is the Gregorian/Chinese year offset
                            start.year - (-2636 - 1),
                            start.month,
                            start.day,
                            icu::calendar::cal::Chinese::new_always_calculating(),
                        )
                        .map(|d| d.to_iso());
                        let temporal = Date::try_new_iso(y, m, d).map_err(|e| e.into());
                        if icu4x != temporal {
                            log::info!("Temporal disagrees with ICU4X: {temporal:?} {icu4x:?} ");
                        }

                        start.year = y;
                        start.month = m;
                        start.day = d;
                    }
                }
                // https://unicode-org.atlassian.net/browse/CLDR-18388 for why we need to do + 2
                ((idx + 2).to_string(), era)
            })
            .collect();
    data
}

impl SourceDataProvider {
    fn load_japanese_eras(
        &self,
        cal: DatagenCalendar,
    ) -> Result<DataResponse<CalendarJapaneseModernV1>, DataError> {
        let mut dates_to_eras = BTreeMap::new();

        for (_, data) in self.all_eras()?[&cal].iter().skip(2) {
            let start_date = data.start.unwrap();
            let code = data.code.as_deref().unwrap();
            let code = code.parse().map_err(|_| {
                DataError::custom("Era code does not fit int TinyStr16").with_display_context(&code)
            })?;

            dates_to_eras.insert(start_date, code);
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(JapaneseEras {
                dates_to_eras: dates_to_eras.into_iter().collect(),
            }),
        })
    }
}

impl DataProvider<CalendarJapaneseModernV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarJapaneseModernV1>, DataError> {
        self.check_req::<CalendarJapaneseModernV1>(req)?;
        self.load_japanese_eras(DatagenCalendar::JapaneseModern)
    }
}

impl DataProvider<CalendarJapaneseExtendedV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CalendarJapaneseExtendedV1>, DataError> {
        self.check_req::<CalendarJapaneseExtendedV1>(req)?;
        let DataResponse { metadata, payload } =
            self.load_japanese_eras(DatagenCalendar::JapaneseExtended)?;

        // Integrity check
        //
        // Era code generation relies on the English era data which could in theory change; we have an integrity check
        // to catch such cases. It is relatively rare for a new era to be added, and in those cases the integrity check can
        // be disabled when generating new data.
        if env::var("ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK").is_err() {
            let snapshot: JapaneseEras = serde_json::from_str(JAPANEXT_FILE)
                .expect("Failed to parse the precached golden. This is a bug.");

            if snapshot != *payload.get() {
                return Err(DataError::custom(
                    "Era data has changed! This can be for two reasons: Either the CLDR locale data for Japanese eras has \
                    changed in an incompatible way, or there is a new Japanese era. Run \
                    `ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK=1 cargo run -p icu4x-datagen -- --markers CalendarJapaneseExtendedV1 --format fs --syntax json \
                    --out provider/source/data/japanese-golden --pretty --overwrite` in the icu4x repo and inspect the diff to \
                    check which situation it is. If a new era has been introduced, commit the diff, if not, it's likely that japanese.rs \
                    in icu_provider_source will need to be updated to handle the data changes."
                ));
            }
        }

        Ok(DataResponse {
            metadata,
            payload: payload.cast(),
        })
    }
}

/// See <https://docs.google.com/document/d/1vMVhMHgCYRyx2gmwEfKRyXWDg_lrQadd8iMVU9uPK1o/edit?usp=chrome_omnibox&ouid=111665445991279316689>
/// for the era identifier spec
pub(crate) fn era_to_code(original: &str, year: i32) -> String {
    // Some examples of CLDR era names:
    // "Shōryaku (1077–1081)", "Nin’an (1166–1169)", "Tenpyō-kampō (749–749)"
    //
    // We want to produce a unique readable era code that
    // contains ascii lowercase letters, followed
    // by a hyphen and then a year name (except for post-Meiji era codes)
    //
    // We also want it to fit in a TinyAsciiStr<16>. What we will do is:
    //
    // - only look at the actual name
    // - normalize by removing hyphens and apostrophes, as well as converting ō/ū
    //   to ascii o/u
    // - truncating to fit 16 characters

    let name = original
        .split(' ')
        .next()
        .expect("split iterator is non-empty");
    let name = name
        .replace(['ō', 'Ō'], "o")
        .replace(['ū', 'Ū'], "u")
        .replace(['-', '\'', '’'], "")
        .to_lowercase();
    if !name.is_ascii() {
        panic!("Era name {name} (parsed from {original}) contains non-ascii characters");
    }

    format!("{name}-{year}")
}

impl crate::IterableDataProviderCached<CalendarJapaneseModernV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl crate::IterableDataProviderCached<CalendarJapaneseExtendedV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[test]
pub fn ethiopic_and_ethioaa_are_compatible() {
    let provider = SourceDataProvider::new_testing();
    let ethiopic = &provider.all_eras().unwrap()[&DatagenCalendar::Ethiopic];
    let ethioaa = &provider.all_eras().unwrap()[&DatagenCalendar::EthiopicAmeteAlem];
    assert_eq!(ethioaa.iter().zip(ethiopic).find(|(e, a)| e != a), None);
}

#[test]
pub fn japanese_and_japanext_are_compatible() {
    let provider = SourceDataProvider::new_testing();
    let japanese = &provider.all_eras().unwrap()[&DatagenCalendar::JapaneseModern];
    let japanext = &provider.all_eras().unwrap()[&DatagenCalendar::JapaneseExtended];
    assert_eq!(
        japanext
            .iter()
            .take(2)
            .zip(japanese.iter().take(2))
            .find(|(e, a)| e != a),
        None,
    );
    assert_eq!(
        japanext
            .iter()
            .skip(2)
            .rev()
            .zip(japanese.iter().skip(2).rev())
            .find(|(e, a)| e != a),
        None,
        "{japanext:?} - {japanese:?}"
    );
}

#[test]
fn test_calendar_eras() {
    use icu::calendar::types::FormattingEra;
    use icu::calendar::Iso;
    use icu::calendar::{AnyCalendar, AnyCalendarKind, Date};
    use icu::datetime::preferences::CalendarAlgorithm;
    use icu::locale::extensions::unicode::Value;

    let provider = crate::SourceDataProvider::new_testing();

    let era_dates_map = &provider
        .cldr()
        .unwrap()
        .core()
        .read_and_parse::<cldr_serde::eras::Resource>("supplemental/calendarData.json")
        .unwrap()
        .supplemental
        .calendar_data;
    let era_dates_map = process_era_dates_map(era_dates_map.clone());

    for (calendar, data) in era_dates_map {
        let kind = match calendar.as_str() {
            "generic" | "islamic" => continue,
            "ethiopic-amete-alem" => AnyCalendarKind::EthiopianAmeteAlem,
            "gregorian" => AnyCalendarKind::Gregorian,
            "japanese" => AnyCalendarKind::JapaneseExtended,
            c => CalendarAlgorithm::try_from(&Value::try_from_str(c).unwrap())
                .unwrap()
                .try_into()
                .expect(&calendar),
        };

        let cal = AnyCalendar::try_new_unstable(&provider, kind).unwrap();
        let cal = icu::calendar::Ref(&cal);

        for (idx, ref era) in data.eras {
            let (in_era_iso, not_in_era_iso) = match (era.start, era.end) {
                (Some(start), None) => {
                    let start = Date::try_new_iso(start.year, start.month, start.day).unwrap();
                    (start, Date::from_rata_die(start.to_rata_die() - 1, Iso))
                }
                (None, Some(end)) => {
                    let end = Date::try_new_iso(end.year, end.month, end.day).unwrap();
                    (end, Date::from_rata_die(end.to_rata_die() + 1, Iso))
                }
                _ => unreachable!(),
            };

            let in_era = in_era_iso.to_calendar(cal);
            let not_in_era = not_in_era_iso.to_calendar(cal);

            // Check that code and aliases produce identical results
            for era in era
                .aliases
                .as_deref()
                .into_iter()
                .flat_map(|s| s.split(' '))
                .chain(era.code.as_deref())
            {
                assert_eq!(
                    Date::try_new_from_codes(
                        Some(era),
                        in_era.year().era_year_or_extended(),
                        in_era.month().standard_code,
                        in_era.day_of_month().0,
                        cal,
                    ),
                    Ok(in_era)
                );
            }

            // Unless this is the first era and it's not an inverse era, check that the
            // not_in_era date is in a different era
            if idx != "0" || era.end.is_some() {
                assert_ne!(
                    not_in_era.year().standard_era(),
                    in_era.year().standard_era()
                );
            }

            // The remaining tests don't work for cyclic calendars
            if calendar == "dangi" || calendar == "chinese" {
                continue;
            }

            if let Some(FormattingEra::Index(i, _)) = in_era.year().formatting_era() {
                assert_eq!(i.to_string(), idx);
            }

            // Check that the correct era code is returned
            if let Some(code) = era.code.as_deref() {
                assert_eq!(in_era.year().standard_era().unwrap().0, code);
            }

            // Check that the start/end date uses year 1, and minimal/maximal month/day
            assert_eq!(in_era.year().era_year_or_extended(), 1);
            if calendar == "japanese" {
                // Japanese is the only calendar that doesn't have its own months
            } else if era.start.is_some() {
                assert_eq!(in_era.month().ordinal, 1);
                assert_eq!(in_era.day_of_month().0, 1);
            } else {
                assert_eq!(in_era.month().ordinal, in_era.months_in_year());
                assert_eq!(in_era.day_of_month().0, in_era.days_in_month());
            }
        }
    }
}
