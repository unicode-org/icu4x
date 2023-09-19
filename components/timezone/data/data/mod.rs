// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_time_zone_bcp47_to_iana_v1!($provider);
        impl_time_zone_iana_to_bcp47_v1!($provider);
        impl_time_zone_metazone_period_v1!($provider);
    };
}
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::timezone::provider::names::Bcp47ToIanaMapV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::timezone::provider::names::Bcp47ToIanaMapV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::timezone::provider::names::IanaToBcp47MapV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::timezone::provider::names::IanaToBcp47MapV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::timezone::provider::MetazonePeriodV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::timezone::provider::MetazonePeriodV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
