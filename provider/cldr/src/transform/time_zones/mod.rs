// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::provider::{key, time_zones::*};
use icu_provider::{
    iter::{IterableDataProviderCore, KeyedDataProvider},
    prelude::*,
};

use std::convert::TryFrom;
use std::marker::PhantomData;

mod cldr_json;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 6] = [
    key::TIMEZONE_FORMATS_V1,
    key::TIMEZONE_EXEMPLAR_CITIES_V1,
    key::TIMEZONE_GENERIC_NAMES_LONG_V1,
    key::TIMEZONE_GENERIC_NAMES_SHORT_V1,
    key::TIMEZONE_SPECIFIC_NAMES_LONG_V1,
    key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1,
];

/// A data provider reading from CLDR JSON zones files.
#[derive(PartialEq, Debug)]
pub struct TimeZonesProvider<'d> {
    data: Vec<(CldrLangID, cldr_json::LangTimeZones)>,
    phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for TimeZonesProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_dates()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("timeZoneNames.json");

            let mut resource: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self {
            data,
            phantom: PhantomData,
        })
    }
}

impl TryFrom<&str> for TimeZonesProvider<'_> {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let resource: cldr_json::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        Ok(Self {
            data: resource.main.0,
            phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for TimeZonesProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::TimeZones || resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> IterableDataProviderCore for TimeZonesProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                langid: Some(l.langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

macro_rules! impl_data_provider {
    ($id:ident: $lt:lifetime, $marker:ident) => {
        impl<$lt, 'dp: $lt> DataProvider<'dp, 'dp, $marker> for TimeZonesProvider<$lt> {
            fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'dp, 'dp, $marker>, DataError> {
                TimeZonesProvider::supports_key(&req.resource_path.key)?;
                let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
                let time_zones = match self
                    .data
                    .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
                {
                    Ok(idx) => &self.data[idx].1.dates.time_zone_names,
                    Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
                };
                Ok(DataResponse {
                    metadata: DataResponseMetadata {
                        data_langid: req.resource_path.options.langid.clone(),
                    },
                    payload: Some(DataPayload::from_owned($id::from(time_zones.clone()))),
                })
            }
        }
    };
}

icu_provider::impl_dyn_provider!(TimeZonesProvider<'d>, {
    key::TIMEZONE_FORMATS_V1 => TimeZoneFormatsV1Marker,
    key::TIMEZONE_EXEMPLAR_CITIES_V1 => ExemplarCitiesV1Marker,
    key::TIMEZONE_GENERIC_NAMES_LONG_V1 => MetaZoneGenericNamesLongV1Marker,
    key::TIMEZONE_GENERIC_NAMES_SHORT_V1 => MetaZoneGenericNamesShortV1Marker,
    key::TIMEZONE_SPECIFIC_NAMES_LONG_V1 => MetaZoneSpecificNamesLongV1Marker,
    key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1 => MetaZoneSpecificNamesShortV1Marker,
}, ERASED, 'd);

icu_provider::impl_dyn_provider!(TimeZonesProvider<'d>, {
    key::TIMEZONE_FORMATS_V1 => TimeZoneFormatsV1Marker,
    key::TIMEZONE_EXEMPLAR_CITIES_V1 => ExemplarCitiesV1Marker,
    key::TIMEZONE_GENERIC_NAMES_LONG_V1 => MetaZoneGenericNamesLongV1Marker,
    key::TIMEZONE_GENERIC_NAMES_SHORT_V1 => MetaZoneGenericNamesShortV1Marker,
    key::TIMEZONE_SPECIFIC_NAMES_LONG_V1 => MetaZoneSpecificNamesLongV1Marker,
    key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1 => MetaZoneSpecificNamesShortV1Marker,
}, SERDE_SE, 'd, 's);

impl_data_provider!(TimeZoneFormatsV1: 'd, TimeZoneFormatsV1Marker);
impl_data_provider!(ExemplarCitiesV1: 'd, ExemplarCitiesV1Marker);
impl_data_provider!(MetaZoneGenericNamesLongV1: 'd, MetaZoneGenericNamesLongV1Marker);
impl_data_provider!(MetaZoneGenericNamesShortV1: 'd, MetaZoneGenericNamesShortV1Marker);
impl_data_provider!(MetaZoneSpecificNamesLongV1: 'd, MetaZoneSpecificNamesLongV1Marker);
impl_data_provider!(MetaZoneSpecificNamesShortV1: 'd, MetaZoneSpecificNamesShortV1Marker);

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
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_FORMATS_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("GMT", time_zone_formats.get().gmt_zero_format);

        let exemplar_cities: DataPayload<ExemplarCitiesV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_EXEMPLAR_CITIES_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("Pohnpei", exemplar_cities.get()["Pacific/Ponape"]);

        let generic_names_long: DataPayload<MetaZoneGenericNamesLongV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_GENERIC_NAMES_LONG_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "Australian Central Western Time",
            generic_names_long.get()["Australia_CentralWestern"]
        );

        let specific_names_long: DataPayload<MetaZoneSpecificNamesLongV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_SPECIFIC_NAMES_LONG_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long.get()["Australia_CentralWestern"][&tinystr8!("standard")]
        );

        let generic_names_short: DataPayload<MetaZoneGenericNamesShortV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_GENERIC_NAMES_SHORT_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!("PT", generic_names_short.get()["America_Pacific"]);

        let specific_names_short: DataPayload<MetaZoneSpecificNamesShortV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
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
