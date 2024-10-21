// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_serde::transforms;
use super::CldrCache;
use crate::SourceDataProvider;
use icu::experimental::transliterate::provider::*;
use icu::experimental::transliterate::RuleCollection;
use icu::locale::Locale;
use icu_provider::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::sync::Mutex;

impl CldrCache {
    pub(crate) fn transforms(&self) -> Result<&Mutex<RuleCollection>, DataError> {
        self.transforms
            .get_or_init(|| {
                fn invent_bcp47(transform: &str) -> Locale {
                    let transform = transform.to_ascii_lowercase();
                    let r = ["und", "x"]
                        .into_iter()
                        .chain(
                            transform
                                .split('-')
                                .map(|t| &t[..core::cmp::min(8, t.len())]),
                        )
                        .join("-")
                        .parse()
                        .unwrap();
                    r
                }

                let mut provider = RuleCollection::default();

                for transform in self.serde_cache.list("cldr-transforms/transforms/")? {
                    let Some(transform) = transform.strip_suffix(".json") else {
                        continue;
                    };

                    if transform == "und-Ethi-t-und-latn-m0-beta_metsehaf-geminate" {
                        // References an unknown transliterator
                        continue;
                    }

                    if transform == "byn-Ethi-t-byn-latn-m0-xaleget" {
                        // Doesn't parse (backreference error)
                        continue;
                    }

                    if transform == "Han-Latin-Names" {
                        // Alias clash with Han-Latin
                        continue;
                    }

                    if transform == "Thai-Latin" {
                        // References an unknown transliterator (Any-BreakInternal)
                        continue;
                    }

                    let metadata = self
                        .serde_cache
                        .read_and_parse_json::<transforms::Resource>(&format!(
                            "cldr-transforms/transforms/{transform}.json"
                        ))?;
                    let source = self
                        .serde_cache
                        .root
                        .read_to_string(&format!(
                            "cldr-transforms/transforms/{}",
                            metadata.rules_file
                        ))?
                        // Declares a sequence of Unicode sets instead of a Unicode set
                        .replace(
                            "$initialPunct = [:Ps:][:Pi:];",
                            "$initialPunct = [[:Ps:][:Pi:]];",
                        )
                        // I'm not sure why this errors
                        .replace("ə̃ {ə̃}+ → ə̃;", "")
                        // This does not escape the $, so the = is interpreted as a variable name
                        .replace(r#"$="#, r#"\$="#)
                        // Any-ASCII does not exist and should probably the Latin-ASCII
                        .replace("Any-ASCII", "Latin-ASCII")
                        // Non-canonical property names
                        .replace("block=", "Block=")
                        .replace("script=", "Script=")
                        .replace("case-ignorable:", "Case_Ignorable:")
                        .replace("cased:", "Cased:")
                        .replace("ideographic:", "Ideographic:")
                        // Non-canonical property values
                        .replace("ccc=above", "ccc=Above")
                        .replace("ccc=below", "ccc=Below")
                        .replace("UppercaseLetter:", "Uppercase_Letter:")
                        .replace("nonspacing mark:", "Nonspacing_Mark:")
                        .replace("letter:", "Letter:")
                        .replace("ARABIC:", "Arabic:")
                        .replace("arabic:", "Arabic:")
                        .replace("bengali:", "Bengali:")
                        .replace("greek:", "Greek:")
                        .replace("han:", "Han:")
                        .replace("latin:", "Latin:")
                        .replace("thaana:", "Thaana:")
                        .replace("thai:", "Thai:");

                    if matches!(
                        metadata.direction,
                        transforms::Direction::Forward | transforms::Direction::Both
                    ) {
                        let bcp47_alias =
                            if let Some(bcp47_aliases) = metadata.alias_bcp47.as_ref() {
                                Locale::try_from_str(bcp47_aliases.split(' ').next().unwrap())
                                    .map_err(|_| {
                                        DataError::custom("invalid locale")
                                            .with_display_context(bcp47_aliases)
                                    })?
                            } else {
                                invent_bcp47(&format!("{transform}-rev"))
                            };

                        provider.register_source(
                            &bcp47_alias,
                            source.clone(),
                            [metadata.alias.as_deref().unwrap_or(
                                format!("{}-{}", metadata.source, metadata.target).as_str(),
                            )]
                            .into_iter()
                            .chain(
                                metadata
                                    .alias_bcp47
                                    .as_deref()
                                    .unwrap_or_default()
                                    .split(' ')
                                    .skip(1),
                            ),
                            false,
                            metadata.visibility == transforms::Visibility::External,
                        );
                    }

                    if matches!(
                        metadata.direction,
                        transforms::Direction::Backward | transforms::Direction::Both
                    ) {
                        let bcp47_alias =
                            if let Some(bcp47_aliases) = metadata.backward_alias_bcp47.as_ref() {
                                Locale::try_from_str(bcp47_aliases.split(' ').next().unwrap())
                                    .map_err(|_| {
                                        DataError::custom("invalid locale")
                                            .with_display_context(bcp47_aliases)
                                    })?
                            } else {
                                invent_bcp47(&format!("{transform}-rev"))
                            };
                        provider.register_source(
                            &bcp47_alias,
                            source,
                            [metadata.backward_alias.as_deref().unwrap_or(
                                format!("{}-{}", metadata.target, metadata.source).as_str(),
                            )]
                            .into_iter()
                            .chain(
                                metadata
                                    .backward_alias_bcp47
                                    .as_deref()
                                    .unwrap_or_default()
                                    .split(' ')
                                    .skip(1),
                            ),
                            true,
                            metadata.visibility == transforms::Visibility::External,
                        );
                    }
                }
                Ok(Mutex::new(provider))
            })
            .as_ref()
            .map_err(|&e| e)
    }
}

impl DataProvider<TransliteratorRulesV1Marker> for SourceDataProvider {
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

impl crate::IterableDataProviderCached<TransliteratorRulesV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .transforms()?
            .lock()
            .expect("poison")
            .as_provider_unstable(self, self)?
            .iter_ids()?
            .into_iter()
            .map(|id| id.as_borrowed().into_owned())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_ascii_forward() {
        let provider = SourceDataProvider::new_testing();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierCow::from_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic("de-t-de-d0-ascii"),
                )
                .as_borrowed(),
                ..Default::default()
            })
            .unwrap()
            .payload;
    }

    #[test]
    fn test_latin_ascii_backward() {
        let provider = SourceDataProvider::new_testing();

        let _data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierCow::from_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic("und-latn-t-s0-ascii"),
                )
                .as_borrowed(),
                ..Default::default()
            })
            .unwrap()
            .payload;
    }
}
