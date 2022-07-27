// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::transform::cldr::cldr_serde::time_zones::CldrTimeZonesData;
use icu_datetime::provider::time_zones::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::HashMap;

mod convert;

macro_rules! impl_data_provider {
    ($($marker:ident),+) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let langid = req.locale.get_langid();

                    let resource: &cldr_serde::time_zones::time_zone_names::Resource = self
                        .source
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(&langid, "timeZoneNames.json")?;
                    let time_zone_names = resource
                        .main
                        .0
                        .get(&langid)
                        .expect("CLDR file contains the expected language")
                        .dates
                        .time_zone_names
                        .clone();

                    let resource: &cldr_serde::time_zones::bcp47_tzid::Resource = self
                        .source
                        .cldr()?
                        .bcp47()
                        .read_and_parse("timezone.json")?;

                    let mut bcp47_tzids = HashMap::new();
                    for (bcp47_tzid, bcp47_tzid_data) in resource.keyword.u.time_zones.values.iter() {
                        if let Some(alias) = &bcp47_tzid_data.alias {
                            for data_value in alias.split(" ") {
                                bcp47_tzids.insert(data_value.to_string(), *bcp47_tzid);
                            }
                        }
                    }

                    let resource: &cldr_serde::time_zones::meta_zones::Resource = self
                        .source
                        .cldr()?
                        .core()
                        .read_and_parse("supplemental/metaZones.json")?;

                    let mut meta_zone_ids = HashMap::new();
                    for (meta_zone_id, meta_zone_id_data) in
                        resource.supplemental.meta_zones.meta_zone_ids.0.iter()
                    {
                        meta_zone_ids.insert(meta_zone_id_data.long_id.to_string(), meta_zone_id.clone());
                    }
                    let meta_zone_periods = resource.supplemental.meta_zones.meta_zone_info.time_zone.0.clone();

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: Some(DataPayload::from_owned(
                            <$marker as DataMarker>::Yokeable::from(&CldrTimeZonesData {
                                time_zone_names,
                                bcp47_tzids,
                                meta_zone_ids,
                                meta_zone_periods,
                            }),
                        )),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                    Ok(self
                        .source
                        .cldr()?
                        .dates("gregorian")
                        .list_langs()?
                        .map(DataLocale::from)
                        .collect())
                }
            }
        )+

    };
}

impl_data_provider!(
    TimeZoneFormatsV1Marker,
    ExemplarCitiesV1Marker,
    MetaZoneGenericNamesLongV1Marker,
    MetaZoneGenericNamesShortV1Marker,
    MetaZoneSpecificNamesLongV1Marker,
    MetaZoneSpecificNamesShortV1Marker,
    MetaZonePeriodV1Marker
);

#[cfg(test)]
mod tests {
    use tinystr::tinystr;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu_locid::langid;

        let provider = crate::DatagenProvider::for_test();

        let time_zone_formats: DataPayload<TimeZoneFormatsV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("GMT", time_zone_formats.get().gmt_zero_format);

        let exemplar_cities: DataPayload<ExemplarCitiesV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "Pohnpei",
            exemplar_cities
                .get()
                .0
                .get(&TimeZoneBcp47Id(tinystr!(8, "fmpni")))
                .unwrap()
        );

        let generic_names_long: DataPayload<MetaZoneGenericNamesLongV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "Australian Central Western Time",
            generic_names_long
                .get()
                .defaults
                .get(&MetaZoneId(tinystr!(4, "aucw")))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_long: DataPayload<MetaZoneSpecificNamesLongV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long
                .get()
                .defaults
                .get(&MetaZoneId(tinystr!(4, "aucw")), &tinystr!(8, "standard"))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .get()
                .overrides
                .get(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &tinystr!(8, "standard")
                )
                .unwrap()
        );

        let generic_names_short: DataPayload<MetaZoneGenericNamesShortV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .get()
                .defaults
                .get(&MetaZoneId(tinystr!(4, "ampa")))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_short: DataPayload<MetaZoneSpecificNamesShortV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .get()
                .defaults
                .get(&MetaZoneId(tinystr!(4, "ampa")), &tinystr!(8, "daylight"))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .get()
                .overrides
                .get(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &tinystr!(8, "standard")
                )
                .unwrap()
        );

        let metazone_period: DataPayload<MetaZonePeriodV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            Some(MetaZoneId(tinystr!(4, "mgmt"))),
            metazone_period
                .get()
                .0
                .get_copied(&TimeZoneBcp47Id(tinystr!(8, "gblon")), &962040)
                .unwrap()
        );
    }
}
