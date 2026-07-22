// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod essentials;
pub(crate) mod language;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

#[cfg(test)]
use crate::cldr_cache::CoverageLevelForXPath;
use crate::cldr_serde::displaynames::{Alt, Menu, WithAlt};
use either::Either;
use std::collections::{BTreeMap, HashMap};
use writeable::Writeable;

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
            // Menu core|extension is handled in LocaleNamesLanguageMenu, and not in the zeromap-based struct.
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

/// Helper to construct CLDR `XPath` string for a display name attribute and subtag.
pub(crate) fn construct_xpath<'a>(
    field: &'a str,
    subtag_str: impl Writeable + 'a,
    alt: Option<Alt>,
    menu: Option<Menu>,
) -> impl Writeable + 'a {
    let alt_str = match (alt, menu) {
        (None, None) => "",
        (None, Some(Menu::Core)) => r#"[@menu="core"]"#,
        (None, Some(Menu::Extension)) => r#"[@menu="extension"]"#,
        (None, Some(Menu::Unknown)) => "",
        (Some(Alt::Short), None) => r#"[@alt="short"]"#,
        (Some(Alt::Long), None) => r#"[@alt="long"]"#,
        (Some(Alt::Variant), None) => r#"[@alt="variant"]"#,
        (Some(Alt::StandAlone), None) => r#"[@alt="stand-alone"]"#,
        (Some(Alt::Official), None) => r#"[@alt="official"]"#,
        (Some(Alt::Secondary), None) => r#"[@alt="secondary"]"#,
        (Some(Alt::Biot), None) => r#"[@alt="biot"]"#,
        (Some(Alt::Chagos), None) => r#"[@alt="chagos"]"#,
        (Some(Alt::Menu), None) => r#"[@alt="menu"]"#,
        (Some(Alt::Unknown), None) => "",
        (Some(_), Some(_)) => {
            debug_assert!(false, "unexpected alt and menu together: {alt:?} {menu:?}");
            ""
        }
    };

    match field {
        "languages" => Either::Left(writeable::concat_writeable!(
            r#"//ldml/localeDisplayNames/languages/language[@type=""#,
            writeable::adapters::Replace {
                source: subtag_str,
                needle: "-",
                replacement: '_'
            },
            r#""]"#,
            alt_str
        )),
        "regions" | "territories" => either::Right(writeable::concat_writeable!(
            r#"//ldml/localeDisplayNames/territories/territory[@type=""#,
            subtag_str,
            r#""]"#,
            alt_str
        )),
        "scripts" => either::Right(writeable::concat_writeable!(
            r#"//ldml/localeDisplayNames/scripts/script[@type=""#,
            subtag_str,
            r#""]"#,
            alt_str
        )),
        "variants" => either::Right(writeable::concat_writeable!(
            r#"//ldml/localeDisplayNames/variants/variant[@type=""#,
            subtag_str,
            r#""]"#,
            alt_str
        )),
        _ => panic!("Unknown field: {}", field),
    }
}

#[cfg(test)]
trait CheckAltCoverage {
    fn contains_key<T>(key: &WithAlt<T>, tier: CoverageLevelForXPath) -> bool;
}

