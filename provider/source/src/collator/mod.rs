// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::collator::provider::*;
use icu::collections::codepointtrie::CodePointTrie;
use icu::collections::codepointtrie::CodePointTrieBuilder;
use icu::collections::codepointtrie::SmallCodePointTrie;
use icu::collections::codepointtrie::Typed;
use icu::collections::codepointtrie::TypedCodePointTrie;
use icu::locale::{
    data_locale,
    subtags::{language, script},
};
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::ops::Range;
use zerovec::ZeroVec;

mod collator_serde;

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
                data_locale!("und-Hans"),
            ));
        } else if variant == "stroke" {
            // Stroke is stored in both und-Hans and und-Hani/stroke
            r.push(DataIdentifierCow::from_borrowed_and_owned(
                Default::default(),
                data_locale!("und-Hant"),
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
    /// Returns the root collation data that tailorings hoist values from.
    fn converted_collation_root(&self) -> Result<&CollationData<'static>, DataError> {
        // The converted root depends on the icuexport source and on the
        // root Han setting, so cache it with the source under a key that
        // includes the latter.
        self.icuexport()?.get_or_compute(
            &format!("collation/{}/root (converted)", self.collation_root_han()),
            || {
                convert_data_from_serde(
                    self.load_toml::<collator_serde::CollationData>(Default::default(), "_data")?,
                    None,
                )
            },
        )
    }

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
            .filter(|name| !name.contains("POSIX")) // No known use cases
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
                    self.check_req::<$marker>(req)?;

                    let has_tailoring = self.list_ids("_data")?
                        .contains(&DataIdentifierCow::from_borrowed_and_owned(&req.id.marker_attributes, *req.id.locale));

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(self.load_toml::<collator_serde::$serde_struct>(req.id, <collator_serde::$serde_struct>::suffix()).and_then(|s| s.convert(has_tailoring)).map_err(|e| e.with_req(<$marker>::INFO, req))?),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    self.list_ids(<collator_serde::$serde_struct>::suffix())
                }
            }
        )+
    };
}

collation_provider!(
    (CollationDiacriticsV1, CollationDiacritics),
    (CollationJamoV1, CollationJamo),
    (CollationMetadataV1, CollationMetadata),
    (CollationReorderingV1, CollationReordering),
    (CollationSpecialPrimariesV1, CollationSpecialPrimaries),
);

#[cfg(test)]
macro_rules! test_fallback_overrides {
    ($($marker:ident),+ $(,)?) => {
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

#[cfg(test)]
test_fallback_overrides!(
    CollationDiacriticsV1,
    CollationJamoV1,
    CollationMetadataV1,
    CollationReorderingV1,
    CollationSpecialPrimariesV1,
    CollationRootV1,
    CollationTailoringV1,
);

impl DataProvider<CollationRootV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CollationRootV1>, DataError> {
        self.check_req::<CollationRootV1>(req)?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.converted_collation_root()
                    .map_err(|e| e.with_req(CollationRootV1::INFO, req))?
                    .clone(),
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
        let root_trie: &SmallCodePointTrie<u32> = &self
            .converted_collation_root()
            .map_err(|e| e.with_req(<CollationTailoringV1>::INFO, req))?
            .trie;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(req.id, "_data")
                    .and_then(|d| convert_data_from_serde(d, Some((&req.id, root_trie))))
                    .map_err(|e| e.with_req(<CollationTailoringV1>::INFO, req))?,
            ),
        })
    }
}

impl IterableDataProviderCached<CollationTailoringV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.list_ids("_data")
    }
}

/// Per-script hoisting within the fast range: if any code point in the
/// first range is tailored, hoist the self-contained root values for the
/// second range.
const SCRIPT_HOISTS: &[(&str, Range<u32>, Range<u32>)] = &[
    ("Cyrillic base", 0x0400..0x0500, 0x0410..0x0450),
    ("Armenian", 0x0531..0x0590, 0x0531..0x0590),
    ("Hebrew base", 0x0591..0x05F5, 0x05D0..0x05EB),
    // TODO: Should the Arabic hoisting be narrower?
    ("Arabic", 0x0600..0x0700, 0x0600..0x0700),
];

/// Returns the root value for `c` if the tailoring's value `current` is the
/// fallback to root (`default_value`) and the root value is self-contained.
fn hoistable(
    current: u32,
    root: &SmallCodePointTrie<u32>,
    c: u32,
    default_value: u32,
) -> Option<u32> {
    if current != default_value {
        return None;
    }
    let ce32 = root.get32(c);
    is_self_contained(ce32).then_some(ce32)
}

/// `true` iff any code point in `range` has a tailored (non-fallback) value.
fn is_tailored(fast: &[u32], default_value: u32, range: Range<u32>) -> bool {
    fast[range.start as usize..range.end as usize]
        .iter()
        .any(|v| *v != default_value)
}

/// Hoists the self-contained root values for the code points in `range`
/// that fall back to root in `fast`.
fn hoist_into_fast(
    fast: &mut [u32],
    root: &SmallCodePointTrie<u32>,
    default_value: u32,
    range: Range<u32>,
) {
    for c in range {
        if let Some(ce32) = hoistable(fast[c as usize], root, c, default_value) {
            fast[c as usize] = ce32;
        }
    }
}

