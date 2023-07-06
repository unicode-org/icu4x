// @generated
/// Implement [`DataProvider<HelloWorldV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_core_helloworld_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_provider::hello_world::HelloWorldV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_provider::hello_world::HelloWorldV1Marker>, icu_provider::DataError> {
                static EN: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("Hello World") };
                static RU: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("Привет, мир") };
                static BN: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("ওহে বিশ\u{9cd}ব") };
                static JA: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("こんにちは世界") };
                static VALUES: [&<icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable; 4usize] = [&BN, &EN, &JA, &RU];
                static KEYS: [&str; 4usize] = ["bn", "en", "ja", "ru"];
                if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_provider::hello_world::HelloWorldV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