/// Test helper that iterates over all display name entries across all locales in CLDR in deterministic order.
///
/// For each entry found in `file_name` (e.g., `"languages.json"`), this function:
/// 1. Extracts the map of subtag keys (`WithAlt<T>`) via `extract_keys`.
/// 2. Constructs the corresponding CLDR `XPath` for `xpath_field` (e.g., `"languages"`).
/// 3. Looks up the coverage tier (`CoverageLevelForXPath`) for that `XPath` in the given locale.
/// 4. Invokes `callback(locale, key, tier)`.
#[cfg(test)]
pub(crate) fn for_each_cldr_key_and_tier<Resource, T>(
    cldr: &crate::cldr_cache::CldrCache,
    file_name: &str,
    xpath_field: &str,
    mut extract_keys: impl FnMut(&Resource) -> &HashMap<WithAlt<T>, String>,
    mut callback: impl FnMut(&icu_provider::DataLocale, &WithAlt<T>, CoverageLevelForXPath),
) where
    Resource: serde::de::DeserializeOwned + Send + Sync + 'static,
    T: Writeable,
{
    let coverage_cldr = crate::cldr_cache::coverage_cldr_cache();
    let displaynames_dir = cldr.displaynames();
    let mut locales = displaynames_dir.list_locales().unwrap().collect::<Vec<_>>();
    locales.sort_by(|a, b| a.total_cmp(b));
    for locale in locales {
        if let Ok(res) = displaynames_dir.read_and_parse::<Resource>(&locale, file_name) {
            let mut keys = extract_keys(res).keys().collect::<Vec<_>>();
            keys.sort_by_cached_key(|k| (k.subtag.write_to_string().to_string(), k.alt, k.menu));
            for key in keys {
                if let Some(Alt::Variant) = key.alt {
                    // TODO(#8012): Handle preference-specific alt variants, perhaps with datagen alt flags.
                    return;
                }
                let xpath = construct_xpath(xpath_field, &key.subtag, key.alt, key.menu);
                let tier = coverage_cldr.coverage_tier(&locale, &xpath).unwrap();
                callback(&locale, key, tier);
            }
        }
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
/// - `$tier`: The target coverage tier.
macro_rules! impl_displaynames_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $alt_variant:expr, $tier:pat,) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let cldr = self.cldr()?;
                let data: &$resource = cldr.displaynames().read_and_parse(req.id.locale, $file)?;

                let subtag =
                    <$subtag_ty as core::str::FromStr>::from_str(req.id.marker_attributes.as_str())
                        .map_err(|_| {
                            DataError::custom("failed to parse subtag").with_req($marker::INFO, req)
                        })?;

                let key = WithAlt {
                    subtag: subtag.clone(),
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
                        DataErrorKind::IdentifierNotFound
                            .into_error()
                            .with_req($marker::INFO, req)
                    })?;

                let field_str = stringify!($field);
                let xpath =
                    $crate::displaynames::construct_xpath(field_str, &subtag, $alt_variant, None);
                let item_tier = crate::cldr_cache::coverage_cldr_cache()
                    .coverage_tier(req.id.locale, &xpath)?;
                if !matches!(item_tier, $tier) {
                    return Err(DataErrorKind::IdentifierNotFound
                        .into_error()
                        .with_req($marker::INFO, req));
                }

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
            $alt_variant,
            $tier
        );

        #[cfg(test)]
        impl $crate::displaynames::CheckAltCoverage for $marker {
            fn contains_key<T>(
                key: &$crate::cldr_serde::displaynames::WithAlt<T>,
                tier: $crate::cldr_cache::CoverageLevelForXPath,
            ) -> bool {
                key.alt == $alt_variant && key.menu.is_none() && matches!(tier, $tier)
            }
        }
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
/// - `$tier`: The target coverage tier.
macro_rules! impl_displaynames_menu_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $tier:pat,) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;

                let cldr = self.cldr()?;
                let data: &$resource = cldr.displaynames().read_and_parse(req.id.locale, $file)?;

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

                let mut used_alt_menu = false;
                let (name_core, name_extension) = if let Some(core) = map.get(&key_core) {
                    let key_extension = WithAlt {
                        subtag: subtag.clone(),
                        alt: None,
                        menu: Some($crate::cldr_serde::displaynames::Menu::Extension),
                    };
                    let extension = map.get(&key_extension).ok_or_else(|| {
                        DataError::custom("found menu-core but missing menu-extension")
                            .with_req($marker::INFO, req)
                    })?;
                    (core.as_str(), extension.as_str())
                } else {
                    used_alt_menu = true;
                    // Fallback to alt-menu
                    let key_alt_menu = WithAlt {
                        subtag: subtag.clone(),
                        alt: Some($crate::cldr_serde::displaynames::Alt::Menu),
                        menu: None,
                    };
                    let alt_menu = map.get(&key_alt_menu).ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound
                            .into_error()
                            .with_req($marker::INFO, req)
                    })?;
                    (alt_menu.as_str(), "")
                };

                let field_str = stringify!($field);
                let xpath = if used_alt_menu {
                    $crate::displaynames::construct_xpath(
                        field_str,
                        &subtag,
                        Some($crate::cldr_serde::displaynames::Alt::Menu),
                        None,
                    )
                } else {
                    $crate::displaynames::construct_xpath(
                        field_str,
                        &subtag,
                        None,
                        Some($crate::cldr_serde::displaynames::Menu::Core),
                    )
                };
                let item_tier = crate::cldr_cache::coverage_cldr_cache()
                    .coverage_tier(req.id.locale, &xpath)?;
                if !matches!(item_tier, $tier) {
                    return Err(DataErrorKind::IdentifierNotFound
                        .into_error()
                        .with_req($marker::INFO, req));
                }

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
                let cldr = self.cldr()?;
                let displaynames = cldr.displaynames();
                let field_str = stringify!($field);
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
                            let xpath =
                                if key.alt == Some($crate::cldr_serde::displaynames::Alt::Menu) {
                                    $crate::displaynames::construct_xpath(
                                        field_str,
                                        &key.subtag,
                                        Some($crate::cldr_serde::displaynames::Alt::Menu),
                                        None,
                                    )
                                } else {
                                    $crate::displaynames::construct_xpath(
                                        field_str,
                                        &key.subtag,
                                        None,
                                        Some($crate::cldr_serde::displaynames::Menu::Core),
                                    )
                                };
                            if matches!(
                                crate::cldr_cache::coverage_cldr_cache()
                                    .coverage_tier(&locale, &xpath)?,
                                $tier
                            ) {
                                let data_identifier = DataIdentifierCow::from_owned(
                                    DataMarkerAttributes::try_from_string(key.subtag.to_string())
                                        .map_err(|_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&key.subtag)
                                    })?,
                                    locale,
                                );
                                result.insert(data_identifier);
                            }
                        }
                    }
                }
                Ok(result)
            }
        }

        #[cfg(test)]
        impl $crate::displaynames::CheckAltCoverage for $marker {
            fn contains_key<T>(
                key: &$crate::cldr_serde::displaynames::WithAlt<T>,
                tier: $crate::cldr_cache::CoverageLevelForXPath,
            ) -> bool {
                ((key.alt.is_none() && key.menu.is_some())
                    || key.alt == Some($crate::cldr_serde::displaynames::Alt::Menu))
                    && matches!(tier, $tier)
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
/// - `$tier`: The target coverage tier.
macro_rules! impl_displaynames_iter_v1 {
    ($marker:ident, $subtag_ty:ty, $resource:path, $file:literal, $field:ident, $alt_variant:expr, $tier:pat) => {
        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                let mut result = HashSet::new();
                let cldr = self.cldr()?;
                let displaynames = cldr.displaynames();
                let field_str = stringify!($field);
                for locale in displaynames.list_locales()?.filter(|locale| {
                    // The directory might exist without the file
                    displaynames.file_exists(locale, $file).unwrap_or_default()
                }) {
                    let data: &$resource = displaynames.read_and_parse(&locale, $file)?;
                    for key in data.main.value.localedisplaynames.$field.keys() {
                        let matches = $alt_variant == key.alt && key.menu.is_none();

                        if matches {
                            let xpath = $crate::displaynames::construct_xpath(
                                field_str,
                                &key.subtag,
                                $alt_variant,
                                None,
                            );
                            if matches!(
                                crate::cldr_cache::coverage_cldr_cache()
                                    .coverage_tier(&locale, &xpath)?,
                                $tier
                            ) {
                                let data_identifier = DataIdentifierCow::from_owned(
                                    DataMarkerAttributes::try_from_string(key.subtag.to_string())
                                        .map_err(|_| {
                                        DataError::custom("Failed to parse attribute")
                                            .with_debug_context(&key.subtag)
                                    })?,
                                    locale,
                                );
                                result.insert(data_identifier);
                            }
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
