// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : path) => {
        impl $provider {
            #[doc(hidden)]
            #[allow(dead_code)]
            pub const MUST_USE_CREATE_PROVIDER_MACRO: () = ();
        }
        impl_collator_data_v1!($provider);
        impl_collator_dia_v1!($provider);
        impl_collator_jamo_v1!($provider);
        impl_collator_meta_v1!($provider);
        impl_collator_prim_v1!($provider);
        impl_collator_reord_v1!($provider);
    };
}
macro_rules! impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::collator::provider::CollationDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::collator::provider::CollationDiacriticsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationDiacriticsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::collator::provider::CollationJamoV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationJamoV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::collator::provider::CollationMetadataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationMetadataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::collator::provider::CollationSpecialPrimariesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationSpecialPrimariesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::collator::provider::CollationReorderingV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::collator::provider::CollationReorderingV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
