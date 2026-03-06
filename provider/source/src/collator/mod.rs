// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collator::provider::*;
use icu::collections::codepointtrie::CodePointTrie;
use icu::locale::subtags::{language, script};
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use icu_codepointtrie_builder::CodePointTrieBuilder;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use writeable::Writeable;
use zerovec::ZeroVec;

mod collator_serde;

fn id_to_file_name(id: DataIdentifierBorrowed) -> String {
    let mut s = if id.locale.is_unknown() {
        "root".to_owned()
    } else {
        id.locale
            .write_to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };

    // und-Hant -> zh_stroke
    // und-Hans -> zh_pinyin
    // und-Hani/x -> zh_x

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

fn file_name_to_id(file_name: &str) -> Vec<DataIdentifierCow<'static>> {
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
                "und-Hans".parse().unwrap(),
            ));
        } else if variant == "stroke" {
            // Stroke is stored in both und-Hans and und-Hani/stroke
            r.push(DataIdentifierCow::from_borrowed_and_owned(
                Default::default(),
                "und-Hant".parse().unwrap(),
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
            .filter_map(|mut file_name| {
                file_name.truncate(file_name.len() - ".toml".len());
                file_name.ends_with(suffix).then(|| {
                    file_name.truncate(file_name.len() - suffix.len());
                    file_name
                })
            })
            .flat_map(|s| file_name_to_id(&s))
            .collect())
    }
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal,),)+) => {
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

                        Ok(DataResponse {
                            metadata: Default::default(),
                            payload: DataPayload::from_owned(self.load_toml::<collator_serde::$serde_struct>(req.id, $suffix).and_then(TryInto::try_into).map_err(|e| e.with_req(<$marker>::INFO, req))?),
                        })
                    }
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.list_ids($suffix)
                }
            }
        )+
    };
}

collation_provider!(
    (CollationDiacriticsV1, CollationDiacritics, "_dia",),
    (CollationJamoV1, CollationJamo, "_jamo",),
    (CollationMetadataV1, CollationMetadata, "_meta",),
    (CollationReorderingV1, CollationReordering, "_reord",),
    (
        CollationSpecialPrimariesV1,
        CollationSpecialPrimaries,
        "_prim",
    ),
);

impl DataProvider<CollationRootV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CollationRootV1>, DataError> {
        self.check_req::<CollationRootV1>(req)?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(Default::default(), "_data")
                    .map_err(|e| e.with_req(CollationRootV1::INFO, req))?
                    .try_into()?,
            ),
        })
    }
}

impl IterableDataProviderCached<CollationRootV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<CollationTailoringV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CollationTailoringV1>, DataError> {
        self.check_req::<CollationTailoringV1>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(req.id, "_data")
                    .and_then(TryInto::try_into)
                    .map_err(|e| e.with_req(<CollationTailoringV1>::INFO, req))?,
            ),
        })
    }
}

impl IterableDataProviderCached<CollationTailoringV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .list_ids("_data")?
            .into_iter()
            .filter(|s| *s != Default::default())
            .collect())
    }
}

fn rebuild_data(trie: CodePointTrie<u32>) -> CodePointTrie<u32> {
    #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
    {
        let _ = trie;
        unreachable!("Should have errored out earlier");
    }
    #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
    {
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

        builder.build()
    }
}

impl TryInto<CollationData<'static>> for &collator_serde::CollationData {
    type Error = DataError;

    fn try_into(self) -> Result<CollationData<'static>, Self::Error> {
        Ok(CollationData {
            trie: rebuild_data(
                CodePointTrie::<u32>::try_from(&self.trie)
                    .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?,
            ),
            contexts: ZeroVec::alloc_from_slice(&self.contexts),
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
            ces: self.ces.iter().map(|i| *i as u64).collect(),
        })
    }
}

impl TryInto<CollationDiacritics<'static>> for &collator_serde::CollationDiacritics {
    type Error = DataError;

    fn try_into(self) -> Result<CollationDiacritics<'static>, Self::Error> {
        Ok(CollationDiacritics {
            secondaries: ZeroVec::alloc_from_slice(&self.secondaries),
        })
    }
}

impl TryInto<CollationJamo<'static>> for &collator_serde::CollationJamo {
    type Error = DataError;

    fn try_into(self) -> Result<CollationJamo<'static>, Self::Error> {
        Ok(CollationJamo {
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
        })
    }
}

impl TryInto<CollationMetadata> for &collator_serde::CollationMetadata {
    type Error = DataError;

    fn try_into(self) -> Result<CollationMetadata, Self::Error> {
        Ok(CollationMetadata { bits: self.bits })
    }
}

impl TryInto<CollationReordering<'static>> for &collator_serde::CollationReordering {
    type Error = DataError;

    fn try_into(self) -> Result<CollationReordering<'static>, Self::Error> {
        Ok(CollationReordering {
            min_high_no_reorder: self.min_high_no_reorder,
            reorder_table: ZeroVec::alloc_from_slice(&self.reorder_table),
            reorder_ranges: ZeroVec::alloc_from_slice(&self.reorder_ranges),
        })
    }
}

impl TryInto<CollationSpecialPrimaries<'static>> for &collator_serde::CollationSpecialPrimaries {
    type Error = DataError;

    fn try_into(self) -> Result<CollationSpecialPrimaries<'static>, Self::Error> {
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
            last_primaries: self
                .last_primaries
                .iter()
                .copied()
                .chain(packed_compressible_bytes)
                .collect(),
            numeric_primary: self.numeric_primary,
        })
    }
}
