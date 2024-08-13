// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use cldr_serde::time_zones::bcp47_tzid::Bcp47TzidAliasData;
use cldr_serde::time_zones::meta_zones::MetazoneAliasData;
use cldr_serde::time_zones::meta_zones::ZonePeriod;
use cldr_serde::time_zones::time_zone_names::TimeZoneNames;
use icu::datetime::provider::time_zones::*;
use icu::datetime::provider::time_zones::{MetazoneId, TimeZoneBcp47Id};
use icu::timezone::provider::*;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;

mod convert;
mod names;

#[derive(Debug, Copy, Clone)]
struct CldrTimeZonesData<'a> {
    pub(crate) time_zone_names_resource: &'a TimeZoneNames,
    pub(crate) bcp47_tzids_resource: &'a BTreeMap<TimeZoneBcp47Id, Bcp47TzidAliasData>,
    pub(crate) meta_zone_ids_resource: &'a BTreeMap<MetazoneId, MetazoneAliasData>,
    pub(crate) meta_zone_periods_resource: &'a BTreeMap<String, ZonePeriod>,
}

macro_rules! impl_data_provider {
    ($($marker:ident),+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;

                    let resource: &cldr_serde::time_zones::time_zone_names::Resource = self
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(req.id.locale, "timeZoneNames.json")?;

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
                        payload: DataPayload::from_owned(
                            <$marker as DynamicDataMarker>::DataStruct::from(CldrTimeZonesData {
                                time_zone_names_resource,
                                bcp47_tzids_resource,
                                meta_zone_ids_resource,
                                meta_zone_periods_resource,
                            }),
                        ),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    if <$marker>::INFO == MetazonePeriodV1Marker::INFO {
                        // MetazonePeriodV1 does not require localized time zone data
                        Ok([Default::default()].into_iter().collect())
                    } else {
                        Ok(self
                            .cldr()?
                            .dates("gregorian")
                            .list_locales()?
                            .map(DataIdentifierCow::from_locale)
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
    use icu::timezone::ZoneVariant;
    use tinystr::tinystr;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu::locale::langid;

        let provider = SourceDataProvider::new_testing();

        let time_zone_formats: DataResponse<TimeZoneFormatsV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!("GMT", time_zone_formats.payload.get().gmt_zero_format);

        let exemplar_cities: DataResponse<ExemplarCitiesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "Pohnpei",
            exemplar_cities
                .payload
                .get()
                .0
                .get(&TimeZoneBcp47Id(tinystr!(8, "fmpni")))
                .unwrap()
        );

        let generic_names_long: DataResponse<MetazoneGenericNamesLongV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "Australian Central Western Time",
            generic_names_long
                .payload
                .get()
                .defaults
                .get(&MetazoneId(tinystr!(4, "aucw")))
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            generic_names_long
                .payload
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_long: DataResponse<MetazoneSpecificNamesLongV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long
                .payload
                .get()
                .defaults
                .get_2d(&MetazoneId(tinystr!(4, "aucw")), &ZoneVariant::standard())
                .unwrap()
        );
        assert_eq!(
            "Coordinated Universal Time",
            specific_names_long
                .payload
                .get()
                .overrides
                .get_2d(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &ZoneVariant::standard()
                )
                .unwrap()
        );

        let generic_names_short: DataResponse<MetazoneGenericNamesShortV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "PT",
            generic_names_short
                .payload
                .get()
                .defaults
                .get(&MetazoneId(tinystr!(4, "ampa")))
                .unwrap()
        );
        assert_eq!(
            "UTC",
            generic_names_short
                .payload
                .get()
                .overrides
                .get(&TimeZoneBcp47Id(tinystr!(8, "utc")))
                .unwrap()
        );

        let specific_names_short: DataResponse<MetazoneSpecificNamesShortV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(
            "PDT",
            specific_names_short
                .payload
                .get()
                .defaults
                .get_2d(&MetazoneId(tinystr!(4, "ampa")), &ZoneVariant::daylight())
                .unwrap()
        );
        assert_eq!(
            "UTC",
            specific_names_short
                .payload
                .get()
                .overrides
                .get_2d(
                    &TimeZoneBcp47Id(tinystr!(8, "utc")),
                    &ZoneVariant::standard()
                )
                .unwrap()
        );

        let metazone_period: DataResponse<MetazonePeriodV1Marker> =
            provider.load(Default::default()).unwrap();
        assert_eq!(
            Some(MetazoneId(tinystr!(4, "mgmt"))),
            metazone_period
                .payload
                .get()
                .0
                .get_copied_2d(&TimeZoneBcp47Id(tinystr!(8, "gblon")), &962040)
                .unwrap()
        );
    }
}
