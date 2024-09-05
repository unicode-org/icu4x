// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collator::provider::*;
use icu::collections::codepointtrie::CodePointTrie;
use icu::locale::subtags::language;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;
use writeable::Writeable;
use zerovec::ZeroVec;

mod collator_serde;

fn id_to_file_name(id: DataIdentifierBorrowed) -> String {
    let mut s = if id.locale.is_default() {
        "root".to_owned()
    } else {
        id.locale
            .write_to_string()
            .replace('-', "_")
            .replace("posix", "POSIX")
    };
    if !id.marker_attributes.is_empty() {
        s.push('_');
        s.push_str(match id.marker_attributes.as_str() {
            "trad" => "traditional",
            "phonebk" => "phonebook",
            "dict" => "dictionary",
            "gb2312" => "gb2312han",
            extension => extension,
        });
    } else if id.locale.language == language!("zh") {
        // "zh" uses "_pinyin" as the default
        s.push_str("_pinyin");
    } else {
        // Everyting else uses "_standard"
        s.push_str("_standard");
    }
    s
}

fn file_name_to_id(file_name: &str) -> Option<DataIdentifierCow<'static>> {
    let (language, mut variant) = file_name.rsplit_once('_').unwrap();
    let locale = if language == "root" {
        DataLocale::default()
    } else {
        language.parse().ok()?
    };

    // See above for the two special cases.
    if language == "zh" {
        if variant == "pinyin" {
            variant = "";
        }
    } else if variant == "standard" {
        variant = "";
    }

    let marker_attributes = match variant {
        "traditional" => DataMarkerAttributes::from_str_or_panic("trad").to_owned(),
        "phonebook" => DataMarkerAttributes::from_str_or_panic("phonebk").to_owned(),
        "dictionary" => DataMarkerAttributes::from_str_or_panic("dict").to_owned(),
        "gb2312han" => DataMarkerAttributes::from_str_or_panic("gb2312").to_owned(),
        v => DataMarkerAttributes::try_from_str(v).ok()?.to_owned(),
    };

    Some(DataIdentifierCow::from_owned(marker_attributes, locale))
}

impl SourceDataProvider {
    fn load_toml<T>(&self, id: DataIdentifierBorrowed, suffix: &str) -> Result<&T, DataError>
    where
        for<'de> T: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.icuexport()?
            .read_and_parse_toml(&format!(
                "collation/{}/{}{}.toml",
                self.collation_han_database(),
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
            .list(&format!("collation/{}", self.collation_han_database()))?
            .filter_map(|mut file_name| {
                file_name.truncate(file_name.len() - ".toml".len());
                file_name.ends_with(suffix).then(|| {
                    file_name.truncate(file_name.len() - suffix.len());
                    file_name
                })
            })
            .filter_map(|s| file_name_to_id(&s))
            .collect())
    }
}

macro_rules! collation_provider {
    ($(($marker:ident, $serde_struct:ident, $suffix:literal,),)+) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(self.load_toml::<collator_serde::$serde_struct>(req.id, $suffix).and_then(TryInto::try_into).map_err(|e| e.with_req(<$marker>::INFO, req))?),
                    })
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
    (CollationDiacriticsV1Marker, CollationDiacritics, "_dia",),
    (CollationJamoV1Marker, CollationJamo, "_jamo",),
    (CollationMetadataV1Marker, CollationMetadata, "_meta",),
    (CollationReorderingV1Marker, CollationReordering, "_reord",),
    (
        CollationSpecialPrimariesV1Marker,
        CollationSpecialPrimaries,
        "_prim",
    ),
);

impl DataProvider<CollationRootV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CollationRootV1Marker>, DataError> {
        self.check_req::<CollationRootV1Marker>(req)?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(Default::default(), "_data")
                    .map_err(|e| e.with_req(CollationRootV1Marker::INFO, req))?
                    .try_into()?,
            ),
        })
    }
}

impl IterableDataProviderCached<CollationRootV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

impl DataProvider<CollationTailoringV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CollationTailoringV1Marker>, DataError> {
        self.check_req::<CollationTailoringV1Marker>(req)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(req.id, "_data")
                    .and_then(TryInto::try_into)
                    .map_err(|e| e.with_req(<CollationTailoringV1Marker>::INFO, req))?,
            ),
        })
    }
}

impl IterableDataProviderCached<CollationTailoringV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .list_ids("_data")?
            .into_iter()
            .filter(|s| *s != Default::default())
            .collect())
    }
}

