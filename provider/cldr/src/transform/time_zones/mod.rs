// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use icu_datetime::provider::time_zones::*;
use icu_locid::LanguageIdentifier;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;

use std::convert::TryFrom;

mod convert;

/// A data provider reading from CLDR JSON zones files.
#[derive(PartialEq, Debug)]
pub struct TimeZonesProvider {
    data: LiteMap<LanguageIdentifier, cldr_serde::time_zone_names::LangTimeZones>,
}

impl TryFrom<&dyn CldrPaths> for TimeZonesProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = LiteMap::new();

        let path = cldr_paths.cldr_dates("gregory")?.join("main");

        let locale_dirs = get_langid_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("timeZoneNames.json");

            let resource: cldr_serde::time_zone_names::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.extend_from_litemap(resource.main.0);
        }

        Ok(Self { data })
    }
}

impl TryFrom<&str> for TimeZonesProvider {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let resource: cldr_serde::time_zone_names::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        Ok(Self {
            data: resource.main.0,
        })
    }
}

impl KeyedDataProvider for TimeZonesProvider {
    fn supported_keys() -> Vec<ResourceKey> {
        vec![
            TimeZoneFormatsV1Marker::KEY,
            ExemplarCitiesV1Marker::KEY,
            MetaZoneGenericNamesLongV1Marker::KEY,
            MetaZoneGenericNamesShortV1Marker::KEY,
            MetaZoneSpecificNamesLongV1Marker::KEY,
            MetaZoneSpecificNamesShortV1Marker::KEY,
        ]
    }
}

impl IterableProvider for TimeZonesProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            self.data
                .iter_keys()
                // TODO(#568): Avoid the clone
                .cloned()
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

macro_rules! impl_data_provider {
    ($id:ident, $marker:ident) => {
        impl ResourceProvider<$marker> for TimeZonesProvider {
            fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<$marker>, DataError> {
                let langid = req
                    .get_langid()
                    .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(<$marker>::KEY, req))?;
                let time_zones = match self.data.get(&langid) {
                    Some(v) => &v.dates.time_zone_names,
                    None => return Err(DataErrorKind::MissingLocale.with_req(<$marker>::KEY, req)),
                };
                let metadata = DataResponseMetadata::default();
                // TODO(#1109): Set metadata.data_langid correctly.
                Ok(DataResponse {
                    metadata,
                    payload: Some(DataPayload::from_owned($id::from(time_zones.clone()))),
                })
            }
        }
    };
}

icu_provider::impl_dyn_provider!(
    TimeZonesProvider,
    [
        TimeZoneFormatsV1Marker,
        ExemplarCitiesV1Marker,
        MetaZoneGenericNamesLongV1Marker,
        MetaZoneGenericNamesShortV1Marker,
        MetaZoneSpecificNamesLongV1Marker,
        MetaZoneSpecificNamesShortV1Marker,
    ],
    SERDE_SE
);

impl_data_provider!(TimeZoneFormatsV1, TimeZoneFormatsV1Marker);
impl_data_provider!(ExemplarCitiesV1, ExemplarCitiesV1Marker);
impl_data_provider!(MetaZoneGenericNamesLongV1, MetaZoneGenericNamesLongV1Marker);
impl_data_provider!(
    MetaZoneGenericNamesShortV1,
    MetaZoneGenericNamesShortV1Marker
);
impl_data_provider!(
    MetaZoneSpecificNamesLongV1,
    MetaZoneSpecificNamesLongV1Marker
);
impl_data_provider!(
    MetaZoneSpecificNamesShortV1,
    MetaZoneSpecificNamesShortV1Marker
);

#[cfg(test)]
mod tests {
    use tinystr::tinystr8;

    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu_locid_macros::langid;

        let cldr_paths = crate::cldr_paths::for_test();
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
        assert_eq!("Pohnpei", exemplar_cities.get()["Pacific/Ponape"]);

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
            generic_names_long.get()["Australia_CentralWestern"]
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
            specific_names_long.get()["Australia_CentralWestern"][&tinystr8!("standard")]
        );

        let generic_names_short: DataPayload<MetaZoneGenericNamesShortV1Marker> = provider
            .load_resource(&DataRequest {
                options: langid!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("PT", generic_names_short.get()["America_Pacific"]);

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
            specific_names_short.get()["America_Pacific"][&tinystr8!("daylight")]
        );
    }
}
