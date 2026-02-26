// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod language;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

pub(crate) const ALT_SUBSTRING: &str = "-alt-";
pub(crate) const ALT_SHORT_SUBSTRING: &str = "-alt-short";
pub(crate) const ALT_LONG_SUBSTRING: &str = "-alt-long";
pub(crate) const ALT_MENU_SUBSTRING: &str = "-alt-menu";

macro_rules! impl_displaynames_v1 {
    ($marker:ident, $resource:path, $file:literal, $field:ident, $suffix:expr, $label:literal) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data: &$resource = self
                    .cldr()?
                    .displaynames()
                    .read_and_parse(req.id.locale, $file)?;

                let mut key = req.id.marker_attributes.as_str().to_string();
                if let Some(suffix) = $suffix {
                    key.push_str(suffix);
                }

                let name = data
                    .main
                    .value
                    .localedisplaynames
                    .$field
                    .get(&key)
                    .ok_or_else(|| {
                        DataError::custom(concat!("data for ", $label)).with_req($marker::INFO, req)
                    })?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(VarZeroCow::from_encodeable(name)),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut result = HashSet::new();
                let displaynames = self.cldr()?.displaynames();
                for locale in displaynames.list_locales()?.filter(|locale| {
                    // The directory might exist without the file
                    self.cldr()
                        .unwrap()
                        .displaynames()
                        .file_exists(locale, $file)
                        .unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key_str in data.main.value.localedisplaynames.$field.keys() {
                        let attr = if let Some(suffix) = $suffix {
                            key_str.strip_suffix(suffix)
                        } else {
                            if key_str.contains(crate::displaynames::ALT_SUBSTRING) {
                                None
                            } else {
                                Some(key_str.as_str())
                            }
                        };

                        if let Some(attr_str) = attr {
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(attr_str.to_string())
                                    .map_err(|_| {
                                        DataError::custom(concat!(
                                            "Failed to parse ",
                                            $label,
                                            " as attribute"
                                        ))
                                        .with_debug_context(&attr_str)
                                    })?,
                                locale,
                            );
                            result.insert(data_identifier);
                        }
                    }
                }
                Ok(result)
            }
        }
    };
}

macro_rules! impl_displaynames_main_iter_v1 {
    ($marker:ident, $file:literal) => {
        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(self
                    .cldr()?
                    .displaynames()
                    .list_locales()?
                    .filter(|locale| {
                        // The directory might exist without the file
                        self.cldr()
                            .unwrap()
                            .displaynames()
                            .file_exists(locale, $file)
                            .unwrap_or_default()
                    })
                    .map(DataIdentifierCow::from_locale)
                    .collect())
            }
        }
    };
}

pub(crate) use impl_displaynames_main_iter_v1;
pub(crate) use impl_displaynames_v1;
