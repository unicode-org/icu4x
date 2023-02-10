// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;

use icu_locid::{
    extensions::unicode::Key,
    extensions_unicode_key, langid,
    subtags::{Language, Region, Script},
    LanguageIdentifier,
};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::provider::*;
use std::collections::BTreeMap;
use tinystr::TinyAsciiStr;
use writeable::Writeable;
use zerovec::{maps::ZeroMap2d, ule::UnvalidatedStr};

impl DataProvider<LocaleFallbackLikelySubtagsV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleFallbackLikelySubtagsV1Marker>, DataError> {
        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if !req.locale.is_empty() {
            return Err(DataErrorKind::ExtraneousLocale.into_error());
        }

        let likely_subtags_data: &cldr_serde::likely_subtags::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/likelySubtags.json")?;

        let metadata = DataResponseMetadata::default();
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(likely_subtags_data.into())),
        })
    }
}

impl DataProvider<LocaleFallbackParentsV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleFallbackParentsV1Marker>, DataError> {
        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if !req.locale.is_empty() {
            return Err(DataErrorKind::ExtraneousLocale.into_error());
        }

        let parents_data: &cldr_serde::parent_locales::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/parentLocales.json")?;

        let metadata = DataResponseMetadata::default();
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(parents_data.into())),
        })
    }
}

impl DataProvider<CollationFallbackSupplementV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<CollationFallbackSupplementV1Marker>, DataError> {
        // TODO(#1964): Load this data from its proper sources. For now, it is copied from:
        // https://github.com/unicode-org/icu/blob/main/tools/cldr/cldr-to-icu/build-icu-data.xml
        // as well as from CLDR XML.
        #[allow(clippy::type_complexity)]
        let parents_list: [(&UnvalidatedStr, (Language, Option<Script>, Option<Region>));
            1] = [
            ("yue".into(), (&langid!("zh-Hant")).into()), //
        ];
        let unicode_extension_defaults_list: [(Key, &UnvalidatedStr, &UnvalidatedStr); 2] = [
            (extensions_unicode_key!("co"), "zh".into(), "pinyin".into()),
            (
                extensions_unicode_key!("co"),
                "zh-Hant".into(),
                "stroke".into(),
            ),
        ];
        let data = LocaleFallbackSupplementV1 {
            parents: parents_list.into_iter().collect(),
            unicode_extension_defaults: unicode_extension_defaults_list.into_iter().collect(),
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl IterableDataProvider<LocaleFallbackLikelySubtagsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<LocaleFallbackParentsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<CollationFallbackSupplementV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl From<&cldr_serde::likely_subtags::Resource> for LocaleFallbackLikelySubtagsV1<'static> {
    fn from(source_data: &cldr_serde::likely_subtags::Resource) -> Self {
        let mut l2s = BTreeMap::<TinyAsciiStr<3>, _>::new();
        let mut lr2s = ZeroMap2d::new();
        let mut l2r = BTreeMap::<TinyAsciiStr<3>, _>::new();
        let mut ls2r = ZeroMap2d::new();

        // First collect the l2s and l2r maps
        for (minimized, maximized) in source_data
            .supplemental
            .likely_subtags
            .iter()
            // Skip "und" for vertical fallback
            .filter(|(lid, _)| !lid.language.is_empty())
            // Find language-only entries
            .filter(|(lid, _)| **lid == LanguageIdentifier::from(lid.language))
        {
            let language = minimized.language;
            let script = maximized.script.expect("maximized");
            let region = maximized.region.expect("maximized");
            if script != DEFAULT_SCRIPT {
                l2s.insert(language.into(), script);
            }
            if region != DEFAULT_REGION {
                l2r.insert(language.into(), region);
            }
        }

        // Now populate the other maps
        for (minimized, maximized) in source_data
            .supplemental
            .likely_subtags
            .iter()
            // Skip "und" for vertical fallback
            .filter(|(lid, _)| !lid.language.is_empty())
            // Find non-language-only entries
            .filter(|(lid, _)| **lid != LanguageIdentifier::from(lid.language))
        {
            let language = maximized.language;
            let script = maximized.script.expect("maximized");
            let region = maximized.region.expect("maximized");
            if minimized.script.is_some() {
                assert!(minimized.region.is_none(), "{minimized:?}");
                let region_for_lang = l2r.get(&language.into()).copied().unwrap_or(DEFAULT_REGION);
                if region != region_for_lang {
                    ls2r.insert(&language.into(), &script.into(), &region);
                }
                continue;
            }
            if minimized.region.is_some() {
                let script_for_lang = l2s.get(&language.into()).copied().unwrap_or(DEFAULT_SCRIPT);
                if script != script_for_lang {
                    lr2s.insert(&language.into(), &region.into(), &script);
                }
                continue;
            }
            unreachable!();
        }

        LocaleFallbackLikelySubtagsV1 {
            l2s: l2s.into_iter().collect(),
            lr2s,
            l2r: l2r.into_iter().collect(),
            ls2r,
        }
    }
}

impl From<&cldr_serde::parent_locales::Resource> for LocaleFallbackParentsV1<'static> {
    fn from(source_data: &cldr_serde::parent_locales::Resource) -> Self {
        let mut parents = BTreeMap::<_, (Language, Option<Script>, Option<Region>)>::new();

        for (source, target) in source_data.supplemental.parent_locales.parent_locale.iter() {
            assert!(!source.language.is_empty());
            if source.script.is_some()
                && source.region.is_none()
                && target == &LanguageIdentifier::UND
            {
                // We always fall back from language-script to und
                continue;
            }
            parents.insert(source.write_to_string(), target.into());
        }

        LocaleFallbackParentsV1 {
            parents: parents
                .iter()
                .map(|(k, v)| (<&UnvalidatedStr>::from(k.as_ref()), v))
                .collect(),
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid::{
        langid, subtags_language as language, subtags_region as region, subtags_script as script,
    };

    let provider = crate::DatagenProvider::for_test();
    let likely_subtags: DataPayload<LocaleFallbackLikelySubtagsV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(
        likely_subtags.get().l2s.get_copied(&language!("zh").into()),
        Some(script!("Hans"))
    );
    assert_eq!(
        likely_subtags
            .get()
            .lr2s
            .get_copied_2d(&language!("zh").into(), &region!("TW").into()),
        Some(script!("Hant"))
    );
    assert_eq!(
        likely_subtags.get().l2r.get_copied(&language!("zh").into()),
        Some(region!("CN"))
    );
    assert_eq!(
        likely_subtags
            .get()
            .ls2r
            .get_copied_2d(&language!("zh").into(), &script!("Hant").into()),
        Some(region!("TW"))
    );

    let parents: DataPayload<LocaleFallbackParentsV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(
        parents
            .get()
            .parents
            .get_copied("zh-Hant-MO".into())
            .map(LanguageIdentifier::from),
        Some(langid!("zh-Hant-HK"))
    );
}
