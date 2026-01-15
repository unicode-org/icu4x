// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::SourceDataProvider;
use icu::calendar::provider::*;
use icu::calendar::{AnyCalendar, Date};
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;

/// calendarData.json
impl SourceDataProvider {
    #[expect(clippy::type_complexity)]
    pub(crate) fn all_eras(
        &self,
    ) -> Result<
        &BTreeMap<DatagenCalendar, (Option<DatagenCalendar>, Vec<(usize, EraData)>)>,
        DataError,
    > {
        let cldr = self.cldr()?;
        cldr.calendar_eras
            .get_or_init(|| {
                let era_dates_map = &cldr
                    .core()
                    .read_and_parse::<cldr_serde::eras::Resource>("supplemental/calendarData.json")?
                    .supplemental
                    .calendar_data;

                let era_dates_map = process_era_dates_map(era_dates_map.clone());

                Ok([
                    DatagenCalendar::Buddhist,
                    DatagenCalendar::Japanese,
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
                    let inherit = era_dates_map[cal.cldr_name()]
                        .inherit_eras
                        .as_ref()
                        .map(|c| DatagenCalendar::from_cldr_name(&c.calendar));

                    let any_cal = AnyCalendar::new_without_data(cal.canonical_any_calendar_kind());

                    let mut vec = era_dates_map[cal.cldr_name()]
                        .eras
                        .iter()
                        .map(|(key, data)| {
                            let mut data = data.clone();
                            let date = data.start.or(data.end).unwrap();
                            let era_year = Date::try_new_gregorian(date.year, date.month, date.day)
                                .unwrap()
                                .to_calendar(icu::calendar::Ref(&any_cal))
                                .year()
                                .era()
                                .unwrap();
                            if era_year.era != data.code {
                                log::warn!("mismatched era code {era_year:?} - {data:?}");
                            }
                            data.icu4x_era_index = era_year.era_index.or((cal
                                == DatagenCalendar::Japanese)
                                .then(|| {
                                    era_dates_map[cal.cldr_name()]
                                        .eras
                                        .iter()
                                        .skip_while(|(_, data)| data.start.unwrap().year < 1868)
                                        .position(|(_, x)| *x == data)
                                        .unwrap() as u8
                                        + 2
                                }));

                            (key.parse::<usize>().unwrap(), data)
                        })
                        .collect::<Vec<_>>();
                    vec.sort_by_key(|&(k, _)| k);
                    (cal, (inherit, vec))
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
    data.get_mut(DatagenCalendar::Japanese.cldr_name())
        .unwrap()
        .eras
        .retain(|_, era| !era.code.is_empty());
    data
}

impl DataProvider<CalendarJapaneseModernV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarJapaneseModernV1>, DataError> {
        self.check_req::<CalendarJapaneseModernV1>(req)?;

        let (inherit, ref eras) = self.all_eras()?[&DatagenCalendar::Japanese];

        let dates_to_eras = inherit
            .iter()
            .flat_map(|i| self.all_eras().unwrap()[i].1.iter())
            .chain(eras)
            .filter(|(_, data)| !matches!(data.code.as_str(), "bce" | "ce"))
            .map(|(_, data)| (data.start.unwrap(), data.code.parse().unwrap()))
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(JapaneseEras { dates_to_eras }),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarJapaneseModernV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

// We use a single data struct for both Ethiopic calendars, make sure their indices agree
#[test]
pub fn ethiopic_and_ethioaa_are_compatible() {
    use icu::calendar::cal::{Ethiopian, EthiopianEraStyle};
    use icu::calendar::types::Month;
    assert_eq!(
        Date::try_new_from_codes(
            Some("aa"),
            1,
            Month::new(1).code(),
            1,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem)
        )
        .unwrap()
        .era_year()
        .era_index,
        Date::try_new_from_codes(
            Some("aa"),
            1,
            Month::new(1).code(),
            1,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteMihret)
        )
        .unwrap()
        .era_year()
        .era_index,
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
                .split(' ')
                .chain(Some(era.code.as_str()))
                .filter(|s| !s.is_empty())
            {
                assert_eq!(
                    Date::try_new_from_codes(
                        Some(era),
                        in_era.year().era().unwrap().year,
                        in_era.month().value.code(),
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
            if !era.code.is_empty() {
                assert_eq!(era_year.era, era.code);
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
