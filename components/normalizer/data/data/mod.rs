// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : path) => {
        impl $provider {
            #[doc(hidden)]
            #[allow(dead_code)]
            pub const MUST_USE_CREATE_PROVIDER_MACRO: () = ();
        }
        impl_normalizer_comp_v1!($provider);
        impl_normalizer_decomp_v1!($provider);
        impl_normalizer_nfd_v1!($provider);
        impl_normalizer_nfdex_v1!($provider);
        impl_normalizer_nfkd_v1!($provider);
        impl_normalizer_nfkdex_v1!($provider);
        impl_normalizer_uts46d_v1!($provider);
    };
}
macro_rules! impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalCompositionsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CanonicalDecompositionDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalDecompositionDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CanonicalDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalDecompositionTablesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CompatibilityDecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CompatibilityDecompositionTablesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::Uts46DecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::Uts46DecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
