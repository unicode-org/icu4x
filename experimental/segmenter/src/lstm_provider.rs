// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct defines for ICU4X component.

pub mod key {
    use icu_provider::ResourceKey;

    #[allow(dead_code)]
    pub const SEGMENTER_LSTM_V1: ResourceKey =
        icu_provider::resource_key!(Core, "segmenter_lstm", 1);
}

#[cfg(test)]
mod tests {
    use icu_locid::LanguageIdentifier;
    use icu_locid_macros::langid;
    use icu_provider::prelude::*;
    use icu_provider::serde::*;
    use icu_provider_fs::FsDataProvider;
    use icu_segmenter_lstm::lstm::Lstm;
    use icu_segmenter_lstm::structs::LstmDataMarker;
    use std::borrow::Cow;

    use super::key;

    fn get_request_v1(langid: LanguageIdentifier) -> DataRequest {
        DataRequest {
            resource_path: ResourcePath {
                key: key::SEGMENTER_LSTM_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid),
                },
            },
        }
    }

    #[test]
    fn test_load_lstm_data_via_data_provider() {
        let provider = FsDataProvider::try_new("./tests/testdata/json")
            .expect("Loading file from testdata directory");

        let lstm_data: DataPayload<LstmDataMarker> = (&provider as &dyn SerdeDeDataProvider)
            .load_payload(&get_request_v1(langid!("th")))
            .expect("The data should be valid")
            .take_payload()
            .expect("The data should be present");

        assert_eq!(
            lstm_data.get().model,
            Cow::<String>::Owned("Thai_codepoints_exclusive_model4_heavy".to_string()),
            "Thai's LSTM data reads correctly"
        );

        Lstm::try_new(lstm_data.get().clone()).expect("The data should be valid LSTM");
    }
}
