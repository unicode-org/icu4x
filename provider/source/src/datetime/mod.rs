// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::datetime::provider::calendar::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use symbols::get_era_code_map;

mod neo;
mod neo_skeleton;
mod patterns;
mod skeletons;
mod symbols;
pub(crate) mod week_data;

/// Another list of calendars. These are the calendars that we can generate data for,
/// which are all CLDR calendars (even the ones we don't support in ICU right now),
/// with the Japanese calendar split.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum DatagenCalendar {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopic,
    EthiopicAmeteAlem,
    Gregorian,
    Hebrew,
    Indian,
    Islamic,
    IslamicCivil,
    IslamicTabular,
    IslamicRgsa,
    IslamicUmmAlQura,
    JapaneseExtended,
    JapaneseModern,
    Persian,
    Roc,
}

impl DatagenCalendar {
    fn cldr_name(self) -> &'static str {
        use DatagenCalendar::*;
        match self {
            Buddhist => "buddhist",
            Chinese => "chinese",
            Coptic => "coptic",
            Dangi => "dangi",
            Ethiopic => "ethiopic",
            EthiopicAmeteAlem => "ethiopic-amete-alem",
            Gregorian => "gregorian",
            Hebrew => "hebrew",
            Indian => "indian",
            Islamic => "islamic",
            IslamicCivil => "islamic-civil",
            IslamicTabular => "islamic-tbla",
            IslamicRgsa => "islamic-rgsa",
            IslamicUmmAlQura => "islamic-umalqura",
            JapaneseExtended => "japanese",
            JapaneseModern => "japanese",
            Persian => "persian",
            Roc => "roc",
        }
    }
}

impl SourceDataProvider {
    fn get_datetime_resources(
        &self,
        locale: &DataLocale,
        calendar: Option<DatagenCalendar>,
    ) -> Result<cldr_serde::ca::Dates, DataError> {
        let cldr_cal = calendar
            .map(DatagenCalendar::cldr_name)
            .unwrap_or("generic");

        let resource: &cldr_serde::ca::Resource = self
            .cldr()?
            .dates(cldr_cal)
            .read_and_parse(locale, &format!("ca-{}.json", cldr_cal))?;

        let mut data = resource
            .main
            .value
            .dates
            .calendars
            .get(cldr_cal)
            .expect("CLDR file contains the expected calendar")
            .clone();

        if cldr_cal == "japanese" {
            let eras = data.eras.as_mut().expect("japanese must have eras");
            // Filter out non-modern eras
            if calendar == Some(DatagenCalendar::JapaneseModern) {
                let modern_japanese_eras = get_era_code_map()[&DatagenCalendar::JapaneseModern]
                    .iter()
                    .map(|&(s, _)| s.to_string())
                    .collect::<std::collections::BTreeSet<_>>();
                eras.names.retain(|e, _| modern_japanese_eras.contains(e));
                eras.abbr.retain(|e, _| modern_japanese_eras.contains(e));
                eras.narrow.retain(|e, _| modern_japanese_eras.contains(e));
            }

            // Splice in gregorian data for pre-meiji
            let greg_eras = self
                .cldr()?
                .dates(DatagenCalendar::Gregorian.cldr_name())
                .read_and_parse::<cldr_serde::ca::Resource>(locale, "ca-gregorian.json")?
                .main
                .value
                .dates
                .calendars
                .get(DatagenCalendar::Gregorian.cldr_name())
                .expect("CLDR file contains a gregorian calendar")
                .eras
                .as_ref()
                .expect("gregorian must have eras");

            eras.names.extend(greg_eras.names.clone());
            eras.abbr.extend(greg_eras.names.clone());
            eras.narrow.extend(greg_eras.names.clone());
        }

        Ok(data)
    }
}

macro_rules! impl_data_provider {
    ($marker:ident, $expr:expr, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data = self.get_datetime_resources(&req.id.locale, Some($calendar))?;

                #[allow(clippy::redundant_closure_call)]
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(($expr)(&data, $calendar)),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut r = HashSet::new();

                r.extend(
                    self.cldr()?
                        .dates($calendar.cldr_name())
                        .list_locales()?
                        .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l))),
                );

                // TODO(#3212): Remove
                if $marker::INFO == TimeLengthsV1::INFO {
                    r.retain(|id| {
                        id.locale != DataLocale::from(icu::locale::langid!("byn"))
                            && id.locale != DataLocale::from(icu::locale::langid!("ssy"))
                    });
                }

                Ok(r)
            }
        }
    };
}

// TODO(#5613): Even though these markers are no longer exported, we need them in order to export
// semantic skeleton data markers. This should be refactored to skip the intermediate data struct.

