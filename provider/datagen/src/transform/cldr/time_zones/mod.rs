// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::transform::cldr::cldr_serde::time_zones::time_zone_names::TimeZoneNames;
use crate::transform::cldr::cldr_serde::time_zones::CldrTimeZonesData;
use crate::SourceData;
use elsa::sync::FrozenBTreeMap;
use icu_datetime::provider::time_zones::*;
use icu_datetime::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};
use icu_locid::LanguageIdentifier;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::sync::RwLock;

mod convert;

/// A data provider reading from CLDR JSON zones files.
#[derive(Debug)]
pub struct TimeZonesProvider {
    source: SourceData,
    time_zone_names_data: FrozenBTreeMap<LanguageIdentifier, Box<TimeZoneNames>>,
    bcp47_tzid_data: RwLock<LiteMap<String, TimeZoneBcp47Id>>,
    meta_zone_id_data: RwLock<LiteMap<String, MetaZoneId>>,
}

impl From<&SourceData> for TimeZonesProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            time_zone_names_data: FrozenBTreeMap::new(),
            bcp47_tzid_data: RwLock::new(LiteMap::new()),
            meta_zone_id_data: RwLock::new(LiteMap::new()),
        }
    }
}

macro_rules! impl_resource_provider {
    ($($marker:ident),+) => {
        $(
            impl ResourceProvider<$marker> for TimeZonesProvider {
                fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let langid = req.options.get_langid();

                    let time_zone_names = if let Some(time_zone_names) = self.time_zone_names_data.get(&langid) {
                        time_zone_names
                    } else {
                        let resource: &cldr_serde::time_zones::time_zone_names::Resource =
                        self
                        .source
                        .get_cldr_paths()?
                        .cldr_dates("gregorian").read_and_parse(&langid, "timeZoneNames.json")?;
                        self.time_zone_names_data.insert(
                            langid.clone(),
                            Box::new(
                                resource
                                    .main
                                    .0
                                    .get(&langid)
                                    .expect("CLDR file contains the expected language")
                                    .dates
                                    .time_zone_names
                                    .clone(),
                            ),
                        )
                    };

                    if self.bcp47_tzid_data.read().expect("poison").len() == 0 {
                        let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
                            self.source.get_cldr_paths()?.cldr_bcp47().read_and_parse("timezone.json")?;

                        let mut data_guard = self.bcp47_tzid_data.write().expect("poison");
                        for (bcp47_tzid, bcp47_tzid_data) in resource.keyword.u.time_zones.values.iter() {
                            if let Some(alias) = &bcp47_tzid_data.alias {
                                for data_value in alias.split(" ") {
                                    data_guard.insert(data_value.to_string(), *bcp47_tzid);
                                }
                            }
                        }
                    }

                    if self.meta_zone_id_data.read().expect("poison").len() == 0 {
                        let resource: &cldr_serde::time_zones::meta_zones::Resource =
                            self.source.get_cldr_paths()?.cldr_core().read_and_parse("supplemental/metaZones.json")?;

                        let mut data_guard = self.meta_zone_id_data.write().expect("poison");
                        for (meta_zone_id, meta_zone_id_data) in resource.supplemental.meta_zones.meta_zone_ids.0.iter() {
                            data_guard.insert(
                                meta_zone_id_data.long_id.to_string(),
                                meta_zone_id.clone(),
                            );
                        }
                    }

                    let cldr_time_zones_data = CldrTimeZonesData {
                        time_zone_names,
                        bcp47_tzids: &self.bcp47_tzid_data.read().expect("poison"),
                        meta_zone_ids: &self.meta_zone_id_data.read().expect("poison"),
                    };

                    let metadata = DataResponseMetadata::default();
                    // TODO(#1109): Set metadata.data_langid correctly.
                    Ok(DataResponse {
                        metadata,
                        payload: Some(DataPayload::from_owned(<$marker as DataMarker>::Yokeable::from(&cldr_time_zones_data))),
                    })
                }
            }

            impl IterableResourceProvider<$marker> for TimeZonesProvider {
                fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
                    Ok(
                            self
                                .source
                                .get_cldr_paths()?
                                .cldr_dates("gregorian").list_langs()?
                        .map(Into::<ResourceOptions>::into).collect(),
                    )
                }
            }
        )+

        icu_provider::make_exportable_provider!(TimeZonesProvider, [$($marker),+,]);
    };
}

impl_resource_provider!(
    TimeZoneFormatsV1Marker,
    ExemplarCitiesV1Marker,
    MetaZoneGenericNamesLongV1Marker,
    MetaZoneGenericNamesShortV1Marker,
    MetaZoneSpecificNamesLongV1Marker,
    MetaZoneSpecificNamesShortV1Marker
);

#[cfg(test)]
mod tests {
    use tinystr::tinystr;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu_locid::langid;

        let provider = TimeZonesProvider::from(&SourceData::for_test());

        let time_zone_formats: DataPayload<TimeZoneFormatsV1Marker> = provider
            .load_resource(&DataRequest {
                options: langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("GMT", time_zone_formats.get().gmt_zero_format);

        let exemplar_cities: DataPayload<ExemplarCitiesV1Marker> = provider
            .load_resource(&DataRequest {
                options: langid!("en").into(),
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
            .load_resource(&DataRequest {
                options: langid!("en").into(),
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
            .load_resource(&DataRequest {
                options: langid!("en").into(),
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
            .load_resource(&DataRequest {
                options: langid!("en").into(),
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
            .load_resource(&DataRequest {
                options: langid!("en").into(),
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
    }
}
