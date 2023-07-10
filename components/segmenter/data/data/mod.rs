// @generated
include!("macros.rs");
/// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
        impl_segmenter_dictionary_w_auto_v1!($provider);
        impl_segmenter_dictionary_wl_ext_v1!($provider);
        impl_segmenter_grapheme_v1!($provider);
        impl_segmenter_line_v1!($provider);
        impl_segmenter_lstm_wl_auto_v1!($provider);
        impl_segmenter_sentence_v1!($provider);
        impl_segmenter_word_v1!($provider);
    };
}
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement [`AnyProvider`](icu_provider::AnyProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                const SEGMENTER_DICTIONARY_W_AUTO_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_DICTIONARY_WL_EXT_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_GRAPHEME_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::GraphemeClusterBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_LINE_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::LineBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_LSTM_WL_AUTO_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::LstmForWordLineAutoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_SENTENCE_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::SentenceBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const SEGMENTER_WORD_V1: icu_provider::DataKeyHash = <icu_segmenter::provider::WordBreakDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    SEGMENTER_DICTIONARY_W_AUTO_V1 => icu_provider::DataProvider::<icu_segmenter::provider::DictionaryForWordOnlyAutoV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_DICTIONARY_WL_EXT_V1 => icu_provider::DataProvider::<icu_segmenter::provider::DictionaryForWordLineExtendedV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_GRAPHEME_V1 => icu_provider::DataProvider::<icu_segmenter::provider::GraphemeClusterBreakDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_LINE_V1 => icu_provider::DataProvider::<icu_segmenter::provider::LineBreakDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_LSTM_WL_AUTO_V1 => icu_provider::DataProvider::<icu_segmenter::provider::LstmForWordLineAutoV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_SENTENCE_V1 => icu_provider::DataProvider::<icu_segmenter::provider::SentenceBreakDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    SEGMENTER_WORD_V1 => icu_provider::DataProvider::<icu_segmenter::provider::WordBreakDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.61"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
