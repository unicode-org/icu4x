// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_serde::transforms;
use super::source::CldrCache;
use crate::provider::DatagenProvider;
use icu_experimental::transliterate::provider::*;
use icu_experimental::transliterate::RuleCollection;
use icu_locale_core::Locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::sync::Mutex;

impl CldrCache {
    fn transforms(&self) -> Result<&Mutex<RuleCollection>, DataError> {
        self.transforms.get_or_init(|| {
            fn find_bcp47(aliases: &[transforms::TransformAlias]) -> Option<&Locale> {
                aliases
                    .iter()
                    .find_map(|alias| {
                        if let transforms::TransformAlias::Bcp47(locale) = alias {
                            Some(locale)
                        } else {
                            None
                        }
                    })
            }

            let mut provider = RuleCollection::default();

            let transforms = &format!("cldr-transforms-{}/main", self.dir_suffix()?);
            for transform in self.serde_cache.list(transforms)? {
                let metadata = self
                    .serde_cache
                    .read_and_parse_json::<transforms::Resource>(&format!(
                        "{transforms}/{transform}/metadata.json"
                    ))?;
                let source = self
                    .serde_cache
                    .root
                    .read_to_string(&format!("{transforms}/{transform}/source.txt",))?;

                if matches!(
                    metadata.direction,
                    transforms::Direction::Forward | transforms::Direction::Both
                ) {
                    if let Some(bcp47) = find_bcp47(&metadata.alias) {
                        provider.register_source(
                            bcp47,
                            source.clone(),
                            metadata
                                .alias
                                .iter()
                                .filter_map(|alias| match alias {
                                    transforms::TransformAlias::LegacyId(s) => Some(s.as_str()),
                                    _ => None,
                                })
                                .chain([
                                    // source, target, and variant may also be used
                                    if let Some(variant) = &metadata.variant {
                                        format!("{}-{}/{}", metadata.source, metadata.target, variant)
                                    } else {
                                        format!("{}-{}", metadata.source, metadata.target)
                                    }
                                    .to_ascii_lowercase()
                                    .as_str(),
                                ]),
                            false,
                            metadata.visibility == transforms::Visibility::External,
                        );
                    } else {
                        log::warn!("Skipping transliterator {transform} (forward) as it does not have a BCP-47 identifier.")
                    }
                }

                if matches!(
                    metadata.direction,
                    transforms::Direction::Backward | transforms::Direction::Both
                ) {
                    if let Some(bcp47) = find_bcp47(&metadata.backward_alias) {
                        provider.register_source(
                            bcp47,
                            source,
                            metadata
                                .backward_alias
                                .iter()
                                .filter_map(|alias| match alias {
                                    transforms::TransformAlias::LegacyId(s) => Some(s.as_str()),
                                    _ => None,
                                })
                                .chain([
                                    // source, target, and variant may also be used
                                    if let Some(variant) = &metadata.variant {
                                        format!("{}-{}/{}", metadata.target, metadata.source, variant)
                                    } else {
                                        format!("{}-{}", metadata.target, metadata.source)
                                    }
                                    .to_ascii_lowercase()
                                    .as_str(),
                                ]),
                            true,
                            metadata.visibility == transforms::Visibility::External,
                        );
                    } else {
                        log::warn!("Skipping transliterator {transform} (backward) as it does not have a BCP-47 identifier.")
                    }
                }
            }
            Ok(Mutex::new(provider))
        })
        .as_ref().map_err(|&e| e)
    }
}

impl DataProvider<TransliteratorRulesV1Marker> for DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
        self.check_req::<TransliteratorRulesV1Marker>(req)?;
        self.cldr()?
            .transforms()?
            .lock()
            .expect("poison")
            .as_provider_unstable(self, self)?
            .load(req)
    }
}

impl IterableDataProvider<TransliteratorRulesV1Marker> for DatagenProvider {
    // Don't do caching for this one. It uses its own mutex
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        self.cldr()?
            .transforms()?
            .lock()
            .expect("poison")
            .as_provider_unstable(self, self)?
            .supported_requests()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_ascii_forward() {
        let provider = DatagenProvider::new_testing();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                key_attributes: &"de-t-de-d0-ascii".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
    }

    #[test]
    fn test_latin_ascii_backward() {
        let provider = DatagenProvider::new_testing();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                key_attributes: &"und-latn-t-s0-ascii".parse().unwrap(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
    }
}
