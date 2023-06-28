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
        impl_normalizer_comp_v1!($provider);
        impl_normalizer_decomp_v1!($provider);
        impl_normalizer_nfd_v1!($provider);
        impl_normalizer_nfdex_v1!($provider);
        impl_normalizer_nfkd_v1!($provider);
        impl_normalizer_nfkdex_v1!($provider);
        impl_normalizer_uts46d_v1!($provider);
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
                const NORMALIZER_COMP_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_DECOMP_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_NFD_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalDecompositionDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_NFDEX_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CanonicalDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_NFKD_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_NFKDEX_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const NORMALIZER_UTS46D_V1: icu_provider::DataKeyHash = <icu_normalizer::provider::Uts46DecompositionSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    NORMALIZER_COMP_V1 => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalCompositionsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_DECOMP_V1 => icu_provider::DataProvider::<icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_NFD_V1 => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalDecompositionDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_NFDEX_V1 => icu_provider::DataProvider::<icu_normalizer::provider::CanonicalDecompositionTablesV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_NFKD_V1 => icu_provider::DataProvider::<icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_NFKDEX_V1 => icu_provider::DataProvider::<icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    NORMALIZER_UTS46D_V1 => icu_provider::DataProvider::<icu_normalizer::provider::Uts46DecompositionSupplementV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
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
