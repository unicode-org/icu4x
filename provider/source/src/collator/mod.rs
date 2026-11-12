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
use icu::normalizer::properties::CanonicalCombiningClassMap;
use icu::normalizer::properties::CanonicalCombiningClassMapBorrowed;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;
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
    /// Returns the canonical combining class map that the collation data
    /// conversion consults.
    fn canonical_combining_class_map(&self) -> Result<&CanonicalCombiningClassMap, DataError> {
        // Loading the map rebuilds the NFD trie, which is expensive, so
        // cache the map with the icuexport source, which is the only thing
        // that it depends on.
        self.icuexport()?
            .get_or_compute("norm/canonical combining class map", || {
                CanonicalCombiningClassMap::try_new_unstable(self)
            })
    }

    /// Returns the root collation data that tailorings hoist values from.
    fn converted_collation_root(
        &self,
        ccc: CanonicalCombiningClassMapBorrowed<'_>,
    ) -> Result<&CollationData<'static>, DataError> {
        // The converted root depends on the icuexport source and on the
        // root Han setting, so cache it with the source under a key that
        // includes the latter.
        self.icuexport()?.get_or_compute(
            &format!("collation/{}/root (converted)", self.collation_root_han()),
            || {
                convert_data_from_serde(
                    self.load_toml::<collator_serde::CollationData>(Default::default(), "_data")?,
                    None,
                    ccc,
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
        let ccc = self.canonical_combining_class_map()?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.converted_collation_root(ccc.as_borrowed())
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
        let ccc = self.canonical_combining_class_map()?;
        let root_trie: &SmallCodePointTrie<u32> =
            &self.converted_collation_root(ccc.as_borrowed())?.trie;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.load_toml::<collator_serde::CollationData>(req.id, "_data")
                    .and_then(|d| {
                        convert_data_from_serde(d, Some((&req.id, root_trie)), ccc.as_borrowed())
                    })
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

fn rewrite_reorderable(
    c: u32,
    ce32: u32,
    ce32s: &mut Vec<u32>,
    ccc: &CanonicalCombiningClassMapBorrowed<'_>,
) -> u32 {
    let ce32 = CollationElement32::new(ce32);
    let tag = ce32.tag_checked();
    if let Some(ref t) = tag {
        match t {
            // Note that the "ForReorderable" cases need to pass through,
            // since we may have hoisted them from root!
            Tag::Reserved3 => {
                // TODO: Make this a real error
                panic!("Collation data contains Reserved3");
            }
            Tag::BuilderData => {
                // TODO: Make this a real error
                panic!("Collation data contains BuilderData");
            }
            Tag::LatinExpansion => {
                // TODO: Make this a real error
                panic!("Collation data contains LatinExpansion");
            }
            Tag::Hangul => {
                // TODO: Make this a real error
                panic!("Collation data contains Hangul");
            }
            _ => {}
        }
    }
    if ccc.get32_u8(c) == 0 {
        // Not reorderable.
        return ce32.bits();
    }
    if let Some(ref t) = tag {
        match t {
            Tag::LongPrimary => {
                // Replace the tag.
                return (ce32.bits() & 0xFFFF_FFF0) | (Tag::LongPrimaryForReorderable as u32);
            }
            Tag::Expansion => {
                // Replace the tag.
                return (ce32.bits() & 0xFFFF_FFF0) | (Tag::ExpansionForReorderable as u32);
            }
            Tag::Digit => {
                // TODO: Error out properly
                panic!("Digit ce32 not allowed for reorderable characters.");
            }
            _ => {
                // The primary quick-check does not examine this ce32 type.
                return ce32.bits();
            }
        }
    }
    // Simple
    if (ce32.bits() & 0xFFFF_0000) == 0 {
        // No primary. We don't need to hide this, because the primary quick-check
        // compares primaries.
        return ce32.bits();
    }
    if ce32.bits() as u16 == COMMON_SEC_AND_TER_CE32 {
        // `LongPrimaryForReorderable` implies common
        // secondary and tertiary weights, so we can rewrite this.
        return (ce32.bits() & 0xFFFF_0000) | (LONG_PRIMARY_FOR_REORDERABLE_CE32_LOW_BYTE as u32);
    }
    // There is no tag to swap. The low byte needs to be preserved, so we
    // can't add a tag. We need to hide this ce32 behind indirection.
    // As of Unicode 17, there should be 34 of these across the collations,
    // so the data size increase is acceptable.
    // Notably, combining Latin and Cyrillic letters above are here.
    // Characters that have an `Expansion32`-typed ce32 to begin with
    // tend to be compatibility characters and symbols, so there isn't
    // performance pressure to handle `Expansion32` in the quick primary
    // check.
    let len = 1; // We store one item.
    let index = ce32s.len();
    if index > MAX_INDEX {
        panic!("Index too large to fit in an Expansion32");
    }
    ce32s.push(ce32.bits());
    // See `makeCE32FromTagIndexAndLength` in ICU4C.
    ((index as u32) << 13)
        | (len << 8)
        | u32::from(SPECIAL_CE32_LOW_BYTE)
        | (Tag::Expansion32 as u32)
}

fn rebuild_data<'a>(
    trie: CodePointTrie<'a, u32>,
    id_and_root: Option<(&DataIdentifierBorrowed, &SmallCodePointTrie<u32>)>,
    ce32s: &[u32],
    ccc: CanonicalCombiningClassMapBorrowed<'_>,
) -> (SmallCodePointTrie<'a, u32>, Vec<u32>) {
    let default_value = trie.get('\u{10FFFF}');
    let mut rewritten_fast: Vec<u32> = Vec::new();
    let mut augmented_ce32s = Vec::from(ce32s);
    if let Some((id, root)) = id_and_root {
        let collation_type: &str = id.marker_attributes;
        // Only optimize the default collation and the three non-unihan Chinese collations
        if collation_type.is_empty()
            || collation_type == "pinyin"
            || collation_type == "zhuyin"
            || collation_type == "stroke"
        {
            let bound = 0x1000;
            rewritten_fast.reserve_exact(bound as usize);
            rewritten_fast.resize(bound as usize, default_value);
            for i in 0..bound {
                rewritten_fast[i as usize] = trie.get32(i);
            }
            // Copy the ASCII range
            for i in 0..0x80 {
                if rewritten_fast[i as usize] == default_value {
                    let ce32 = root.get32(i);
                    if icu::collator::is_self_contained(ce32) {
                        rewritten_fast[i as usize] = ce32;
                    }
                }
            }
            // Hoist Cyrillic base if Cyrillic tailored.
            for i in 0x0400..0x0500 {
                if rewritten_fast[i] != default_value {
                    log::info!("Hoist Cyrillic base for {:?}", id);
                    for j in 0x0410..0x0450 {
                        if rewritten_fast[j as usize] == default_value {
                            let ce32 = root.get32(j);
                            if icu::collator::is_self_contained(ce32) {
                                rewritten_fast[j as usize] = ce32;
                            }
                        }
                    }
                    break;
                }
            }
            // Hoist Armenian if Armenian tailored.
            for i in 0x0531..0x0590 {
                if rewritten_fast[i] != default_value {
                    log::info!("Hoist Armenian for {:?}", id);
                    for j in 0x0531..0x0590 {
                        if rewritten_fast[j as usize] == default_value {
                            let ce32 = root.get32(j);
                            if icu::collator::is_self_contained(ce32) {
                                rewritten_fast[j as usize] = ce32;
                            }
                        }
                    }
                    break;
                }
            }
            // Hoist Hebrew base if Hebrew tailored.
            for i in 0x0591..0x05F5 {
                if rewritten_fast[i] != default_value {
                    log::info!("Hoist Hebrew base for {:?}", id);
                    for j in 0x05D0..0x05EB {
                        if rewritten_fast[j as usize] == default_value {
                            let ce32 = root.get32(j);
                            if icu::collator::is_self_contained(ce32) {
                                rewritten_fast[j as usize] = ce32;
                            }
                        }
                    }
                    break;
                }
            }
            // Hoist Arabic if Arabic tailored.
            // TODO: Should the hoisting be narrower?
            for i in 0x0600..0x0700 {
                if rewritten_fast[i] != default_value {
                    log::info!("Hoist Arabic for {:?}", id);
                    for j in 0x0600..0x0700 {
                        if rewritten_fast[j as usize] == default_value {
                            let ce32 = root.get32(j);
                            if icu::collator::is_self_contained(ce32) {
                                rewritten_fast[j as usize] = ce32;
                            }
                        }
                    }
                    break;
                }
            }

            // We currently don't have tailorings using the Syriac, Thaana,
            // N'Ko, Samaritan, or Mandaic scripts.

            // Fill the 7-bit blocks for Brahmic scripts
            let mut i = 0x0900;
            while i < bound {
                for j in 0..0x80 {
                    if rewritten_fast[(i + j) as usize] != default_value {
                        log::info!("Hoist 7-bit block starting at {:X} for {:?}", i, id);
                        for k in 0..0x80 {
                            if rewritten_fast[(i + k) as usize] == default_value {
                                let ce32 = root.get32(i + k);
                                if icu::collator::is_self_contained(ce32) {
                                    rewritten_fast[(i + k) as usize] = ce32;
                                }
                            }
                        }
                        break;
                    }
                }
                i += 0x80;
            }
        }
    }
    let mut builder = CodePointTrieBuilder::new(
        default_value,
        trie.get32(u32::MAX),
        icu::collections::codepointtrie::TrieType::Small,
    );

    for i in 0..0xAC00 {
        if let Some(trie_val) = rewritten_fast.get(i as usize) {
            builder.set_value(
                i,
                rewrite_reorderable(i, *trie_val, &mut augmented_ce32s, &ccc),
            );
        } else {
            builder.set_value(
                i,
                rewrite_reorderable(i, trie.get32(i), &mut augmented_ce32s, &ccc),
            );
        }
    }
    // Skip 0xAC00..0xD7A4 so that Hangul syllables get the default value.
    // We are not relying on the collation data to catch Hangul syllables.
    // Furthermore, having non-default values in this range is
    // bad for tailorings whose characters of interest are
    // below the fast-access boundary for the small trie type.
    for i in 0xD7A4..=(char::MAX as u32) {
        if let Some(trie_val) = rewritten_fast.get(i as usize) {
            builder.set_value(
                i,
                rewrite_reorderable(i, *trie_val, &mut augmented_ce32s, &ccc),
            );
        } else {
            builder.set_value(
                i,
                rewrite_reorderable(i, trie.get32(i), &mut augmented_ce32s, &ccc),
            );
        }
    }
    if let Some((id, root)) = id_and_root {
        // let collation_type: &str = &id.marker_attributes;
        let lang = id.locale.language;
        if lang == language!("ja") {
            // Hoist Hiragana
            log::info!("Hoist Hiragana for {:?}", id);
            for i in 0x3041..0x30A0 {
                if trie.get32(i) == default_value {
                    let ce32 = root.get32(i);
                    if icu::collator::is_self_contained(ce32) {
                        builder
                            .set_value(i, rewrite_reorderable(i, ce32, &mut augmented_ce32s, &ccc));
                    }
                }
            }
        }
        let collation_type: &str = id.marker_attributes;
        match collation_type {
            "search" | "emoji" | "eor" | "unihan" => {}
            _ => {
                // `und` means Chinese after we've excluded `search`, `emoji`, and `eor`.
                if lang == language!("ja") || lang == language!("und") {
                    log::info!("Hoist ideographic punctuation for {:?}", id);
                    for i in 0x3000..0x3040 {
                        if trie.get32(i) == default_value {
                            let ce32 = root.get32(i);
                            if icu::collator::is_self_contained(ce32) {
                                builder.set_value(
                                    i,
                                    rewrite_reorderable(i, ce32, &mut augmented_ce32s, &ccc),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    let Typed::Small(t) = builder.build().to_typed() else {
        panic!("Must have small trie type");
    };
    (t, augmented_ce32s)
}

fn convert_data_from_serde(
    data: &collator_serde::CollationData,
    id_and_root: Option<(&DataIdentifierBorrowed, &SmallCodePointTrie<u32>)>,
    ccc: CanonicalCombiningClassMapBorrowed<'_>,
) -> Result<CollationData<'static>, DataError> {
    let trie = CodePointTrie::<u32>::try_from(&data.trie)
        .map_err(|e| DataError::custom("trie conversion").with_display_context(&e))?;
    let (rebuilt_trie, augmented_ce32s) = rebuild_data(trie, id_and_root, &data.ce32s, ccc);
    Ok(CollationData {
        trie: rebuilt_trie,
        contexts: ZeroVec::alloc_from_slice(&data.contexts),
        ce32s: ZeroVec::alloc_from_slice(&augmented_ce32s),
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
    let ccc = implicit.canonical_combining_class_map().unwrap();
    // The test data lacks the unihan root, so having loaded the implicit root
    // must not make a unihan root available, and the unihan failure must not
    // affect the implicit root.
    assert!(implicit.converted_collation_root(ccc.as_borrowed()).is_ok());
    assert!(unihan.converted_collation_root(ccc.as_borrowed()).is_err());
    assert!(implicit.converted_collation_root(ccc.as_borrowed()).is_ok());
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
