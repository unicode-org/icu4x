// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use cldr_serde::time_zones::bcp47_tzid::Bcp47TzidAliasData;
use cldr_serde::time_zones::meta_zones::MetazoneAliasData;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::TimeZoneNames;
use icu_datetime::provider::time_zones::*;
use icu_datetime::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_timezone::provider::*;
use std::collections::HashMap;

mod convert;

#[derive(Debug, Copy, Clone)]
struct CldrTimeZonesData<'a> {
    pub time_zone_names_resource: &'a TimeZoneNames,
    pub bcp47_tzids_resource: &'a HashMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
    pub meta_zone_ids_resource: &'a HashMap<MetazoneId, MetazoneAliasData>,
    pub meta_zone_periods_resource: &'a HashMap<String, ZonePeriod>,
}

macro_rules! impl_data_provider {
    ($($marker:ident),+) => {
        $(
            impl DataProvider<$marker> for crate::DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let langid = req.locale.get_langid();

                    let resource: &cldr_serde::time_zones::time_zone_names::Resource = self
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(&langid, "timeZoneNames.json")?;

                    let time_zone_names_resource = &resource.main.value.dates.time_zone_names;

                    let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
                        self.cldr()?.bcp47().read_and_parse("timezone.json")?;

                    let bcp47_tzids_resource = &resource.keyword.u.time_zones.values;

                    let resource: &cldr_serde::time_zones::meta_zones::Resource = self
                        .cldr()?
                        .core()
                        .read_and_parse("supplemental/metaZones.json")?;

                    let meta_zone_ids_resource = &resource.supplemental.meta_zones.meta_zone_ids.0;

                    let meta_zone_periods_resource =
                        &resource.supplemental.meta_zones.meta_zone_info.time_zone.0;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: Some(DataPayload::from_owned(
                            <$marker as DataMarker>::Yokeable::from(CldrTimeZonesData {
                                time_zone_names_resource,
                                bcp47_tzids_resource,
                                meta_zone_ids_resource,
                                meta_zone_periods_resource,
                            }),
                        )),
                    })
                }
            }

            impl IterableDataProvider<$marker> for crate::DatagenProvider {
                fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                    if <$marker>::KEY == MetazonePeriodV1Marker::KEY {
                        // MetazonePeriodV1 does not require localized time zone data
                        Ok(vec![Default::default()])
                    } else {
                        Ok(self
                            .cldr()?
                            .dates("gregorian")
                            .list_langs()?
                            .map(DataLocale::from)
                            .collect())
                    }
                }
            }
        )+
    };
}

impl_data_provider!(
    TimeZoneFormatsV1Marker,
    ExemplarCitiesV1Marker,
    MetazoneGenericNamesLongV1Marker,
    MetazoneGenericNamesShortV1Marker,
    MetazoneSpecificNamesLongV1Marker,
    MetazoneSpecificNamesShortV1Marker,
    MetazonePeriodV1Marker
);

#[cfg(test)]
mod tests {
    use icu_timezone::ZoneVariant;
    use tinystr::tinystr;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu_locid::langid;

        let provider = crate::DatagenProvider::latest_tested_offline_subset();

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

        let generic_names_long: DataPayload<MetazoneGenericNamesLongV1Marker> = provider
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
                .get(&MetazoneId(tinystr!(4, "aucw")))
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

        let specific_names_long: DataPayload<MetazoneSpecificNamesLongV1Marker> = provider
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
                .get_2d(&MetazoneId(tinystr!(4, "aucw")), &ZoneVariant::standard())
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .get()
                .overrides
                .get_2d(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &ZoneVariant::standard()
                )
                .unwrap()
        );

        let generic_names_short: DataPayload<MetazoneGenericNamesShortV1Marker> = provider
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
                .get(&MetazoneId(tinystr!(4, "ampa")))
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

        let specific_names_short: DataPayload<MetazoneSpecificNamesShortV1Marker> = provider
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
                .get_2d(&MetazoneId(tinystr!(4, "ampa")), &ZoneVariant::daylight())
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .get()
                .overrides
                .get_2d(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &ZoneVariant::standard()
                )
                .unwrap()
        );

        let metazone_period: DataPayload<MetazonePeriodV1Marker> = provider
            .load(Default::default())
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            Some(MetazoneId(tinystr!(4, "mgmt"))),
            metazone_period
                .get()
                .0
                .get_copied_2d(&TimeZoneBcp47Id(tinystr!(8, "gblon")), &962040)
                .unwrap()
        );
    }
}
