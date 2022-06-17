// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_datetime::provider::calendar::*;
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;

mod patterns;
mod skeletons;
mod symbols;
pub mod week_data;

/// A data provider reading from CLDR JSON dates files.
#[derive(Debug)]
pub struct CommonDateProvider {
    source: SourceData,
    // BCP-47 value -> CLDR identifier
    supported_cals: LiteMap<icu_locid::extensions::unicode::Value, &'static str>,
}

impl From<&SourceData> for CommonDateProvider {
    fn from(source: &SourceData) -> Self {
        CommonDateProvider {
            source: source.clone(),
            supported_cals: [
                (value!("gregory"), "gregorian"),
                (value!("buddhist"), "buddhist"),
                (value!("japanese"), "japanese"),
                (value!("coptic"), "coptic"),
                (value!("indian"), "indian"),
                (value!("ethiopic"), "ethiopic"),
            ]
            .into_iter()
            .collect(),
        }
    }
}

macro_rules! impl_resource_provider {
    ($(($marker:ident, $expr:expr)),+) => {
        $(
            impl ResourceProvider<$marker> for CommonDateProvider {
                fn load_resource(
                    &self,
                    req: &DataRequest,
                ) -> Result<DataResponse<$marker>, DataError> {
                    let langid = req.options.get_langid();
                    let calendar = req
                        .options
                        .get_unicode_ext(&key!("ca"))
                        .ok_or_else(|| DataErrorKind::NeedsVariant.into_error())?;

                    let cldr_cal = self
                        .supported_cals
                        .get(&calendar)
                        .ok_or_else(|| DataErrorKind::MissingVariant.into_error())?;

                    let resource: &cldr_serde::ca::Resource = self
                        .source
                        .get_cldr_paths()?
                        .cldr_dates(cldr_cal).read_and_parse(&langid, &format!("ca-{}.json", cldr_cal))?;

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
                        let ethioaa: &cldr_serde::ca::Resource = self.source.get_cldr_paths()?.cldr_dates("ethiopic").read_and_parse(&langid, "ca-ethiopic-amete-alem.json")?;

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

                    let metadata = DataResponseMetadata::default();
                    // TODO(#1109): Set metadata.data_langid correctly.
                    Ok(DataResponse {
                        metadata,
                        #[allow(clippy::redundant_closure_call)]
                        payload: Some(DataPayload::from_owned(($expr)(&data, &calendar.to_string()))),
                    })
                }
            }

            impl IterableResourceProvider<$marker> for CommonDateProvider {
                fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
                    let mut r = Vec::new();
                    for (cal_value, cldr_cal) in self.supported_cals.iter() {
                        r.extend(self
                                    .source
                                    .get_cldr_paths()?
                                    .cldr_dates(cldr_cal).list_langs()?
                            .map(|lid| {
                                let mut locale: Locale = lid.into();
                                locale
                                    .extensions
                                    .unicode
                                    .keywords
                                    .set(key!("ca"), cal_value.clone());
                                ResourceOptions::from(locale)
                            }));
                    }
                    Ok(r)
                }
            }
        )+

        icu_provider::make_exportable_provider!(CommonDateProvider, [$($marker),+,]);
    };
}

impl_resource_provider!(
    (DateSymbolsV1Marker, symbols::convert_dates),
    (DateSkeletonPatternsV1Marker, |dates, _| {
        DateSkeletonPatternsV1::from(dates)
    }),
    (DatePatternsV1Marker, |dates, _| DatePatternsV1::from(dates))
);

#[cfg(test)]
mod test {
    use super::*;
    use icu_datetime::pattern::runtime::{Pattern, PluralPattern};
    use icu_plurals::PluralCategory;

    #[test]
    fn test_basic_patterns() {
        let provider = CommonDateProvider::from(&SourceData::for_test());

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DatePatternsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.into(),
                metadata: Default::default(),
            })
            .expect("Failed to load payload")
            .take_payload()
            .expect("Failed to retrieve payload");

        assert_eq!("d. M. y", cs_dates.get().date.medium.to_string());
    }

    #[test]
    fn test_with_numbering_system() {
        let provider = CommonDateProvider::from(&SourceData::for_test());

        let locale: Locale = "haw-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DatePatternsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.into(),
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

        let provider = CommonDateProvider::from(&SourceData::for_test());

        let locale: Locale = "fil-u-ca-gregory".parse().unwrap();
        let skeletons: DataPayload<DateSkeletonPatternsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.into(),
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
        let provider = CommonDateProvider::from(&SourceData::for_test());

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.into(),
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
        let provider = CommonDateProvider::from(&SourceData::for_test());

        let locale: Locale = "cs-u-ca-gregory".parse().unwrap();
        let cs_dates: DataPayload<DateSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.into(),
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
