// @generated
/// Implement [`DataProvider<CollationSpecialPrimariesV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_prim_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_collator::provider::CollationSpecialPrimariesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_collator::provider::CollationSpecialPrimariesV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                #[allow(unused_mut)]
                let mut metadata = icu_provider::DataResponseMetadata::default();
                match { Err(icu_provider::DataErrorKind::MissingLocale) } {
                    Ok(payload) => Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata }),
                    Err(e) => Err(e.with_req(<icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
