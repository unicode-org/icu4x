// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::{langid, locale};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use std::fmt::Debug;
use zerovec::ZeroVec;

#[derive(serde::Deserialize, Debug)]
struct SegmenterDictionaryData {
    trie_data: Vec<u16>,
}

impl crate::DatagenProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<UCharDictionaryBreakDataV1<'static>, DataError> {
        let filename = if req.locale.get_langid() == langid!("km") {
            "segmenter/dictionary/khmerdict.toml"
        } else if req.locale.get_langid() == langid!("ja") {
            "segmenter/dictionary/cjdict.toml"
        } else if req.locale.get_langid() == langid!("lo") {
            "segmenter/dictionary/laodict.toml"
        } else if req.locale.get_langid() == langid!("my") {
            "segmenter/dictionary/burmesedict.toml"
        } else if req.locale.get_langid() == langid!("th") {
            "segmenter/dictionary/thaidict.toml"
        } else {
            Err(DataErrorKind::MissingLocale.into_error())?
        };

        let toml_data: &SegmenterDictionaryData = self
            .source
            .icuexport()
            .and_then(|e| e.read_and_parse_toml(filename))
            .or_else(|e| {
                self.source
                    .icuexport_fallback()
                    .read_and_parse_toml(filename)
                    .map_err(|_| e)
            })?;

        Ok(UCharDictionaryBreakDataV1 {
            trie_data: ZeroVec::alloc_from_slice(&toml_data.trie_data),
        })
    }
}

macro_rules! implement {
    ($marker:ident, $($locale:literal),*) => {
        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let data = self.load_dictionary_data(req)?;
                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(data)),
                })
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
            // TODO(#3408): Do we actually want to filter these by the user-selected locales?
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                Ok(self.filter_data_locales(vec![$(locale!($locale).into()),*]))
            }
        }
    }
}

implement!(DictionaryForWordOnlyAutoV1Marker, "ja");
implement!(
    DictionaryForWordLineExtendedV1Marker,
    "th",
    "km",
    "lo",
    "my"
);
