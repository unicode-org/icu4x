// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : path) => {
        impl $provider {
            #[doc(hidden)]
            #[allow(dead_code)]
            pub const MUST_USE_CREATE_PROVIDER_MACRO: () = ();
        }
        impl_list_and_v1!($provider);
        impl_list_or_v1!($provider);
        impl_list_unit_v1!($provider);
    };
}
macro_rules! impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::list::provider::AndListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::AndListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::list::provider::OrListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::OrListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::list::provider::UnitListV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::list::provider::UnitListV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
