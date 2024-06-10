// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_normalizer_comp_v1!($provider);
        impl_normalizer_decomp_v1!($provider);
        impl_normalizer_nfd_v1!($provider);
        impl_normalizer_nfdex_v1!($provider);
        impl_normalizer_nfkd_v1!($provider);
        impl_normalizer_nfkdex_v1!($provider);
        impl_normalizer_uts46d_v1!($provider);
        impl_transliterator_rules_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalCompositionsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::CanonicalDecompositionDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalDecompositionDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalDecompositionTablesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_normalizer::provider::Uts46DecompositionSupplementV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu_experimental::transliterate::provider::TransliteratorRulesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_experimental::transliterate::provider::TransliteratorRulesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.70"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