fn rebuild_data<'a>(
    trie: CodePointTrie<'a, u32>,
    id_and_root: Option<(&DataIdentifierBorrowed, &SmallCodePointTrie<u32>)>,
) -> SmallCodePointTrie<'a, u32> {
    let default_value = trie.get('\u{10FFFF}');
    let mut rewritten_fast: Vec<u32> = Vec::new();
    if let Some((id, root)) = id_and_root {
        let collation_type: &str = id.marker_attributes;
        // Only optimize the default collation and the three non-unihan Chinese collations
        if collation_type.is_empty()
            || collation_type == "pinyin"
            || collation_type == "zhuyin"
            || collation_type == "stroke"
        {
            let bound = 0x1000;
            rewritten_fast = (0..bound).map(|i| trie.get32(i)).collect();
            // Copy the ASCII range
            hoist_into_fast(&mut rewritten_fast, root, default_value, 0..0x80);
            for (script, detect, hoist) in SCRIPT_HOISTS {
                if is_tailored(&rewritten_fast, default_value, detect.clone()) {
                    log::info!("Hoist {script} for {id:?}");
                    hoist_into_fast(&mut rewritten_fast, root, default_value, hoist.clone());
                }
            }

            // We currently don't have tailorings using the Syriac, Thaana,
            // N'Ko, Samaritan, or Mandaic scripts.

            // Fill the 7-bit blocks for Brahmic scripts
            for block in (0x0900..bound).step_by(0x80) {
                let block = block..block + 0x80;
                if is_tailored(&rewritten_fast, default_value, block.clone()) {
                    log::info!("Hoist 7-bit block starting at {:X} for {id:?}", block.start);
                    hoist_into_fast(&mut rewritten_fast, root, default_value, block);
                }
            }
        }
    }
    let mut builder = CodePointTrieBuilder::new(
        default_value,
        trie.get32(u32::MAX),
        icu::collections::codepointtrie::TrieType::Small,
    );

    // `rewritten_fast` is either empty or covers 0..0x1000.
    for (i, trie_val) in (0u32..).zip(rewritten_fast.iter()) {
        builder.set_value(i, *trie_val);
    }
    for i in (rewritten_fast.len() as u32)..0xAC00 {
        builder.set_value(i, trie.get32(i));
    }
    // Skip 0xAC00..0xD7A4 so that Hangul syllables get the default value.
    // We are not relying on the collation data to catch Hangul syllables.
    // Furthermore, having non-default values in this range is
    // bad for tailorings whose characters of interest are
    // below the fast-access boundary for the small trie type.
    for i in 0xD7A4..=(char::MAX as u32) {
        builder.set_value(i, trie.get32(i));
    }
    if let Some((id, root)) = id_and_root {
        let lang = id.locale.language;
        let collation_type: &str = id.marker_attributes;
        match collation_type {
            "search" | "emoji" | "eor" | "unihan" => {}
            _ => {
                if lang == language!("ja") {
                    // Hoist Hiragana
                    // Note: Since we excluded `unihan` above,
                    // we aren't hoisting for `unihan`, which is
                    // assumed to be pretty much unused. Hoisting
                    // Hiragana for `ja-u-co-unihan` would cost 368
                    // bytes in baked data.
                    log::info!("Hoist Hiragana for {:?}", id);
                    for i in 0x3041..0x30A0 {
                        if let Some(ce32) = hoistable(trie.get32(i), root, i, default_value) {
                            builder.set_value(i, ce32);
                        }
                    }
                }
                // `und` means Chinese after we've excluded `search`, `emoji`, and `eor`.
                if lang == language!("ja") || lang == language!("und") {
                    log::info!("Hoist ideographic punctuation for {:?}", id);
                    for i in 0x3000..0x3040 {
                        if let Some(ce32) = hoistable(trie.get32(i), root, i, default_value) {
                            builder.set_value(i, ce32);
                        }
                    }
                }
            }
        }
    }
    let Typed::Small(t) = builder.build().to_typed() else {
        unreachable!("The builder was configured with `TrieType::Small`");
    };
    t
}

fn convert_data_from_serde(
    data: &collator_serde::CollationData,
    id_and_root: Option<(&DataIdentifierBorrowed, &SmallCodePointTrie<u32>)>,
) -> Result<CollationData<'static>, DataError> {
    let trie = CodePointTrie::<u32>::try_from(&data.trie)
        .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;
    let rebuilt_trie = rebuild_data(trie, id_and_root);
    Ok(CollationData {
        trie: rebuilt_trie,
        contexts: ZeroVec::alloc_from_slice(&data.contexts),
        ce32s: ZeroVec::alloc_from_slice(&data.ce32s),
        ces: data.ces.iter().map(|i| *i as u64).collect(),
    })
}

impl collator_serde::CollationDiacritics {
    fn suffix() -> &'static str {
        "_dia"
    }

    #[allow(clippy::unnecessary_wraps)]
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

#[test]
fn test_collation_root_not_shared_across_providers() {
    let implicit = SourceDataProvider::new_testing();
    let unihan = implicit
        .clone()
        .with_collation_root_han(crate::CollationRootHan::Unihan);
    // The test data lacks the unihan root, so having loaded the implicit root
    // must not make a unihan root available, and the unihan failure must not
    // affect the implicit root.
    assert!(implicit.converted_collation_root().is_ok());
    assert!(unihan.converted_collation_root().is_err());
    assert!(implicit.converted_collation_root().is_ok());
}

#[test]
fn test_collation_ids_not_shared_across_root_han() {
    let locale: DataLocale = icu::locale::langid!("ar").into();
    let request = DataRequest {
        id: DataIdentifierBorrowed::for_locale(&locale),
        ..Default::default()
    };
    let implicit = SourceDataProvider::new_testing();
    let unihan = implicit
        .clone()
        .with_collation_root_han(crate::CollationRootHan::Unihan);
    // The test data lacks unihan collations, so listing them fails. That
    // failure must not leak into the implicit provider.
    assert!(DataProvider::<CollationTailoringV1>::load(&unihan, request).is_err());
    assert!(DataProvider::<CollationTailoringV1>::load(&implicit, request).is_ok());
}
