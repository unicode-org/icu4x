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
                let locale = req.locale;
                match {
                    static EN: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("Hello World") };
                    static RU: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("Привет, мир") };
                    static BN: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("ওহে বিশ\u{9cd}ব") };
                    static JA: <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::Yokeable = icu_provider::hello_world::HelloWorldV1 { message: alloc::borrow::Cow::Borrowed("こんにちは世界") };
                    match ["bn", "en", "ja", "ru"].binary_search_by(|k| locale.strict_cmp(k.as_bytes()).reverse()) {
                        Ok(i) => Ok(*unsafe { [&BN, &EN, &JA, &RU].get_unchecked(i) }),
                        Err(_) => Err(icu_provider::DataErrorKind::MissingLocale),
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_provider::hello_world::HelloWorldV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
