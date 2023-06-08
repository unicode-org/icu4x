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
        impl_collator_data_v1!($provider);
        impl_collator_dia_v1!($provider);
        impl_collator_jamo_v1!($provider);
        impl_collator_meta_v1!($provider);
        impl_collator_prim_v1!($provider);
        impl_collator_reord_v1!($provider);
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
                const COLLATOR_DATA_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const COLLATOR_DIA_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationDiacriticsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const COLLATOR_JAMO_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationJamoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const COLLATOR_META_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationMetadataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const COLLATOR_PRIM_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const COLLATOR_REORD_V1: icu_provider::DataKeyHash = <icu_collator::provider::CollationReorderingV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    COLLATOR_DATA_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    COLLATOR_DIA_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationDiacriticsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    COLLATOR_JAMO_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationJamoV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    COLLATOR_META_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationMetadataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    COLLATOR_PRIM_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationSpecialPrimariesV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    COLLATOR_REORD_V1 => icu_provider::DataProvider::<icu_collator::provider::CollationReorderingV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
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
