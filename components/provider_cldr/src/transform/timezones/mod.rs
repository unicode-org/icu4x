// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_datetime::provider::{
    key,
    timezones::{TimeZoneFormatsV1, TimeZoneNamesLongV1, TimeZoneNamesShortV1, TimeZoneNameVariantsLongV1, TimeZoneNameVariantsShortV1},
};
use icu_provider::{erased::{ErasedDataProvider, ErasedDataReceiver}, prelude::*};
use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;

mod cldr_json;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 5] = [
    key::TIMEZONE_FORMATS_V1,
    key::TIMEZONE_NAMES_LONG_V1,
    key::TIMEZONE_NAMES_SHORT_V1,
    key::TIMEZONE_NAME_VARIANTS_LONG_V1,
    key::TIMEZONE_NAME_VARIANTS_SHORT_V1,
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

impl<'d> IterableDataProvider<'d> for TimeZonesProvider<'d> {
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
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn ErasedDataReceiver<'d, '_>,
    ) -> Result<DataResponseMetadata, DataError> {
        match req.resource_path.key {
            key::TIMEZONE_FORMATS_V1 => {
                let mut result: DataResponse<TimeZoneFormatsV1> = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
            key::TIMEZONE_NAMES_LONG_V1 => {
                let mut result: DataResponse<TimeZoneNamesLongV1> = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
            key::TIMEZONE_NAMES_SHORT_V1 => {
                let mut result: DataResponse<TimeZoneNamesShortV1> = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
            key::TIMEZONE_NAME_VARIANTS_LONG_V1 => {
                let mut result: DataResponse<TimeZoneNameVariantsLongV1> = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
            key::TIMEZONE_NAME_VARIANTS_SHORT_V1 => {
                let mut result: DataResponse<TimeZoneNameVariantsShortV1> = self.load_payload(req)?;
                receiver.receive_payload(result.take_payload()?)?;
                Ok(result.metadata)
            }
            _ => Err(DataError::UnsupportedResourceKey(req.resource_path.key)),
        }
    }
}

macro_rules! impl_data_provider {
    ($id: ident) => {
        impl<'d> DataProvider<'d, $id> for TimeZonesProvider<'d> {
            fn load_payload(
                &self,
                req: &DataRequest,
            ) -> Result<DataResponse<'d, $id>, DataError> {
                TimeZonesProvider::supports_key(&req.resource_path.key)?;
                let cldr_langid: CldrLangID = req.try_langid()?.clone().into();
                let zones = match self
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
                    payload: Some(Cow::Owned($id::from(zones.clone()))),
                })
            }
        }
    };
}

impl_data_provider!(TimeZoneFormatsV1);
impl_data_provider!(TimeZoneNamesLongV1);
impl_data_provider!(TimeZoneNamesShortV1);
impl_data_provider!(TimeZoneNameVariantsLongV1);
impl_data_provider!(TimeZoneNameVariantsShortV1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cldr_timezones() {
        use icu_locid_macros::langid;

        let json_str = std::fs::read_to_string(
            "/Users/enordin/src/cldr-json/cldr-json/cldr-dates-full/main/en/timeZoneNames.json",
        )
        .unwrap();
        let provider = TimeZonesProvider::try_from(json_str.as_str()).unwrap();

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
            .take_payload()
            .unwrap();
        dbg!(&time_zone_formats);

        let time_zone_names_long: Cow<TimeZoneNamesLongV1> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_NAMES_LONG_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        dbg!(&time_zone_names_long);

        let time_zone_name_variants_long: Cow<TimeZoneNameVariantsLongV1> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_NAME_VARIANTS_LONG_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        dbg!(&time_zone_name_variants_long);

        let time_zone_name_variants_short: Cow<TimeZoneNameVariantsShortV1> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::TIMEZONE_NAME_VARIANTS_SHORT_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(langid!("en")),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        dbg!(&time_zone_name_variants_short);
    }
}