impl_data_provider!(
    BuddhistDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Buddhist
);
impl_data_provider!(
    BuddhistDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Buddhist
);
impl_data_provider!(
    ChineseDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Chinese
);
impl_data_provider!(
    ChineseDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Chinese
);
impl_data_provider!(
    CopticDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Coptic
);
impl_data_provider!(
    CopticDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Coptic
);
impl_data_provider!(
    DangiDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Dangi
);
impl_data_provider!(
    DangiDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Dangi
);
impl_data_provider!(
    EthiopianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Ethiopic
);
impl_data_provider!(
    EthiopianDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Ethiopic
);
impl_data_provider!(
    GregorianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Gregorian
);
impl_data_provider!(
    GregorianDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Gregorian
);
impl_data_provider!(
    HebrewDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Hebrew
);
impl_data_provider!(
    HebrewDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Hebrew
);
impl_data_provider!(
    IndianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Indian
);
impl_data_provider!(
    IndianDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Indian
);
impl_data_provider!(
    HijriDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::IslamicCivil
);
impl_data_provider!(
    HijriDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::IslamicCivil
);
impl_data_provider!(
    JapaneseDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::JapaneseModern
);
impl_data_provider!(
    JapaneseDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::JapaneseModern
);
impl_data_provider!(
    JapaneseExtendedDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::JapaneseExtended
);
impl_data_provider!(
    JapaneseExtendedDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::JapaneseExtended
);
impl_data_provider!(
    PersianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Persian
);
impl_data_provider!(
    PersianDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Persian
);
impl_data_provider!(
    RocDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    DatagenCalendar::Roc
);
impl_data_provider!(
    RocDateSymbolsV1,
    symbols::convert_dates,
    DatagenCalendar::Roc
);

impl_data_provider!(
    TimeLengthsV1,
    |dates, _| TimeLengths::from(dates),
    DatagenCalendar::Gregorian
);
impl_data_provider!(
    TimeSymbolsV1,
    |dates, _| { symbols::convert_times(dates) },
    DatagenCalendar::Gregorian
);

#[cfg(test)]
mod test {
    use super::*;
    use icu::locale::langid;

    #[test]
    fn test_basic_patterns() {
        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let cs_dates = DateLengths::from(&data);

        assert_eq!("d. M. y", cs_dates.date.medium.to_string());
    }

    #[test]
    fn test_with_numbering_system() {
        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("haw").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let haw_dates = DateLengths::from(&data);

        assert_eq!("d MMM y", haw_dates.date.medium.to_string());
        // TODO(#308): Support numbering system variations. We currently throw them away.
        assert_eq!("d/M/yy", haw_dates.date.short.to_string());
    }

    #[test]
    #[ignore] // TODO(#5643)
    #[allow(unreachable_code, unused_variables, unused_mut)]
    fn test_datetime_skeletons() {
        use icu::datetime::provider::pattern::runtime::Pattern;
        use icu::datetime::provider::skeleton::PluralPattern;
        use icu::plurals::PluralCategory;
        use std::convert::TryFrom;

        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("fil").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let skeletons = DateSkeletonPatterns::from(&data.datetime_formats.available_formats).0;

        assert_eq!(
            Some(
                &"L".parse::<Pattern>()
                    .expect("Failed to create pattern")
                    .into()
            ),
            skeletons.get(&SkeletonData::try_from("M").expect("Failed to create Skeleton"))
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
            skeletons.get(&SkeletonData::try_from("yw").expect("Failed to create Skeleton"))
        );
    }

    #[test]
    fn test_basic_symbols() {
        use icu::calendar::types::MonthCode;
        use tinystr::tinystr;

        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let cs_dates = symbols::convert_dates(&data, DatagenCalendar::Gregorian);

        assert_eq!(
            "srpna",
            cs_dates
                .months
                .format
                .wide
                .get(MonthCode(tinystr!(4, "M08")))
                .unwrap()
        );

        assert_eq!("po", cs_dates.weekdays.format.short.as_ref().unwrap().0[1]);
    }

    #[test]
    fn unalias_contexts() {
        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("cs").into(), Some(DatagenCalendar::Gregorian))
            .unwrap();

        let cs_dates = symbols::convert_dates(&data, DatagenCalendar::Gregorian);

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates.months.stand_alone.is_some());

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .abbreviated
            .is_none());
        assert!(cs_dates
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .short
            .is_none());
        assert!(cs_dates
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .narrow
            .is_none());
        assert!(cs_dates.months.stand_alone.as_ref().unwrap().wide.is_some());

        // Czech weekdays are unaliased because they completely overlap.
        assert!(cs_dates.weekdays.stand_alone.is_none());
    }
}
