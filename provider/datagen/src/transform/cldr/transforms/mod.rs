// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde::transforms;
use crate::transform::cldr::source::CldrDirTransform;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_transliteration::provider::*;
use std::collections::HashMap;

use super::cldr_serde::transforms::TransformAlias;

// TODO(#3736): This could benefit from avoiding recomputation across `load` calls.
//  Maybe a OnceCell?
struct TransliteratorCollection<'a> {
    cldr_transforms: CldrDirTransform<'a>,
}

impl<'a> TransliteratorCollection<'a> {
    fn new(cldr_transforms: CldrDirTransform<'a>) -> Self {
        Self { cldr_transforms }
    }

    /// Given an internal ID for an existing transliterator, returns the directory name for a
    /// source that maps to the given internal ID. Additionally returns the direction (relative to the metadata)
    /// of the passed internal ID.
    fn lookup_dir_from_internal_id(
        &self,
        internal_id: &str,
    ) -> Result<Option<(String, icu_transliterator_parser::Direction)>, DataError> {
        for transform in self.cldr_transforms.list_transforms()? {
            let metadata = self.cldr_transforms.read_and_parse_metadata(&transform)?;
            let (forwards, backwards) = internal_ids_from_metadata(metadata);
            if let Some(forwards) = forwards {
                if forwards == internal_id {
                    return Ok(Some((
                        transform,
                        icu_transliterator_parser::Direction::Forward,
                    )));
                }
            }
            if let Some(backwards) = backwards {
                if backwards == internal_id {
                    return Ok(Some((
                        transform,
                        icu_transliterator_parser::Direction::Reverse,
                    )));
                }
            }
        }
        Ok(None)
    }

    /// Returns a mapping from known legacy IDs to internal ICU4X IDs. The legacy ID keys are normalized to ASCII lowercase.
    ///
    /// The compilation process uses this mapping to go from legacy IDs to internal IDs, if possible.
    /// Otherwise [`icu_transliterator_parser::legacy_id_to_internal_id`](icu_transliterator_parser::legacy_id_to_internal_id) is used.
    fn generate_mapping(&self) -> Result<HashMap<String, String>, DataError> {
        let mut mapping = HashMap::new();
        for transform in self.cldr_transforms.list_transforms()? {
            let metadata = self.cldr_transforms.read_and_parse_metadata(&transform)?;
            let (forwards, backwards) = internal_ids_from_metadata(metadata);
            if let Some(forwards) = forwards {
                // for all forwards legacy aliases, map them to the internal ID
                // bcp47 aliases are skipped, because those are not valid legacy IDs
                let legacy_iter = metadata.alias.iter().filter_map(|alias| match alias {
                    TransformAlias::LegacyId(s) => Some(s),
                    _ => None,
                });
                for alias in legacy_iter {
                    mapping.insert(alias.to_ascii_lowercase(), forwards.clone());
                }
                // source, target, and variant may also be used
                let mut legacy_id = format!("{}-{}", metadata.source, metadata.target);
                if let Some(variant) = &metadata.variant {
                    legacy_id.push_str(&format!("/{}", variant));
                }
                legacy_id.make_ascii_lowercase();
                mapping.insert(legacy_id, forwards.clone());
            }
            if let Some(backwards) = backwards {
                // for all backwards aliases, map them to the internal ID
                // bcp47 aliases are skipped, because those are not valid legacy IDs
                let legacy_iter = metadata
                    .backward_alias
                    .iter()
                    .filter_map(|alias| match alias {
                        TransformAlias::LegacyId(s) => Some(s),
                        _ => None,
                    });
                for backward_alias in legacy_iter {
                    mapping.insert(backward_alias.to_ascii_lowercase(), backwards.clone());
                }
                // target, source, and variant may also be used
                let mut legacy_id = format!("{}-{}", metadata.target, metadata.source);
                if let Some(variant) = &metadata.variant {
                    legacy_id.push_str(&format!("/{}", variant));
                }
                legacy_id.make_ascii_lowercase();
                mapping.insert(legacy_id, backwards.clone());
            }
        }
        Ok(mapping)
    }
}

