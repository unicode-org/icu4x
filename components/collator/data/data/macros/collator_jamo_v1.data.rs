// @generated
/// Implement [`DataProvider<CollationJamoV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_collator_jamo_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_collator::provider::CollationJamoV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_collator::provider::CollationJamoV1Marker>, icu_provider::DataError> {
                Err(icu_provider::DataErrorKind::MissingLocale)
            }
        }
    };
}
