// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::SourceDataProvider;
use icu::calendar::provider::*;
use icu::calendar::{types::MonthCode, AnyCalendar, Date};
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;

pub(crate) const JAPANEXT_FILE: &str =
    include_str!("../../data/japanese-golden/calendar/CalendarJapaneseExtendedV1.json");

/// calendarData.json
impl SourceDataProvider {
    #[expect(clippy::type_complexity)]
    pub(crate) fn all_eras(
        &self,
    ) -> Result<&BTreeMap<DatagenCalendar, Vec<(usize, EraData)>>, DataError> {
        let cldr = self.cldr()?;
        cldr.calendar_eras
            .get_or_init(|| {
                // The Japanese era codes depend on the Latin romanizations of the eras, found
                // in the root locale.
                let japanese_names = &self
                    .get_dates_resource(Default::default(), Some(DatagenCalendar::JapaneseModern))?
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
                    DatagenCalendar::Indian,
                    DatagenCalendar::Hijri,
                    DatagenCalendar::Persian,
                    DatagenCalendar::Hebrew,
                    DatagenCalendar::Ethiopic,
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
                        let calendar =
                            AnyCalendar::try_new_unstable(self, cal.canonical_any_calendar_kind())
                                .unwrap();

                        era_dates_map[cal.cldr_name()]
                            .clone()
                            .eras
                            .into_iter()
                            .filter_map(|(key, mut data)| {
                                let code = data.code.as_deref()?;
                                // Check what ICU4X returns for the date 1-1-1 era
                                data.icu4x_era_index = Date::try_new_from_codes(
                                    Some(code),
                                    1,
                                    MonthCode::new_normal(1).unwrap(),
                                    1,
                                    icu::calendar::Ref(&calendar),
                                )
                                .inspect_err(|e| {
                                    log::warn!("Era '{code}' unknown by icu::calendar ({e:?})");
                                })
                                .ok()?
                                .year()
                                .era()?
                                .era_index;
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

/// Aplies some fixes to the data
fn process_era_dates_map(
    mut data: BTreeMap<String, cldr_serde::eras::CalendarData>,
) -> BTreeMap<String, cldr_serde::eras::CalendarData> {
    data.get_mut("japanese").unwrap().eras =
        core::mem::take(&mut data.get_mut("japanese").unwrap().eras)
            .into_iter()
            .map(|(idx, mut era)| {
                // https://unicode-org.atlassian.net/browse/CLDR-18388 for why we need to do + 2
                let idx = (idx.parse::<usize>().unwrap() + 2).to_string();
                if let Some(start) = era.start.as_mut() {
                    // All pre-Taisho start dates are known to be wrong, this at least makes them valid.
                    // See https://unicode-org.atlassian.net/browse/CLDR-11400
                    if start.month == 2 && start.day > 28 {
                        start.day = if calendrical_calculations::gregorian::is_leap_year(start.year)
                        {
                            29
                        } else {
                            28
                        };
                    }
                }
                (idx, era)
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

// We use a single data struct for both Ethiopic calendars, make sure their indices agree
#[test]
pub fn ethiopic_and_ethioaa_are_compatible() {
    use icu::calendar::{
        cal::{Ethiopian, EthiopianEraStyle},
        types::MonthCode,
    };
    assert_eq!(
        Date::try_new_from_codes(
            Some("aa"),
            1,
            MonthCode::new_normal(1).unwrap(),
            1,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)
        )
        .unwrap()
        .era_year()
        .era_index,
        Date::try_new_from_codes(
            Some("aa"),
            1,
            MonthCode::new_normal(1).unwrap(),
            1,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)
        )
        .unwrap()
        .era_year()
        .era_index,
    );
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

        for (idx, (_, era)) in data.eras.iter().enumerate() {
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
                        in_era.year().era().unwrap().year,
                        in_era.month().standard_code,
                        in_era.day_of_month().0,
                        cal,
                    ),
                    Ok(in_era)
                );
            }

            let icu::calendar::types::YearInfo::Era(era_year) = in_era.year() else {
                continue;
            };

            // Unless this is the first era and it's not an inverse era, check that the
            // not_in_era date is in a different era
            if idx != 0 || era.end.is_some() {
                assert_ne!(not_in_era.year().era().unwrap().era, era_year.era);
            }

            // Check that the correct era code is returned
            if let Some(code) = era.code.as_deref() {
                assert_eq!(era_year.era, code);
            }

            // Check that the start/end date uses year 1, and minimal/maximal month/day
            assert_eq!(era_year.year, 1, "Didn't get correct year for {in_era:?}");

            if calendar == "japanese" {
                // Japanese is the only calendar that doesn't start eras on a new year
            } else if era.start.is_some() {
                assert_eq!(in_era.month().ordinal, 1);
                assert_eq!(in_era.day_of_month().0, 1);
                assert_eq!(in_era.day_of_year().0, 1);
            } else {
                assert_eq!(in_era.month().ordinal, in_era.months_in_year());
                assert_eq!(in_era.day_of_month().0, in_era.days_in_month());
                assert_eq!(in_era.day_of_year().0, in_era.days_in_year());
            }
        }
    }
}
