// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collator::provider::*;
use icu::locale::{
    locale,
    subtags::{language, script},
};
use icu_provider::prelude::*;
use std::collections::HashSet;
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use zerovec::ZeroVec;

mod collator_serde;

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn id_to_file_name(id: DataIdentifierBorrowed) -> String {
    let mut s = if id.locale.is_unknown() {
        "root".to_owned()
    } else {
        id.locale
            .to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };

    // und_Hant -> zh_stroke
    // und_Hans -> zh_pinyin
    // und_Hani/x -> zh_x

    if s == "und_Hant" {
        return "zh_stroke".into();
    } else if s == "und_Hans" {
        return "zh_pinyin".into();
    } else if s == "und_Hani" {
        s = "zh".into();
    }

    s.push('_');
    s.push_str(match id.marker_attributes.as_str() {
        "" => "standard",
        "trad" => "traditional",
        "phonebk" => "phonebook",
        "dict" => "dictionary",
        extension => extension,
    });
    s
}

fn file_name_to_ids(file_name: &str) -> Vec<DataIdentifierCow<'static>> {
    let (mut language, mut variant) = file_name.rsplit_once('_').unwrap();
    if language == "root" {
        language = "und";
    }

    let mut r = vec![];

    let Ok(mut locale) = DataLocale::try_from_str(&language.replace('_', "-")) else {
        return Default::default();
    };

    if language == "zh" {
        locale.language = language!("und");
        locale.script = Some(script!("Hani"));
        if variant == "pinyin" {
            // Pinyin is stored in both und-Hans and und-Hani/pinyin
            r.push(DataIdentifierCow::from_borrowed_and_owned(
                Default::default(),
                locale!("und-Hans").into(),
            ));
        } else if variant == "stroke" {
            // Stroke is stored in both und-Hans and und-Hani/stroke
            r.push(DataIdentifierCow::from_borrowed_and_owned(
                Default::default(),
                locale!("und-Hant").into(),
            ));
        }
    } else if variant == "standard" {
        variant = "";
    }

    let marker_attributes = match variant {
        "traditional" => DataMarkerAttributes::from_str_or_panic("trad").to_owned(),
        "phonebook" => DataMarkerAttributes::from_str_or_panic("phonebk").to_owned(),
        "dictionary" => DataMarkerAttributes::from_str_or_panic("dict").to_owned(),
        v => match DataMarkerAttributes::try_from_str(v) {
            Ok(s) => s.to_owned(),
            _ => return r,
        },
    };

    r.push(DataIdentifierCow::from_owned(marker_attributes, locale));
    r
}

impl SourceDataProvider {
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn load_toml<T>(&self, id: DataIdentifierBorrowed, suffix: &str) -> Result<&T, DataError>
    where
        for<'de> T: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.icuexport()?
            .read_and_parse_toml(&format!(
                "collation/{}/{}{}.toml",
                self.collation_root_han(),
                id_to_file_name(id),
                suffix
            ))
            .map_err(|e| match e.kind {
                DataErrorKind::Io(std::io::ErrorKind::NotFound) => {
                    DataErrorKind::IdentifierNotFound.into_error()
                }
                _ => e,
            })
    }

    fn list_ids(&self, suffix: &str) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .icuexport()?
            .list(&format!("collation/{}", self.collation_root_han()))?
            // Root data should not be listed for tailorings
            .filter(|p| !p.ends_with("root_standard_data.toml"))
            .filter_map(|mut file_name| {
                file_name.truncate(file_name.len() - ".toml".len());
                file_name.ends_with(suffix).then(|| {
                    file_name.truncate(file_name.len() - suffix.len());
                    file_name
                })
            })
            .flat_map(|s| file_name_to_ids(&s))
            .collect())
    }
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident),)+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
                    return Err(DataError::custom(
                        "icu_provider_source must be built with use_icu4c or use_wasm to build collation data",
                    )
                    .with_req($marker::INFO, req));
                    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
                    {
                        self.check_req::<$marker>(req)?;

                        let has_tailoring = self.list_ids("_data")?
                            .contains(&DataIdentifierCow::from_borrowed_and_owned(&req.id.marker_attributes, *req.id.locale));

                        Ok(DataResponse {
                            metadata: Default::default(),
                            payload: DataPayload::from_owned(self.load_toml::<collator_serde::$serde_struct>(req.id, <collator_serde::$serde_struct>::suffix()).and_then(|s| s.convert(has_tailoring)).map_err(|e| e.with_req(<$marker>::INFO, req))?),
                        })
                    }
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.list_ids(<collator_serde::$serde_struct>::suffix())
                }
            }
        )+

        #[test]
        fn test_all_fallback_overrides_handled() {
            let provider = SourceDataProvider::new_testing();
            let fallback_provider = icu_provider_adapters::fallback::LocaleFallbackProvider::new(
                &provider,
                icu::locale::LocaleFallbacker::try_new_unstable(&provider).unwrap(),
            );

            let required_overrides = &provider
                .cldr()
                .unwrap()
                .core()
                .read_and_parse::<super::cldr_serde::parent_locales::Resource>(
                    "supplemental/parentLocales.json",
                )
                .unwrap()
                .supplemental
                .parent_locales
                .collations;

            for (locale, parent) in required_overrides {
                // TODO(CLDR49): Remove, https://unicode-org.atlassian.net/browse/CLDR-19386
                if (locale, parent) == (&locale!("sr-Cyrl-ME").id, &locale!("sr-ME").id) {
                    continue;
                }

                let locale = locale.into();
                let parent = parent.into();

                $(
                    if !$marker::INFO.is_singleton {
                        for attribute in IterableDataProvider::<$marker>::iter_ids(&provider)
                            .unwrap()
                            .into_iter()
                            .filter(|id| id.locale == locale)
                            .map(|id| id.marker_attributes)
                            .collect::<HashSet<_>>()
                        {
                            let locale = DataIdentifierBorrowed::for_marker_attributes_and_locale(&*attribute, &locale);
                            let parent = DataIdentifierBorrowed::for_marker_attributes_and_locale(&*attribute, &parent);
                            assert_eq!(
                                DataProvider::<$marker>::load(&fallback_provider, DataRequest { id: locale, ..Default::default() })
                                    .as_ref()
                                    .map(|response| response.payload.get()),
                                DataProvider::<$marker>::load(&fallback_provider, DataRequest { id: parent, ..Default::default() })
                                    .as_ref()
                                    .map(|response| response.payload.get()),
                                "{locale:?} should match {parent:?} for {:?}", $marker::INFO
                            );
                        }
                    }
                )+
            }
        }
    };
}