impl<'a> TryInto<CollationDataV1<'static>> for &'a collator_serde::CollationData {
    type Error = DataError;

    fn try_into(self) -> Result<CollationDataV1<'static>, Self::Error> {
        Ok(CollationDataV1 {
            trie: CodePointTrie::<u32>::try_from(&self.trie)
                .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?,
            contexts: ZeroVec::alloc_from_slice(&self.contexts),
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
            ces: self.ces.iter().map(|i| *i as u64).collect(),
        })
    }
}

impl<'a> TryInto<CollationDiacriticsV1<'static>> for &'a collator_serde::CollationDiacritics {
    type Error = DataError;

    fn try_into(self) -> Result<CollationDiacriticsV1<'static>, Self::Error> {
        Ok(CollationDiacriticsV1 {
            secondaries: ZeroVec::alloc_from_slice(&self.secondaries),
        })
    }
}

impl<'a> TryInto<CollationJamoV1<'static>> for &'a collator_serde::CollationJamo {
    type Error = DataError;

    fn try_into(self) -> Result<CollationJamoV1<'static>, Self::Error> {
        Ok(CollationJamoV1 {
            ce32s: ZeroVec::alloc_from_slice(&self.ce32s),
        })
    }
}

impl<'a> TryInto<CollationMetadataV1> for &'a collator_serde::CollationMetadata {
    type Error = DataError;

    fn try_into(self) -> Result<CollationMetadataV1, Self::Error> {
        Ok(CollationMetadataV1 { bits: self.bits })
    }
}

impl<'a> TryInto<CollationReorderingV1<'static>> for &'a collator_serde::CollationReordering {
    type Error = DataError;

    fn try_into(self) -> Result<CollationReorderingV1<'static>, Self::Error> {
        Ok(CollationReorderingV1 {
            min_high_no_reorder: self.min_high_no_reorder,
            reorder_table: ZeroVec::alloc_from_slice(&self.reorder_table),
            reorder_ranges: ZeroVec::alloc_from_slice(&self.reorder_ranges),
        })
    }
}

impl<'a> TryInto<CollationSpecialPrimariesV1<'static>>
    for &'a collator_serde::CollationSpecialPrimaries
{
    type Error = DataError;

    fn try_into(self) -> Result<CollationSpecialPrimariesV1<'static>, Self::Error> {
        Ok(CollationSpecialPrimariesV1 {
            last_primaries: ZeroVec::alloc_from_slice(&self.last_primaries),
            numeric_primary: self.numeric_primary,
        })
    }
}

#[test]

fn test_zh_non_baked() {
    use core::cmp::Ordering;
    use icu::collator::{Collator, CollatorOptions};
    use icu::locale::fallback::LocaleFallbacker;
    use icu_provider_adapters::fallback::LocaleFallbackProvider;

    let provider = LocaleFallbackProvider::new(
        SourceDataProvider::new_testing(),
        LocaleFallbacker::new_without_data(),
    );

    // Note: ㄅ is Bopomofo.
    {
        let locale: icu::locale::Locale = "zh-u-co-gb2312".parse().unwrap();
        let collator =
            Collator::try_new_unstable(&provider, &locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "ж"), Ordering::Greater);

        // TODO(#5136): broken, these should be equal
        assert_ne!(collator.compare("艾", "佰"), Ordering::Less);
        // In GB2312 proper, Bopomofo comes before Han, but the
        // collation leaves Bopomofo unreordered, so it comes after.
        assert_ne!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_ne!(collator.compare("佰", "ㄅ"), Ordering::Less);
        assert_ne!(collator.compare("不", "把"), Ordering::Greater);
    }
    {
        let locale: icu::locale::Locale = "zh-u-co-big5han".parse().unwrap();
        let collator =
            Collator::try_new_unstable(&provider, &locale.into(), CollatorOptions::new()).unwrap();
        assert_eq!(collator.compare("艾", "a"), Ordering::Greater);
        assert_eq!(collator.compare("佰", "a"), Ordering::Greater);
        assert_eq!(collator.compare("ㄅ", "a"), Ordering::Greater);
        assert_eq!(collator.compare("不", "把"), Ordering::Less);

        // TODO(#5136): broken, these should be equal
        assert_ne!(collator.compare("ㄅ", "ж"), Ordering::Less);
        assert_ne!(collator.compare("艾", "佰"), Ordering::Less);
        assert_ne!(collator.compare("艾", "ㄅ"), Ordering::Less);
        assert_ne!(collator.compare("佰", "ㄅ"), Ordering::Less);
    }
}
