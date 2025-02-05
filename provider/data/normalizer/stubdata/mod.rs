// @generated
include!("uts46_decomposition_data_v2.rs.data");
include!("compatibility_decomposition_tables_v1.rs.data");
include!("canonical_compositions_v1.rs.data");
include!("non_recursive_decomposition_supplement_v1.rs.data");
include!("canonical_decomposition_tables_v1.rs.data");
include!("canonical_decomposition_data_v2.rs.data");
include!("compatibility_decomposition_data_v2.rs.data");
/// Marks a type as a data provider. You can then use macros like
/// `impl_core_helloworld_v1` to add implementations.
///
/// ```ignore
/// struct MyProvider;
/// const _: () = {
///     include!("path/to/generated/macros.rs");
///     make_provider!(MyProvider);
///     impl_core_helloworld_v1!(MyProvider);
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __make_provider {
    ($ name : ty) => {
        #[clippy::msrv = "1.81"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::marker::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_uts46_decomposition_data_v2!($provider);
        impl_compatibility_decomposition_tables_v1!($provider);
        impl_canonical_compositions_v1!($provider);
        impl_non_recursive_decomposition_supplement_v1!($provider);
        impl_canonical_decomposition_tables_v1!($provider);
        impl_canonical_decomposition_data_v2!($provider);
        impl_compatibility_decomposition_data_v2!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.81"]
        impl icu_provider::any::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.id.hashed() {
                    h if h == <icu::normalizer::provider::Uts46DecompositionDataV2 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::Uts46DecompositionDataV2>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CompatibilityDecompositionTablesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CompatibilityDecompositionTablesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CanonicalCompositionsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalCompositionsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::NonRecursiveDecompositionSupplementV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::NonRecursiveDecompositionSupplementV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CanonicalDecompositionTablesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalDecompositionTablesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CanonicalDecompositionDataV2 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CanonicalDecompositionDataV2>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::normalizer::provider::CompatibilityDecompositionDataV2 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::normalizer::provider::CompatibilityDecompositionDataV2>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MarkerNotFound.with_req(marker, req)),
                }
            }
        }
    };
}
