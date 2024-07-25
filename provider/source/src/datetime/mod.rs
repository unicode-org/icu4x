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
            (value!("islamicc"), "islamic"),
            (value!("japanese"), "japanese"),
            (value!("japanext"), "japanese"),
            (value!("persian"), "persian"),
            (value!("roc"), "roc"),
            (value!("tbla"), "islamic"),
            (value!("umalqura"), "islamic"),
        ]
        .into_iter()
        .collect()
    })
}

impl SourceDataProvider {
    fn get_datetime_resources(
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

        // CLDR treats ethiopian and ethioaa as separate calendars; however we treat them as a single resource key that
        // supports symbols for both era patterns based on the settings on the date. Load in ethioaa data as well when dealing with
        // ethiopian.
        if cldr_cal == "ethiopic" {
            let ethioaa: &cldr_serde::ca::Resource = self
                .cldr()?
                .dates("ethiopic")
                .read_and_parse(locale, "ca-ethiopic-amete-alem.json")?;

            let ethioaa_data = ethioaa
                .main
                .value
                .dates
                .calendars
                .get("ethiopic-amete-alem")
                .expect("CLDR ca-ethiopic-amete-alem.json contains the expected calendar")
                .clone();

            let ethioaa_eras = ethioaa_data.eras.as_ref().expect("ethioaa must have eras");
            let mundi_name = ethioaa_eras
                .names
                .get("0")
                .expect("ethiopic-amete-alem calendar must have 0 era");
            let mundi_abbr = ethioaa_eras
                .abbr
                .get("0")
                .expect("ethiopic-amete-alem calendar must have 0 era");
            let mundi_narrow = ethioaa_eras
                .narrow
                .get("0")
                .expect("ethiopic-amete-alem calendar must have 0 era");

            let eras = data.eras.as_mut().expect("ethiopic must have eras");
            eras.names.insert("2".to_string(), mundi_name.clone());
            eras.abbr.insert("2".to_string(), mundi_abbr.clone());
            eras.narrow.insert("2".to_string(), mundi_narrow.clone());
        }

        if cldr_cal == "japanese" {
            let eras = data.eras.as_mut().expect("japanese must have eras");
            // Filter out non-modern eras
            if !is_japanext {
                let modern_japanese_eras = self.cldr()?.modern_japanese_eras()?;
                eras.names.retain(|e, _| modern_japanese_eras.contains(e));
                eras.abbr.retain(|e, _| modern_japanese_eras.contains(e));
                eras.narrow.retain(|e, _| modern_japanese_eras.contains(e));
            }

            // Splice in gregorian data for pre-meiji
            let greg_resource: &cldr_serde::ca::Resource = self
                .cldr()?
                .dates("gregorian")
                .read_and_parse(locale, "ca-gregorian.json")?;

            let greg = greg_resource
                .main
                .value
                .dates
                .calendars
                .get("gregorian")
                .expect("CLDR file contains a gregorian calendar")
                .clone();

            let greg_eras = greg.eras.as_ref().expect("gregorian must have eras");

            eras.names.insert(
                "-2".into(),
                greg_eras
                    .names
                    .get("0")
                    .expect("Gregorian calendar must have data for BC")
                    .into(),
            );
            eras.names.insert(
                "-1".into(),
                greg_eras
                    .names
                    .get("1")
                    .expect("Gregorian calendar must have data for AD")
                    .into(),
            );
            eras.abbr.insert(
                "-2".into(),
                greg_eras
                    .abbr
                    .get("0")
                    .expect("Gregorian calendar must have data for BC")
                    .into(),
            );
            eras.abbr.insert(
                "-1".into(),
                greg_eras
                    .abbr
                    .get("1")
                    .expect("Gregorian calendar must have data for AD")
                    .into(),
            );
            eras.narrow.insert(
                "-2".into(),
                greg_eras
                    .narrow
                    .get("0")
                    .expect("Gregorian calendar must have data for BC")
                    .into(),
            );
            eras.narrow.insert(
                "-1".into(),
                greg_eras
                    .narrow
                    .get("1")
                    .expect("Gregorian calendar must have data for AD")
                    .into(),
            );
        }

        Ok(data)
    }
}

macro_rules! impl_data_provider {
    ($marker:ident, $expr:expr, $calendar:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data =
                    self.get_datetime_resources(&req.id.locale, Either::Left(&value!($calendar)))?;

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
                    .get(&value!($calendar))
                    .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;
                r.extend(
                    self.cldr()?
                        .dates(cldr_cal)
                        .list_locales()?
                        .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l))),
                );

                // TODO(#3212): Remove
                if $marker::INFO == TimeLengthsV1Marker::INFO {
                    r.retain(|id| {
                        *id.locale != DataLocale::from(icu::locale::langid!("byn"))
                            && *id.locale != DataLocale::from(icu::locale::langid!("ssy"))
                    });
                }

                Ok(r)
            }
        }
    };
}

