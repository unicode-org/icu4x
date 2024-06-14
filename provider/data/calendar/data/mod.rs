// @generated
include!("calendar_chinesecache_v1.rs.data");
include!("calendar_dangicache_v1.rs.data");
include!("calendar_islamicobservationalcache_v1.rs.data");
include!("calendar_islamicummalquracache_v1.rs.data");
include!("calendar_japanese_v1.rs.data");
include!("calendar_japanext_v1.rs.data");
include!("datetime_week_data_v1.rs.data");
include!("datetime_week_data_v2.rs.data");
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
        #[clippy::msrv = "1.70"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_calendar_chinesecache_v1!($provider);
        impl_calendar_dangicache_v1!($provider);
        impl_calendar_islamicobservationalcache_v1!($provider);
        impl_calendar_islamicummalquracache_v1!($provider);
        impl_calendar_japanese_v1!($provider);
        impl_calendar_japanext_v1!($provider);
        impl_datetime_week_data_v1!($provider);
        impl_datetime_week_data_v2!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu::calendar::provider::ChineseCacheV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::ChineseCacheV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::DangiCacheV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::DangiCacheV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::IslamicObservationalCacheV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::IslamicObservationalCacheV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::IslamicUmmAlQuraCacheV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::IslamicUmmAlQuraCacheV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::JapaneseErasV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::JapaneseErasV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::JapaneseExtendedErasV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::JapaneseExtendedErasV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::WeekDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::WeekDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::calendar::provider::WeekDataV2Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::calendar::provider::WeekDataV2Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
