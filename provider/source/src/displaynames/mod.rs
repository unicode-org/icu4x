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
pub(crate) const ALT_VARIANT_SUBSTRING: &str = "-alt-variant";
pub(crate) const ALT_STANDALONE_SUBSTRING: &str = "-alt-stand-alone";
pub(crate) const ALT_OFFICIAL_SUBSTRING: &str = "-alt-official";
/// Secondary name variant, used in languages and scripts.
pub(crate) const ALT_SECONDARY_SUBSTRING: &str = "-alt-secondary";
/// Abbreviation for territory code `IO` (British Indian Ocean Territory).
pub(crate) const ALT_BIOT_SUBSTRING: &str = "-alt-biot";
/// Alternate name for territory code `IO` (British Indian Ocean Territory) mapping to "Chagos Archipelago".
pub(crate) const ALT_CHAGOS_SUBSTRING: &str = "-alt-chagos";
pub(crate) const MENU_SUBSTRING: &str = "-menu-";
pub(crate) const MENU_CORE_SUBSTRING: &str = "-menu-core";
pub(crate) const MENU_EXTENSION_SUBSTRING: &str = "-menu-extension";

// TODO: ALT_MENU_SUBSTRING should be dead. Remove when possible.
pub(crate) const ALT_MENU_SUBSTRING: &str = "-alt-menu";

/// Macro for implementing a single-name display names data provider.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$suffix`: An optional string to append to the marker attribute to form the CLDR key.
macro_rules! impl_displaynames_v1 {
    ($marker:ident, $resource:path, $file:literal, $field:ident, $suffix:expr,) => {
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
                        DataError::custom("failed to find attribute").with_req($marker::INFO, req)
                    })?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(VarZeroCow::from_encodeable(name)),
                })
            }
        }

        $crate::displaynames::impl_displaynames_iter_v1!(
            $marker, $resource, $file, $field, $suffix
        );
    };
}

/// Macro for implementing a menu display names data provider.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
macro_rules! impl_displaynames_menu_v1 {
    ($marker:ident, $resource:path, $file:literal, $field:ident,) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data: &$resource = self
                    .cldr()?
                    .displaynames()
                    .read_and_parse(req.id.locale, $file)?;

                let mut key_core = req.id.marker_attributes.as_str().to_string();
                let mut key_extension = key_core.clone();
                key_core.push_str(MENU_CORE_SUBSTRING);
                key_extension.push_str(MENU_EXTENSION_SUBSTRING);

                let name_core = data
                    .main
                    .value
                    .localedisplaynames
                    .$field
                    .get(&key_core)
                    .ok_or_else(|| {
                        DataError::custom("failed to find attribute").with_req($marker::INFO, req)
                    })?;

                let name_extension = data
                    .main
                    .value
                    .localedisplaynames
                    .$field
                    .get(&key_extension)
                    .ok_or_else(|| {
                        DataError::custom("failed to find attribute").with_req($marker::INFO, req)
                    })?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(VarZeroCow::from_encodeable(&MenuNameParts {
                        core: VarZeroCow::from_encodeable(&name_core),
                        extension: VarZeroCow::from_encodeable(&name_extension),
                    })),
                })
            }
        }

        $crate::displaynames::impl_displaynames_iter_v1!(
            $marker,
            $resource,
            $file,
            $field,
            Some(MENU_CORE_SUBSTRING)
        );
    };
}

/// Macro for implementing the iterable data provider for display names.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$suffix`: An optional string that marks which entries to include in this provider.
macro_rules! impl_displaynames_iter_v1 {
    ($marker:ident, $resource:path, $file:literal, $field:ident, $suffix:expr) => {
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
                        } else if key_str.contains(crate::displaynames::ALT_SUBSTRING)
                            || key_str.contains(crate::displaynames::MENU_SUBSTRING)
                        {
                            None
                        } else {
                            Some(key_str.as_str())
                        };

                        if let Some(attr_str) = attr {
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(attr_str.to_string())
                                    .map_err(|_| {
                                        DataError::custom("Failed to parse attribute")
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

/// Macro for implementing the iterable data provider for legacy display name markers.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$file`: The JSON file name in CLDR.
macro_rules! impl_displaynames_legacy_iter_v1 {
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

pub(crate) use impl_displaynames_iter_v1;
pub(crate) use impl_displaynames_legacy_iter_v1;
pub(crate) use impl_displaynames_menu_v1;
pub(crate) use impl_displaynames_v1;
