// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod essentials;
pub(crate) mod language;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

/// Macro for implementing a single-name display names data provider.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$subtag_ty`: The subtag type (e.g., `Language`, `Script`).
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$alt_variant`: The alt variant string (e.g., `None`, `Some("short")`).
macro_rules! impl_displaynames_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $alt_variant:expr,) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data: &$resource = self
                    .cldr()?
                    .displaynames()
                    .read_and_parse(req.id.locale, $file)?;

                let subtag =
                    <$subtag_ty as core::str::FromStr>::from_str(req.id.marker_attributes.as_str())
                        .map_err(|_| {
                            DataError::custom("failed to parse subtag").with_req($marker::INFO, req)
                        })?;

                let key = ModifiedSubtag {
                    subtag,
                    alt_variant: $alt_variant.map(String::from),
                    menu_variant: None,
                };

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
            $marker,
            $subtag_ty,
            $resource,
            $file,
            $field,
            $alt_variant
        );
    };
}

/// Macro for implementing a menu display names data provider.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$subtag_ty`: The subtag type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
macro_rules! impl_displaynames_menu_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident,) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let data: &$resource = self
                    .cldr()?
                    .displaynames()
                    .read_and_parse(req.id.locale, $file)?;

                let subtag =
                    <$subtag_ty as core::str::FromStr>::from_str(req.id.marker_attributes.as_str())
                        .map_err(|_| {
                            DataError::custom("failed to parse subtag").with_req($marker::INFO, req)
                        })?;

                let key_core = ModifiedSubtag {
                    subtag: subtag.clone(),
                    alt_variant: None,
                    menu_variant: Some("core".to_string()),
                };
                let key_extension = ModifiedSubtag {
                    subtag: subtag.clone(),
                    alt_variant: None,
                    menu_variant: Some("extension".to_string()),
                };

                let map = &data.main.value.localedisplaynames.$field;

                let (name_core, name_extension) = if let Some(core) = map.get(&key_core) {
                    let extension = map.get(&key_extension).ok_or_else(|| {
                        DataError::custom("found menu-core but missing menu-extension")
                            .with_req($marker::INFO, req)
                    })?;
                    (core.as_str(), extension.as_str())
                } else {
                    // Fallback to alt-menu
                    let key_alt_menu = ModifiedSubtag {
                        subtag,
                        alt_variant: Some("menu".to_string()),
                        menu_variant: None,
                    };
                    let alt_menu = map.get(&key_alt_menu).ok_or_else(|| {
                        DataError::custom("failed to find menu-core or alt-menu")
                            .with_req($marker::INFO, req)
                    })?;
                    (alt_menu.as_str(), "")
                };

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(VarZeroCow::from_encodeable(&MenuNameParts {
                        core: VarZeroCow::from_encodeable(&name_core),
                        extension: VarZeroCow::from_encodeable(&name_extension),
                    })),
                })
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut result = HashSet::new();
                let displaynames = self.cldr()?.displaynames();
                for locale in displaynames.list_locales()?.filter(|locale| {
                    self.cldr()
                        .unwrap()
                        .displaynames()
                        .file_exists(locale, $file)
                        .unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key in data.main.value.localedisplaynames.$field.keys() {
                        let matches = key.menu_variant.as_deref() == Some("core")
                            || key.alt_variant.as_deref() == Some("menu");

                        if matches {
                            let attr_str = key.subtag.to_string();
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(attr_str.clone()).map_err(
                                    |_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&attr_str)
                                    },
                                )?,
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

/// Macro for implementing the iterable data provider for display names.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$subtag_ty`: The subtag type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$alt_variant`: The alt variant string (e.g., `None`, `Some("short")`).
macro_rules! impl_displaynames_iter_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $alt_variant:expr) => {
        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut result = HashSet::new();
                let displaynames = self.cldr()?.displaynames();
                for locale in displaynames.list_locales()?.filter(|locale| {
                    self.cldr()
                        .unwrap()
                        .displaynames()
                        .file_exists(locale, $file)
                        .unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key in data.main.value.localedisplaynames.$field.keys() {
                        let matches = match ($alt_variant, &key.alt_variant) {
                            (Some(expected), Some(actual)) => expected == actual,
                            (None, None) => true,
                            _ => false,
                        } && key.menu_variant.is_none();

                        if matches {
                            let attr_str = key.subtag.to_string();
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(attr_str.clone()).map_err(
                                    |_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&attr_str)
                                    },
                                )?,
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
