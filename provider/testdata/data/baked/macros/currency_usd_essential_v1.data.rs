// @generated
/// Implement [`DataProvider<CurrencyEssentialUsdV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_currency_usd_essential_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker>, icu_provider::DataError> {
                static RU: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("#,##0.00\u{a0}¤") } };
                static EN: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("¤#,##0.00") } };
                static FR: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("$US"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("#,##0.00\u{a0}¤") } };
                static BN: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("US$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("#,##,##0.00¤") } };
                static ES: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("US$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("#,##0.00\u{a0}¤") } };
                static AR: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("US$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("\u{200f}#,##0.00\u{a0}¤;\u{200f}-#,##0.00\u{a0}¤") } };
                static EN_001: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("US$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("¤#,##0.00") } };
                static ES_AR: <icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable = icu_singlenumberformatter::provider::CurrencyEssentialV1 { symbol: alloc::borrow::Cow::Borrowed("US$"), pattern: icu_singlenumberformatter::provider::CurrencyPattern { index: 0u8, pattern: alloc::borrow::Cow::Borrowed("¤\u{a0}#,##0.00") } };
                static VALUES: [&<icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::DataMarker>::Yokeable; 19usize] = [&AR, &AR, &BN, &BN, &EN, &EN_001, &EN_001, &ES, &ES_AR, &EN, &FR, &EN, &RU, &ES, &ES, &ES, &EN_001, &EN, &ES_AR];
                static KEYS: [&str; 19usize] = ["ar", "ar-EG", "bn", "ccp", "en", "en-001", "en-ZA", "es", "es-AR", "fil", "fr", "ja", "ru", "sr", "sr-Cyrl", "sr-Latn", "th", "tr", "und"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_singlenumberformatter::provider::CurrencyEssentialUsdV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
