// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderInternal;
use icu_locid::langid;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use std::collections::HashSet;
use std::fmt::Debug;
use zerovec::ZeroVec;

#[derive(serde::Deserialize, Debug)]
struct SegmenterDictionaryData {
    trie_data: Vec<u16>,
}

impl DatagenProvider {
    fn load_dictionary_data(
        &self,
        req: DataRequest,
    ) -> Result<UCharDictionaryBreakDataV1<'static>, DataError> {
        let model = crate::dictionary_data_locale_to_model_name(req.locale)
            .ok_or(DataErrorKind::MissingLocale.into_error())?;

        let filename = format!("segmenter/dictionary/{model}.toml");

        let toml_data = self
            .icuexport()
            .and_then(|e| e.read_and_parse_toml::<SegmenterDictionaryData>(&filename));

        #[cfg(feature = "legacy_api")]
        #[allow(deprecated)]
        let toml_data = toml_data.or_else(|e| {
            self.source
                .icuexport_dictionary_fallback
                .as_ref()
                .ok_or(e)?
                .read_and_parse_toml(&filename)
        });

        Ok(UCharDictionaryBreakDataV1 {
            trie_data: ZeroVec::alloc_from_slice(&toml_data?.trie_data),
        })
    }
}

macro_rules! implement {
    ($marker:ident, $supported:expr) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let data = self.load_dictionary_data(req)?;
                Ok(DataResponse {
                    metadata: DataResponseMetadata::default(),
                    payload: Some(DataPayload::from_owned(data)),
                })
            }
        }

        impl IterableDataProviderInternal<$marker> for DatagenProvider {
            fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
                Ok($supported
                    .into_iter()
                    .filter_map(crate::dictionary_model_name_to_data_locale)
                    .collect())
            }
        }
    };
}

implement!(DictionaryForWordOnlyAutoV1Marker, ["cjdict"]);
implement!(
    DictionaryForWordLineExtendedV1Marker,
    ["khmerdict", "laodict", "burmesedict", "thaidict"]
);