impl DataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
        self.check_req::<TransliteratorRulesV1Marker>(req)?;

        // all our `supported_locales` have an auxiliary key
        #[allow(clippy::unwrap_used)]
        let internal_id = req.locale.get_aux().unwrap().to_string();

        let tc = TransliteratorCollection::new(self.cldr()?.transforms());

        let mapping = tc.generate_mapping()?;

        // our `supported_locales` use the same mapping mechanism as in lookup_dir_from_internal_id
        #[allow(clippy::unwrap_used)]
        let (transform, want_direction) = tc.lookup_dir_from_internal_id(&internal_id)?.unwrap();

        let metadata = self
            .cldr()?
            .transforms()
            .read_and_parse_metadata(&transform)?;
        let visibility = metadata.visibility;

        let metadata_dir = match metadata.direction {
            transforms::Direction::Forward => icu_transliterator_parser::Direction::Forward,
            transforms::Direction::Backward => icu_transliterator_parser::Direction::Reverse,
            transforms::Direction::Both => icu_transliterator_parser::Direction::Both,
        };
        let metadata = icu_transliterator_parser::Metadata::new(
            visibility == transforms::Visibility::External,
            metadata_dir,
        );

        let source = self.cldr()?.transforms().read_source(&transform)?;

        let (forwards, backwards) = icu_transliterator_parser::parse_unstable(
            &source,
            want_direction,
            metadata,
            mapping,
            self,
        )
        .map_err(|e| DataError::custom("transliterator parsing failed").with_debug_context(&e))?;
        let transliterator = match want_direction {
            icu_transliterator_parser::Direction::Forward => {
                // the parser guarantees we receive this
                #[allow(clippy::unwrap_used)]
                forwards.unwrap()
            }
            icu_transliterator_parser::Direction::Reverse => {
                // the parser guarantees we receive this
                #[allow(clippy::unwrap_used)]
                backwards.unwrap()
            }
            _ => {
                // unreachable because `lookup_dir_from_internal_id` only ever returns one direction.
                unreachable!("unexpected want_direction")
            }
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(transliterator)),
        })
    }
}

impl IterableDataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        let mut locales = Vec::new();
        for transform in self.cldr()?.transforms().list_transforms()? {
            let metadata = self
                .cldr()?
                .transforms()
                .read_and_parse_metadata(&transform)?;
            let (forwards, backwards) = internal_ids_from_metadata(metadata);
            if let Some(forwards) = forwards {
                locales.push(format!("und+{forwards}").parse()?);
            }
            if let Some(backwards) = backwards {
                locales.push(format!("und+{backwards}").parse()?);
            }
        }
        Ok(locales)
    }
}

/// Get the internal ICU4X ID for this transliterator from CLDR metadata. This is what will end up in
/// the DataLocale's auxiliary key.
///
/// Returns (forwards, backwards) internal IDs if the corresponding direction is supported according
/// to the metadata.
fn internal_ids_from_metadata(metadata: &transforms::Resource) -> (Option<String>, Option<String>) {
    let forwards = if matches!(
        metadata.direction,
        transforms::Direction::Forward | transforms::Direction::Both
    ) {
        Some(internal_id_from_parts(
            &metadata.alias,
            &metadata.source,
            &metadata.target,
            metadata.variant.as_deref(),
        ))
    } else {
        None
    };
    let backwards = if matches!(
        metadata.direction,
        transforms::Direction::Backward | transforms::Direction::Both
    ) {
        Some(internal_id_from_parts(
            &metadata.backward_alias,
            &metadata.target,
            &metadata.source,
            metadata.variant.as_deref(),
        ))
    } else {
        None
    };

    (forwards, backwards)
}

fn internal_id_from_parts(
    aliases: &[TransformAlias],
    source: &str,
    target: &str,
    variant: Option<&str>,
) -> String {
    find_bcp47_in_list(aliases).unwrap_or_else(|| {
        icu_transliterator_parser::legacy_id_to_internal_id(source, target, variant)
    })
}

fn find_bcp47_in_list(list: &[TransformAlias]) -> Option<String> {
    list.iter().find_map(|alias| {
        if let TransformAlias::Bcp47(locale) = alias {
            Some(locale.to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_ascii_forward() {
        let provider = crate::DatagenProvider::latest_tested_offline_subset();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                locale: &"und+de-t-de-d0-ascii".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
    }

    #[test]
    fn test_latin_ascii_backward() {
        let provider = crate::DatagenProvider::latest_tested_offline_subset();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                locale: &"und+und-Latn-t-s0-ascii".parse().unwrap(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
    }
}