collation_provider!(
    (CollationDiacriticsV1, CollationDiacritics),
    (CollationJamoV1, CollationJamo),
    (CollationMetadataV1, CollationMetadata),
    (CollationReorderingV1, CollationReordering),
    (CollationSpecialPrimariesV1, CollationSpecialPrimaries),
    (CollationRootV1, CollationData),
    (CollationTailoringV1, CollationData),
);

impl collator_serde::CollationData {
    fn suffix() -> &'static str {
        "_data"
    }

    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(&self, _has_tailoring: bool) -> Result<CollationData<'static>, DataError> {
        use icu::collections::codepointtrie::CodePointTrie;
        use icu_codepointtrie_builder::CodePointTrieBuilder;

        let trie = CodePointTrie::<u32>::try_from(&self.trie)
            .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;

        let mut builder = CodePointTrieBuilder::new(
            trie.get('\u{10FFFF}'),
            trie.get32(u32::MAX),
            icu::collections::codepointtrie::TrieType::Small,
        );

        for i in 0..0xAC00 {
            builder.set_value(i, trie.get32(i));
        }
        for _ in 0xAC00..0xD7A4 {
            // Use the default value for Hangul syllables. We are not
            // relying on the collation data to catch Hangul syllables.
            // Furthermore, having non-default values in this range is
            // bad for tailorings whose characters of interest are
            // below the fast-access boundary for the small trie type.
        }
        for i in 0xD7A4..=(char::MAX as u32) {
            builder.set_value(i, trie.get32(i));
        }

        Ok(CollationData {
            trie: builder.build(),
            contexts: ZeroVec::alloc_from_slice(&self.contexts),
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
            ces: self.ces.iter().map(|i| *i as u64).collect(),
        })
    }
}

impl collator_serde::CollationDiacritics {
    fn suffix() -> &'static str {
        "_dia"
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(&self, _has_tailoring: bool) -> Result<CollationDiacritics<'static>, DataError> {
        Ok(CollationDiacritics {
            secondaries: ZeroVec::alloc_from_slice(&self.secondaries),
        })
    }
}

impl collator_serde::CollationJamo {
    fn suffix() -> &'static str {
        "_jamo"
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(&self, _has_tailoring: bool) -> Result<CollationJamo<'static>, DataError> {
        Ok(CollationJamo {
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
        })
    }
}

impl collator_serde::CollationMetadata {
    fn suffix() -> &'static str {
        "_meta"
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(&self, has_tailoring: bool) -> Result<CollationMetadata, DataError> {
        if has_tailoring {
            // ICU seems to not be setting the tailoring bit correctly.
            Ok(CollationMetadata {
                bits: self.bits | 1 << 3,
            })
        } else {
            Ok(CollationMetadata { bits: self.bits })
        }
    }
}

impl collator_serde::CollationReordering {
    fn suffix() -> &'static str {
        "_reord"
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(&self, _has_tailoring: bool) -> Result<CollationReordering<'static>, DataError> {
        Ok(CollationReordering {
            min_high_no_reorder: self.min_high_no_reorder,
            reorder_table: ZeroVec::alloc_from_slice(&self.reorder_table),
            reorder_ranges: ZeroVec::alloc_from_slice(&self.reorder_ranges),
        })
    }
}

impl collator_serde::CollationSpecialPrimaries {
    fn suffix() -> &'static str {
        "_prim"
    }

    #[allow(clippy::unnecessary_wraps)]
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    fn convert(
        &self,
        _has_tailoring: bool,
    ) -> Result<CollationSpecialPrimaries<'static>, DataError> {
        // Note, at least for icu4x/2025-05-01/77.x, both `implicithan` and `unihan` have the same `compressible_bytes`.
        let compressible_bytes = self.compressible_bytes.as_deref().unwrap_or(&[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, true, false,
        ]);

        assert_eq!(compressible_bytes.len(), 256);

        let mut packed_compressible_bytes = [0u16; 16];
        for (i, &is_compressible) in compressible_bytes.iter().enumerate() {
            if is_compressible {
                let arr_index = i >> 4;
                let mask = 1 << (i & 0b1111);
                packed_compressible_bytes[arr_index] |= mask;
            }
        }

        Ok(CollationSpecialPrimaries {
            last_primaries: self.last_primaries.iter().copied().collect(),
            numeric_primary: self.numeric_primary,
            compressible_bytes: packed_compressible_bytes.into_iter().collect(),
        })
    }
}
