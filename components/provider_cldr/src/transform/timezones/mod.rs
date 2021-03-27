// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::provider::{
    key,
    timezones::{
        ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
        MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, TimeZoneFormatsV1,
    },
};
use icu_provider::{
    erased::ErasedDataProvider,
    iter::{IterableDataProviderCore, KeyedDataProvider},
    prelude::*,
    serde::SerdeSeDataStruct,
};
use std::borrow::Cow;
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
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
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
            _phantom: PhantomData,
        })
    }
}

impl TryFrom<&str> for TimeZonesProvider<'_> {
    type Error = Error;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let mut resource: cldr_json::Resource =
            serde_json::from_str(input).map_err(|e| Error::Json(e, None))?;
        data.append(&mut resource.main.0);

        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for TimeZonesProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::TimeZones {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
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

impl<'d> ErasedDataProvider<'d> for TimeZonesProvider<'d> {
    fn load_erased(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, dyn icu_provider::erased::ErasedDataStruct>, DataError> {
        match req.resource_path.key {
            key::TIMEZONE_FORMATS_V1 => {
                let result: DataResponse<TimeZoneFormatsV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_EXEMPLAR_CITIES_V1 => {
                let result: DataResponse<ExemplarCitiesV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_GENERIC_NAMES_LONG_V1 => {
                let result: DataResponse<MetaZoneGenericNamesLongV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_GENERIC_NAMES_SHORT_V1 => {
                let result: DataResponse<MetaZoneGenericNamesShortV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_SPECIFIC_NAMES_LONG_V1 => {
                let result: DataResponse<MetaZoneSpecificNamesLongV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1 => {
                let result: DataResponse<MetaZoneSpecificNamesShortV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key)),
        }
    }
}

macro_rules! impl_data_provider {
    ($id: ident: $lt: lifetime) => {
        impl<$lt, 'dp: $lt> DataProvider<'dp, $id<'dp>> for TimeZonesProvider<$lt> {
            fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'dp, $id<'dp>>, DataError> {
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
                    payload: DataPayload {
                        cow: Some(Cow::Owned($id::from(time_zones.clone()))),
                    },
                })
            }
        }
    };
}

impl<'d, 's: 'd> DataProvider<'d, dyn SerdeSeDataStruct<'s> + 's> for TimeZonesProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, dyn SerdeSeDataStruct<'s> + 's>, DataError> {
        match req.resource_path.key {
            key::TIMEZONE_FORMATS_V1 => {
                let result: DataResponse<TimeZoneFormatsV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_EXEMPLAR_CITIES_V1 => {
                let result: DataResponse<ExemplarCitiesV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_GENERIC_NAMES_LONG_V1 => {
                let result: DataResponse<MetaZoneGenericNamesLongV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_GENERIC_NAMES_SHORT_V1 => {
                let result: DataResponse<MetaZoneGenericNamesShortV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_SPECIFIC_NAMES_LONG_V1 => {
                let result: DataResponse<MetaZoneSpecificNamesLongV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1 => {
                let result: DataResponse<MetaZoneSpecificNamesShortV1> = self.load_payload(req)?;
                Ok(DataResponse {
                    metadata: result.metadata,
                    payload: result.payload.into(),
                })
            }
            _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key)),
        }
    }
}

impl_data_provider!(TimeZoneFormatsV1: 'd);
impl_data_provider!(ExemplarCitiesV1: 'd);
impl_data_provider!(MetaZoneGenericNamesLongV1: 'd);
impl_data_provider!(MetaZoneGenericNamesShortV1: 'd);
impl_data_provider!(MetaZoneSpecificNamesLongV1: 'd);
impl_data_provider!(MetaZoneSpecificNamesShortV1: 'd);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_cldr_time_zones() {
        use icu_locid_macros::langid;

        let cldr_paths = crate::cldr_paths::for_test();
        let provider = TimeZonesProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

        let time_zone_formats: Cow<TimeZoneFormatsV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!("GMT", time_zone_formats.gmt_zero_format);

        let exemplar_cities: Cow<ExemplarCitiesV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!("Pohnpei", exemplar_cities["Pacific/Ponape"]);

        let generic_names_long: Cow<MetaZoneGenericNamesLongV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!(
            "Australian Central Western Time",
            generic_names_long["Australia_CentralWestern"]
        );

        let specific_names_long: Cow<MetaZoneSpecificNamesLongV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!(
            "Australian Central Western Standard Time",
            specific_names_long["Australia_CentralWestern"]["standard"]
        );

        let generic_names_short: Cow<MetaZoneGenericNamesShortV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!("PT", generic_names_short["America_Pacific"]);

        let specific_names_short: Cow<MetaZoneSpecificNamesShortV1> = provider
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
            .payload
            .take()
            .unwrap();
        assert_eq!("PDT", specific_names_short["America_Pacific"]["daylight"]);
    }
}
