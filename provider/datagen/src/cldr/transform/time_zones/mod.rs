// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr::cldr_serde;
use crate::cldr::cldr_serde::time_zones::time_zone_names::TimeZoneNames;
use crate::cldr::cldr_serde::time_zones::CldrTimeZonesData;
use crate::cldr::error::Error;
use crate::cldr::reader::{get_langid_subdirectories, get_langid_subdirectory, open_reader};
use crate::cldr::CldrPaths;
use elsa::sync::FrozenBTreeMap;
use icu_datetime::provider::time_zones::*;
use icu_locid::LanguageIdentifier;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::RwLock;

mod convert;

/// A data provider reading from CLDR JSON zones files.
#[derive(Debug)]
pub struct TimeZonesProvider {
    path: PathBuf,
    time_zone_names_data: FrozenBTreeMap<LanguageIdentifier, Box<TimeZoneNames>>,
    bcp47_tzid_path: PathBuf,
    bcp47_tzid_data: RwLock<LiteMap<String, String>>,
    meta_zone_id_path: PathBuf,
    meta_zone_id_data: RwLock<LiteMap<String, String>>,
}

impl TryFrom<&dyn CldrPaths> for TimeZonesProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        Ok(Self {
            path: cldr_paths.cldr_dates_gregorian()?.join("main"),
            time_zone_names_data: FrozenBTreeMap::new(),
            bcp47_tzid_path: cldr_paths.cldr_bcp47()?.join("bcp47"),
            bcp47_tzid_data: RwLock::new(LiteMap::new()),
            meta_zone_id_path: cldr_paths.cldr_core()?.join("supplemental"),
            meta_zone_id_data: RwLock::new(LiteMap::new()),
        })
    }
}

macro_rules! impl_resource_provider {
    ($($marker:ident),+) => {
        $(
            impl ResourceProvider<$marker> for TimeZonesProvider {
                fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    let langid = req.options.langid();

                    let time_zone_names = if let Some(time_zone_names) = self.time_zone_names_data.get(&langid) {
                        time_zone_names
                    } else {
                        let path = get_langid_subdirectory(&self.path, &langid)?
                            .ok_or_else(|| DataErrorKind::MissingLocale.with_req(<$marker>::KEY, req))?
                            .join("timeZoneNames.json");
                        let mut resource: cldr_serde::time_zones::time_zone_names::Resource =
                            serde_json::from_reader(open_reader(&path)?)
                                .map_err(|e| Error::Json(e, Some(path)))?;
                        self.time_zone_names_data.insert(langid.clone(), Box::new(resource
                            .main
                            .0
                            .remove(&langid)
                            .expect("CLDR file contains the expected language")
                            .dates
                            .time_zone_names))
                    };

                    if self.bcp47_tzid_data.read().unwrap().len() == 0 {
                        let bcp47_time_zone_path = self.bcp47_tzid_path.join("timezone.json");

                        let resource: cldr_serde::time_zones::bcp47_tzid::Resource =
                            serde_json::from_reader(open_reader(&bcp47_time_zone_path)?)
                                .map_err(|e| Error::Json(e, Some(bcp47_time_zone_path)))?;
                         let r = resource
                            .keyword
                            .u
                            .time_zones
                            .values;

                        let mut data_guard = self.bcp47_tzid_data.write().unwrap();
                        for (bcp47_tzid, bcp47_tzid_data) in r.iter() {
                            if let Some(alias) = &bcp47_tzid_data.alias {
                                for data_value in alias.split(" ") {
                                    data_guard.insert(data_value.to_string(), bcp47_tzid.to_string());
                                }
                            }
                        }
                    }

                    if self.meta_zone_id_data.read().unwrap().len() == 0 {
                        let meta_zone_id_path = self.meta_zone_id_path.join("metaZones.json");

                        let resource: cldr_serde::time_zones::meta_zones::Resource =
                            serde_json::from_reader(open_reader(&meta_zone_id_path)?)
                                .map_err(|e| Error::Json(e, Some(meta_zone_id_path)))?;
                         let r = resource
                            .supplemental
                            .meta_zones
                            .meta_zone_ids
                            .0;

                        let mut data_guard = self.meta_zone_id_data.write().unwrap();
                        for (meta_zone_id, meta_zone_id_data) in r.iter() {
                            data_guard.insert(meta_zone_id_data.long_id.to_string(), meta_zone_id.to_string());
                        }
                    }

                    let cldr_time_zones_data = CldrTimeZonesData {
                        time_zone_names,
                        bcp47_tzids: &self.bcp47_tzid_data.read().unwrap(),
                        meta_zone_ids: &self.meta_zone_id_data.read().unwrap(),
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
                fn supported_options(
                    &self,
                ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
                    Ok(Box::new(
                        get_langid_subdirectories(&self.path)?
                            .map(Into::<ResourceOptions>::into),
                    ))
                }
            }
        )+

        icu_provider::impl_dyn_provider!(TimeZonesProvider, [$($marker),+,], SERDE_SE, ITERABLE_SERDE_SE, DATA_CONVERTER);
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

        let cldr_paths = crate::cldr::cldr_paths::for_test();
        let provider = TimeZonesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

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
        assert_eq!("Pohnpei", exemplar_cities.get().0.get("fmpni").unwrap());

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
            generic_names_long.get().defaults.get("aucw").unwrap()
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
                .get("aucw", &tinystr!(8, "standard"))
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
            generic_names_short.get().defaults.get("ampa").unwrap()
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
                .get("ampa", &tinystr!(8, "daylight"))
                .unwrap()
        );
    }
}