impl_data_provider!(
    BuddhistDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "buddhist"
);
impl_data_provider!(
    BuddhistDateSymbolsV1Marker,
    symbols::convert_dates,
    "buddhist"
);
impl_data_provider!(
    ChineseDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "chinese"
);
impl_data_provider!(
    ChineseDateSymbolsV1Marker,
    symbols::convert_dates,
    "chinese"
);
impl_data_provider!(
    CopticDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "coptic"
);
impl_data_provider!(CopticDateSymbolsV1Marker, symbols::convert_dates, "coptic");
impl_data_provider!(
    DangiDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "dangi"
);
impl_data_provider!(DangiDateSymbolsV1Marker, symbols::convert_dates, "dangi");
impl_data_provider!(
    EthiopianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "ethiopic"
);
impl_data_provider!(
    EthiopianDateSymbolsV1Marker,
    symbols::convert_dates,
    "ethiopic"
);
impl_data_provider!(
    GregorianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "gregory"
);
impl_data_provider!(
    GregorianDateSymbolsV1Marker,
    symbols::convert_dates,
    "gregory"
);
impl_data_provider!(
    HebrewDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "hebrew"
);
impl_data_provider!(HebrewDateSymbolsV1Marker, symbols::convert_dates, "hebrew");
impl_data_provider!(
    IndianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "indian"
);
impl_data_provider!(IndianDateSymbolsV1Marker, symbols::convert_dates, "indian");
impl_data_provider!(
    IslamicDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "islamicc"
);
impl_data_provider!(
    IslamicDateSymbolsV1Marker,
    symbols::convert_dates,
    "islamicc"
);
impl_data_provider!(
    JapaneseDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "japanese"
);
impl_data_provider!(
    JapaneseDateSymbolsV1Marker,
    symbols::convert_dates,
    "japanese"
);
impl_data_provider!(
    JapaneseExtendedDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "japanext"
);
impl_data_provider!(
    JapaneseExtendedDateSymbolsV1Marker,
    symbols::convert_dates,
    "japanext"
);
impl_data_provider!(
    PersianDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "persian"
);
impl_data_provider!(
    PersianDateSymbolsV1Marker,
    symbols::convert_dates,
    "persian"
);
impl_data_provider!(
    RocDateLengthsV1Marker,
    |dates, _| DateLengthsV1::from(dates),
    "roc"
);
impl_data_provider!(RocDateSymbolsV1Marker, symbols::convert_dates, "roc");

impl_data_provider!(
    TimeLengthsV1Marker,
    |dates, _| TimeLengthsV1::from(dates),
    "gregory"
);
impl_data_provider!(
    TimeSymbolsV1Marker,
    |dates, _| { symbols::convert_times(dates) },
    "gregory"
);

#[cfg(test)]
mod test {
    use super::*;
    use icu::locale::langid;

    #[test]
    fn test_basic_patterns() {
        let provider = SourceDataProvider::new_testing();

        let cs_dates: DataResponse<GregorianDateLengthsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("cs").into()),
                ..Default::default()
            })
            .expect("Failed to load payload");

        assert_eq!("d. M. y", cs_dates.payload.get().date.medium.to_string());
    }

    #[test]
    fn test_with_numbering_system() {
        let provider = SourceDataProvider::new_testing();

        let cs_dates: DataResponse<GregorianDateLengthsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("haw").into()),
                ..Default::default()
            })
            .expect("Failed to load payload");

        assert_eq!("d MMM y", cs_dates.payload.get().date.medium.to_string());
        // TODO(#308): Support numbering system variations. We currently throw them away.
        assert_eq!("d/M/yy", cs_dates.payload.get().date.short.to_string());
    }

    #[test]
    fn test_datetime_skeletons() {
        use icu::datetime::pattern::runtime::{Pattern, PluralPattern};
        use icu::plurals::PluralCategory;
        use std::convert::TryFrom;

        let data = SourceDataProvider::new_testing()
            .get_datetime_resources(&langid!("fil").into(), Either::Left(&value!("gregory")))
            .unwrap();

        let skeletons = DateSkeletonPatternsV1::from(&data).0;

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
        use icu::calendar::types::MonthCode;
        use tinystr::tinystr;
        let provider = SourceDataProvider::new_testing();

        let cs_dates: DataResponse<GregorianDateSymbolsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("cs").into()),
                ..Default::default()
            })
            .unwrap();

        assert_eq!(
            "srpna",
            cs_dates
                .payload
                .get()
                .months
                .format
                .wide
                .get(MonthCode(tinystr!(4, "M08")))
                .unwrap()
        );

        assert_eq!(
            "po",
            cs_dates
                .payload
                .get()
                .weekdays
                .format
                .short
                .as_ref()
                .unwrap()
                .0[1]
        );
    }

    #[test]
    fn unalias_contexts() {
        let provider = SourceDataProvider::new_testing();

        let cs_dates: DataResponse<GregorianDateSymbolsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("cs").into()),
                ..Default::default()
            })
            .unwrap();

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates.payload.get().months.stand_alone.is_some());

        // Czech months are not unaliased because `wide` differs.
        assert!(cs_dates
            .payload
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .abbreviated
            .is_none());
        assert!(cs_dates
            .payload
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .short
            .is_none());
        assert!(cs_dates
            .payload
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .narrow
            .is_none());
        assert!(cs_dates
            .payload
            .get()
            .months
            .stand_alone
            .as_ref()
            .unwrap()
            .wide
            .is_some());

        // Czech weekdays are unaliased because they completely overlap.
        assert!(cs_dates.payload.get().weekdays.stand_alone.is_none());
    }
}
