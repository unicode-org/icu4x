// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use either::Either;
use icu::datetime::provider::calendar::*;
use icu::locale::extensions::unicode::value;
use icu::locale::extensions::unicode::Value;
use icu_provider::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::OnceLock;
use symbols::get_era_code_map;

mod neo;
mod neo_skeleton;
mod patterns;
mod skeletons;
mod symbols;
pub(crate) mod week_data;

pub(crate) static SUPPORTED_CALS: OnceLock<
    HashMap<icu::locale::extensions::unicode::Value, &'static str>,
> = OnceLock::new();

fn supported_cals() -> &'static HashMap<icu::locale::extensions::unicode::Value, &'static str> {
    SUPPORTED_CALS.get_or_init(|| {
        [
            (value!("buddhist"), "buddhist"),
            (value!("chinese"), "chinese"),
            (value!("coptic"), "coptic"),
            (value!("dangi"), "dangi"),
            (value!("ethiopic"), "ethiopic"),
            (value!("gregory"), "gregorian"),
            (value!("hebrew"), "hebrew"),
            (value!("indian"), "indian"),
            (value!("islamic"), "islamic"),
            ("islamic-civil".parse().unwrap(), "islamic"),
            (value!("japanese"), "japanese"),
            (value!("japanext"), "japanese"),
            (value!("persian"), "persian"),
            (value!("roc"), "roc"),
            ("islamic-tbla".parse().unwrap(), "islamic"),
            ("islamic-umalqura".parse().unwrap(), "islamic"),
        ]
        .into_iter()
        .collect()
    })
}

impl SourceDataProvider {
    pub(crate) fn get_datetime_resources(
        &self,
        locale: &DataLocale,
        calendar: Either<&Value, &str>,
    ) -> Result<cldr_serde::ca::Dates, DataError> {
        let is_japanext = calendar == Either::Left(&value!("japanext"));
        let cldr_cal = match calendar {
            Either::Left(value) => supported_cals()
                .get(value)
                .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?,
            Either::Right(s) => s,
        };

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
            if !is_japanext {
                let modern_japanese_eras = get_era_code_map()["japanese"]
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
                .dates("gregorian")
                .read_and_parse::<cldr_serde::ca::Resource>(locale, "ca-gregorian.json")?
                .main
                .value
                .dates
                .calendars
                .get("gregorian")
                .expect("CLDR file contains a gregorian calendar")
                .eras
                .as_ref()
                .expect("gregorian must have eras");

            eras.names
                .extend(greg_eras.names.iter().map(|(k, v)| (k.clone(), v.clone())));
            eras.abbr
                .extend(greg_eras.names.iter().map(|(k, v)| (k.clone(), v.clone())));
            eras.narrow
                .extend(greg_eras.names.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        Ok(data)
    }
}

macro_rules! impl_data_provider {
    ($marker:ident, $expr:expr, $calendar:literal) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data = self.get_datetime_resources(
                    &req.id.locale,
                    Either::Left(&$calendar.parse().unwrap()),
                )?;

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

                let cldr_cal = supported_cals()
                    .get(&$calendar.parse().unwrap())
                    .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
                r.extend(
                    self.cldr()?
                        .dates(cldr_cal)
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
    "buddhist"
);
impl_data_provider!(BuddhistDateSymbolsV1, symbols::convert_dates, "buddhist");
impl_data_provider!(
    ChineseDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "chinese"
);
impl_data_provider!(ChineseDateSymbolsV1, symbols::convert_dates, "chinese");
impl_data_provider!(
    CopticDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "coptic"
);
impl_data_provider!(CopticDateSymbolsV1, symbols::convert_dates, "coptic");
impl_data_provider!(
    DangiDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "dangi"
);
impl_data_provider!(DangiDateSymbolsV1, symbols::convert_dates, "dangi");
impl_data_provider!(
    EthiopianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "ethiopic"
);
impl_data_provider!(EthiopianDateSymbolsV1, symbols::convert_dates, "ethiopic");
impl_data_provider!(
    GregorianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "gregory"
);
impl_data_provider!(GregorianDateSymbolsV1, symbols::convert_dates, "gregory");
impl_data_provider!(
    HebrewDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "hebrew"
);
impl_data_provider!(HebrewDateSymbolsV1, symbols::convert_dates, "hebrew");
impl_data_provider!(
    IndianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "indian"
);
impl_data_provider!(IndianDateSymbolsV1, symbols::convert_dates, "indian");
impl_data_provider!(
    HijriDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "islamic-civil"
);
impl_data_provider!(HijriDateSymbolsV1, symbols::convert_dates, "islamic-civil");
impl_data_provider!(
    JapaneseDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "japanese"
);
impl_data_provider!(JapaneseDateSymbolsV1, symbols::convert_dates, "japanese");
impl_data_provider!(
    JapaneseExtendedDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "japanext"
);
impl_data_provider!(
    JapaneseExtendedDateSymbolsV1,
    symbols::convert_dates,
    "japanext"
);
impl_data_provider!(
    PersianDateLengthsV1,
    |dates, _| DateLengths::from(dates),
    "persian"
);
impl_data_provider!(PersianDateSymbolsV1, symbols::convert_dates, "persian");
impl_data_provider!(RocDateLengthsV1, |dates, _| DateLengths::from(dates), "roc");
impl_data_provider!(RocDateSymbolsV1, symbols::convert_dates, "roc");

impl_data_provider!(
    TimeLengthsV1,
    |dates, _| TimeLengths::from(dates),
    "gregory"
);
impl_data_provider!(
    TimeSymbolsV1,
    |dates, _| { symbols::convert_times(dates) },
    "gregory"
);

#[cfg(test)]
mod test {
    use super::*;
    use icu::locale::langid;

    #[test]
    fn test_basic_patterns() {
        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("cs").into(), Either::Left(&value!("gregory")))
            .unwrap();

        let cs_dates = DateLengths::from(&data);

        assert_eq!("d. M. y", cs_dates.date.medium.to_string());
    }

    #[test]
    fn test_with_numbering_system() {
        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("haw").into(), Either::Left(&value!("gregory")))
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
            .get_datetime_resources(&langid!("fil").into(), Either::Left(&value!("gregory")))
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
            .get_datetime_resources(&langid!("cs").into(), Either::Left(&value!("gregory")))
            .unwrap();

        let cs_dates = symbols::convert_dates(&data, "gregory");

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
            .get_datetime_resources(&langid!("cs").into(), Either::Left(&value!("gregory")))
            .unwrap();

        let cs_dates = symbols::convert_dates(&data, "gregory");

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
