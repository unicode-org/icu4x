// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod essentials;
pub(crate) mod language;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

use crate::cldr_serde::displaynames::{Alt, WithAlt};
use std::collections::{BTreeMap, HashMap};

pub(crate) struct ExtractedNames<'a, K> {
    pub(crate) names: BTreeMap<K, &'a str>,
    pub(crate) short_names: BTreeMap<K, &'a str>,
    pub(crate) long_names: BTreeMap<K, &'a str>,
    pub(crate) menu_names: BTreeMap<K, &'a str>,
}

/// Extracts locale display names from a `cldr_serde` struct into `BTreeMap`s.
///
/// This helper is used by the legacy (ZeroMap-based) providers, rather than the newer
/// attributes-based providers.
pub(crate) fn extract_names_for_zeromap_struct<'a, T, K, F>(
    map: &'a HashMap<WithAlt<T>, String>,
    ignored_alts: &[Alt],
    log_context: &str,
    filter_project: F,
) -> ExtractedNames<'a, K>
where
    K: Ord + PartialEq<str>,
    F: Fn(&T) -> Option<K>,
{
    let mut names = BTreeMap::new();
    let mut short_names = BTreeMap::new();
    let mut long_names = BTreeMap::new();
    let mut menu_names = BTreeMap::new();
    for (key, value) in map.iter() {
        if key.menu.is_some() {
            // Menu core|extension is handled in LocaleNamesLanguageMenu,
            // and not in the zeromap-based struct.
            continue;
        }
        let val_str = value.as_str();
        if let Some(k) = filter_project(&key.subtag) {
            // Old CLDR versions may contain trivial entries, so filter
            if k == *val_str {
                continue;
            }
            match key.alt {
                Some(Alt::Short) => {
                    short_names.insert(k, val_str);
                }
                Some(Alt::Long) => {
                    long_names.insert(k, val_str);
                }
                Some(Alt::Menu) => {
                    menu_names.insert(k, val_str);
                }
                None => {
                    names.insert(k, val_str);
                }
                Some(alt) => {
                    if alt == Alt::Unknown {
                        // Discard unknown alts
                    } else if ignored_alts.contains(&alt) {
                        // TODO(#8012): Handle preference-specific alt variants,
                        //   perhaps with datagen alt flags.
                        // TODO(#8011): Support standalone display names.
                    } else {
                        log::warn!("Unhandled alt variant for {}: {:?}", log_context, alt);
                    }
                }
            }
        }
    }
    ExtractedNames {
        names,
        short_names,
        long_names,
        menu_names,
    }
}

/// Macro for implementing a single-name display names data provider.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$subtag_ty`: The subtag type (e.g., `Language`, `Script`).
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$alt_variant`: The alt variant (e.g., `None`, `Some(Alt::Short)`).
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

                let key = WithAlt {
                    subtag,
                    alt: $alt_variant,
                    menu: None,
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

                let key_core = WithAlt {
                    subtag: subtag.clone(),
                    alt: None,
                    menu: Some($crate::cldr_serde::displaynames::Menu::Core),
                };

                let map = &data.main.value.localedisplaynames.$field;

                let (name_core, name_extension) = if let Some(core) = map.get(&key_core) {
                    let key_extension = WithAlt {
                        subtag,
                        alt: None,
                        menu: Some($crate::cldr_serde::displaynames::Menu::Extension),
                    };
                    let extension = map.get(&key_extension).ok_or_else(|| {
                        DataError::custom("found menu-core but missing menu-extension")
                            .with_req($marker::INFO, req)
                    })?;
                    (core.as_str(), extension.as_str())
                } else {
                    // Fallback to alt-menu
                    let key_alt_menu = WithAlt {
                        subtag,
                        alt: Some($crate::cldr_serde::displaynames::Alt::Menu),
                        menu: None,
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
                    // The directory might exist without the file
                    displaynames.file_exists(locale, $file).unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key in data.main.value.localedisplaynames.$field.keys() {
                        let matches = key.menu
                            == Some($crate::cldr_serde::displaynames::Menu::Core)
                            || key.alt == Some($crate::cldr_serde::displaynames::Alt::Menu);

                        if matches {
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(key.subtag.to_string())
                                    .map_err(|_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&key.subtag.to_string())
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

/// Macro for implementing the iterable data provider for display names.
///
/// Parameters:
/// - `$marker`: The data marker type.
/// - `$subtag_ty`: The subtag type.
/// - `$resource`: The CLDR serde resource type.
/// - `$file`: The JSON file name in CLDR.
/// - `$field`: The field name in `LocaleDisplayNames` containing the data.
/// - `$alt_variant`: The alt variant (e.g., `None`, `Some(Alt::Short)`).
macro_rules! impl_displaynames_iter_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $alt_variant:expr) => {
        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut result = HashSet::new();
                let displaynames = self.cldr()?.displaynames();
                for locale in displaynames.list_locales()?.filter(|locale| {
                    // The directory might exist without the file
                    displaynames.file_exists(locale, $file).unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key in data.main.value.localedisplaynames.$field.keys() {
                        let matches = $alt_variant == key.alt && key.menu.is_none();

                        if matches {
                            let data_identifier = DataIdentifierCow::from_owned(
                                DataMarkerAttributes::try_from_string(key.subtag.to_string())
                                    .map_err(|_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&key.subtag.to_string())
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
                let displaynames = self.cldr()?.displaynames();
                Ok(displaynames
                    .list_locales()?
                    .filter(|locale| {
                        // The directory might exist without the file
                        displaynames.file_exists(locale, $file).unwrap_or_default()
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
